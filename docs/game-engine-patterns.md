# Game Engine Patterns

*"Premature abstraction is the root of all evil in game development."*
*-- Adapted from Robert Nystrom, Game Programming Patterns*

---

This document dissects the design patterns used across all games in the
retrogames collection. These are not theoretical patterns borrowed from a
textbook. They are working patterns extracted from real games that run in
browsers and on ARM hardware.

---

## The Game Loop

The game loop is the heartbeat of every game. Get it wrong, and your game runs
at different speeds on different machines. Get it right, and physics behaves
identically whether you are running on a gaming PC at 144 Hz or on a Miyoo Mini
Plus at 60 Hz.

### The Problem

`requestAnimationFrame` does not guarantee consistent timing. A browser tab in
the background throttles to 1 FPS. A garbage collection pause can eat 50ms. If
you tie physics to frame rate, your character jumps twice as high on a fast
machine and falls through the floor on a slow one.

### The Solution: Fixed Timestep with Accumulator

Every game in this project uses the same pattern:

```javascript
// JavaScript (Browser)
const TIMESTEP = 1000 / 60;  // 16.667ms per tick
let accumulator = 0;
let lastTime = 0;

function gameLoop(timestamp) {
    let delta = timestamp - lastTime;
    lastTime = timestamp;

    // Death spiral prevention: cap delta
    if (delta > 250) delta = 250;

    accumulator += delta;

    while (accumulator >= TIMESTEP) {
        update();              // Fixed-rate physics
        accumulator -= TIMESTEP;
    }

    draw();                    // Render at display rate
    requestAnimationFrame(gameLoop);
}
```

```rust
// Rust (Miyoo)
const TIME_STEP: f64 = 1.0 / 60.0;

let mut accumulator: f64 = 0.0;
let mut last_time = get_time();

loop {
    let current_time = get_time();
    let mut frame_time = current_time - last_time;
    last_time = current_time;

    if frame_time > 0.25 { frame_time = 0.25; }
    accumulator += frame_time;

    input.poll();

    while accumulator >= TIME_STEP {
        world.update(&mut input);
        accumulator -= TIME_STEP;
    }

    draw_world(&mut world, &sprites);
    next_frame().await;
}
```

### Death Spiral Prevention

The `if (delta > 250)` guard is critical. Without it, imagine this sequence:

1. Browser tab loses focus for 10 seconds.
2. Tab regains focus. `delta` is 10,000ms.
3. The loop tries to run 600 update ticks before rendering.
4. Each tick takes time, so the next frame's delta is even larger.
5. The game freezes permanently -- a "death spiral."

By capping delta at 250ms (roughly 4 frames of lag), we accept a brief slowdown
instead of a catastrophic freeze.

### Why Not Variable Timestep?

Variable timestep (`position += velocity * delta`) is simpler but fragile:

- Collision detection misses at low frame rates (tunneling).
- Physics behavior varies subtly between machines.
- Replay systems become impossible.
- Jump height depends on frame rate.

Fixed timestep eliminates all of these problems at the cost of a few extra lines
of code.

---

## The Sprite System

### Character Arrays: Art as Data

Every sprite in this project is defined as an array of strings, where each
character maps to a color index:

```
Character Array          Rendered Result
+--------+              +--------+
|..1111..|              |  ####  |
|.122221.|              | #@@@@# |
|13122131|              |#.#@@#.#|
|13322331|              |#..@@..#|
|.122221.|              | #@@@@# |
|..1111..|              |  ####  |
|.121121.|              | #@##@# |
|12211221|              |#@@##@@#|
+--------+              +--------+

'.' = transparent
'1' = color[0] (e.g., BLACK)
'2' = color[1] (e.g., CYAN)
'3' = color[2] (e.g., WHITE)
```

This is the same approach used by classic 8-bit games, where sprites were
defined as bitmaps in ROM. The advantages are substantial:

- **No external assets.** No PNG files to load, no CORS issues, no CDN.
- **Readable in source.** You can see the shape by reading the code.
- **Easy to modify.** Change a character, change a pixel.
- **Tiny size.** An 8x8 sprite is 72 bytes of string data.

### Browser Implementation

```javascript
function createSprite(art, colors) {
    const size = 8;
    const offscreen = document.createElement('canvas');
    offscreen.width = size;
    offscreen.height = size;
    const octx = offscreen.getContext('2d');

    for (let y = 0; y < size; y++) {
        for (let x = 0; x < size; x++) {
            const ch = art[y][x];
            if (ch !== '.') {
                octx.fillStyle = colors[parseInt(ch, 10) - 1];
                octx.fillRect(x, y, 1, 1);
            }
        }
    }
    return offscreen;  // Reusable as drawImage source
}
```

