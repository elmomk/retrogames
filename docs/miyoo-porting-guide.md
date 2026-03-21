# Miyoo Porting Guide

*"Rust's type system is like a strict but fair mentor: it will stop you from*
*doing things that seem reasonable but are actually dangerous."*
*-- Jim Blandy & Jason Orendorff, Programming Rust*

---

This guide covers everything you need to know to translate a browser game from
the `web/` directory into a native Rust binary for the Miyoo Mini Plus handheld.
It assumes you can read JavaScript and have basic Rust familiarity. It does not
assume you have fought the borrow checker before.

---

## Macroquad 0.4 Setup

### Cargo.toml

Every Miyoo port uses the same minimal `Cargo.toml`:

```toml
[package]
name = "<game>_miyoo"
version = "0.1.0"
edition = "2021"

[dependencies]
macroquad = "0.4"
```

No other dependencies. No proc macros. No build scripts. Macroquad provides
everything: window creation, input polling, 2D drawing, texture management,
and random number generation.

### The Import

Every `main.rs` starts with:

```rust
use macroquad::prelude::*;
```

This imports all Macroquad types and functions into scope. In a larger project,
you might use qualified imports for clarity. In a single-file game, the glob
import is fine.

### Window Configuration

```rust
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
    // ...
}
```

The Miyoo Mini Plus has a 640x480 screen. The window is fixed-size and
non-resizable. On desktop (for testing), this creates a 640x480 window. On the
Miyoo, it fills the screen.

### The Async Main

Macroquad's main function is `async`. This is not because the game does network
I/O -- it is because `next_frame().await` yields control back to the framework,
which handles the platform event loop and VSync timing.

```rust
loop {
    // ... update and draw ...
    next_frame().await;  // Wait for next VSync
}
```

---

## Translating JavaScript Sprites to Rust

### JavaScript Original

```javascript
const mageTex = createSprite(
    ["..1111..",".122221.","13122131","13322331",
     ".122221.","..1111..",".121121.","12211221"],
    ['#000000', '#00FFFF', '#FFFFFF']
);
```

### Rust Translation

```rust
const MAGE_ART: [&str; 8] = [
    "..1111..",
    ".122221.",
    "13122131",
    "13322331",
    ".122221.",
    "..1111..",
    ".121121.",
    "12211221",
];

const MAGE_COLORS: [Color; 3] = [
    BLACK,                              // #000000
    Color::new(0.0, 1.0, 1.0, 1.0),    // #00FFFF (Cyan)
    WHITE,                              // #FFFFFF
];
```

### Color Conversion

CSS hex colors convert to Macroquad `Color::new(r, g, b, a)` where each
component is a float from 0.0 to 1.0:

```
Hex         RGB (0-255)      Macroquad Color
------      -----------      ---------------
#000000     (0, 0, 0)        BLACK
#FFFFFF     (255, 255, 255)  WHITE
#FF0000     (255, 0, 0)      Color::new(1.0, 0.0, 0.0, 1.0)
#00FFFF     (0, 255, 255)    Color::new(0.0, 1.0, 1.0, 1.0)
#8B4513     (139, 69, 19)    Color::new(0.545, 0.271, 0.075, 1.0)

Formula: component / 255.0
```

### The Sprite Builder Function

```rust
fn create_sprite(art: &[&str], colors: &[Color]) -> Texture2D {
    let width = art[0].len() as u16;
    let height = art.len() as u16;
    let mut img = Image::gen_image_color(width, height, BLANK);

    for (y, row) in art.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch != '.' {
                if let Some(digit) = ch.to_digit(10) {
                    let idx = (digit as usize).wrapping_sub(1);
                    if idx < colors.len() {
                        img.set_pixel(x as u32, y as u32, colors[idx]);
                    }
                }
            }
        }
    }
    let tex = Texture2D::from_image(&img);
    tex.set_filter(FilterMode::Nearest);
    tex
}
```

