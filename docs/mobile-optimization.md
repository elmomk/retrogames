# Mobile Optimization Guide

*"Latency is the mind-killer. Latency is the little-death that brings total*
*abandonment. I will face my latency. I will permit it to pass through me."*
*-- Adapted from Ilya Grigorik, High Performance Browser Networking*

---

The retrogames collection runs on everything from high-end desktop browsers to
budget Android phones. This document covers every optimization technique used
to ensure smooth 60 FPS gameplay on mobile devices, responsive touch controls
that feel as precise as physical buttons, and canvas rendering that fills any
screen without letterboxing or distortion.

---

## The Viewport: Getting Full Control

### The Meta Tag

Every game starts with this viewport meta tag:

```html
<meta name="viewport" content="width=device-width, initial-scale=1.0,
      maximum-scale=1.0, user-scalable=no, viewport-fit=cover">
```

Each attribute matters:

| Attribute | Purpose |
|---|---|
| `width=device-width` | Match the viewport to the physical screen width |
| `initial-scale=1.0` | No zoom on load |
| `maximum-scale=1.0` | Prevent pinch-to-zoom (it conflicts with game touch) |
| `user-scalable=no` | Redundant with max-scale but needed for older browsers |
| `viewport-fit=cover` | Extend into notch/safe areas (we handle insets in CSS) |

### Preventing Browser Interference

Mobile browsers love to interfere with games. These CSS rules prevent it:

```css
html, body {
    overflow: hidden;          /* No scroll bars */
    margin: 0; padding: 0;    /* No default spacing */
    width: 100%; height: 100%; /* Fill everything */
    touch-action: none;        /* Disable browser gesture handling */
    -webkit-touch-callout: none;  /* No long-press callout */
    -webkit-user-select: none;    /* No text selection */
    user-select: none;            /* No text selection (standard) */
}
```

`touch-action: none` is critical. Without it, the browser intercepts touch
events for scrolling, pull-to-refresh, or navigation gestures. With it, all
touch events pass directly to your JavaScript.

---

## Canvas Scaling

### The Problem

The game logic operates at a fixed 640x480 resolution. Screens vary from
320x568 (iPhone SE) to 2560x1440 (desktop monitor). We need to scale the game
canvas to fill the screen while maintaining aspect ratio.

### The Solution: Uniform Scale with Offset

```javascript
const GAME_W = 640;
const GAME_H = 480;
let renderScale = 1, offsetX = 0, offsetY = 0;

function resizeCanvas() {
    const w = window.innerWidth;
    const h = window.innerHeight;
    canvas.width = w;
    canvas.height = h;

    // Uniform scale: fit 640x480 into screen, maintaining aspect ratio
    renderScale = Math.min(w / GAME_W, h / GAME_H);
    offsetX = (w - GAME_W * renderScale) / 2;
    offsetY = (h - GAME_H * renderScale) / 2;
}
```

```
Phone (9:16 portrait):        Desktop (16:9):
+------------------+          +---------------------------+
|   (letterbox)    |          |   (pillarbox)  GAME  (pb) |
| +==============+ |          +---------------------------+
| |              | |
| |    GAME      | |          Phone (9:16 landscape):
| |   640x480    | |          +---------------------------+
| |   scaled     | |          | +=======================+ |
| |              | |          | |        GAME            | |
| +==============+ |          | +=======================+ |
|   (letterbox)    |          +---------------------------+
+------------------+
```

### Drawing with Scale

Every draw call applies the scale transformation:

```javascript
ctx.save();
ctx.translate(offsetX, offsetY);
ctx.scale(renderScale, renderScale);

// All game drawing here uses 640x480 coordinates
ctx.drawImage(sprite, x, y, w, h);
ctx.fillRect(tileX, tileY, TILE_SIZE, TILE_SIZE);

ctx.restore();
```

### Handling Resize and Orientation

```javascript
resizeCanvas();
window.addEventListener('resize', resizeCanvas);
window.addEventListener('orientationchange', () => {
    setTimeout(resizeCanvas, 200);  // Delay for orientation animation
});
```

The 200ms delay on `orientationchange` is necessary because some browsers
report the old dimensions during the rotation animation. By the time the
timeout fires, the new dimensions are settled.

### Pixel-Perfect Rendering

