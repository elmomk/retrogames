# Adding New Games

*"Every piece of knowledge must have a single, unambiguous, authoritative*
*representation within a system." -- Andrew Hunt & David Thomas, The Pragmatic*
*Programmer*

---

This guide walks you through adding a new game to the retrogames collection,
from blank directory to deployed and playable on both browser and Miyoo Mini
Plus. Follow it step by step. Do not skip ahead.

---

## Before You Begin

### Naming Convention

Choose a short, lowercase name for your game directory. This name will be used
as:
- The URL path: `https://retrogames.<tailnet>/<name>/`
- The Rust crate name: `<name>_miyoo`
- The CI/CD artifact name: `miyoo-<name>`

Good names: `micro`, `space`, `shadow`, `arena`, `dragon`
Bad names: `my-cool-game`, `Game_V2`, `untitled`

### Genre and Story

Every game needs:
1. A **genre** (Platformer, Shoot 'em Up, Beat 'em Up, Twin-Stick Shooter, etc.)
2. A **story title** (e.g., "The Obsidian Spire", "The Last Signal")
3. A **narrative twist** that recontextualizes the gameplay

The story does not need to be long. It needs to be surprising.

---

## Step 1: Create the Web Version

### 1.1 Create the Directory

```bash
mkdir -p web/<name>
```

### 1.2 Write the Technical Specification

Create `web/<name>/spec.md` with a detailed technical specification covering:
- Game mechanics
- Entity types and behaviors
- Level structure
- Scoring system
- Story beats

This document is for developers (and AI agents) who will implement or modify
the game.

### 1.3 Create the Game File

Create `web/<name>/index.html`. This is a single HTML file containing
everything: HTML, CSS, JavaScript. No external dependencies except the Google
Font for the retro aesthetic.

#### Required HTML Structure

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0,
          maximum-scale=1.0, user-scalable=no, viewport-fit=cover">
    <title>Game Title - Story Subtitle</title>
    <style>
        /* See CSS Template below */
    </style>
</head>
<body>
    <canvas id="gameCanvas"></canvas>

    <!-- Touch controls (hidden on desktop) -->
    <div id="joystickZone">
        <div id="joystickBase">
            <div id="joystickStick"></div>
        </div>
    </div>
    <div class="action-btns">
        <div class="btn" id="btnB">B</div>
        <div class="btn" id="btnA">A</div>
    </div>

    <script>
        /* See JavaScript Template below */
    </script>
</body>
</html>
```

#### Required CSS Patterns

```css
/* Full viewport, no scrolling */
html, body {
    overflow: hidden; margin: 0; padding: 0;
    width: 100%; height: 100%;
    background-color: #0f0f19;
    touch-action: none;
    -webkit-touch-callout: none;
    -webkit-user-select: none;
    user-select: none;
}

/* Canvas fills screen, pixel-perfect rendering */
canvas {
    position: fixed; top: 0; left: 0;
    width: 100vw; height: 100vh;
    image-rendering: pixelated;
    padding: env(safe-area-inset-top) env(safe-area-inset-right)
             env(safe-area-inset-bottom) env(safe-area-inset-left);
    box-sizing: border-box;
}

/* Touch controls: left half = joystick, right = buttons */
#joystickZone {
    position: fixed; bottom: 0; left: 0;
    width: 50vw; height: 50vh; z-index: 10;
}

.action-btns {
    position: fixed; bottom: 30px; right: 20px;
    display: flex; gap: 18px; z-index: 11;
}

/* Hide touch controls on desktop */
@media (hover: hover) and (pointer: fine) {
    .action-btns { display: none; }
    #joystickZone { display: none; }
}
```

#### Required JavaScript Patterns

Every game script must include these sections in order:

```javascript
// 1. CANVAS SETUP
const canvas = document.getElementById('gameCanvas');
const ctx = canvas.getContext('2d', { willReadFrequently: false });
ctx.imageSmoothingEnabled = false;