**Critical:** `FilterMode::Nearest` must be set on every texture. Without it,
Macroquad uses bilinear filtering, which blurs pixel art when scaled up.

### Storing Textures

All textures are built at startup and stored in a `Sprites` struct:

```rust
struct Sprites {
    player: Texture2D,
    brick: Texture2D,
    enemy_patrol: Texture2D,
    enemy_bat: Texture2D,
    bullet: Texture2D,
    // ... one field per sprite
}

// In main():
let sprites = Sprites {
    player: create_sprite(&MAGE_ART, &MAGE_COLORS),
    brick: create_sprite(&BRICK_ART, &BRICK_COLORS),
    // ...
};
```

The `Sprites` struct is passed by reference to the draw function. It is never
mutated after initialization.

---

## Data-Oriented Design

### Why Flat Vecs, Not ECS

Entity Component Systems (ECS) like Bevy's are powerful but bring complexity:
dependency injection, system scheduling, archetype tables, query syntax. For a
single-file game with under 200 entities, they are overkill.

Flat `Vec` arrays give us the same cache-friendly memory layout with none of
the abstraction cost:

```rust
struct World {
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    enemy_bullets: Vec<Bullet>,
    particles: Vec<Particle>,
    gems: Vec<Gem>,
    popups: Vec<Popup>,
    dust_motes: Vec<DustMote>,
}
```

### Why Not Trait Objects

```rust
// DO NOT DO THIS in these games
let entities: Vec<Box<dyn Entity>> = vec![
    Box::new(PatrolEnemy::new()),
    Box::new(FlyingEnemy::new()),
];
```

Trait objects (dynamic dispatch) have three costs:
1. **Heap allocation** for each entity via `Box`.
2. **Vtable indirection** for every method call.
3. **Cache misses** because entities are scattered across the heap.

Instead, use a `kind` field and `match`:

```rust
#[derive(Clone, Copy, PartialEq)]
enum EnemyKind { Patrol, Bat, Turret }

struct Enemy {
    kind: EnemyKind,
    x: f32, y: f32,
    vx: f32, vy: f32,
    // ... shared fields
}

// In update:
for i in 0..self.enemies.len() {
    match self.enemies[i].kind {
        EnemyKind::Patrol => { /* walk back and forth */ }
        EnemyKind::Bat    => { /* sine wave flight */ }
        EnemyKind::Turret => { /* stationary, shoots */ }
    }
}
```

This gives identical behavior with contiguous memory layout, zero allocations,
and branch prediction instead of vtable lookups.

---

## The Borrow Checker: A Practical Guide

The borrow checker is the primary obstacle when porting JavaScript games to
Rust. JavaScript lets you mutate anything from anywhere. Rust does not. Here
are the three patterns you will encounter and how to handle each one.

### Pattern 1: Index-Based Loops

**The JavaScript:**
```javascript
for (const enemy of enemies) {
    if (overlaps(bullet, enemy)) {
        spawnParticles(enemy.x, enemy.y);  // Mutates particles array
        score += 100;                       // Mutates game state
    }
}
```

**The Wrong Rust:**
```rust
for enemy in self.enemies.iter_mut() {
    if overlaps(bullet, enemy) {
        self.spawn_particles(enemy.x, enemy.y);  // ERROR!
        // Cannot borrow `self` mutably while `self.enemies` is borrowed
    }
}
```

**The Correct Rust:**
```rust
for i in 0..self.enemies.len() {
    let ex = self.enemies[i].x;
    let ey = self.enemies[i].y;
    if overlaps_bullet(bullet, &self.enemies[i]) {
        self.spawn_particles(ex, ey);  // OK: no outstanding borrows
        self.score += 100;
    }
}
```

The key insight: extracting data into local variables (`ex`, `ey`) before
calling `self.method()` releases the borrow on `self.enemies[i]`. The method
call then has exclusive access to `self`.

### Pattern 2: Deferred Side Effects

