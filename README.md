# Retro Arcade

A collection of retro browser games playable on any device, with TIC-80 ports for the Miyoo Mini Plus handheld. Installable as a PWA for offline play.

## Games

| Game | Genre | Web | Miyoo (TIC-80) |
|------|-------|:---:|:--------------:|
| Nano Wizards | Platformer | [Play](web/micro/) | [Port](tic80/micro/) |
| Neon Defender | Shoot 'em Up | [Play](web/space/) | [Port](tic80/space/) |
| Shadow Blade | Action Platformer | [Play](web/shadow/) | [Port](tic80/shadow/) |
| Arena Blitz | Twin-Stick Shooter | [Play](web/arena/) | [Port](tic80/arena/) |
| Dragon Fury | Beat 'em Up | [Play](web/dragon/) | [Port](tic80/dragon/) |
| Pixel Knight | Mario-like Platformer | [Play](web/mariolike/) | [Port](tic80/mariolike/) |
| Nova Evader | Bullet Hell | [Play](web/nova/) | [Port](tic80/nova/) |
| Chrome Viper | Cyberpunk Shooter | [Play](web/cyber/) | [Port](tic80/cyber/) |
| Neon Runner | Cyberpunk Platformer | [Play](web/neon/) | [Port](tic80/neon/) |

### Screenshots

| | | |
|:---:|:---:|:---:|
| ![Nano Wizards](screenshots/micro.png) | ![Neon Defender](screenshots/space.png) | ![Shadow Blade](screenshots/shadow.png) |
| Nano Wizards | Neon Defender | Shadow Blade |
| ![Arena Blitz](screenshots/arena.png) | ![Dragon Fury](screenshots/dragon.png) | ![Pixel Knight](screenshots/mariolike.png) |
| Arena Blitz | Dragon Fury | Pixel Knight |
| ![Nova Evader](screenshots/nova.png) | ![Chrome Viper](screenshots/cyber.png) | ![Neon Runner](screenshots/neon.png) |
| Nova Evader | Chrome Viper | Neon Runner |

## Features

- Single-file HTML5 Canvas games -- no dependencies, no build step
- Fullscreen mobile with floating touch joystick
- Installable as a PWA with offline caching
- Procedural pixel-art sprites and Web Audio API sound effects
- CRT scanline overlay and retro aesthetic
- All 9 games ported to TIC-80 Lua for the Miyoo Mini Plus (via tic80_libretro RetroArch core)

## Quick Start

### Play in browser
```bash
cd web && python3 -m http.server 8000
# Open http://localhost:8000
```

### Build TIC-80 cartridges
```bash
python3 tic80/build_tic.py --all          # build all 9 games
python3 tic80/build_tic.py micro          # build one game
```

No TIC-80 PRO CLI needed -- `build_tic.py` generates `.tic` binary cartridges directly from Lua source.

### Deploy to Miyoo Mini Plus (TIC-80)
```bash
sh miyoo/install/install.sh              # build & deploy all games
sh miyoo/install/install.sh --run micro  # build, deploy, and launch one
```

Requires `sshpass` and the Miyoo on the network (OnionOS with SSH enabled). Cartridges are deployed to `/mnt/SDCARD/Roms/TIC/` and run via the `tic80_libretro.so` RetroArch core.

### Deploy with Docker + Tailscale
```bash
cp .env.example .env  # Add your TS_AUTHKEY
./scripts/deploy.sh
```

## Directory Structure