// 2. PERFORMANCE DETECTION
const LOW_END = (navigator.hardwareConcurrency &&
                 navigator.hardwareConcurrency <= 4);
const MAX_PARTICLES = LOW_END ? 40 : 120;

// 3. SCALING
const GAME_W = 640, GAME_H = 480;
let renderScale = 1, offsetX = 0, offsetY = 0;
function resizeCanvas() { /* uniform scale to fit */ }
resizeCanvas();
window.addEventListener('resize', resizeCanvas);

// 4. CONSTANTS (physics, speeds, timings)

// 5. SPRITE DEFINITIONS (character arrays + createSprite)

// 6. AUDIO (Web Audio API, procedural sounds)

// 7. GAME STATE (variables, state machine)

// 8. STORY DATA (text arrays per level)

// 9. LEVEL DATA (tile maps, enemy placements)

// 10. INPUT HANDLING (keyboard + touch)

// 11. UPDATE FUNCTIONS (per state)

// 12. DRAW FUNCTIONS (per state, including CRT effects)

// 13. GAME LOOP (fixed timestep with accumulator)
```

See [Game Engine Patterns](game-engine-patterns.md) for detailed explanations
of each system.

---

## Step 2: Add to the Launcher

Edit `web/index.html` to add a game card. Each card includes:

1. **A color theme** -- `--card-color`, `--card-glow`, `--card-bg`
2. **A genre label** -- displayed as a tag
3. **A description** -- one sentence
4. **A mini preview canvas** -- animated pixel art that plays automatically

Find the game card section in `web/index.html` and add a new card following the
existing pattern. Cards are displayed in a responsive grid.

### Launcher Redirect Script

The launcher has a redirect script at the top of the file:

```javascript
if (!window.location.pathname.endsWith('/'))
    window.location.replace(window.location.pathname + '/');
```

This ensures all game URLs have a trailing slash, which is required for the
busybox httpd to serve `index.html` from subdirectories. Always use trailing
slashes in links: `href="micro/"` not `href="micro"` or `href="micro/index.html"`.

---

## Step 3: Create the Miyoo Rust Port

### 3.1 Create the Cargo Project

```bash
mkdir -p miyoo/<name>/src
```

Create `miyoo/<name>/Cargo.toml`:

```toml
[package]
name = "<name>_miyoo"
version = "0.1.0"
edition = "2021"

[dependencies]
macroquad = "0.4"
```

### 3.2 Create the Rust Source

Create `miyoo/<name>/src/main.rs`. The file follows this structure:

```rust
use macroquad::prelude::*;

// --------------- Constants ---------------
const SCREEN_W: f32 = 640.0;
const SCREEN_H: f32 = 480.0;
const TIME_STEP: f64 = 1.0 / 60.0;
// ... game-specific constants ...

// --------------- Sprite Data ---------------
const PLAYER_ART: [&str; 8] = [ /* ... */ ];
const PLAYER_COLORS: [Color; 3] = [ /* ... */ ];

// --------------- Sprite Builder ---------------
fn create_sprite(art: &[&str], colors: &[Color]) -> Texture2D {
    // Same implementation as other games
}

// --------------- Data Structures ---------------
#[derive(Clone, Copy, PartialEq)]
enum GameState { Start, Story, LevelStory, Playing, GameOver, Win }

struct Player { x: f32, y: f32, /* ... */ }
struct Enemy  { kind: EnemyKind, x: f32, y: f32, /* ... */ }
// ... other entity types ...

// --------------- World ---------------
struct World {
    state: GameState,
    player: Player,
    enemies: Vec<Enemy>,
    // ... all mutable game state ...
}

impl World {
    fn new() -> Self { /* initialize everything */ }
    fn update(&mut self, input: &mut Input) { /* game logic */ }
}

// --------------- Input ---------------
struct Input { /* keyboard state */ }
impl Input {
    fn new() -> Self { /* ... */ }
    fn poll(&mut self) { /* read Macroquad input */ }
}