When the loop body needs to both read from and write to the same collection:

**The Problem:**
```rust
// We want to remove dead enemies AND spawn particles for each dead one
// But we can't modify the Vec while iterating it
```

**The Solution:**
```rust
// Collect indices to remove and particles to spawn
let mut dead_indices = Vec::new();
let mut particle_spawns = Vec::new();

for i in 0..self.enemies.len() {
    if self.enemies[i].hp <= 0 {
        dead_indices.push(i);
        particle_spawns.push((self.enemies[i].x, self.enemies[i].y));
    }
}

// Apply removals (reverse order to preserve indices)
for &i in dead_indices.iter().rev() {
    self.enemies.swap_remove(i);
}

// Spawn particles
for (x, y) in particle_spawns {
    self.spawn_particles(x, y);
}
```

### Pattern 3: The Retain Pattern

For simple removal based on a condition:

```rust
self.bullets.retain(|b| {
    b.x > 0.0 && b.x < SCREEN_W &&
    b.y > 0.0 && b.y < SCREEN_H
});

self.particles.retain(|p| p.life > 0.0);
```

`retain()` is Rust's equivalent of JavaScript's `filter()`. It keeps elements
where the closure returns `true`.

### Why This Matters

In JavaScript, `enemies.forEach(e => { particles.push(...); })` is fine because
JavaScript has no concept of borrow exclusivity. Everything is reference-counted
and garbage-collected.

In Rust, `self.enemies.iter()` borrows `self.enemies`, and
`self.particles.push()` borrows `self.particles`. Since both fields live in
`self`, Rust conservatively considers both as borrows of `self`, which conflicts.

The borrow checker is not being pedantic. It is preventing data races that are
real (though rare) bugs in JavaScript. In Rust, if it compiles, there are no
data races. Period.

---

## Input Mapping for Miyoo Mini Plus

### The Physical Layout

```
        +-----------------------------------+
        |            MIYOO MINI PLUS        |
        |   +---------------------------+   |
        |   |                           |   |
        |   |        640 x 480          |   |
        |   |         SCREEN            |   |
        |   |                           |   |
        |   +---------------------------+   |
        |                                   |
        |   D-PAD          [X] [Y]          |
        |  +----+                           |
        |  |U   |          [A] [B]          |
        |  |L R |                           |
        |  |D   |    [SEL]  [START]         |
        |  +----+                           |
        +-----------------------------------+
```

### Macroquad Key Mapping

```rust
struct Input {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    jump_pressed: bool,
    shoot_pressed: bool,
    start_pressed: bool,
    jump_buffer: i32,
}

impl Input {
    fn poll(&mut self) {
        self.left = is_key_down(KeyCode::Left);
        self.right = is_key_down(KeyCode::Right);
        self.up = is_key_down(KeyCode::Up);
        self.down = is_key_down(KeyCode::Down);

        // A button = X key (jump/confirm in most games)
        self.jump_pressed = is_key_pressed(KeyCode::X);

        // B button = Space (shoot/secondary action)
        self.shoot_pressed = is_key_pressed(KeyCode::Space);

        // Start = Enter
        self.start_pressed = is_key_pressed(KeyCode::Enter);

        if self.jump_pressed {
            self.jump_buffer = 6;  // Buffer for 6 frames
        } else if self.jump_buffer > 0 {
            self.jump_buffer -= 1;
        }
    }
}
```

### `is_key_down` vs `is_key_pressed`

- `is_key_down(key)` -- Returns `true` every frame the key is held. Use for
  continuous actions: movement, holding to charge.
- `is_key_pressed(key)` -- Returns `true` only on the frame the key transitions
  from up to down. Use for discrete actions: jumping, shooting, menu selection.

Getting this wrong is a common bug:

```rust
// BUG: Player jumps every frame the button is held
if is_key_down(KeyCode::X) { player.vy = JUMP_FORCE; }

// CORRECT: Player jumps once per press
if is_key_pressed(KeyCode::X) { player.vy = JUMP_FORCE; }
```

