# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Retro games collection with two deployment targets:
- **Browser**: Self-contained HTML5 Canvas games in `web/<game>/index.html`
- **Miyoo Mini Plus**: Rust (Macroquad 0.4) native ports in `miyoo/<game>/`

`web/index.html` is the Arcade Launcher menu linking to all browser games.

## Current Games

| Game | Web Dir | Miyoo Dir | Genre | Story |
|------|---------|-----------|-------|-------|
| Micro Mages | `web/micro/` | `miyoo/micro/` | Platformer | "The Obsidian Spire" |
| Neon Defender | `web/space/` | `miyoo/space/` | Shoot 'em Up | "The Last Signal" |
| Shadow Blade | `web/shadow/` | `miyoo/shadow/` | Action Platformer | "The Crimson Oath" |
| Arena Blitz | `web/arena/` | `miyoo/arena/` | Twin-Stick Shooter | "Protocol Omega" |
| Dragon Fury | `web/dragon/` | `miyoo/dragon/` | Beat 'em Up | "Streets of Vengeance" |

## Commands

### Serving Web Games Locally
```bash
cd web && python3 -m http.server 8000
```

### Check All Rust Ports Compile
```bash
for g in micro space shadow arena dragon; do echo "$g:"; (cd miyoo/$g && cargo check 2>&1 | tail -1); done
```

### Build Native (for desktop testing)
```bash
cd miyoo/<game> && cargo build --release
# Binary: target/release/<game>_miyoo
```

### Cross-Compile for Miyoo Mini Plus
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
- Web Audio API for procedural sound effects (no audio files)
- Story system with typewriter text reveal between levels/waves

**Miyoo ports** use:
- Data-Oriented Design (flat `Vec` arrays, no ECS framework)
- 640×480 logical resolution (4:3 for Miyoo screen)
- Procedural sprite system rendering to `Texture2D` buffers
- Fixed 60fps timestep with death spiral prevention
- Index-based loops to avoid borrow checker issues with `iter_mut()` + `self` methods
- CRT scanline overlay and vignette for retro feel
- Input: D-Pad (arrows), A=KeyCode::X, B=KeyCode::Space, Start=KeyCode::Enter

## Adding a New Game

1. Create `web/<game>/index.html` — single-file HTML5 Canvas game
2. Create `web/<game>/spec.md` — technical specification
3. Add card to `web/index.html` launcher
4. Create `miyoo/<game>/Cargo.toml` with `macroquad = "0.4"`
5. Create `miyoo/<game>/src/main.rs` — Rust port
6. Run `cargo check` in `miyoo/<game>/` to verify
7. CI/CD auto-discovers new `miyoo/*/` dirs on tag push

## Common Rust Pitfalls

- **Borrow checker**: Use index-based loops (`for i in 0..self.vec.len()`) instead of `iter_mut()` when the loop body needs to call `self.method()`
- **Float ambiguity**: Annotate `let angle: f32 = ...` when calling `.cos()`/`.sin()` on `gen_range()` results
- **Match exhaustiveness**: Add new enum variants to ALL match blocks (draw + update)
