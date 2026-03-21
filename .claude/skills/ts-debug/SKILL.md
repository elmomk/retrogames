---
name: ts-debug
description: Debug Tailscale connectivity issues in the retrogames deployment
user_invocable: true
---

Diagnose and fix Tailscale connectivity issues for the retrogames deployment.

Run a full diagnostic:

1. **Container health**:
```bash
cd /home/mo/data/Documents/git/retrogames && docker compose ps
```

2. **Tailscale logs** (look for auth/connection errors):
```bash
docker compose logs --tail=30 tailscale
```

3. **Tailscale status**:
```bash
docker compose exec tailscale tailscale status 2>&1
```

4. **Verify serve config is mounted**:
```bash
docker compose exec tailscale cat /config/serve.json
```

5. **Test internal proxy path**:
```bash
docker compose exec tailscale wget -qO- --timeout=5 http://127.0.0.1:8080/ 2>&1 | head -5
```

6. **Check auth key** (verify .env has TS_AUTHKEY set, don't print the full key):
```bash
grep -c TS_AUTHKEY /home/mo/data/Documents/git/retrogames/.env
```

Based on findings, report:
- Whether Tailscale is authenticated and connected to the tailnet
- Whether the serve proxy is working (HTTPS → app)
- Whether the app is responding internally
- Specific fix recommendations for any issues found

Common fixes:
- Expired auth key → regenerate at Tailscale admin console, update `.env`, `docker compose down && docker compose up -d`
- Serve not working → verify `ts-serve.json` format and mount path
- App not reachable → check `network_mode: service:tailscale` in docker-compose.yml
