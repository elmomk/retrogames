# Deployment Guide

*"Any improvement made after the bottleneck is useless. Any improvement made*
*before the bottleneck is useful only if the bottleneck is also improved."*
*-- Eliyahu Goldratt, The Goal*

---

## The Situation

You have a collection of retro arcade games. They need to be accessible over
HTTPS from any device on your Tailscale network. The solution is two Docker
containers, a handful of configuration files, and a deployment that takes under
60 seconds.

No Kubernetes. No load balancers. No CDN. No cloud provider console. Just
Docker, Tailscale, and shell scripts.

---

## Prerequisites

Before you begin, you need:

1. **Docker and Docker Compose** (v2.x or later)
   ```bash
   docker --version    # Docker Engine 24.x+
   docker compose version  # Docker Compose v2.x+
   ```

2. **A Tailscale account** at [login.tailscale.com](https://login.tailscale.com)

3. **A Tailscale auth key** generated from the
   [admin console](https://login.tailscale.com/admin/settings/keys):
   - Create a new auth key
   - Enable "Reusable" if you plan to redeploy frequently
   - Enable "Ephemeral" if you want the node to auto-deregister when stopped
   - Copy the key (you will only see it once)

4. **Git** and a clone of the repository:
   ```bash
   git clone <repo-url> retrogames
   cd retrogames
   ```

---

## Environment Setup

### Step 1: Create the `.env` File

The `.env` file contains the Tailscale auth key. It is gitignored and must be
created manually on each deployment machine.

```bash
echo "TS_AUTHKEY=tskey-auth-xxxxxxxxxxxx-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx" > .env
```

Replace the value with your actual Tailscale auth key.

### Step 2: Verify the Configuration Files

Three files drive the deployment:

**`Dockerfile`** -- Builds the static file server:
```dockerfile
FROM busybox:1.37

RUN adduser -D -u 1000 app
COPY web/ /srv/www/
RUN chown -R app:app /srv/www

USER app
EXPOSE 8080

CMD ["busybox", "httpd", "-f", "-p", "8080", "-h", "/srv/www"]
```

**`docker-compose.yml`** -- Orchestrates both containers:
```yaml
services:
  tailscale:
    image: tailscale/tailscale:v1.82.5
    hostname: retrogames
    environment:
      - TS_AUTHKEY=${TS_AUTHKEY}
      - TS_STATE_DIR=/var/lib/tailscale
      - TS_SERVE_CONFIG=/config/serve.json
    volumes:
      - tailscale-state:/var/lib/tailscale
      - ./ts-serve.json:/config/serve.json:ro
    cap_add:
      - NET_ADMIN
    restart: unless-stopped

  app:
    build: .
    network_mode: service:tailscale
    depends_on:
      - tailscale
    healthcheck:
      test: ["CMD", "wget", "--quiet", "--tries=1",
             "--spider", "http://127.0.0.1:8080/"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 5s
    restart: unless-stopped

volumes:
  tailscale-state:
```

**`ts-serve.json`** -- Tailscale HTTPS reverse proxy config:
```json
{
  "TCP": {
    "443": { "HTTPS": true }
  },
  "Web": {
    "${TS_CERT_DOMAIN}:443": {
      "Handlers": {
        "/": {
          "Proxy": "http://127.0.0.1:8080"
        }
      }
    }
  }
}
```

### How It Fits Together

```
Internet               Tailscale Network
   X                        |
   |                   +----v----+
(blocked)              | Tailscale|
                       | Container|
                       | :443 HTTPS
                       +----+----+
                            |
                       (shared network namespace)
                            |
                       +----v----+
                       |   App   |
                       | busybox |
                       | httpd   |
                       | :8080   |
                       +---------+
                            |
                       /srv/www/
                       (your games)
```

The `network_mode: service:tailscale` directive is the critical piece. It makes
the app container share the tailscale container's network namespace. This means:
- The app listens on `127.0.0.1:8080` inside the tailscale container's network.
- Tailscale's serve proxy forwards HTTPS requests to that address.
- From the outside, only the tailscale container is visible on the network.

---

## Building and Deploying

### First Deployment

```bash
./scripts/deploy.sh
```

This script:
1. Builds the Docker image (copies `web/` into the busybox container)
2. Starts both containers with `docker compose up -d`
3. Shows container status
4. Shows Tailscale connection status

Expected output:
```
==> Building image...
[+] Building 2.3s (8/8) FINISHED

==> Starting containers...
[+] Running 2/2
 ✓ Container retrogames-tailscale-1  Started
 ✓ Container retrogames-app-1        Started

NAME                        STATUS
retrogames-tailscale-1      Up 2 seconds
retrogames-app-1            Up 1 second

==> Tailscale status:
Startup complete, hostname: retrogames
```

### Restarting Without Rebuilding

```bash
./scripts/deploy.sh --restart
```

Use this after changing container environment variables or when containers need
a kick. Does not rebuild the image, so code changes will not be reflected.

### Rebuilding After Code Changes

```bash
./scripts/deploy.sh
```

The default (no flags) rebuilds with `--no-cache` and restarts. Always use this
after modifying files in `web/`.

### Tearing Down

```bash
./scripts/deploy.sh --down
```

Stops and removes containers. The Tailscale state volume persists, so the next
`up` will reuse the existing Tailscale identity.

---

## Monitoring

### Quick Health Check

```bash
./scripts/status.sh
```

Output:
```
=== Container Status ===
NAME                        STATUS
retrogames-tailscale-1      Up 2 hours (healthy)
retrogames-app-1            Up 2 hours (healthy)

=== Tailscale ===
retrogames   100.x.y.z   linux   -

=== Health Check ===
OK

=== Recent Errors ===
none
```

This script checks:
1. Docker container status
2. Tailscale connectivity (`tailscale status`)
3. Internal HTTP health (wget to `127.0.0.1:8080`)
4. Recent error/warning log lines

### Viewing Logs

```bash
# All containers, last 50 lines
./scripts/logs.sh

# Specific service, custom line count
./scripts/logs.sh tailscale 100
./scripts/logs.sh app 20
```

### Smoke Testing

```bash
./scripts/test.sh http://localhost:8080
```

Runs Playwright browser tests against each game:
- Loads every game page
- Checks for JavaScript errors
- Verifies canvas elements exist
- Takes screenshots to `/tmp/retrogames-test/`

Output:
```
PAGE          STATUS  DETAIL          JS ERRORS
--------------------------------------------------------------
launcher      OK      7 cards         none
micro         OK      1 canvas        none
space         OK      1 canvas        none
shadow        OK      1 canvas        none
arena         OK      1 canvas        none
dragon        OK      1 canvas        none

Screenshots saved to /tmp/retrogames-test/
```

---

## Troubleshooting

### Container won't start

**Symptom:** `docker compose ps` shows container restarting or exited.

**Diagnosis:**
```bash
./scripts/logs.sh app 30
```

**Common causes:**
- Port conflict: Another process using port 8080 inside the network namespace.
  (Unlikely with `network_mode: service:tailscale`, but check.)
- Build error: `web/` directory missing or malformed HTML.

### Tailscale won't authenticate

**Symptom:** `./scripts/status.sh` shows "tailscale container not running" or
Tailscale logs show authentication errors.

**Diagnosis:**
```bash
./scripts/logs.sh tailscale 30
```

**Common causes:**

| Log Message | Fix |
|---|---|
| `auth key expired` | Generate new key at [admin console](https://login.tailscale.com/admin/settings/keys), update `.env`, redeploy |
| `auth key not found` | Check `.env` file exists and has correct `TS_AUTHKEY=...` format |
| `failed to create TUN device` | Ensure `cap_add: NET_ADMIN` is in docker-compose.yml |

### Site loads but pages 404

**Symptom:** Launcher works but individual games return 404.

**Diagnosis:**
```bash
docker compose exec app ls /srv/www/
```

**Common causes:**
- Game directory not in `web/` at build time. Rebuild: `./scripts/deploy.sh`
- Missing trailing slash in URL. Use `/micro/` not `/micro`.

### HTTPS certificate errors

**Symptom:** Browser shows certificate warning when accessing via Tailscale
hostname.

**Diagnosis:**
```bash
docker compose exec tailscale tailscale cert retrogames
```

**Common causes:**
- Tailscale not fully authenticated yet. Wait 30 seconds.
- `ts-serve.json` misconfigured. Verify the `${TS_CERT_DOMAIN}` placeholder.
- DNS not propagated on tailnet. Try accessing by IP: `https://100.x.y.z/`

### Games work locally but not via Tailscale

**Symptom:** `curl http://127.0.0.1:8080/` works inside the container but
HTTPS via Tailscale hostname does not.

**Diagnosis:**
```bash
docker compose exec tailscale tailscale status
docker compose exec tailscale cat /config/serve.json
docker compose exec tailscale wget -qO- http://127.0.0.1:8080/ | head -5
```

**Common causes:**
- Serve config not mounted. Check volume mount in docker-compose.yml.
- `TS_SERVE_CONFIG` not set in environment.
- App container not sharing network namespace (missing `network_mode`).

---

## Security Considerations

### What Is Exposed

- The games are only accessible via your Tailscale network. They are NOT on the
  public internet.
- HTTPS is handled by Tailscale with automatic Let's Encrypt certificates.
- The busybox httpd serves only static files. No CGI, no server-side code, no
  database.

### What to Protect

- **`.env` file:** Contains the Tailscale auth key. Never commit this. It is
  gitignored by default.
- **Tailscale state volume:** Contains the node's identity. If compromised,
  someone could impersonate your node on your tailnet.
- **Docker socket:** Standard Docker security applies. The containers do not
  mount the Docker socket.

### Principle of Least Privilege

- The app container runs as a non-root user (`app`, UID 1000).
- The app container has no network capabilities (only the tailscale container
  does, via `cap_add: NET_ADMIN`).
- The tailscale container has the minimum capabilities needed for VPN operation.

---

## CI/CD Pipeline

### GitHub Actions Workflow

The workflow at `.github/workflows/build-and-publish-release.yml` handles Miyoo
binary releases:

```
Trigger: git tag push matching v*
         OR manual workflow_dispatch

Step 1: Check if miyoo/ files changed
Step 2: Discover game directories (miyoo/*/)
Step 3: Build each game in parallel:
        - Install Rust + armv7 target
        - Install gcc-arm-linux-gnueabihf
        - cargo build --release --target armv7-unknown-linux-gnueabihf
        - Upload binary as artifact
Step 4: Download all artifacts
Step 5: Publish GitHub Release with all binaries
```

### Creating a Release

```bash
git tag v1.0.0
git push origin v1.0.0
```

The pipeline auto-discovers all `miyoo/*/` directories, builds each one, and
publishes the ARM binaries as GitHub Release assets.

### Adding a New Game to CI/CD

No action needed. The pipeline uses Python to scan `miyoo/*/` for directories
containing `Cargo.toml`. Adding a new `miyoo/<game>/` directory and tagging
automatically includes it in the next release.

---

## Quick Reference

| Task | Command |
|------|---------|
| Full deploy | `./scripts/deploy.sh` |
| Restart containers | `./scripts/deploy.sh --restart` |
| Stop everything | `./scripts/deploy.sh --down` |
| Check health | `./scripts/status.sh` |
| View logs | `./scripts/logs.sh [service] [lines]` |
| Run smoke tests | `./scripts/test.sh [base-url]` |
| Check Rust compiles | `./scripts/check-rust.sh` |
| Build Miyoo binaries | `./scripts/build-miyoo.sh [game\|all] [--native\|--arm]` |
| Local dev server | `cd web && python3 -m http.server 8000` |

---

## Cross-References

- [Architecture](architecture.md) -- System overview and container architecture
- [Adding New Games](adding-games.md) -- How new games are automatically picked
  up by CI/CD
- [Skills & Agents](skills-and-agents.md) -- The `/deploy`, `/status`, and
  `/logs` slash commands
- [Miyoo Porting Guide](miyoo-porting-guide.md) -- Cross-compilation details
