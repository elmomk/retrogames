# retrogames

A collection of retro-style games with two targets: a browser-based **Arcade Launcher** for quick play, and native **Miyoo Mini Plus** binaries built from Rust.

---

## Project Structure

```
retrogames/
├── web/                        # Browser frontend
│   ├── index.html              # Arcade Launcher (game selection page)
│   ├── nginx                   # Nginx config for local serving
│   └── micro/                  # Micro Mages JS prototype
│       ├── index.html          # Playable HTML5 Canvas demo
│       └── spec.md             # Tech spec for the Rust/Miyoo port
└── miyoo/                      # Rust source trees (one sub-dir per game)
    └── <game>/                 # Independent Cargo project, one binary target
```

---

## Games

| Game | Browser | Miyoo Binary |
|------|---------|--------------|
| Micro Mages | `web/micro/index.html` | `miyoo/micro_mages` (planned) |

---

## Running Locally (Browser)

Serve the `web/` folder with any static file server. An Nginx config is included:

1. Edit `web/nginx` — replace the `root` path with the absolute path to your `web/` folder.
2. Copy the config to your Nginx sites:
   ```bash
   sudo cp web/nginx /etc/nginx/sites-available/retrogames
   sudo ln -s /etc/nginx/sites-available/retrogames /etc/nginx/sites-enabled/
   sudo nginx -s reload
   ```
3. Open `http://localhost:8080` in your browser.

Or use Python's built-in server:
```bash
cd web && python3 -m http.server 8080
```

---

## Building for Miyoo Mini Plus

Each game under `miyoo/<game>/` is an independent Rust/[Macroquad](https://macroquad.rs/) project. Cross-compilation targets `armv7-unknown-linux-gnueabihf`.

**Prerequisites**
```bash
rustup target add armv7-unknown-linux-gnueabihf
sudo apt-get install gcc-arm-linux-gnueabihf
```

**Build a game manually**
```bash
cd miyoo/<game>
CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
  cargo build --release --target armv7-unknown-linux-gnueabihf
```

The binary will be at `target/armv7-unknown-linux-gnueabihf/release/<binary>`.

---

## CI / Releases

A GitHub Actions workflow (`.github/workflows/build-and-publish-release.yml`) runs automatically when:
- A tag matching `v*` is pushed **and** files under `miyoo/**` have changed, or
- The workflow is triggered manually (`workflow_dispatch`).

It discovers every sub-directory under `miyoo/`, cross-compiles each one in parallel, stages the binary as `<game>_miyoo`, and publishes all assets to the same GitHub Release.

To cut a release:
```bash
git tag v1.0.0
git push origin v1.0.0
```

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Browser prototypes | HTML5 Canvas + vanilla JavaScript |
| Miyoo native ports | Rust 2021 + Macroquad 0.4 |
| Cross-compilation | `armv7-unknown-linux-gnueabihf` via `gcc-arm-linux-gnueabihf` |
| CI/CD | GitHub Actions |
| Local web serving | Nginx / Python `http.server` |
