# System Architecture

*"The only way to go fast, is to go well." -- Robert C. Martin*

---

## The Shape of the System

The retrogames project is a collection of retro-style arcade games that runs on
two radically different targets from a single codebase:

1. **Browser** -- Self-contained HTML5 Canvas games, each living in a single
   `index.html` file. No build step. No bundler. No framework. Just the
   platform.

2. **Miyoo Mini Plus** -- Native ARM binaries written in Rust with Macroquad,
   cross-compiled for a pocket-sized Linux handheld with a 640x480 screen and
   physical D-pad controls.

The architecture honors the **Dependency Rule**: high-level game logic depends on
nothing external. Each game is a self-contained unit. The deployment
infrastructure (Docker, Tailscale, CI/CD) wraps around the games without the
games knowing or caring.

```
    +--------------------------------------------------+
    |                   PLAYER                         |
    +--------------------------------------------------+
           |                            |
    +------v------+            +--------v--------+
    |   Browser   |            |  Miyoo Mini Plus |
    |  (any device)|           |  (ARM handheld)  |
    +------+------+            +--------+--------+
           |                            |
    +------v------+            +--------v--------+
    |  Tailscale  |            |  GitHub Release  |
    |  HTTPS Proxy|            |  Binary Download |
    +------+------+            +-----------------+
           |
    +------v------+
    |  Docker:     |
    |  busybox     |
    |  httpd :8080 |
    +------+------+
           |
    +------v------+
    |  web/        |
    |  Static HTML |
    +--------------+
```

---

## Directory Structure

```
retrogames/
|
+-- web/                        # Browser games (the "product")
|   +-- index.html              # Arcade Launcher -- links to all games
|   +-- micro/index.html        # Nano Wizards (Platformer)
|   +-- space/index.html        # Neon Defender (Shoot 'em Up)
|   +-- shadow/index.html       # Shadow Blade (Action Platformer)
|   +-- arena/index.html        # Arena Blitz (Twin-Stick Shooter)
|   +-- dragon/index.html       # Dragon Fury (Beat 'em Up)
|   +-- mariolike/index.html    # Additional game
|   +-- nova/index.html         # Additional game
|   +-- cyber/index.html        # Additional game
|   +-- */spec.md               # Technical specifications per game
|   +-- nginx                   # Reference nginx config (local dev)
|
+-- miyoo/                      # Miyoo Mini Plus native ports
|   +-- micro/                  # Each game is a standalone Rust project
|   |   +-- Cargo.toml
|   |   +-- src/main.rs
|   +-- space/
|   +-- shadow/
|   +-- arena/
|   +-- dragon/
|
+-- scripts/                    # Operational scripts
|   +-- deploy.sh               # Build & deploy Docker stack
|   +-- status.sh               # Health check: containers + Tailscale
|   +-- logs.sh                 # View container logs
|   +-- test.sh                 # Playwright smoke tests
|   +-- check-rust.sh           # cargo check all Miyoo ports
|   +-- build-miyoo.sh          # Build Miyoo binaries (native or ARM)
|
+-- .claude/                    # Claude Code skills and agents
|   +-- skills/                 # Slash-command automations
|   +-- agents/                 # Specialized sub-agents
|
+-- .github/workflows/          # CI/CD
|   +-- build-and-publish-release.yml
|
+-- Dockerfile                  # busybox httpd serving web/
+-- docker-compose.yml          # Tailscale sidecar + app container
+-- ts-serve.json               # Tailscale Serve HTTPS config
+-- CLAUDE.md                   # AI agent instructions
```

### The Single-File Principle

Every browser game is a single HTML file. This is not laziness -- it is a
deliberate architectural decision:

- **Zero dependencies.** No `npm install`, no broken `node_modules`, no
  version conflicts. The game works in 2024 and it will work in 2034.
- **Atomic deployment.** Copy the file. It works. No missing assets, no CORS
  issues, no CDN cache invalidation.
- **Complete readability.** Open the file. Read top to bottom. Understand
  everything. There is no hidden framework magic.
- **Instant portability.** Email it. Put it on a USB stick. Host it anywhere
  that can serve a static file.

The same principle applies to Miyoo ports: each game is a single `main.rs` file
with a single dependency (`macroquad = "0.4"`).

---

## How Browser Games Work

Each web game follows an identical structural pattern. Understanding one means
understanding all of them.

### The HTML Shell

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0,
          maximum-scale=1.0, user-scalable=no, viewport-fit=cover">
    <title>Game Title</title>
    <style>
        /* Full-viewport canvas, touch controls, safe area insets */
    </style>
</head>
<body>
    <canvas id="gameCanvas"></canvas>
    <!-- Touch control overlays (hidden on desktop) -->
    <div id="joystickZone">...</div>
    <div class="action-btns">...</div>
    <script>
        /* The entire game lives here */
    </script>