---

## CRT Effects

### Scanlines

Horizontal lines drawn every 4 pixels at low opacity:

```rust
// After all game drawing, before next_frame()
let scanline_color = Color::new(0.0, 0.0, 0.0, 0.15);
let mut y = 0.0_f32;
while y < SCREEN_H {
    draw_line(0.0, y, SCREEN_W, y, 1.0, scanline_color);
    y += 4.0;
}
```

This creates the look of interlaced CRT scanlines:

```
Pixel row 0:  ==============================  (game pixel)
Pixel row 1:  ==============================  (game pixel)
Pixel row 2:  ==============================  (game pixel)
Pixel row 3:  ==============================  (game pixel)
Pixel row 4:  -----DARK LINE (alpha 0.15)----  (scanline)
Pixel row 5:  ==============================  (game pixel)
Pixel row 6:  ==============================  (game pixel)
Pixel row 7:  ==============================  (game pixel)
Pixel row 8:  -----DARK LINE (alpha 0.15)----  (scanline)
```

### Vignette

A radial darkening effect at the screen edges:

```rust
// Draw vignette as concentric rectangles with increasing opacity
let steps = 20;
for i in 0..steps {
    let t = i as f32 / steps as f32;
    let alpha = t * t * 0.5;  // Quadratic falloff
    let inset = (1.0 - t) * 80.0;

    draw_rectangle_lines(
        inset, inset,
        SCREEN_W - inset * 2.0, SCREEN_H - inset * 2.0,
        4.0,
        Color::new(0.0, 0.0, 0.0, alpha),
    );
}
```

Alternative approach using corner rectangles:

```rust
let vignette_alpha = 0.4;
let vignette_size = 100.0;

// Top-left corner
for i in 0..20 {
    let a = vignette_alpha * (1.0 - i as f32 / 20.0);
    draw_rectangle(0.0, 0.0,
        vignette_size * (1.0 - i as f32 / 20.0),
        vignette_size * (1.0 - i as f32 / 20.0),
        Color::new(0.0, 0.0, 0.0, a));
}
// ... repeat for other corners
```

---

## Cross-Compilation for ARM

### Prerequisites

```bash
# Install the ARM cross-compiler
sudo apt-get install gcc-arm-linux-gnueabihf

# Add the Rust target
rustup target add armv7-unknown-linux-gnueabihf
```

### Building

```bash
cd miyoo/<game>

CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
  cargo build --release --target armv7-unknown-linux-gnueabihf
```

The binary is at:
```
target/armv7-unknown-linux-gnueabihf/release/<game>_miyoo
```

### Using the Build Script

```bash
# Build all games for ARM
./scripts/build-miyoo.sh all --arm

# Build one game natively (for desktop testing)
./scripts/build-miyoo.sh micro --native
```

### Installing on the Miyoo

1. Copy the binary to the Miyoo's SD card (typically `/mnt/SDCARD/App/`)
2. Create a launch script or add to the device's app menu
3. The binary runs fullscreen at 640x480

### Common Cross-Compilation Errors

**Error: `linker 'cc' not found`**
```
You forgot to set the CARGO_TARGET_...LINKER environment variable.
```

**Error: `cannot find -lGL`**
```
The ARM cross-compiler cannot find OpenGL libraries.
Macroquad should handle this, but if it occurs:
sudo apt-get install libgles2-mesa-dev:armhf
```

**Error: `undefined reference to 'dlopen'`**
```
The ARM sysroot is missing libdl. Try:
sudo apt-get install libc6-dev-armhf-cross
```

---

## Common Compile Errors and Fixes

### Error: `type annotations needed`

```
error[E0282]: type annotations needed
  --> src/main.rs:500:21
   |
500|     let angle = rand::gen_range(0.0, 6.28);
   |         ^^^^^ consider giving `angle` a type
```