```
retrogames/
├── web/              # Browser games (HTML5 Canvas, single-file)
│   ├── index.html    # Arcade launcher
│   ├── micro/        # Nano Wizards
│   ├── space/        # Neon Defender
│   ├── shadow/       # Shadow Blade
│   ├── arena/        # Arena Blitz
│   ├── dragon/       # Dragon Fury
│   ├── mariolike/    # Pixel Knight
│   ├── nova/         # Nova Evader
│   ├── cyber/        # Chrome Viper
│   ├── neon/         # Neon Runner
│   ├── sw.js         # Service worker (PWA)
│   └── manifest.json # PWA manifest
├── tic80/            # TIC-80 Miyoo ports (primary Miyoo target)
│   ├── build_tic.py  # Cartridge builder (Lua -> .tic binary)
│   ├── micro/        # Nano Wizards
│   ├── space/        # Neon Defender
│   ├── shadow/       # Shadow Blade
│   ├── arena/        # Arena Blitz
│   ├── dragon/       # Dragon Fury
│   ├── mariolike/    # Pixel Knight
│   ├── nova/         # Nova Evader
│   ├── cyber/        # Chrome Viper
│   ├── neon/         # Neon Runner
│   ├── imgs/         # TIC-80 game screenshots
│   └── test/         # TIC-80 hello world test
├── miyoo/            # Miyoo tooling + experimental Rust SDL2 ports
│   ├── install/      # OnionOS deploy scripts (install.sh)
│   ├── retro-sdl2/   # Shared Rust SDL2 rendering crate
│   ├── micro/        # Nano Wizards Rust SDL2 port (experimental)
│   └── LEARN_RUST.md # Rust tutorial (2943 lines)
├── chailove/         # Experimental ChaiLove/ChaiScript ports
│   └── LEARN_CHAILOVE.md
├── zig/              # Experimental Zig SDL2 ports
│   ├── common/       # Shared Zig SDL2 module
│   ├── micro/        # Nano Wizards Zig port
│   └── LEARN_ZIG.md  # Zig tutorial (2487 lines)
├── miniquad/         # Original Rust Macroquad ports (archived)
├── scripts/          # Automation scripts
├── docs/             # Guides and post-mortems
├── .claude/          # Claude Code skills & agents
├── Dockerfile        # busybox httpd static server
└── docker-compose.yml # Tailscale + app stack
```

## Scripts

| Script | Description |
|--------|-------------|
| `tic80/build_tic.py` | Build `.tic` cartridges from Lua sources |
| `miyoo/install/install.sh` | Build and deploy cartridges to Miyoo via SCP |
| `scripts/deploy.sh` | Build and deploy Docker containers |
| `scripts/status.sh` | Check deployment health |
| `scripts/logs.sh` | View container logs |
| `scripts/test.sh` | Run Playwright smoke tests |
| `scripts/check-rust.sh` | Cargo check all Rust ports |
| `scripts/build-miyoo.sh` | Cross-compile Rust SDL2 ports |

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Browser games | HTML5 Canvas, Web Audio API, vanilla JS |
| Miyoo ports | TIC-80 Lua via tic80_libretro RetroArch core |
| Experimental native | Rust 2024 + SDL2, Zig + SDL2 |
| Deploy | Docker (busybox httpd), Tailscale Serve |
| CI/CD | GitHub Actions |
| Testing | Playwright (headless Chromium) |
| PWA | Service worker, Web App Manifest |

## Documentation

| Document | Description |
|----------|-------------|
| [Architecture](docs/architecture.md) | System design and data flow |
| [Game Engine Patterns](docs/game-engine-patterns.md) | Game loop, sprites, physics |
| [Deployment Guide](docs/deployment-guide.md) | Docker + Tailscale setup |
| [Adding Games](docs/adding-games.md) | Step-by-step tutorial |
| [Mobile Optimization](docs/mobile-optimization.md) | Touch controls, viewport |
| [Miyoo Porting Guide](docs/miyoo-porting-guide.md) | ChaiLove and native patterns |
| [Skills and Agents](docs/skills-and-agents.md) | Claude Code automation |
| [AAR: Miyoo Porting](docs/AAR-miyoo-porting.md) | Post-mortem: 4 approaches tried |
| [Learn Rust](miyoo/LEARN_RUST.md) | Rust tutorial (2943 lines) |
| [Learn Zig](zig/LEARN_ZIG.md) | Zig tutorial (2487 lines) |
| [Learn ChaiLove](chailove/LEARN_CHAILOVE.md) | ChaiLove tutorial |

## CI/CD

GitHub Actions auto-discovers `tic80/*/` directories, builds `.tic` cartridges with `build_tic.py`, and publishes to GitHub Releases on `v*` tags. Rust cross-compilation runs when `miyoo/` files change.

```bash
git tag v3.0.0
git push origin v3.0.0
```

## License

MIT