```css
canvas {
    image-rendering: pixelated;
    image-rendering: crisp-edges;    /* Firefox fallback */
}
```

```javascript
ctx.imageSmoothingEnabled = false;
```

Without these, scaled-up pixel art gets bilinear filtering, turning crisp pixels
into blurry mush.

---

## Touch Control Design

### Layout

```
+-----------------------------------+
|                                   |
|           GAME AREA               |
|                                   |
|                                   |
+-----------------+-----------------+
|                 |                 |
|   JOYSTICK      |      [B]  [A]  |
|   ZONE          |                 |
|  (left 50%)     |   (right side)  |
+-----------------+-----------------+
```

The left half of the lower screen is the joystick zone. The right side has
action buttons. This mirrors a standard gamepad layout.

### Virtual Joystick Implementation

```javascript
const joystickZone = document.getElementById('joystickZone');
const joystickBase = document.getElementById('joystickBase');
const joystickStick = document.getElementById('joystickStick');

let joystickActive = false;
let joystickStartX = 0, joystickStartY = 0;
let joystickDX = 0, joystickDY = 0;

const DEADZONE = 15;     // Pixels of deadzone radius
const MAX_DIST = 50;     // Maximum joystick displacement

joystickZone.addEventListener('touchstart', (e) => {
    e.preventDefault();
    const touch = e.changedTouches[0];
    joystickActive = true;
    joystickStartX = touch.clientX;
    joystickStartY = touch.clientY;

    // Show joystick at touch position
    joystickBase.style.left = touch.clientX + 'px';
    joystickBase.style.top = touch.clientY + 'px';
    joystickBase.style.display = 'block';
});

joystickZone.addEventListener('touchmove', (e) => {
    e.preventDefault();
    if (!joystickActive) return;
    const touch = e.changedTouches[0];

    let dx = touch.clientX - joystickStartX;
    let dy = touch.clientY - joystickStartY;

    // Clamp to max distance
    const dist = Math.sqrt(dx * dx + dy * dy);
    if (dist > MAX_DIST) {
        dx = dx / dist * MAX_DIST;
        dy = dy / dist * MAX_DIST;
    }

    // Apply deadzone
    if (dist < DEADZONE) {
        joystickDX = 0;
        joystickDY = 0;
    } else {
        joystickDX = dx / MAX_DIST;  // Normalized: -1 to +1
        joystickDY = dy / MAX_DIST;
    }

    // Move stick visual
    joystickStick.style.transform =
        `translate(calc(-50% + ${dx}px), calc(-50% + ${dy}px))`;
});

joystickZone.addEventListener('touchend', (e) => {
    e.preventDefault();
    joystickActive = false;
    joystickDX = 0;
    joystickDY = 0;
    joystickBase.style.display = 'none';
    joystickStick.style.transform = 'translate(-50%, -50%)';
});
```

### Deadzone Design

```
         No input
      +----------+
      |          |
      |  DEAD    |
      |  ZONE    |  DEADZONE = 15px
      |  (15px)  |
      +----------+
     /            \
    /   Active     \
   /    Input       \
  /    Zone          \
 /   (15-50px)        \
+----------------------+
      MAX_DIST = 50px
```

The deadzone prevents accidental movement from thumb resting on the joystick.
The normalized output (-1 to +1) maps directly to movement velocity, giving
analog-like control:

```javascript
player.x += joystickDX * MOVE_SPEED;
player.y += joystickDY * MOVE_SPEED;
```

### Action Buttons

```javascript
const btnA = document.getElementById('btnA');
const btnB = document.getElementById('btnB');
let touchA = false, touchB = false;

btnA.addEventListener('touchstart', (e) => {
    e.preventDefault();
    touchA = true;
    btnA.classList.add('pressed');
});

btnA.addEventListener('touchend', (e) => {
    e.preventDefault();
    touchA = false;
    btnA.classList.remove('pressed');
});
```

Button press feedback is immediate via CSS class toggle:

```css
.btn {
    transition: background 0.08s, transform 0.08s;
}
.btn.pressed {
    transform: scale(0.92);
    background: rgba(255, 100, 100, 0.55);
    box-shadow: 0 0 18px rgba(255, 100, 100, 0.6);
}
```

### Hiding Touch Controls on Desktop