**Fix:** Add `: f32` to the variable:
```rust
let angle: f32 = rand::gen_range(0.0, std::f32::consts::TAU);
```

### Error: `temporary value dropped while borrowed`

```
error[E0716]: temporary value dropped while borrowed
  --> src/main.rs:50:20
   |
50 |     let text = format!("{}", score).as_str();
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^ - temporary value is freed at end of statement
```

**Fix:** Bind the `String` first, then borrow:
```rust
let text = format!("{}", score);
draw_text(&text, x, y, size, color);
```

### Error: `cannot move out of index`

```
error[E0507]: cannot move out of index of `Vec<Enemy>`
```

**Fix:** Clone the value, or work with a reference:
```rust
// Instead of: let enemy = self.enemies[i];
let enemy = &self.enemies[i];        // Borrow
// or
let enemy = self.enemies[i].clone();  // Clone (if Clone is derived)
```

### Error: `no method named cos found for type {float}`

```
error[E0689]: can't call method `cos` on ambiguous numeric type `{float}`
```

**Fix:** Annotate the float type:
```rust
let angle: f32 = rand::gen_range(0.0, 6.28);
let vx = angle.cos();
```

### Error: `use of moved value`

```
error[E0382]: use of moved value: `player`
```

**Fix:** This usually means you passed ownership somewhere and then tried to
use the value again. Use a reference instead:
```rust
// Instead of: draw_player(player);
draw_player(&player);
```

### Error: `mismatched types: expected f32, found f64`

Macroquad uses `f32` for all coordinates and colors. Rust float literals default
to `f64`. Fix by adding `_f32` suffix or using `as f32`:

```rust
// ERROR:
let x = 3.14;  // f64 by default

// FIX:
let x = 3.14_f32;
// or
let x = 3.14 as f32;
// or
let x: f32 = 3.14;
```

---

## Translation Cheat Sheet

| JavaScript | Rust (Macroquad) |
|---|---|
| `Math.random()` | `rand::gen_range(0.0f32, 1.0)` |
| `Math.floor(x)` | `x.floor()` or `x as i32` |
| `Math.ceil(x)` | `x.ceil()` |
| `Math.abs(x)` | `x.abs()` |
| `Math.min(a, b)` | `a.min(b)` or `f32::min(a, b)` |
| `Math.max(a, b)` | `a.max(b)` or `f32::max(a, b)` |
| `Math.sqrt(x)` | `x.sqrt()` |
| `Math.cos(x)` | `x.cos()` (on f32) |
| `Math.sin(x)` | `x.sin()` (on f32) |
| `Math.PI` | `std::f32::consts::PI` |
| `Math.PI * 2` | `std::f32::consts::TAU` |
| `array.push(item)` | `vec.push(item)` |
| `array.filter(fn)` | `vec.retain(fn)` |
| `array.length` | `vec.len()` |
| `array.splice(i, 1)` | `vec.remove(i)` or `vec.swap_remove(i)` |
| `ctx.fillRect(x,y,w,h)` | `draw_rectangle(x, y, w, h, color)` |
| `ctx.drawImage(img,x,y,w,h)` | `draw_texture_ex(tex, x, y, params)` |
| `ctx.fillText(text,x,y)` | `draw_text(text, x, y, size, color)` |
| `requestAnimationFrame` | `next_frame().await` in a `loop` |
| `Date.now()` | `get_time()` (returns f64 seconds) |
| `console.log()` | `println!()` or `eprintln!()` |

---

## Cross-References

- [Architecture](architecture.md) -- How Miyoo ports fit in the project
  structure
- [Game Engine Patterns](game-engine-patterns.md) -- Detailed pattern docs
  applicable to both platforms
- [Adding New Games](adding-games.md) -- Step-by-step for creating a new port
- [Skills & Agents](skills-and-agents.md) -- The `/check-rust` and
  `/build-miyoo` skills, and the `rust-fixer` agent
