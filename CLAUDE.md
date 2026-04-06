# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Retro games collection with two deployment targets:
- **Browser**: Self-contained HTML5 Canvas games in `web/<game>/index.html`
- **Miyoo Mini Plus**: TIC-80 Lua cartridges in `tic80/<game>/` (run via tic80_libretro RetroArch core)

`web/index.html` is the Arcade Launcher menu linking to all browser games.

## Current Games

| Game | Web Dir | TIC-80 Dir | Genre | Story |
|------|---------|------------|-------|-------|
| Nano Wizards | `web/micro/` | `tic80/micro/` | Platformer | "The Obsidian Spire" |
| Neon Defender | `web/space/` | `tic80/space/` | Shoot 'em Up | "The Last Signal" |
| Shadow Blade | `web/shadow/` | `tic80/shadow/` | Action Platformer | "The Crimson Oath" |
| Arena Blitz | `web/arena/` | `tic80/arena/` | Twin-Stick Shooter | "Protocol Omega" |
| Dragon Fury | `web/dragon/` | `tic80/dragon/` | Beat 'em Up | "Streets of Vengeance" |
| Pixel Knight | `web/mariolike/` | `tic80/mariolike/` | Mario-like Platformer | — |
| Nova Evader | `web/nova/` | `tic80/nova/` | Bullet Hell | — |
| Chrome Viper | `web/cyber/` | `tic80/cyber/` | Cyberpunk Shooter | — |
| Neon Runner | `web/neon/` | `tic80/neon/` | Cyberpunk Platformer | — |

## Commands

### Serving Web Games Locally
```bash
cd web && python3 -m http.server 8000
```

### Build All TIC-80 Cartridges
```bash
python3 tic80/build_tic.py --all
```

### Build One TIC-80 Cartridge
```bash
python3 tic80/build_tic.py micro
```

### Deploy TIC-80 Cartridges to Miyoo Mini Plus
```bash
sh miyoo/install/install.sh              # build & deploy all
sh miyoo/install/install.sh micro        # build & deploy one game
sh miyoo/install/install.sh --run micro  # build, deploy, and launch
```

### Check All Rust Ports Compile (Experimental)
```bash
for g in micro space shadow arena dragon; do echo "$g:"; (cd miyoo/$g && cargo check 2>&1 | tail -1); done
```

### Cross-Compile Rust for Miyoo Mini Plus (Experimental)
```bash
cd miyoo/<game>
CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
  cargo build --release --target armv7-unknown-linux-gnueabihf
```

## CI/CD

GitHub Actions (`.github/workflows/build-and-publish-release.yml`) handles two pipelines:

1. **TIC-80 (primary)**: Auto-discovers `tic80/*/` directories, builds `.tic` cartridges using `build_tic.py`, publishes to GitHub Releases on `v*` tags.
2. **Miyoo Rust (experimental)**: Auto-discovers `miyoo/*/` directories with `Cargo.toml`, cross-compiles for ARM, publishes binaries.

## Architecture Notes

**Browser games** are single-file HTML with embedded JS/CSS. Key patterns:
- `requestAnimationFrame` main loop at 60 FPS fixed timestep
- Procedural pixel-art sprites defined as character arrays (e.g., `"..1111..","1221.."`) rendered to Canvas
- Touch controls (virtual joystick + buttons) alongside keyboard input
- Web Audio API for procedural sound effects (no audio files)
- Story system with typewriter text reveal between levels/waves

**TIC-80 Miyoo ports** use:
- TIC-80 Lua API: `TIC()`, `spr()`, `map()`, `btn()`, `sfx()`, `print()`
- 240x136 resolution, 16-color Sweetie 16 palette
- 8x8 sprite tiles, up to 256 sprites per bank
- Cartridges built by `tic80/build_tic.py` (no TIC-80 PRO CLI needed)
- Deployed to Miyoo via `miyoo/install/install.sh` over SCP
- Run via `tic80_libretro.so` RetroArch core on OnionOS

**Experimental Rust/Macroquad ports** (in `miyoo/`):
- Data-Oriented Design (flat `Vec` arrays, no ECS framework)
- 640x480 logical resolution (4:3 for Miyoo screen)
- Procedural sprite system rendering to `Texture2D` buffers
- Fixed 60fps timestep with death spiral prevention
- Index-based loops to avoid borrow checker issues with `iter_mut()` + `self` methods
- CRT scanline overlay and vignette for retro feel
- Input: D-Pad (arrows), A=KeyCode::X, B=KeyCode::Space, Start=KeyCode::Enter

## Adding a New Game

1. Create `web/<game>/index.html` -- single-file HTML5 Canvas game
2. Create `web/<game>/spec.md` -- technical specification
3. Add card to `web/index.html` launcher
4. Create `tic80/<game>/<game>.lua` -- TIC-80 Lua port (240x136, Sweetie 16 palette)
5. Add game to `GAMES` dict in `tic80/build_tic.py`
6. Test: `python3 tic80/build_tic.py <game>` then deploy with `install.sh`
7. CI/CD auto-discovers new `tic80/*/` dirs on tag push

## Key Scripts

| Script | Description |
|--------|-------------|
| `tic80/build_tic.py` | Build `.tic` cartridges from Lua sources (no TIC-80 CLI needed) |
| `miyoo/install/install.sh` | Build and deploy cartridges to Miyoo Mini Plus via SCP |

## Common Rust Pitfalls (Experimental Ports)

- **Borrow checker**: Use index-based loops (`for i in 0..self.vec.len()`) instead of `iter_mut()` when the loop body needs to call `self.method()`
- **Float ambiguity**: Annotate `let angle: f32 = ...` when calling `.cos()`/`.sin()` on `gen_range()` results
- **Match exhaustiveness**: Add new enum variants to ALL match blocks (draw + update)
