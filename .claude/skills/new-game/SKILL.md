---
name: new-game
description: Scaffold a new retro game with both web and Miyoo versions
user_invocable: true
args: "<game-name> <genre> <description>"
---

Create a new retro game for both browser and Miyoo Mini Plus.

Given the game name, genre, and description from the user's arguments (or ask if not provided), do the following:

## Step 1: Create the web version
1. Create `web/<game>/spec.md` with a detailed technical specification
2. Create `web/<game>/index.html` — a complete, playable single-file HTML5 Canvas game following these patterns:
   - Press Start 2P Google font
   - requestAnimationFrame at 60fps with fixed timestep
   - Procedural pixel-art sprites from character arrays
   - Touch controls (virtual joystick + buttons) alongside keyboard
   - Scanline overlay effect
   - Web Audio API sound effects
   - Title screen, gameplay, game over, victory screens
   - Story system with typewriter text

## Step 2: Add to launcher
3. Add a game card to `web/index.html` with:
   - Appropriate color theme (--card-color, --card-glow, --card-bg)
   - Genre label, description, tag
   - Mini animated preview canvas

## Step 3: Create the Miyoo Rust port
4. Create `miyoo/<game>/Cargo.toml`:
```toml
[package]
name = "<game>_miyoo"
version = "0.1.0"
edition = "2021"

[dependencies]
macroquad = "0.4"
```
5. Create `miyoo/<game>/src/main.rs` — complete Rust/Macroquad port with:
   - 640x480, fixed 60fps timestep
   - Data-oriented design (flat Vec arrays)
   - Procedural sprites → Texture2D with FilterMode::Nearest
   - D-Pad + A(X)/B(Space)/Start(Enter) input mapping
   - CRT scanlines + vignette
   - Same gameplay and story as web version

## Step 4: Verify
6. Run `cd miyoo/<game> && cargo check` to verify compilation
7. Fix any errors until it compiles clean
8. Update CLAUDE.md game table if needed
