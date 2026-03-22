---
name: new-game
description: Scaffold a new retro game with both web and Miyoo versions
user_invocable: true
args: "<game-name> <genre> <description>"
---

Create a new retro game for browser and Miyoo Mini Plus. Ask for name/genre/description if not provided.

## Steps

1. **Spec**: Create `web/<game>/spec.md`
2. **Web game**: Create `web/<game>/index.html` — single-file HTML5 Canvas game with Press Start 2P font, 640x480, 60fps, procedural sprites, touch+keyboard, Web Audio, scanlines, story system
3. **Launcher card**: Add to `web/index.html` with color theme, genre label, animated preview
4. **Miyoo port**: Create `miyoo/<game>/Cargo.toml` (edition 2024, macroquad 0.4) + `src/main.rs` (640x480, 60fps, data-oriented design, D-Pad+A/B/Start mapping, CRT effects, same story)
5. **Verify**: `cd miyoo/<game> && cargo check` — fix until clean
6. **Update CLAUDE.md** game table