```css
@media (hover: hover) and (pointer: fine) {
    .action-btns { display: none; }
    #joystickZone { display: none; }
}
```

This media query detects a precise pointer (mouse) with hover capability, which
excludes touchscreens. On desktop, only keyboard input is available.

### Unified Input Reading

The game reads from both keyboard and touch inputs:

```javascript
function getInput() {
    return {
        left:   keys['ArrowLeft']  || joystickDX < -0.3,
        right:  keys['ArrowRight'] || joystickDX > 0.3,
        up:     keys['ArrowUp']    || joystickDY < -0.3,
        down:   keys['ArrowDown']  || joystickDY > 0.3,
        jump:   keys['z'] || keys['Z'] || touchA,
        shoot:  keys['x'] || keys['X'] || touchB,
        start:  keys['Enter'] || touchA || touchB,
    };
}
```

---

## Safe Area Insets

### The Notch Problem

Modern phones have display cutouts (notches, camera holes, rounded corners)
that can obscure game content if not handled:

```
+-------+-------+
|       |NOTCH  |
|       +---+   |
|               |
|    GAME       |  <-- Content hidden behind notch
|               |
+---------------+
```

### The Solution: `env(safe-area-inset-*)`

```css
canvas {
    padding: env(safe-area-inset-top)
             env(safe-area-inset-right)
             env(safe-area-inset-bottom)
             env(safe-area-inset-left);
    box-sizing: border-box;
}

#joystickZone {
    padding-left: env(safe-area-inset-left);
    padding-bottom: env(safe-area-inset-bottom);
    box-sizing: border-box;
}

.action-btns {
    padding-right: env(safe-area-inset-right);
    padding-bottom: env(safe-area-inset-bottom);
    box-sizing: border-box;
}
```

This pushes the canvas content and touch controls away from the notch and
rounded corners. `viewport-fit=cover` in the meta tag is required for these
values to be non-zero.

```
With safe-area insets:
+-------+-------+
|       |NOTCH  |
|  +----+---+   |
|  |         |   |
|  |  GAME   |   |  <-- Content safely inside insets
|  |         |   |
|  +---------+   |
+----------------+
```

---

## Performance Optimization

### Entity Caps

Every entity type has a maximum count:

```javascript
const MAX_PARTICLES = LOW_END ? 40 : 120;
const MAX_ENEMIES = 30;
const MAX_BULLETS = 50;
```

When the cap is hit, the oldest entity is removed before a new one is added.
This prevents the entity count from growing without bound during intense
gameplay.

### Low-End Detection

```javascript
const LOW_END = (navigator.hardwareConcurrency &&
                 navigator.hardwareConcurrency <= 4);
```

`navigator.hardwareConcurrency` reports the number of logical CPU cores. Devices
with 4 or fewer cores are treated as low-end and get reduced particle counts,
simpler effects, and fewer ambient particles.

This is a heuristic, not a guarantee. Some 4-core devices are fast (high-end
tablets), and some 8-core devices are slow (budget phones with big.LITTLE where
only small cores are active). But for our use case, it is a reasonable proxy.

### Batched Drawing

Instead of drawing each tile individually:

```javascript
// SLOW: one drawImage per tile, even off-screen
for (const tile of tiles) {
    ctx.drawImage(tileTex, tile.x, tile.y, TILE_SIZE, TILE_SIZE);
}
```

We cull off-screen tiles and batch by texture:

```javascript
// FAST: skip off-screen tiles
const startRow = Math.max(0, Math.floor(cameraY / TILE_SIZE));
const endRow = Math.min(mapHeight,
    Math.ceil((cameraY + GAME_H) / TILE_SIZE));

for (let row = startRow; row < endRow; row++) {
    for (let col = 0; col < mapWidth; col++) {
        const tile = map[row][col];
        if (tile) {
            ctx.drawImage(tileTex, col * TILE_SIZE,
                row * TILE_SIZE - cameraY, TILE_SIZE, TILE_SIZE);
        }
    }
}
```

### Canvas Context Options

```javascript
const ctx = canvas.getContext('2d', { willReadFrequently: false });
```

`willReadFrequently: false` tells the browser to optimize for drawing, not for
reading pixel data back (which we never do). Some browsers use this hint to
keep the canvas on the GPU.

### Avoiding Layout Thrashing