</body>
</html>
```

### The Game Loop

All games use a fixed-timestep loop at 60 FPS with `requestAnimationFrame`:

```javascript
const GAME_W = 640;
const GAME_H = 480;
let lastTime = 0;
const TIMESTEP = 1000 / 60;
let accumulator = 0;

function gameLoop(timestamp) {
    let delta = timestamp - lastTime;
    lastTime = timestamp;
    if (delta > 250) delta = 250;    // Death spiral prevention
    accumulator += delta;

    while (accumulator >= TIMESTEP) {
        update();                     // Physics at fixed rate
        accumulator -= TIMESTEP;
    }
    draw();                           // Render at display rate
    requestAnimationFrame(gameLoop);
}
```

### The Sprite System

Sprites are defined as character arrays, where each digit maps to a color:

```javascript
const mageTex = createSprite(
    ["..1111..",
     ".122221.",
     "13122131",
     "13322331",
     ".122221.",
     "..1111..",
     ".121121.",
     "12211221"],
    ['#000000', '#00FFFF', '#FFFFFF']
);
```

The `createSprite` function renders this to a small offscreen canvas, which is
then drawn scaled up with `image-rendering: pixelated` for that crisp pixel-art
look.

### The State Machine

Every game follows the same state flow:

```
    START --> STORY --> PLAYING --> LEVEL_STORY --> PLAYING --> ... --> WIN
                                       |
                                       v
                                   GAME_OVER
```

States are simple string comparisons in JavaScript, enum variants in Rust. Each
state has its own update and draw logic, typically in a single `switch`/`match`
block.

---

## How Miyoo Rust Ports Work

The Miyoo ports mirror the browser games in Rust using the Macroquad framework.

### Project Structure

```toml
# miyoo/<game>/Cargo.toml
[package]
name = "<game>_miyoo"
version = "0.1.0"
edition = "2021"

[dependencies]
macroquad = "0.4"
```

### The Rust Game Loop

```rust
const TIME_STEP: f64 = 1.0 / 60.0;

#[macroquad::main(window_conf)]
async fn main() {
    let sprites = Sprites { /* build all textures at boot */ };
    let mut world = World::new();
    let mut input = Input::new();
    let mut accumulator: f64 = 0.0;
    let mut last_time = get_time();

    loop {
        let current_time = get_time();
        let mut frame_time = current_time - last_time;
        last_time = current_time;

        if frame_time > 0.25 { frame_time = 0.25; }  // Death spiral cap
        accumulator += frame_time;

        input.poll();

        while accumulator >= TIME_STEP {
            match world.state {
                GameState::Playing => world.update(&mut input),
                _ => { /* handle menu/story transitions */ }
            }
            accumulator -= TIME_STEP;
        }

        draw_world(&mut world, &sprites);
        next_frame().await;
    }
}
```

### Sprite Translation

The same character-array sprites are translated to Rust constants:

```rust
const MAGE_ART: [&str; 8] = [
    "..1111..",
    ".122221.",
    "13122131",
    // ...
];
const MAGE_COLORS: [Color; 3] = [BLACK, Color::new(0.0, 1.0, 1.0, 1.0), WHITE];
```

These are rendered to `Texture2D` at boot time using `Image::gen_image_color()`
with `FilterMode::Nearest` for pixel-perfect scaling.

### CRT Effects

Both the browser and Miyoo versions render CRT effects for retro authenticity:

```rust
// Scanlines: dark horizontal lines every 4 pixels
let scanline_color = Color::new(0.0, 0.0, 0.0, 0.15);
let mut y = 0.0_f32;
while y < SCREEN_H {
    draw_line(0.0, y, SCREEN_W, y, 1.0, scanline_color);
    y += 4.0;
}

// Vignette: dark edges fading toward center
// (drawn as a radial gradient overlay)
```

---

## The Deployment Pipeline

The deployment architecture uses Docker Compose with a Tailscale sidecar to
expose the games over HTTPS on a private tailnet.

### Container Architecture

```
+-------------------------------------------------------+
|                  Docker Compose Stack                  |
|                                                       |
|  +------------------+     +------------------------+  |
|  |    tailscale      |     |         app            |  |
|  |                  |     |                        |  |
|  |  Tailscale daemon|     |  busybox httpd         |  |
|  |  HTTPS :443      |---->|  :8080                 |  |
|  |  (ts-serve.json) |     |  serves /srv/www/      |  |
|  |                  |     |                        |  |
|  |  Volumes:        |     |  network_mode:         |  |
|  |  - state         |     |   service:tailscale    |  |
|  |  - serve config  |     |                        |  |
|  +------------------+     +------------------------+  |
+-------------------------------------------------------+
```

The key insight is `network_mode: service:tailscale`. This makes the app
container share the tailscale container's network namespace. From the outside,
only the Tailscale container exists on the network. It terminates HTTPS and
proxies to `127.0.0.1:8080`, which is the app container's httpd -- because they
share a network namespace, localhost is shared.

### The Dockerfile

```dockerfile
FROM busybox:1.37