The returned canvas is used as a source for `ctx.drawImage()`, which is hardware
accelerated on all modern browsers. The game draws scaled-up versions of these
tiny canvases with `image-rendering: pixelated` to maintain the pixel-art
aesthetic.

### Rust Implementation

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
    tex.set_filter(FilterMode::Nearest);  // Pixel-perfect scaling
    tex
}
```

`FilterMode::Nearest` is essential. Without it, Macroquad's default bilinear
filtering will blur the pixels when scaled up, destroying the retro look.

### Sprite Atlasing: Why We Don't

These games do not use sprite atlases or sprite sheets. Each sprite is its own
tiny texture. On modern hardware, the overhead of switching between 8x8 textures
is negligible for our entity counts (typically under 100 draw calls per frame).
The simplicity of individual textures outweighs the minor performance gain of
atlasing.

---

## The State Machine

Every game follows a finite state machine pattern for managing game flow:

```
                    +-------+
           +------->| START |
           |        +---+---+
           |            |
           |            | (Press Start)
           |            v
           |        +-------+
           |        | STORY |  (Typewriter intro text)
           |        +---+---+
           |            |
           |            | (Text complete + keypress)
           |            v
      +----+---+    +--------+
      |GAMEOVER|<---|PLAYING |<--------+
      +----+---+    +---+----+         |
           |            |              |
           |            | (Level end)  |
           |            v              |
           |     +------+------+       |
           |     |LEVEL_STORY  |-------+
           |     +-------------+
           |
           |    (Press Start to retry)
           +------------+
                        |
                    +---v--+
                    | START |
                    +------+
```

### JavaScript Implementation

```javascript
let gameState = 'START';

function update() {
    switch (gameState) {
        case 'START':
            updateTitleScreen();
            if (startPressed) gameState = 'STORY';
            break;
        case 'STORY':
            updateTypewriter();
            if (storyComplete && actionPressed) gameState = 'PLAYING';
            break;
        case 'PLAYING':
            updateGameplay();
            break;
        case 'GAMEOVER':
            if (startPressed) resetGame();
            break;
    }
}
```

### Rust Implementation

```rust
#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Start,
    Story,
    LevelStory,
    Playing,
    GameOver,
    Win,
}

// In the game loop:
while accumulator >= TIME_STEP {
    match world.state {
        GameState::Playing => world.update(&mut input),
        GameState::Start => {
            if input.start_pressed {
                world.show_story(false);
            }
        }
        // ... etc
    }
    accumulator -= TIME_STEP;
}
```

### Why Not a State Stack?

Some game engines use a state stack (push/pop) for things like pause menus,
inventory screens, or dialogue overlays. Our games do not need this complexity.
Each game has at most 6 states, transitions are linear, and there is no concept
of "pausing" a state while another runs on top. A flat enum is simpler, faster,
and easier to reason about.

---

## Entity Management: Data-Oriented Design

### The Problem with OOP Entities

Traditional object-oriented game engines create class hierarchies:

```
Entity
  +-- Character
  |     +-- Player
  |     +-- Enemy
  |           +-- PatrolEnemy
  |           +-- FlyingEnemy
  |           +-- TurretEnemy
  +-- Projectile
  +-- Pickup
```

This creates deep coupling, virtual dispatch overhead, cache-hostile memory
layouts, and borrow checker nightmares in Rust.

### The Solution: Flat Vectors

Every game in this project uses flat arrays of plain data structures:

```rust
struct Enemy {
    kind: EnemyKind,
    x: f32, y: f32,
    w: f32, h: f32,
    vx: f32, vy: f32,
    start_x: f32,
    range: f32,
    shoot_timer: f32,
}

struct Bullet {
    x: f32, y: f32,
    w: f32, h: f32,
    vx: f32, vy: f32,
}