Never read layout properties (offsetWidth, getBoundingClientRect) during the
game loop. All layout-dependent calculations happen once in `resizeCanvas()`,
which only runs on resize events.

### requestAnimationFrame Over setInterval

`requestAnimationFrame` is always preferred over `setInterval` for game loops:

- It automatically pauses when the tab is hidden, saving battery.
- It synchronizes with the display refresh rate, preventing tearing.
- The browser can optimize compositing when it knows rendering is happening.

---

## Orientation Handling

### Landscape Preference

Games are designed for landscape orientation (640x480). On portrait phones, the
game is letterboxed with black bars:

```javascript
renderScale = Math.min(w / GAME_W, h / GAME_H);
```

On a 9:16 phone in portrait:
- w = 375, h = 812
- Scale = min(375/640, 812/480) = min(0.586, 1.692) = 0.586
- Game area: 375 x 281, centered vertically

On the same phone in landscape:
- w = 812, h = 375
- Scale = min(812/640, 375/480) = min(1.269, 0.781) = 0.781
- Game area: 500 x 375, centered horizontally

Landscape gives a 33% larger game area. The games are fully playable in both
orientations, but landscape is recommended.

### Orientation Change Recovery

```javascript
window.addEventListener('orientationchange', () => {
    setTimeout(resizeCanvas, 200);
});
```

Some mobile browsers report incorrect dimensions during the orientation
animation. The 200ms delay allows the animation to complete before
recalculating.

---

## Touch Performance Tips

### 1. Always Use `preventDefault()`

Every touch event handler must call `e.preventDefault()`. Without it, the
browser queues the touch event for default handling (scrolling, zooming), which
adds 300ms of latency on some browsers.

### 2. Passive Event Listeners Are the Enemy (Here)

For game touch controls, do NOT use `{ passive: true }`. Passive listeners
cannot call `preventDefault()`, which means the browser will try to scroll while
you are trying to move your character.

### 3. Use `changedTouches`, Not `touches`

```javascript
// CORRECT
const touch = e.changedTouches[0];

// INCORRECT (includes ALL active touches, not just this event's)
const touch = e.touches[0];
```

### 4. CSS `touch-action: none`

This is the single most important performance optimization for touch controls.
It tells the browser: "I will handle all touch events. Do not try to scroll,
zoom, or gesture."

Without it, the browser's gesture recognizer runs in parallel with your game
input, adding latency and occasionally stealing events.

---

## Memory and Battery

### Image Memory

Each 8x8 sprite canvas uses approximately 256 bytes of pixel data (8 * 8 * 4
RGBA channels). Even 100 sprites consume only 25 KB. This is negligible on any
device.

### Audio Memory

Web Audio API oscillators are generated in real-time and immediately discarded.
No audio buffers are stored in memory.

### Battery Conservation

- `requestAnimationFrame` pauses when the tab is hidden.
- No background timers, workers, or network requests.
- No continuous audio playback (sounds are short, event-triggered).
- Canvas compositing is GPU-accelerated, keeping CPU usage low.

A typical game session uses less battery than watching a video.

---

## Testing Matrix

When testing mobile optimization, check these scenarios:

| Device | Screen | Test Focus |
|---|---|---|
| iPhone SE (old) | 320x568 | Smallest viewport, scaling |
| iPhone 14 Pro | 393x852 + notch | Safe area insets |
| iPad | 1024x768 | Tablet scaling, touch zones |
| Pixel 6 | 412x915 | Android touch, 120Hz |
| Galaxy S21 | 360x800 | Samsung browser quirks |
| Miyoo Mini Plus | 640x480 | Native resolution, no scaling |

For automated testing, use Playwright with device emulation:

```javascript
const { devices } = require('playwright');
const iPhone = devices['iPhone 13'];
const browser = await chromium.launch();
const context = await browser.newContext({ ...iPhone });
```

---

## Cross-References

- [Architecture](architecture.md) -- How the viewport and canvas fit in the
  overall page structure
- [Game Engine Patterns](game-engine-patterns.md) -- The game loop, scaling
  system, and particle caps
- [Adding New Games](adding-games.md) -- Required CSS and viewport setup for
  new games
- [Miyoo Porting Guide](miyoo-porting-guide.md) -- The Miyoo has a fixed
  640x480 screen with no scaling needed
