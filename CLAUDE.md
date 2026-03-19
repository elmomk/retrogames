# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Retro games collection with two deployment targets:
- **Browser**: Self-contained HTML5 Canvas games in `web/<game>/index.html`
- **Miyoo Mini Plus**: Rust (Macroquad 0.4) native ports in `miyoo/<game>/`

`web/index.html` is the Arcade Launcher menu linking to all browser games.

## Current Games

- **Micro Mages** (`web/micro/`) — Platformer with physics (wall-jump, double-jump, coyote time), 8-dir shooting, destructible terrain, rising lava
- **Neon Defender** (`web/space/`) — Wave-based space shooter with power-ups, enemy types, boss fights

## Serving Locally

```bash
cd web && python3 -m http.server 8080
```

Or use the Nginx config at `web/nginx` (serves on port 8080).

## Miyoo Cross-Compilation

```bash
cd miyoo/<game>
CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
  cargo build --release --target armv7-unknown-linux-gnueabihf
```

Target: ARM Cortex-A7 (`armv7-unknown-linux-gnueabihf`), Rust Edition 2021.

## CI/CD

GitHub Actions (`.github/workflows/build-and-publish-release.yml`) auto-discovers `miyoo/*/` subdirectories, cross-compiles each in parallel, and publishes to GitHub Releases on `v*` tags.

## Architecture Notes

**Browser games** are single-file HTML with embedded JS/CSS. Key patterns:
- `requestAnimationFrame` main loop at 60 FPS fixed timestep
- Procedural pixel-art sprites defined as character arrays (e.g., `"..1111..","1221.."`) rendered to Canvas
- Touch controls (virtual joystick + buttons) alongside keyboard input

**Miyoo ports** use Data-Oriented Design (flat `Vec` arrays, no ECS framework), 640×480 logical resolution (4:3 for Miyoo screen), and the same procedural sprite system rendering to `Texture2D` buffers. See `miyoo/<game>/spec.md` and `web/<game>/spec.md` for detailed specs.