RUN adduser -D -u 1000 app
COPY web/ /srv/www/
RUN chown -R app:app /srv/www

USER app
EXPOSE 8080

CMD ["busybox", "httpd", "-f", "-p", "8080", "-h", "/srv/www"]
```

No nginx. No Apache. No Node.js. Just `busybox httpd` serving static files. The
image is roughly 5 MB. This is what "do one thing well" looks like.

### Tailscale Serve Configuration

```json
{
  "TCP": {
    "443": { "HTTPS": true }
  },
  "Web": {
    "${TS_CERT_DOMAIN}:443": {
      "Handlers": {
        "/": {
          "Proxy": "http://127.0.0.1:8080"
        }
      }
    }
  }
}
```

Tailscale provides automatic HTTPS certificates via Let's Encrypt. No manual
cert management. No cert renewal cron jobs.

---

## The CI/CD Pipeline

GitHub Actions handles cross-compilation and release publishing for the Miyoo
ports.

### Pipeline Flow

```
Tag push (v*)
    |
    v
Check Miyoo Changes -------> No changes? Skip.
    |
    v
Discover Games
    |  (python3 scans miyoo/*/ for subdirectories)
    v
Build Matrix ----+----> Build micro (ARM cross-compile)
                 +----> Build space (ARM cross-compile)
                 +----> Build shadow (ARM cross-compile)
                 +----> Build arena (ARM cross-compile)
                 +----> Build dragon (ARM cross-compile)
                 |
                 v
         Publish GitHub Release
         (all binaries as assets)
```

Key design decisions:

- **Auto-discovery.** The pipeline scans `miyoo/*/` for game directories. Adding
  a new game requires zero CI/CD configuration changes.
- **Parallel builds.** Each game builds in its own matrix job with
  `fail-fast: false`, so one broken game does not block the others.
- **Tag-triggered.** Only `v*` tags trigger releases. Development pushes build
  but do not publish.

### Cross-Compilation

The target is ARM Cortex-A7 (`armv7-unknown-linux-gnueabihf`):

```bash
CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
  cargo build --release --target armv7-unknown-linux-gnueabihf
```

The CI installs `gcc-arm-linux-gnueabihf` via apt, adds the Rust target via
`rustup`, and builds each game individually.

---

## Data Flow: From Code to Player

### Browser Path

```
Developer writes index.html
        |
        v
git push to main
        |
        v
docker compose build      (copies web/ into busybox image)
        |
        v
docker compose up -d       (starts tailscale + app)
        |
        v
Tailscale joins tailnet    (hostname: retrogames)
        |
        v
HTTPS :443 --> proxy --> busybox httpd :8080
        |
        v
Player opens https://retrogames.<tailnet>/micro/
        |
        v
Browser downloads single HTML file
        |
        v
JavaScript boots, creates Canvas, runs game loop
```

### Miyoo Path

```
Developer writes main.rs
        |
        v
git push + tag v1.x
        |
        v
GitHub Actions: cross-compile for armv7
        |
        v
Publish binary to GitHub Releases
        |
        v
User downloads <game>_miyoo binary
        |
        v
Copy to Miyoo Mini Plus SD card
        |
        v
Launch from device menu, plays at 640x480 on hardware
```

---

## Design Principles

### 1. Self-Containment Over Modularity

Each game is complete in itself. There is no shared game engine, no utility
library, no common component. This means some code is duplicated across games.
That is acceptable. The alternative -- a shared framework -- would couple the
games together and make each one harder to understand in isolation.

### 2. Platform Parity

The browser version and the Miyoo version of each game play identically. Same
physics constants, same level layouts, same story text, same sprite art. The
only differences are rendering API (Canvas vs Macroquad) and input method
(keyboard/touch vs D-pad/buttons).

### 3. Operational Simplicity

- One Dockerfile, 6 lines.
- One docker-compose.yml, 30 lines.
- One CI workflow that auto-discovers games.
- Shell scripts for every operational task.

There is no Kubernetes. No Terraform. No Ansible. The system is small enough
to hold in your head, and that is by design.

---

## Cross-References

- [Game Engine Patterns](game-engine-patterns.md) -- Deep dive into the game
  loop, sprite system, state machine, and physics
- [Deployment Guide](deployment-guide.md) -- Step-by-step deployment with Docker
  and Tailscale
- [Adding New Games](adding-games.md) -- How to add a game to both platforms
- [Miyoo Porting Guide](miyoo-porting-guide.md) -- Translating browser games
  to Rust for the Miyoo handheld
- [Mobile Optimization](mobile-optimization.md) -- Touch controls, scaling,
  and performance
- [Skills & Agents](skills-and-agents.md) -- Claude Code automations for this
  project