struct World {
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    enemy_bullets: Vec<Bullet>,
    particles: Vec<Particle>,
    gems: Vec<Gem>,
    popups: Vec<Popup>,
}
```

Behavior differences between enemy types are handled with a simple `match` on
the `kind` field, not with virtual dispatch:

```rust
match enemy.kind {
    EnemyKind::Patrol => { /* walk back and forth */ }
    EnemyKind::Bat    => { /* fly in sine wave */ }
    EnemyKind::Turret => { /* stand still, shoot */ }
}
```

### Why This Works

1. **Cache-friendly.** All enemies are contiguous in memory. Iterating them is
   a linear scan, which modern CPUs handle at near-memory-bandwidth speeds.

2. **Borrow-checker friendly.** No `Rc<RefCell<>>`, no `Arc<Mutex<>>`, no
   lifetimes threading through object graphs. Just owned data in flat vectors.

3. **Easy to serialize.** Want to save game state? Serialize the vectors.
   No object graph traversal needed.

4. **Easy to debug.** Print the vector length. Print a single element's fields.
   No virtual dispatch to trace through.

### Removal Pattern

Entities are removed using `retain()` in Rust or `filter()` in JavaScript:

```rust
self.bullets.retain(|b| {
    b.x > 0.0 && b.x < SCREEN_W && b.y > 0.0 && b.y < SCREEN_H
});
```

```javascript
bullets = bullets.filter(b =>
    b.x > 0 && b.x < GAME_W && b.y > 0 && b.y < GAME_H
);
```

No "dead" flags. No free lists. No object pools. For our entity counts
(typically under 200), allocation and deallocation are negligible.

---

## Physics & Collision

### AABB Collision

All collision detection uses Axis-Aligned Bounding Boxes:

```javascript
function overlaps(ax, ay, aw, ah, bx, by, bw, bh) {
    return ax < bx + bw && ax + aw > bx &&
           ay < by + bh && ay + ah > by;
}
```

```rust
fn overlaps(ax: f32, ay: f32, aw: f32, ah: f32,
            bx: f32, by: f32, bw: f32, bh: f32) -> bool {
    ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
}
```

No broad phase. No spatial hashing. No quad trees. With fewer than 200 entities,
brute-force O(n*m) collision checks take microseconds.

### Platformer Physics

The platformer games (Nano Wizards, Shadow Blade) implement a full platformer
physics suite:

```
Constants:
  GRAVITY         = 0.35      Pixels per frame^2 (downward)
  MAX_FALL_SPEED  = 7.0       Terminal velocity
  MOVE_SPEED      = 3.5       Horizontal pixels per frame
  JUMP_FORCE      = -7.0      Initial upward velocity
  WALL_SLIDE_SPEED = 1.5      Reduced fall speed on walls
  WALL_JUMP_Y     = -6.5      Wall jump vertical force
  WALL_JUMP_X     = 6.0       Wall jump horizontal kick
```

### Coyote Time

Coyote time gives the player a few frames of "grace period" after walking off a
ledge, during which they can still jump. This is one of the most important
game-feel techniques in platformers:

```
+--------+
|        |  Frame 0: Player walks off edge
|  PLAYER|  on_ground = false
|        |  coyote_frames = COYOTE_MAX (6)
+--------+
         \
          \  Frames 1-6: Falling, but coyote_frames > 0
           \             Player CAN still jump!
            \
             \  Frame 7: coyote_frames = 0
              \           Too late. Must wall-jump or die.
               v
```

```javascript
if (player.onGround) {
    coyoteFrames = 6;
} else if (coyoteFrames > 0) {
    coyoteFrames--;
}

// Jump allowed if grounded OR within coyote window
if (jumpPressed && (player.onGround || coyoteFrames > 0)) {
    player.vy = JUMP_FORCE;
}
```

### Jump Buffering

The complement to coyote time. If the player presses jump slightly before
landing, the jump is "buffered" and executes the moment they touch ground:

```javascript
if (jumpPressed) {
    jumpBufferFrames = 6;
} else if (jumpBufferFrames > 0) {
    jumpBufferFrames--;
}