// --------------- Drawing ---------------
struct Sprites { /* all Texture2D handles */ }

fn draw_world(world: &mut World, sprites: &Sprites) {
    clear_background(Color::new(0.06, 0.06, 0.1, 1.0));
    // ... draw everything based on world.state ...
    // CRT scanlines
    // Vignette
}

// --------------- Entry Point ---------------
fn window_conf() -> Conf {
    Conf {
        window_title: "Game Title".to_owned(),
        window_width: 640,
        window_height: 480,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let sprites = Sprites { /* build textures */ };
    let mut world = World::new();
    let mut input = Input::new();
    let mut accumulator: f64 = 0.0;
    let mut last_time = get_time();

    loop {
        // Fixed timestep with accumulator
        let current_time = get_time();
        let mut frame_time = current_time - last_time;
        last_time = current_time;
        if frame_time > 0.25 { frame_time = 0.25; }
        accumulator += frame_time;

        input.poll();

        while accumulator >= TIME_STEP {
            match world.state {
                GameState::Playing => world.update(&mut input),
                _ => { /* state transitions */ }
            }
            accumulator -= TIME_STEP;
        }

        draw_world(&mut world, &sprites);
        next_frame().await;
    }
}
```

### 3.3 Input Mapping

The Miyoo Mini Plus maps its physical buttons to keyboard keys:

```
+---------------------------------------------+
|   Miyoo Button    |   Macroquad KeyCode      |
|-------------------|--------------------------|
|   D-Pad Up        |   KeyCode::Up            |
|   D-Pad Down      |   KeyCode::Down          |
|   D-Pad Left      |   KeyCode::Left          |
|   D-Pad Right     |   KeyCode::Right         |
|   A Button        |   KeyCode::X             |
|   B Button        |   KeyCode::Space         |
|   Start           |   KeyCode::Enter         |
+---------------------------------------------+
```

### 3.4 Verify Compilation

```bash
cd miyoo/<name>
cargo check
```

Fix any errors. See the "Common Pitfalls" section below and the
[Miyoo Porting Guide](miyoo-porting-guide.md) for detailed solutions.

---

## Step 4: Test

### Local Web Testing

```bash
cd web && python3 -m http.server 8000
# Open http://localhost:8000/<name>/
```

### Playwright Smoke Test

```bash
./scripts/test.sh http://localhost:8000
```

### Rust Compilation Check

```bash
./scripts/check-rust.sh
```

### Native Desktop Test (Miyoo Port)

```bash
cd miyoo/<name>
cargo run --release
```

This opens a 640x480 window on your desktop, simulating the Miyoo display. Use
arrow keys for D-pad, X for A button, Space for B button, Enter for Start.

---

## Step 5: Deploy

### Web Deployment

```bash
./scripts/deploy.sh
```

The Docker image rebuilds, copying all `web/` contents. Your new game is
immediately available at `https://retrogames.<tailnet>/<name>/`.

### Miyoo Release

```bash
git add web/<name>/ miyoo/<name>/ web/index.html
git commit -m "feat: add <name> game"
git tag v1.x.0
git push origin main --tags
```

GitHub Actions automatically discovers the new `miyoo/<name>/` directory, builds
the ARM binary, and adds it to the release.

---

## Common Rust Pitfalls

These are the three errors you will encounter in every Miyoo port. Learn to
recognize and fix them quickly.

### Pitfall 1: The Borrow Checker (E0499)

**The error:**
```
error[E0499]: cannot borrow `*self` as mutable more than once at a time
  --> src/main.rs:500:17
```

**The cause:** You are iterating over `self.enemies` with `iter_mut()` while
also calling `self.spawn_particle()` inside the loop. Rust sees two mutable
borrows of `self` and refuses.

**The wrong fix:**
```rust
// DO NOT DO THIS
for enemy in self.enemies.iter_mut() {
    self.spawn_particle(enemy.x, enemy.y);  // ERROR!
}
```

**The correct fix:** Use index-based loops:
```rust
for i in 0..self.enemies.len() {
    let x = self.enemies[i].x;
    let y = self.enemies[i].y;
    self.spawn_particle(x, y);  // OK: no simultaneous borrows
}
```

Or defer side effects:
```rust
let mut spawns = Vec::new();
for enemy in &self.enemies {
    spawns.push((enemy.x, enemy.y));
}
for (x, y) in spawns {
    self.spawn_particle(x, y);
}
```

### Pitfall 2: Float Ambiguity (E0689)

**The error:**
```
error[E0689]: can't call method `cos` on ambiguous numeric type `{float}`
```

**The cause:** `rand::gen_range(0.0, 6.28)` returns an ambiguous float type.
Calling `.cos()` requires knowing the concrete type.

**The fix:** Add a type annotation:
```rust
let angle: f32 = rand::gen_range(0.0, std::f32::consts::TAU);
let vx = angle.cos() * speed;
let vy = angle.sin() * speed;
```

### Pitfall 3: Non-Exhaustive Match (E0004)

**The error:**
```
error[E0004]: non-exhaustive patterns: `GameState::LevelStory` not covered
```

**The cause:** You added a new variant to an enum but did not add it to all
`match` blocks.

**The fix:** Search the file for every `match world.state` (or `match
self.state`, `match enemy.kind`, etc.) and add the new variant to each one:

```rust
match world.state {
    GameState::Start => { /* ... */ }
    GameState::Story => { /* ... */ }
    GameState::LevelStory => { /* ... */ }  // ADD THIS
    GameState::Playing => { /* ... */ }
    GameState::GameOver => { /* ... */ }
    GameState::Win => { /* ... */ }
}
```

Use your editor's search to find ALL match sites. Missing even one will prevent
compilation.

---

## Using the /new-game Skill

If you are using Claude Code with this project, you can scaffold a new game
automatically:

```
/new-game space-runner "Shoot 'em Up" "Side-scrolling spaceship shooter with gravity wells"
```

This skill:
1. Creates `web/<name>/spec.md` with a technical specification
2. Creates `web/<name>/index.html` with a complete, playable game
3. Adds a game card to `web/index.html`
4. Creates `miyoo/<name>/Cargo.toml` and `src/main.rs`
5. Runs `cargo check` and fixes any compilation errors
6. Updates `CLAUDE.md` if needed

See [Skills & Agents](skills-and-agents.md) for all available automations.

---

## Checklist

Before marking a new game as complete, verify:

- [ ] `web/<name>/index.html` exists and is a single self-contained file
- [ ] `web/<name>/spec.md` exists with technical specification
- [ ] Game card added to `web/index.html` launcher
- [ ] `miyoo/<name>/Cargo.toml` exists with `macroquad = "0.4"`
- [ ] `miyoo/<name>/src/main.rs` exists and compiles clean
- [ ] Game has all states: Start, Story, Playing, GameOver, Win
- [ ] Touch controls work (joystick + A/B buttons)
- [ ] Keyboard controls work (arrows + Z/X or platform equivalents)
- [ ] Story text appears between levels with typewriter effect
- [ ] CRT scanlines and vignette render on both platforms
- [ ] Screen shake triggers on damage/kills
- [ ] Particle effects spawn on entity deaths and pickups
- [ ] Score popups float and fade
- [ ] `./scripts/test.sh` passes for the new game
- [ ] `./scripts/check-rust.sh` passes for all games
- [ ] `./scripts/deploy.sh` succeeds and game is accessible

---

## Cross-References

- [Architecture](architecture.md) -- Where new games fit in the system
- [Game Engine Patterns](game-engine-patterns.md) -- Detailed pattern
  documentation for all required systems
- [Miyoo Porting Guide](miyoo-porting-guide.md) -- Deep dive into Rust-specific
  porting concerns
- [Mobile Optimization](mobile-optimization.md) -- Touch control and
  performance requirements
- [Skills & Agents](skills-and-agents.md) -- The `/new-game` skill and
  `game-builder` agent