if (player.onGround && jumpBufferFrames > 0) {
    player.vy = JUMP_FORCE;
    jumpBufferFrames = 0;
}
```

Together, coyote time and jump buffering make platformer controls feel
responsive and forgiving without being imprecise.

---

## Camera Systems

### Smooth Follow

The camera in vertically-scrolling games uses a smooth follow algorithm:

```javascript
// Smooth camera follow with lookahead
const targetY = player.y - GAME_H * 0.4;
cameraY += (targetY - cameraY) * 0.08;
```

The `0.08` lerp factor creates a smooth, slightly delayed follow that prevents
jarring camera snaps when the player moves quickly.

### Parallax Backgrounds

Background layers move at different rates to create depth:

```javascript
// Background tiles scroll at half camera speed
const bgOffsetY = cameraY * 0.5;
for (let y = startY; y < endY; y += tileSize) {
    for (let x = 0; x < GAME_W; x += tileSize) {
        ctx.drawImage(bgTex, x, y - bgOffsetY);
    }
}
```

### Camera Bounds

The camera is clamped to prevent showing empty space:

```javascript
cameraY = Math.max(0, Math.min(cameraY, mapHeight - GAME_H));
```

---

## Particle Systems

Particles provide visual feedback for events: explosions, pickups, damage, and
ambient atmosphere.

### The Particle Structure

```rust
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,    // Counts down from 1.0 to 0.0
    color: Color,
    size: f32,
}
```

### Explosion Pattern

When an enemy dies, spawn a burst of particles in random directions:

```javascript
for (let i = 0; i < 8; i++) {
    const angle = Math.random() * Math.PI * 2;
    const speed = 1 + Math.random() * 3;
    particles.push({
        x: enemy.x + enemy.w / 2,
        y: enemy.y + enemy.h / 2,
        vx: Math.cos(angle) * speed,
        vy: Math.sin(angle) * speed,
        life: 1.0,
        color: ['#ff0000', '#ffaa00', '#ffff00'][Math.floor(Math.random() * 3)],
        size: 2 + Math.random() * 3
    });
}
```

### Performance Cap

Low-end devices get fewer particles:

```javascript
const MAX_PARTICLES = (navigator.hardwareConcurrency <= 4) ? 40 : 120;

if (particles.length >= MAX_PARTICLES) {
    particles.shift();  // Remove oldest
}
```

### Ambient Particles

Dust motes and embers drift across the screen for atmosphere:

```rust
struct DustMote {
    x: f32, y: f32,
    vx: f32, vy: f32,
    size: f32,
    alpha: f32,
}
```

These move slowly with sinusoidal drift and fade in/out for a subtle living
world effect.

---

## Screen Effects

### Screen Shake

Impact feedback through camera displacement:

```javascript
let shakeMagnitude = 0;

function triggerShake(mag) {
    shakeMagnitude = Math.max(shakeMagnitude, mag);
}

function applyShake() {
    if (shakeMagnitude > 0.5) {
        screenShakeX = (Math.random() - 0.5) * shakeMagnitude;
        screenShakeY = (Math.random() - 0.5) * shakeMagnitude;
        shakeMagnitude *= 0.85;  // Decay
    } else {
        screenShakeX = screenShakeY = 0;
        shakeMagnitude = 0;
    }
}
```

Three shake levels:
- **Small (2px):** Bullet hits, minor collisions
- **Medium (5px):** Enemy deaths, player damage
- **Large (8px):** Boss kills, player death

### Damage Flash

A brief red border flash on taking damage:

```javascript
if (damageFlashTimer > 0) {
    ctx.fillStyle = `rgba(255, 0, 0, ${damageFlashTimer / 10})`;
    ctx.fillRect(0, 0, GAME_W, GAME_H);
    damageFlashTimer--;
}
```

### CRT Scanlines

Horizontal lines every 4 pixels at low opacity:

```javascript
ctx.fillStyle = 'rgba(0, 0, 0, 0.12)';
for (let y = 0; y < GAME_H; y += 4) {
    ctx.fillRect(0, y, GAME_W, 1);
}
```

```rust
let scanline_color = Color::new(0.0, 0.0, 0.0, 0.15);
let mut y = 0.0_f32;
while y < SCREEN_H {
    draw_line(0.0, y, SCREEN_W, y, 1.0, scanline_color);
    y += 4.0;
}
```

### Vignette

Darkened edges for a CRT monitor feel:

```javascript
const grad = ctx.createRadialGradient(
    GAME_W/2, GAME_H/2, GAME_W * 0.3,
    GAME_W/2, GAME_H/2, GAME_W * 0.7
);
grad.addColorStop(0, 'rgba(0,0,0,0)');
grad.addColorStop(1, 'rgba(0,0,0,0.7)');
ctx.fillStyle = grad;
ctx.fillRect(0, 0, GAME_W, GAME_H);
```

---

## Sound Design

### Web Audio API Procedural Sounds

All sounds are generated procedurally. No audio files exist anywhere in the
project.

```
Sound Type     Waveform    Freq Sweep        Duration
-----------------------------------------------------------
Jump           Square      250 -> 500 Hz     80ms
Shoot          Sawtooth    800 -> 200 Hz     80ms
Hit            Square      300 -> 100 Hz     50ms
Death          Sawtooth    400 -> 30 Hz      800ms
Enemy Death    Sawtooth    500 -> 50 Hz      150ms
Pickup         Square      C-E-G arpegg      180ms (3x60ms)
Wall Jump      Square      350 -> 700 Hz     100ms
Anchor Fire    Sine        1200 -> 600 Hz    100ms
```

### Implementation Pattern

```javascript
function playSound(type) {
    if (!audioCtx || audioCtx.state !== 'running') return;
    const t = audioCtx.currentTime;

    if (type === 'jump') {
        const o = audioCtx.createOscillator();
        const g = audioCtx.createGain();
        o.type = 'square';
        o.frequency.setValueAtTime(250, t);
        o.frequency.linearRampToValueAtTime(500, t + 0.08);
        g.gain.setValueAtTime(0.15, t);
        g.gain.linearRampToValueAtTime(0, t + 0.08);
        o.connect(g);
        g.connect(audioCtx.destination);
        o.start(t);
        o.stop(t + 0.08);
    }
}
```

### Audio Context Initialization

Browsers block audio until user interaction. The audio context is created and
resumed on the first click or touch:

```javascript
let audioCtx = null;

function initAudio() {
    if (!audioCtx) {
        audioCtx = new (window.AudioContext || window.webkitAudioContext)();
    }
    if (audioCtx.state === 'suspended') {
        audioCtx.resume();
    }
}

document.addEventListener('click', initAudio, { once: false });
document.addEventListener('touchstart', initAudio, { once: false });
```

Note: `once: false` is intentional. Some browsers suspend the context again
after a period of inactivity, so we re-try on every interaction.

---

## The Typewriter System

### Story Between Levels

The typewriter effect reveals text character by character, creating dramatic
pacing for narrative beats between levels:

```
Frame 0:   T
Frame 2:   Th
Frame 4:   The
Frame 6:   The
Frame 8:   The O
Frame 10:  The Ob
...
Frame 60:  The Obsidian Spire has awakened.
```

### Story Data Structure

```javascript
const storyScreens = {
    intro: [
        "The Obsidian Spire has awakened after a thousand years.",
        "Its corruption spreads across the land --",
        "forests wither, rivers turn black.",
        "",
        "You are Vael, the last of your order."
    ],
    afterLevel1: [
        "The walls pulse with a dark rhythm.",
        "Something inside the Spire recognizes you."
    ],
    victory: [
        "You ARE the heart.",
        "You shatter the crystal,",
        "and the Spire crumbles.",
        "Free at last."
    ]
};
```

### Typewriter State Machine

```javascript
let storyLines = [];
let storyLineIndex = 0;
let storyCharIndex = 0;
let storyFrameCounter = 0;

function updateTypewriter() {
    storyFrameCounter++;
    if (storyFrameCounter % 2 === 0 &&       // Every 2 frames
        storyLineIndex < storyLines.length) {
        storyCharIndex++;
        if (storyCharIndex >= storyLines[storyLineIndex].length) {
            storyLineIndex++;
            storyCharIndex = 0;
        }
    }
}
```

### Skip Mechanic

Players can press a button to instantly reveal all text:

```javascript
if (actionPressed) {
    storyLineIndex = storyLines.length;
    storyCharIndex = 0;
}
```

---

## Floating Score Popups

When the player collects a gem or defeats an enemy, a score popup floats upward
and fades:

```rust
struct Popup {
    text: String,
    x: f32,
    y: f32,
    life: f32,
}

// Spawn
self.popups.push(Popup {
    text: format!("+{}", amount),
    x, y,
    life: 1.0,
});

// Update
for popup in &mut self.popups {
    popup.y -= 1.0;
    popup.life -= 0.02;
}
self.popups.retain(|p| p.life > 0.0);

// Draw
for popup in &self.popups {
    let alpha = popup.life;
    draw_text(&popup.text, popup.x, popup.y,
              20.0, Color::new(1.0, 1.0, 0.0, alpha));
}
```

---

## Cross-References

- [Architecture](architecture.md) -- How these patterns fit into the overall
  system
- [Adding New Games](adding-games.md) -- Templates that use these patterns
- [Mobile Optimization](mobile-optimization.md) -- Performance considerations
  for the particle and entity systems
- [Miyoo Porting Guide](miyoo-porting-guide.md) -- Translating these patterns
  from JavaScript to Rust
