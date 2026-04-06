# Learn ChaiLove: Retro Games for Miyoo Mini Plus

A practical guide for JavaScript and Rust developers building games with ChaiLove on the Miyoo Mini Plus handheld console.

---

## Table of Contents

1. [Why ChaiLove](#part-1-why-chailove)
2. [ChaiScript Language](#part-2-chaiscript-language)
3. [ChaiLove API Reference](#part-3-chailove-api-reference)
4. [Game Development Patterns](#part-4-game-development-patterns)
5. [Deploying to Miyoo Mini Plus](#part-5-deploying-to-miyoo-mini-plus)
6. [Complete Game: Asteroids Clone](#part-6-complete-game-asteroids-clone)

---

## Part 1: Why ChaiLove

### What Is ChaiLove?

ChaiLove is a game framework inspired by [LÖVE2D](https://love2d.org/) but using ChaiScript instead of Lua. It runs as a **libretro core** inside RetroArch, which means your game is a `.chailove` file that RetroArch loads like a ROM. The core handles everything: windowing, audio output, input polling, display scaling.

From your game's perspective, you write a handful of callback functions — `load()`, `update(dt)`, `draw()` — and ChaiLove calls them. That's the entire contract.

The source lives at [github.com/RobLoach/chailove](https://github.com/RobLoach/chailove). The ChaiScript language it embeds is a C++-adjacent scripting language designed to be embedded in C++ hosts, with syntax that will feel familiar coming from JS or Rust.

### Why ChaiLove Is Right for Miyoo Mini Plus

The Miyoo Mini Plus runs OnionOS on an ARM Cortex-A7. RetroArch ships pre-installed. The ChaiLove libretro core is a single `.so` file you drop into the RetroArch cores directory.

The alternative approaches all have friction:

- **Rust (Macroquad)** — requires cross-compiling to `armv7-unknown-linux-gnueabihf`, managing linker toolchains, and dealing with driver quirks for display and audio. Every iteration cycle is a full cross-compile.
- **C/SDL2** — same cross-compilation pain, plus manual SDL2 linkage against the device libraries.
- **Web/Electron** — the Miyoo does not run a browser. You'd need a custom launcher.
- **Python/pygame** — not realistically packaged for Miyoo.

With ChaiLove, your iteration loop is:

```
edit main.chai → zip to game.chailove → scp to /mnt/SDCARD/Roms/CHAILOVE/ → launch in RetroArch
```

No compiler. No linker flags. The ChaiScript interpreter runs at runtime inside the core.

### What You Get for Free

Because ChaiLove runs inside RetroArch, every RetroArch feature applies to your game automatically:

- **Save states** — RetroArch can snapshot and restore the entire emulator state, including your game's memory. Players get save states in any game you write.
- **Rewind** — hold a button and time reverses. This works out of the box.
- **Shader overlays** — RetroArch can apply CRT, scanline, or pixel-art shaders on top of your game's output without you writing any shader code.
- **Screenshots** — RetroArch hotkey saves a PNG.
- **Input remapping** — players can rebind any button in RetroArch's menu.
- **Netplay** — RetroArch's netplay protocol can theoretically synchronize ChaiLove games between two devices.

You write none of this. You get all of it.

### ChaiScript vs Lua vs JavaScript

| Feature | ChaiScript | Lua | JavaScript |
|---|---|---|---|
| Typing | Dynamic with optional hints | Dynamic | Dynamic |
| Syntax family | C++/Java-like | Unique | C-like |
| Classes | Built-in `class` keyword | Tables + metatables | `class` or prototype |
| Arrays | `Vector()` (typed) | Tables (1-indexed) | `[]` (0-indexed) |
| Maps | `Map()` | Tables | `{}` or `Map` |
| Switch statement | Not available | Not available | Available |
| String concat | `+` operator | `..` operator | `+` or template literals |
| Null value | `null` (rare) | `nil` | `null` / `undefined` |
| Error model | C++ exceptions | `pcall` | try/catch |
| Embedding target | C++ hosts | C hosts | V8 / browser |

The biggest adjustments coming from JavaScript:

- There is no `switch`. Use `if/else if/else` chains.
- Vectors are typed — a `Vector` holding ints cannot hold strings without explicit conversion.
- `for (item : vec)` is the range-based for loop. C-style `for (var i = 0; i < n; i++)` also works.
- Functions are defined with `def`, not `function`.
- Classes use `this.field` to access members but you declare fields with bare `var field;` at class scope.

---

## Part 2: ChaiScript Language

### Variables

```chaiscript
var x = 10;          // local variable, type inferred as int
var name = "hero";   // inferred as string
var speed = 2.5;     // inferred as float
global score = 0;    // global scope — accessible from any function
```

There is no `const`. Convention is to write constant-like globals in ALL_CAPS and not reassign them.

ChaiScript is dynamically typed, so a variable can hold any value. Unlike JavaScript, though, you will hit runtime errors if you mix types carelessly in operations — ChaiScript does not silently coerce.

```chaiscript
var x = 5;
var y = 2.0;
var z = x + y;   // runtime error in strict mode — int + float
var z = to_float(x) + y;   // correct
```

### Types

**Primitive types:**

```chaiscript
var i = 42;          // int
var f = 3.14;        // float
var s = "hello";     // string
var b = true;        // bool
```

**Collections:**

```chaiscript
var vec = Vector();  // dynamic array (like JS Array)
var m = Map();       // key-value store (like JS Object/Map)
```

### Strings

```chaiscript
var name = "Nano";
var greeting = "Hello, " + name + "!";

// Length
var len = name.size();       // 4

// Convert other types to string
var score = 100;
var display = "Score: " + to_string(score);

// Character access (returns single-char string)
var first = name[0];         // "N"

// Comparison
if (name == "Nano") { }

// Common string methods (subset available in ChaiScript)
name.find("an");             // returns index or string::npos
name.substr(1, 2);           // "an" — start index, length
```

String concatenation with `+` always requires both sides to be strings. Convert first:

```chaiscript
// Wrong:
love.graphics.print("Lives: " + lives, 10, 10);   // lives is int — error

// Correct:
love.graphics.print("Lives: " + to_string(lives), 10, 10);
```

### Vectors

`Vector` is ChaiScript's dynamic array. It is 0-indexed.

```chaiscript
var bullets = Vector();

// Add elements
bullets.push_back(10);
bullets.push_back(20);
bullets.push_back(30);

// Size
var count = bullets.size();   // 3

// Index access
var first = bullets[0];       // 10

// Modify in place
bullets[0] = 99;

// Range-based for loop
for (val : bullets) {
    love.graphics.print(to_string(val), 10, 10);
}

// Index-based for loop (use when you need the index or remove elements)
for (var i = 0; i < bullets.size(); i++) {
    love.graphics.print(to_string(bullets[i]), 10, i * 16);
}
```

Vectors of objects work the same way. You can push instances of your own classes.

**Removing elements while iterating — iterate backwards:**

```chaiscript
// Remove dead bullets
for (var i = bullets.size() - 1; i >= 0; i--) {
    if (bullets[i].dead) {
        bullets.erase(i);
    }
}
```

`bullets.erase(i)` removes the element at index `i`. If you iterate forwards while erasing, you skip elements. Iterate backwards to be safe.

### Maps

```chaiscript
var config = Map();
config["width"] = 320;
config["height"] = 240;
config["title"] = "My Game";

// Access
var w = config["width"];

// Check if key exists — access returns null if missing
// Safer pattern: initialize all keys in load()
```

Maps are useful for keyed config data. For game entities with many instances, Vectors of class objects are usually faster and clearer.

### Functions

```chaiscript
def add(a, b) {
    return a + b;
}

var result = add(3, 4);   // 7
```

Functions are first-class values. You can assign them to variables:

```chaiscript
def greet(name) {
    return "Hello, " + name;
}

var fn = greet;
fn("world");   // "Hello, world"
```

Default arguments are not supported. Use conditional defaults at the function top:

```chaiscript
def drawBox(x, y, w, h, filled) {
    var mode = "fill";
    if (!filled) { mode = "line"; }
    love.graphics.rectangle(mode, x, y, w, h);
}
```

### Classes

```chaiscript
class Bullet {
    var x;
    var y;
    var vx;
    var vy;
    var alive;

    def Bullet(startX, startY, velX, velY) {
        this.x = startX;
        this.y = startY;
        this.vx = velX;
        this.vy = velY;
        this.alive = true;
    }

    def update(dt) {
        this.x += this.vx * dt;
        this.y += this.vy * dt;
        if (this.x < 0 || this.x > 320) {
            this.alive = false;
        }
    }

    def draw() {
        love.graphics.setColor(255, 255, 0, 255);
        love.graphics.rectangle("fill", this.x - 2, this.y - 2, 4, 4);
    }
}

// Instantiate
var b = Bullet(100.0, 200.0, 150.0, 0.0);

// Use
b.update(0.016);
b.draw();
```

Key rules for classes:

- Declare all fields at class scope with `var field;` (no initializer — that goes in the constructor).
- The constructor method has the same name as the class.
- Use `this.field` everywhere inside methods — there is no implicit `this`.
- ChaiScript classes do not support inheritance. Compose by storing instances as fields.

### Control Flow

**if/else:**

```chaiscript
if (hp <= 0) {
    state = "gameover";
} else if (hp < 10) {
    color = "red";
} else {
    color = "green";
}
```

**There is no switch statement.** This is not an oversight — ChaiScript simply doesn't have one. Use if/else chains:

```chaiscript
// In JS you might write: switch(state) { case "menu": ... }
// In ChaiScript:
if (state == "menu") {
    drawMenu();
} else if (state == "playing") {
    drawGame();
} else if (state == "gameover") {
    drawGameOver();
}
```

**while:**

```chaiscript
var i = 0;
while (i < 10) {
    doSomething(i);
    i += 1;
}
```

**C-style for:**

```chaiscript
for (var i = 0; i < 10; i++) {
    doSomething(i);
}
```

**Range-based for:**

```chaiscript
for (item : myVector) {
    item.update(dt);
}
```

Note: range-based for gives you a copy of each element, not a reference. If you need to modify elements in-place, use an index-based loop:

```chaiscript
for (var i = 0; i < entities.size(); i++) {
    entities[i].x += entities[i].vx * dt;
}
```

### Math Functions

ChaiScript exposes standard C math functions globally:

```chaiscript
sin(angle)          // angle in radians
cos(angle)
sqrt(x)
abs(x)
floor(x)            // returns float
ceil(x)             // returns float
to_int(floor(x))    // convert float to int

// Power
pow(2.0, 8.0)       // 256.0

// Min/max — not built in, write your own:
def min_val(a, b) { if (a < b) { return a; } return b; }
def max_val(a, b) { if (a > b) { return a; } return b; }
def clamp(v, lo, hi) { return min_val(hi, max_val(lo, v)); }
```

### Type Conversion

```chaiscript
to_string(42)        // "42"
to_string(3.14)      // "3.14"
to_int("42")         // 42
to_float("3.14")     // 3.14
to_int(3.9)          // 3 (truncates, does not round)
```

### Scope

Variables declared with `var` inside a function are local to that function. Variables declared with `global` at the top level (or using the `global` keyword inside a function) are accessible everywhere.

```chaiscript
global playerX = 160.0;   // accessible from any function
global playerY = 120.0;

def update(dt) {
    playerX += 50.0 * dt;   // reads and writes global
    var localTemp = 0;       // only exists in this call to update()
}
```

Convention in ChaiLove games: put all mutable game state in globals or in a single global "game" object. This mirrors how LÖVE2D games typically work.

### Common Gotchas for JS/Rust Developers

**No automatic number coercion:**
```chaiscript
// JS: "x: " + 5 == "x: 5"
// ChaiScript: runtime error
love.graphics.print("x: " + to_string(playerX), 10, 10);   // correct
```

**Integer division:**
```chaiscript
var result = 7 / 2;    // 3, not 3.5 — both operands are int
var result = 7.0 / 2;  // 3.5 — float division
```

**Range-based for gives copies:**
```chaiscript
for (bullet : bullets) {
    bullet.x += 10;   // modifies the COPY, not the Vector element
}
// Use index-based loop to modify in place
```

**No undefined — missing Map keys return null:**
```chaiscript
var m = Map();
var val = m["missing"];   // null, not undefined
// Nulls can cause confusing runtime errors later
// Initialize all expected keys in load()
```

**No arrow functions or closures with captured variables:**
```chaiscript
// JS: bullets.filter(b => b.alive)
// ChaiScript: no filter method — do it manually with a for loop
```

**PI is not globally defined:**
```chaiscript
global PI = 3.14159265358979;
// Or use love.math.rad() which converts degrees without needing PI
```

---

## Part 3: ChaiLove API Reference

### Game Loop Callbacks

Your `main.chai` file defines these functions. ChaiLove calls them in order.

```chaiscript
def conf(t) {
    t.window.width = 320;
    t.window.height = 240;
    t.window.title = "My Game";
}

def load() {
    // Called once at startup
    // Load assets, initialize state
}

def update(dt) {
    // Called every frame before draw()
    // dt = seconds elapsed since last frame (typically ~0.016 at 60fps)
}

def draw() {
    // Called every frame after update()
    // All rendering happens here
}
```

`conf()` is optional. Default resolution is 800x600. For Miyoo Mini Plus, use 320x240 (the screen's native resolution after scaling) or 640x480.

### Graphics

All colors use 0-255 range (not 0.0-1.0 like LÖVE2D defaults).

**Color and clearing:**

```chaiscript
love.graphics.setColor(255, 255, 255, 255);       // white, fully opaque
love.graphics.setColor(255, 0, 0, 128);           // red, half transparent
love.graphics.setBackgroundColor(0, 0, 20);       // dark blue background
love.graphics.clear();                             // clear with background color
love.graphics.clear(10, 10, 30);                  // clear with specific color
```

Always call `love.graphics.setColor()` before drawing. Color state persists across calls.

**Shapes:**

```chaiscript
-- Filled rectangle
love.graphics.rectangle("fill", x, y, width, height);

-- Outlined rectangle
love.graphics.rectangle("line", x, y, width, height);

-- Filled circle
love.graphics.circle("fill", cx, cy, radius);

-- Outlined circle
love.graphics.circle("line", cx, cy, radius);

-- Ellipse
love.graphics.ellipse("fill", cx, cy, rx, ry);

-- Arc (angles in radians)
love.graphics.arc("fill", cx, cy, radius, startAngle, endAngle);

-- Line between two points
love.graphics.line(x1, y1, x2, y2);

-- Single pixel
love.graphics.point(x, y);
```

**Text:**

```chaiscript
-- Print with default font
love.graphics.print("Hello, world!", x, y);

-- Load a font
var font = love.graphics.newFont(16);              -- built-in font at size 16
var font = love.graphics.newFont("font.ttf", 16); -- custom TTF

-- Set active font
love.graphics.setFont(font);

-- Screen dimensions
var w = love.graphics.getWidth();
var h = love.graphics.getHeight();

-- Center text (manual calculation)
var textW = 100;   -- estimate or measure
var cx = (love.graphics.getWidth() - textW) / 2.0;
love.graphics.print("GAME OVER", cx, 100.0);
```

**Images:**

```chaiscript
-- In load():
var img = love.graphics.newImage("player.png");

-- In draw():
love.graphics.draw(img, x, y);

-- With rotation and scale:
-- draw(image, x, y, rotation, scaleX, scaleY, originX, originY)
love.graphics.draw(img, x, y, 0.0, 1.0, 1.0, 0.0, 0.0);

-- Pixel-perfect scaling (set before loading images):
love.graphics.setDefaultFilter("nearest");
```

**Sprite sheets with Quads:**

```chaiscript
-- In load():
var sheet = love.graphics.newImage("sprites.png");
-- newQuad(x, y, width, height, sheetWidth, sheetHeight)
var playerIdle = love.graphics.newQuad(0, 0, 16, 16, 128, 128);
var playerRun  = love.graphics.newQuad(16, 0, 16, 16, 128, 128);

-- In draw():
-- draw(image, quad, x, y)
love.graphics.draw(sheet, playerIdle, playerX, playerY);
```

### Input

**Keyboard:**

```chaiscript
-- Check if key is currently held down
love.keyboard.isDown("left")
love.keyboard.isDown("right")
love.keyboard.isDown("up")
love.keyboard.isDown("down")
love.keyboard.isDown("space")
love.keyboard.isDown("z")
love.keyboard.isDown("x")
love.keyboard.isDown("return")    -- Enter key
love.keyboard.isDown("escape")
love.keyboard.isDown("a")         -- letter keys by name
```

**Joystick/Gamepad (preferred for Miyoo):**

```chaiscript
-- Check if any joystick is connected
love.joystick.getJoystickCount();   -- returns int

-- Check button state (joystick index starts at 0)
love.joystick.isDown(0, "a")        -- A button (right face)
love.joystick.isDown(0, "b")        -- B button (bottom face)
love.joystick.isDown(0, "x")        -- X button (top face)
love.joystick.isDown(0, "y")        -- Y button (left face)
love.joystick.isDown(0, "start")
love.joystick.isDown(0, "select")
love.joystick.isDown(0, "dpup")
love.joystick.isDown(0, "dpdown")
love.joystick.isDown(0, "dpleft")
love.joystick.isDown(0, "dpright")
love.joystick.isDown(0, "leftshoulder")   -- L button
love.joystick.isDown(0, "rightshoulder")  -- R button
```

On Miyoo Mini Plus via RetroArch, the D-pad and face buttons map to these names. The physical button labeled "A" on the Miyoo is typically button `b` in RetroArch's SDL mapping (confirming in RetroArch input settings is recommended for your specific OnionOS build).

**Input that works on both desktop and Miyoo (recommended pattern):**

```chaiscript
def inputDown() {
    return love.keyboard.isDown("down") || love.joystick.isDown(0, "dpdown");
}

def inputUp() {
    return love.keyboard.isDown("up") || love.joystick.isDown(0, "dpup");
}

def inputLeft() {
    return love.keyboard.isDown("left") || love.joystick.isDown(0, "dpleft");
}

def inputRight() {
    return love.keyboard.isDown("right") || love.joystick.isDown(0, "dpright");
}

def inputFire() {
    return love.keyboard.isDown("z") || love.joystick.isDown(0, "a");
}

def inputStart() {
    return love.keyboard.isDown("return") || love.joystick.isDown(0, "start");
}
```

**Mouse (desktop only, not useful on Miyoo):**

```chaiscript
var pos = love.mouse.getPosition();
var mx = pos[0];
var my = pos[1];
```

### Math

```chaiscript
love.math.random()           -- float 0.0 to 1.0
love.math.random(max)        -- int 0 to max (inclusive? check docs — assume exclusive)
love.math.random(min, max)   -- int min to max

love.math.rad(degrees)       -- convert degrees to radians
love.math.degrees(radians)   -- convert radians to degrees
```

### System and Filesystem

```chaiscript
love.system.getOS()          -- "Linux", "Windows", "Android", etc.
love.timer.getDelta()        -- seconds since last frame (same as dt parameter)
love.filesystem.load("levels.chai")   -- load and execute another .chai file
```

---

## Part 4: Game Development Patterns

### Game State Machine

The simplest reliable pattern for ChaiLove games. Use a global string or integer to track which state is active.

```chaiscript
global STATE_TITLE   = 0;
global STATE_PLAYING = 1;
global STATE_DEAD    = 2;
global STATE_WIN     = 3;

global state = STATE_TITLE;

def update(dt) {
    if (state == STATE_TITLE) {
        updateTitle(dt);
    } else if (state == STATE_PLAYING) {
        updatePlaying(dt);
    } else if (state == STATE_DEAD) {
        updateDead(dt);
    } else if (state == STATE_WIN) {
        updateWin(dt);
    }
}

def draw() {
    love.graphics.clear();
    if (state == STATE_TITLE) {
        drawTitle();
    } else if (state == STATE_PLAYING) {
        drawPlaying();
    } else if (state == STATE_DEAD) {
        drawDead();
    } else if (state == STATE_WIN) {
        drawWin();
    }
}

def updateTitle(dt) {
    if (inputStart()) {
        state = STATE_PLAYING;
        initGame();
    }
}
```

State transitions happen by assigning a new value to `state`. Always call an `initGame()` or similar reset function when entering PLAYING to clear stale data from a previous run.

### Entity Storage with Vectors

Store game entities as Vectors of class instances.

```chaiscript
class Enemy {
    var x; var y;
    var vx; var vy;
    var hp;
    var alive;

    def Enemy(startX, startY) {
        this.x = to_float(startX);
        this.y = to_float(startY);
        this.vx = love.math.random(40, 80) * 0.1;  // hack: random float
        this.vy = 0.0;
        this.hp = 3;
        this.alive = true;
    }

    def update(dt) {
        this.x += this.vx * dt;
        if (this.x > 320.0 || this.x < 0.0) { this.vx = -this.vx; }
    }

    def draw() {
        love.graphics.setColor(255, 60, 60, 255);
        love.graphics.rectangle("fill", this.x - 8, this.y - 8, 16, 16);
    }

    def takeDamage(amount) {
        this.hp -= amount;
        if (this.hp <= 0) { this.alive = false; }
    }
}

global enemies = Vector();

def spawnEnemies() {
    enemies = Vector();   // clear
    for (var i = 0; i < 5; i++) {
        enemies.push_back(Enemy(love.math.random(20, 300), love.math.random(20, 100)));
    }
}

def updateEnemies(dt) {
    for (var i = 0; i < enemies.size(); i++) {
        enemies[i].update(dt);
    }
    // Remove dead enemies (backwards to preserve indices)
    for (var i = enemies.size() - 1; i >= 0; i--) {
        if (!enemies[i].alive) {
            enemies.erase(i);
        }
    }
}
```

### Procedural Sprite Rendering

No image files needed. Define sprites as arrays of strings, render pixel by pixel.

```chaiscript
// 8x8 sprite — 0 = transparent, 1 = color1, 2 = color2
global SPRITE_SHIP = Vector();

def loadSprites() {
    SPRITE_SHIP.push_back("...11...");
    SPRITE_SHIP.push_back("..111...");
    SPRITE_SHIP.push_back(".11111..");
    SPRITE_SHIP.push_back("1111111.");
    SPRITE_SHIP.push_back(".11111..");
    SPRITE_SHIP.push_back("..1.1...");
    SPRITE_SHIP.push_back("..1.1...");
    SPRITE_SHIP.push_back("........");
}

def drawSprite(sprite, px, py, scale, r, g, b) {
    for (var row = 0; row < sprite.size(); row++) {
        var rowStr = sprite[row];
        for (var col = 0; col < rowStr.size(); col++) {
            var pixel = rowStr[col];
            if (pixel == "1") {
                love.graphics.setColor(r, g, b, 255);
                love.graphics.rectangle("fill",
                    px + col * scale,
                    py + row * scale,
                    scale, scale);
            } else if (pixel == "2") {
                love.graphics.setColor(255, 255, 255, 255);
                love.graphics.rectangle("fill",
                    px + col * scale,
                    py + row * scale,
                    scale, scale);
            }
        }
    }
}

// Usage:
drawSprite(SPRITE_SHIP, playerX - 4, playerY - 4, 2, 100, 200, 255);
```

### Tile-Based Level Maps

```chaiscript
// Level as an array of strings
// . = empty, # = wall, c = coin, e = enemy spawn
global level1 = Vector();

def loadLevel() {
    level1 = Vector();
    level1.push_back("####################");
    level1.push_back("#..................#");
    level1.push_back("#..c........c......#");
    level1.push_back("#..................#");
    level1.push_back("####....####.......#");
    level1.push_back("#.......e..........#");
    level1.push_back("#..................#");
    level1.push_back("####################");
}

global TILE_SIZE = 16;

def drawLevel(camX, camY) {
    for (var row = 0; row < level1.size(); row++) {
        var rowStr = level1[row];
        for (var col = 0; col < rowStr.size(); col++) {
            var tile = rowStr[col];
            var sx = col * TILE_SIZE - camX;
            var sy = row * TILE_SIZE - camY;

            // Skip tiles off-screen
            if (sx < -TILE_SIZE || sx > 320 || sy < -TILE_SIZE || sy > 240) {
                continue;
            }

            if (tile == "#") {
                love.graphics.setColor(80, 80, 120, 255);
                love.graphics.rectangle("fill", sx, sy, TILE_SIZE, TILE_SIZE);
            } else if (tile == "c") {
                love.graphics.setColor(255, 220, 0, 255);
                love.graphics.circle("fill", sx + TILE_SIZE/2, sy + TILE_SIZE/2, 4.0);
            }
        }
    }
}

// Tile collision helper
def getTile(row, col) {
    if (row < 0 || row >= level1.size()) { return "#"; }
    var rowStr = level1[row];
    if (col < 0 || col >= rowStr.size()) { return "#"; }
    return rowStr[col];
}

def isSolid(worldX, worldY) {
    var col = to_int(worldX / TILE_SIZE);
    var row = to_int(worldY / TILE_SIZE);
    var tile = getTile(row, col);
    return tile == "#";
}
```

### Camera Systems

```chaiscript
global camX = 0.0;
global camY = 0.0;

// Smooth follow camera
def updateCamera(targetX, targetY, dt) {
    var speed = 5.0;
    camX += (targetX - love.graphics.getWidth() / 2.0 - camX) * speed * dt;
    camY += (targetY - love.graphics.getHeight() / 2.0 - camY) * speed * dt;

    // Clamp to level bounds
    var maxCamX = to_float(20 * TILE_SIZE - love.graphics.getWidth());
    var maxCamY = to_float(8 * TILE_SIZE - love.graphics.getHeight());
    camX = clamp(camX, 0.0, maxCamX);
    camY = clamp(camY, 0.0, maxCamY);
}

// To draw with camera offset, subtract camX/camY from all world positions:
def drawEntity(worldX, worldY) {
    var sx = worldX - camX;
    var sy = worldY - camY;
    love.graphics.rectangle("fill", sx, sy, 16.0, 16.0);
}
```

### AABB Collision Detection

```chaiscript
// Returns true if two axis-aligned rectangles overlap
def rectOverlap(ax, ay, aw, ah, bx, by, bw, bh) {
    return ax < bx + bw &&
           ax + aw > bx &&
           ay < by + bh &&
           ay + ah > by;
}

// Check bullet vs enemy
def checkBulletEnemyCollisions() {
    for (var bi = bullets.size() - 1; bi >= 0; bi--) {
        for (var ei = enemies.size() - 1; ei >= 0; ei--) {
            if (rectOverlap(
                    bullets[bi].x - 2, bullets[bi].y - 2, 4.0, 4.0,
                    enemies[ei].x - 8, enemies[ei].y - 8, 16.0, 16.0)) {
                enemies[ei].takeDamage(1);
                bullets[bi].alive = false;
            }
        }
    }
}
```

### Input Edge Detection

`isDown()` returns true every frame the button is held. For actions that should fire once per press (jump, shoot, menu select), track the previous frame's state.

```chaiscript
global prevFire = false;
global prevStart = false;

def inputFirePressed() {
    var cur = love.joystick.isDown(0, "a") || love.keyboard.isDown("z");
    var pressed = cur && !prevFire;
    prevFire = cur;
    return pressed;
}

def inputStartPressed() {
    var cur = love.joystick.isDown(0, "start") || love.keyboard.isDown("return");
    var pressed = cur && !prevStart;
    prevStart = cur;
    return pressed;
}

// Call these in update() — the tracking happens automatically:
def update(dt) {
    if (state == STATE_TITLE) {
        if (inputStartPressed()) {
            state = STATE_PLAYING;
        }
    }
    if (state == STATE_PLAYING) {
        if (inputFirePressed()) {
            spawnBullet();
        }
    }
}
```

### Typewriter Text Effect

```chaiscript
global storyText = "The signal has been silent for three years.";
global storyPos = 0;      // how many characters revealed
global storyTimer = 0.0;
global CHAR_DELAY = 0.04; // seconds per character

def updateTypewriter(dt) {
    storyTimer += dt;
    while (storyTimer >= CHAR_DELAY && storyPos < storyText.size()) {
        storyPos += 1;
        storyTimer -= CHAR_DELAY;
    }
}

def drawTypewriter(x, y) {
    var displayed = storyText.substr(0, storyPos);
    love.graphics.setColor(255, 200, 80, 255);   // amber
    love.graphics.print(displayed, x, y);
}

def isTypewriterDone() {
    return storyPos >= storyText.size();
}

def skipTypewriter() {
    storyPos = storyText.size();
}
```

### CRT Scanline Overlay

Draw horizontal lines with low alpha over the entire screen after all game content:

```chaiscript
def drawScanlines() {
    love.graphics.setColor(0, 0, 0, 40);   // very dark, semi-transparent
    var h = love.graphics.getHeight();
    var w = love.graphics.getWidth();
    var y = 0;
    while (y < h) {
        love.graphics.rectangle("fill", 0, y, w, 1);
        y += 2;
    }
}

// Call at the end of draw(), after everything else:
def draw() {
    love.graphics.clear();
    drawGame();
    drawScanlines();   // on top of everything
}
```

### Screen Shake

```chaiscript
global shakeTimer = 0.0;
global shakeMagnitude = 0.0;
global shakeX = 0.0;
global shakeY = 0.0;

def startShake(duration, magnitude) {
    shakeTimer = duration;
    shakeMagnitude = magnitude;
}

def updateShake(dt) {
    if (shakeTimer > 0.0) {
        shakeTimer -= dt;
        var intensity = shakeTimer / shakeMagnitude;
        shakeX = to_float(love.math.random(-to_int(shakeMagnitude), to_int(shakeMagnitude)));
        shakeY = to_float(love.math.random(-to_int(shakeMagnitude), to_int(shakeMagnitude)));
        if (shakeTimer <= 0.0) {
            shakeX = 0.0;
            shakeY = 0.0;
        }
    }
}

// Apply shake offset to all world-space drawing:
def drawEntity(worldX, worldY) {
    love.graphics.rectangle("fill", worldX + shakeX, worldY + shakeY, 16.0, 16.0);
}
```

### Particle System

```chaiscript
class Particle {
    var x; var y;
    var vx; var vy;
    var life; var maxLife;
    var r; var g; var b;

    def Particle(px, py, pvx, pvy, plife, pr, pg, pb) {
        this.x = px; this.y = py;
        this.vx = pvx; this.vy = pvy;
        this.life = plife; this.maxLife = plife;
        this.r = pr; this.g = pg; this.b = pb;
    }

    def update(dt) {
        this.x += this.vx * dt;
        this.y += this.vy * dt;
        this.vy += 60.0 * dt;   // gravity
        this.life -= dt;
    }

    def draw() {
        var alpha = to_int(255.0 * (this.life / this.maxLife));
        love.graphics.setColor(this.r, this.g, this.b, alpha);
        love.graphics.rectangle("fill", this.x - 1, this.y - 1, 3, 3);
    }
}

global particles = Vector();

def spawnExplosion(px, py, pr, pg, pb) {
    for (var i = 0; i < 12; i++) {
        var angle = to_float(love.math.random(628)) / 100.0;
        var speed = to_float(love.math.random(40, 120));
        particles.push_back(Particle(
            px, py,
            cos(angle) * speed, sin(angle) * speed,
            to_float(love.math.random(40, 80)) / 100.0,
            pr, pg, pb
        ));
    }
}

def updateParticles(dt) {
    for (var i = 0; i < particles.size(); i++) {
        particles[i].update(dt);
    }
    for (var i = particles.size() - 1; i >= 0; i--) {
        if (particles[i].life <= 0.0) {
            particles.erase(i);
        }
    }
}

def drawParticles() {
    for (var i = 0; i < particles.size(); i++) {
        particles[i].draw();
    }
}
```

### Fixed Timestep Accumulator

For consistent physics regardless of frame rate:

```chaiscript
global FIXED_DT = 0.016667;   // 60hz physics step
global accumulator = 0.0;
global MAX_ACCUMULATION = 0.1; // death spiral prevention

def update(dt) {
    accumulator += dt;
    if (accumulator > MAX_ACCUMULATION) {
        accumulator = MAX_ACCUMULATION;
    }

    while (accumulator >= FIXED_DT) {
        physicsStep(FIXED_DT);
        accumulator -= FIXED_DT;
    }
}

def physicsStep(dt) {
    // All physics here, guaranteed to run at 60hz
    for (var i = 0; i < enemies.size(); i++) {
        enemies[i].update(dt);
    }
}
```

---

## Part 5: Deploying to Miyoo Mini Plus

### Directory Structure

A ChaiLove game is a directory with `main.chai` at its root:

```
my-game/
    main.chai          -- required: entry point
    assets/
        player.png     -- optional images
        theme.ogg      -- optional audio (if ChaiLove build supports it)
    levels/
        level1.chai    -- optional: loaded via love.filesystem.load()
```

### Packaging as .chailove

A `.chailove` file is just a ZIP archive with `main.chai` at the root (not inside a subdirectory).

```bash
# From inside the game directory:
cd my-game/
zip -r ../my-game.chailove .

# Verify main.chai is at root of the ZIP (not inside a folder):
unzip -l ../my-game.chailove | head -5
```

The result should show `main.chai` not `my-game/main.chai`.

### Uploading to Miyoo Mini Plus

With OnionOS, the Miyoo appears as a network share or you can SCP to it if SSH is enabled:

```bash
# Enable SSH in OnionOS settings first, then:
scp my-game.chailove root@miyoo-ip:/mnt/SDCARD/Roms/CHAILOVE/

# Or copy directly via USB-mounted SD card:
cp my-game.chailove /run/media/USERNAME/SDCARD/Roms/CHAILOVE/
```

The `CHAILOVE` directory may need to be created if this is the first ChaiLove game:

```bash
mkdir -p /mnt/SDCARD/Roms/CHAILOVE
```

### OnionOS Integration

OnionOS uses an emulator configuration system. The ChaiLove core needs to be registered:

1. Download `chailove_libretro.so` from the ChaiLove releases page and place it in `/mnt/SDCARD/.tmp_update/retroarch/cores/`.

2. Create an emulator entry. In OnionOS, go to **Apps → Package Manager → Emulators** and see if ChaiLove is listed. If not, you can manually create `/mnt/SDCARD/Roms/CHAILOVE/chailove.json` or use RetroArch directly.

3. In RetroArch on the Miyoo, navigate to **Load Content**, browse to `CHAILOVE/`, and select your `.chailove` file. Use **Load Core** to select `chailove_libretro.so`.

4. Once you've loaded it once, RetroArch remembers the association. Files in `Roms/CHAILOVE/` will appear in the Games menu automatically after a rescan.

### RetroArch Launch Command (desktop testing)

To test ChaiLove games on your desktop before uploading:

```bash
# Linux with RetroArch and chailove core:
retroarch -L /path/to/chailove_libretro.so my-game.chailove

# Or run unpackaged during development:
retroarch -L /path/to/chailove_libretro.so /path/to/my-game/main.chai
```

The core also has standalone builds for desktop. Check the ChaiLove releases page for pre-built binaries.

### Button Mapping on Miyoo Mini Plus

Physical button to RetroArch input name:

| Physical Label | RetroArch Name | ChaiLove joystick name |
|---|---|---|
| D-Pad Up | up | `"dpup"` |
| D-Pad Down | down | `"dpdown"` |
| D-Pad Left | left | `"dpleft"` |
| D-Pad Right | right | `"dpright"` |
| A (right face) | b | `"b"` |
| B (bottom face) | a | `"a"` |
| X (top face) | y | `"y"` |
| Y (left face) | x | `"x"` |
| L | leftshoulder | `"leftshoulder"` |
| R | rightshoulder | `"rightshoulder"` |
| Start | start | `"start"` |
| Select | select | `"select"` |
| Menu | n/a | n/a |

Note: The Miyoo Mini Plus labels follow the Nintendo layout (A=confirm, B=cancel), but RetroArch by default uses the Xbox layout internally. The mapping above may vary based on your OnionOS version. Always test on device and provide remapping instructions to players.

### Testing Workflow

The fast iteration loop:

```bash
# 1. Edit main.chai on desktop
# 2. Package and deploy:
cd my-game && zip -r ../my-game.chailove . && scp ../my-game.chailove root@192.168.1.X:/mnt/SDCARD/Roms/CHAILOVE/
# 3. On Miyoo: RetroArch → Load Recent (the .chailove file re-executes fresh each load)
```

For desktop testing during development, the standalone ChaiLove binary is faster:

```bash
# Download standalone chailove binary:
./chailove my-game/   # run unpackaged directory
./chailove my-game.chailove   # run packaged
```

### Performance Tips for ARM Cortex-A7

The Miyoo Mini Plus CPU is not fast. Keep these constraints in mind:

- **Target 320x240**, not 640x480. Quarter the pixel count = 4x the performance margin.
- **Avoid drawing thousands of individual pixels per frame.** Procedural sprites using small rectangles (2x2 or 4x4 pixels each) are fine. Drawing 64 rectangles for an 8x8 sprite is much cheaper than drawing 64 individual 1x1 pixels.
- **Keep entity counts low.** 20-30 enemies, 10-20 bullets, 50 particles is comfortable. 200+ entities will stutter.
- **No floating-point math in hot paths that can be done with int.** For tile positions and indices, use integers throughout.
- **String operations in update() are expensive.** Build display strings in `load()` or only when they change, not every frame. The `"Score: " + to_string(score)` pattern in `draw()` creates a new string every frame — tolerable for a few HUD elements, problematic in a loop.
- **Scanline overlays are expensive.** 120 `rectangle("fill", ...)` calls for a 240-pixel-tall screen adds up. Consider every-4-pixels or skip entirely.
- **Profile on device.** What runs at 200fps on your desktop might run at 45fps on the Miyoo. Always final-test on hardware.

---

## Part 6: Complete Game — Asteroids Clone

This is a complete, runnable ChaiLove game in under 350 lines. Copy it to `main.chai`, package as a `.chailove` file, and it runs.

Features: ship rotation and thrust, asteroid splitting, screen wrap, scoring, lives, title/playing/gameover states, explosion particles, all procedural drawing.

```chaiscript
// ============================================================
// VOIDRIFT — A ChaiLove Asteroids Clone
// ============================================================

global PI      = 3.14159265358979;
global TWO_PI  = PI * 2.0;
global W       = 320.0;
global H       = 240.0;

// --- Game state constants ---
global ST_TITLE    = 0;
global ST_PLAYING  = 1;
global ST_GAMEOVER = 2;

global state       = ST_TITLE;
global score       = 0;
global lives       = 3;
global level       = 1;
global hiScore     = 0;

// --- Input edge tracking ---
global prevFire    = false;
global prevStart   = false;

// --- Screen shake ---
global shakeX      = 0.0;
global shakeY      = 0.0;
global shakeTime   = 0.0;

// --- Particles ---
global particles   = Vector();

// --- Player ---
global px      = 160.0;
global py      = 120.0;
global pvx     = 0.0;
global pvy     = 0.0;
global pAngle  = 0.0;   // radians, 0 = pointing up
global pAlive  = true;
global pRespawn = 0.0;  // respawn countdown

// --- Bullets ---
global bullets = Vector();
global BULLET_SPEED = 200.0;
global BULLET_LIFE  = 1.5;

// --- Asteroids ---
global asteroids = Vector();

// ============================================================
// CONFIG
// ============================================================

def conf(t) {
    t.window.width  = to_int(W);
    t.window.height = to_int(H);
    t.window.title  = "VOIDRIFT";
}

// ============================================================
// INIT
// ============================================================

def load() {
    love.graphics.setDefaultFilter("nearest");
    love.graphics.setBackgroundColor(5, 5, 15);
}

def initGame() {
    score    = 0;
    lives    = 3;
    level    = 1;
    bullets  = Vector();
    particles = Vector();
    resetPlayer();
    spawnAsteroids(4 + level);
}

def resetPlayer() {
    px     = W / 2.0;
    py     = H / 2.0;
    pvx    = 0.0;
    pvy    = 0.0;
    pAngle = 0.0;
    pAlive = true;
    pRespawn = 0.0;
}

// ============================================================
// ASTEROID CLASS
// ============================================================

class Asteroid {
    var x; var y;
    var vx; var vy;
    var size;   // 3=big, 2=medium, 1=small
    var radius;
    var alive;
    var spin;
    var angle;

    def Asteroid(ax, ay, avx, avy, asize) {
        this.x     = ax;
        this.y     = ay;
        this.vx    = avx;
        this.vy    = avy;
        this.size  = asize;
        this.alive = true;
        this.angle = to_float(love.math.random(628)) / 100.0;
        this.spin  = to_float(love.math.random(30, 90)) / 100.0;
        if (love.math.random(2) == 1) { this.spin = -this.spin; }

        if (asize == 3) { this.radius = 22.0; }
        else if (asize == 2) { this.radius = 14.0; }
        else { this.radius = 7.0; }
    }

    def update(dt) {
        this.x += this.vx * dt;
        this.y += this.vy * dt;
        this.angle += this.spin * dt;
        // Wrap
        if (this.x < -this.radius) { this.x = W + this.radius; }
        if (this.x > W + this.radius) { this.x = -this.radius; }
        if (this.y < -this.radius) { this.y = H + this.radius; }
        if (this.y > H + this.radius) { this.y = -this.radius; }
    }

    def draw() {
        var r = this.radius;
        var pts = 7;   // vertices
        var step = TWO_PI / to_float(pts);
        love.graphics.setColor(160, 160, 180, 255);
        for (var i = 0; i < pts; i++) {
            var a1 = this.angle + step * to_float(i);
            var a2 = this.angle + step * to_float(i + 1);
            var jag = to_float(love.math.random(80, 110)) / 100.0;
            love.graphics.line(
                this.x + cos(a1) * r * jag,
                this.y + sin(a1) * r * jag,
                this.x + cos(a2) * r,
                this.y + sin(a2) * r
            );
        }
    }
}

def spawnAsteroids(count) {
    asteroids = Vector();
    for (var i = 0; i < count; i++) {
        var ax = to_float(love.math.random(to_int(W)));
        var ay = to_float(love.math.random(to_int(H)));
        // Keep away from player center
        while (abs(ax - W/2.0) < 50.0 && abs(ay - H/2.0) < 50.0) {
            ax = to_float(love.math.random(to_int(W)));
            ay = to_float(love.math.random(to_int(H)));
        }
        var angle = to_float(love.math.random(628)) / 100.0;
        var speed = to_float(love.math.random(20, 50));
        asteroids.push_back(Asteroid(ax, ay, cos(angle)*speed, sin(angle)*speed, 3));
    }
}

def splitAsteroid(i) {
    var a = asteroids[i];
    asteroids[i].alive = false;

    // Spawn explosion
    spawnExplosion(a.x, a.y, 160, 160, 180);
    startShake(0.15, 4.0);

    if (a.size > 1) {
        var newSize = a.size - 1;
        for (var j = 0; j < 2; j++) {
            var angle = to_float(love.math.random(628)) / 100.0;
            var speed = to_float(love.math.random(40, 90));
            asteroids.push_back(Asteroid(a.x, a.y, cos(angle)*speed, sin(angle)*speed, newSize));
        }
    }
}

// ============================================================
// BULLETS
// ============================================================

class Bullet {
    var x; var y;
    var vx; var vy;
    var life;
    var alive;

    def Bullet(bx, by, bvx, bvy) {
        this.x     = bx; this.y = by;
        this.vx    = bvx; this.vy = bvy;
        this.life  = BULLET_LIFE;
        this.alive = true;
    }

    def update(dt) {
        this.x    += this.vx * dt;
        this.y    += this.vy * dt;
        this.life -= dt;
        if (this.life <= 0.0) { this.alive = false; }
        // Wrap
        if (this.x < 0.0)  { this.x = W; }
        if (this.x > W)    { this.x = 0.0; }
        if (this.y < 0.0)  { this.y = H; }
        if (this.y > H)    { this.y = 0.0; }
    }

    def draw() {
        var alpha = to_int(255.0 * (this.life / BULLET_LIFE));
        love.graphics.setColor(255, 255, 100, alpha);
        love.graphics.circle("fill", this.x, this.y, 2.0);
    }
}

def fireBullet() {
    var bvx = sin(pAngle) * BULLET_SPEED + pvx;
    var bvy = -cos(pAngle) * BULLET_SPEED + pvy;
    bullets.push_back(Bullet(px, py, bvx, bvy));
}

// ============================================================
// PARTICLES
// ============================================================

class Particle {
    var x; var y;
    var vx; var vy;
    var life; var maxLife;
    var r; var g; var b;

    def Particle(px, py, pvx, pvy, plife, pr, pg, pb) {
        this.x = px; this.y = py;
        this.vx = pvx; this.vy = pvy;
        this.life = plife; this.maxLife = plife;
        this.r = pr; this.g = pg; this.b = pb;
    }

    def update(dt) {
        this.x += this.vx * dt;
        this.y += this.vy * dt;
        this.vx *= 0.97;
        this.vy *= 0.97;
        this.life -= dt;
    }

    def draw() {
        var t = this.life / this.maxLife;
        var alpha = to_int(255.0 * t);
        love.graphics.setColor(this.r, this.g, this.b, alpha);
        love.graphics.rectangle("fill", this.x - 1, this.y - 1, 3, 3);
    }
}

def spawnExplosion(ex, ey, er, eg, eb) {
    for (var i = 0; i < 10; i++) {
        var angle = to_float(love.math.random(628)) / 100.0;
        var speed = to_float(love.math.random(30, 100));
        var life  = to_float(love.math.random(40, 80)) / 100.0;
        particles.push_back(Particle(ex, ey, cos(angle)*speed, sin(angle)*speed, life, er, eg, eb));
    }
}

// ============================================================
// SCREEN SHAKE
// ============================================================

def startShake(duration, magnitude) {
    shakeTime = duration;
    shakeX = magnitude;
    shakeY = magnitude;
}

def updateShake(dt) {
    if (shakeTime > 0.0) {
        shakeTime -= dt;
        var m = shakeTime * 20.0;
        shakeX = to_float(love.math.random(to_int(-m), to_int(m)));
        shakeY = to_float(love.math.random(to_int(-m), to_int(m)));
        if (shakeTime <= 0.0) {
            shakeX = 0.0; shakeY = 0.0;
        }
    }
}

// ============================================================
// INPUT HELPERS
// ============================================================

def keyLeft()  { return love.keyboard.isDown("left")  || love.joystick.isDown(0, "dpleft");  }
def keyRight() { return love.keyboard.isDown("right") || love.joystick.isDown(0, "dpright"); }
def keyUp()    { return love.keyboard.isDown("up")    || love.joystick.isDown(0, "dpup");    }

def keyFireNow() {
    var cur = love.keyboard.isDown("z") || love.keyboard.isDown("space") || love.joystick.isDown(0, "a");
    var pressed = cur && !prevFire;
    prevFire = cur;
    return pressed;
}

def keyStartNow() {
    var cur = love.keyboard.isDown("return") || love.joystick.isDown(0, "start");
    var pressed = cur && !prevStart;
    prevStart = cur;
    return pressed;
}

// ============================================================
// UPDATE
// ============================================================

def update(dt) {
    updateShake(dt);

    if (state == ST_TITLE) {
        if (keyStartNow()) {
            initGame();
            state = ST_PLAYING;
        }
        return;
    }

    if (state == ST_GAMEOVER) {
        if (keyStartNow()) {
            state = ST_TITLE;
        }
        return;
    }

    // --- ST_PLAYING ---

    // Update particles
    for (var i = 0; i < particles.size(); i++) { particles[i].update(dt); }
    for (var i = particles.size() - 1; i >= 0; i--) {
        if (particles[i].life <= 0.0) { particles.erase(i); }
    }

    // Player respawn countdown
    if (!pAlive) {
        pRespawn -= dt;
        if (pRespawn <= 0.0) {
            if (lives <= 0) {
                if (score > hiScore) { hiScore = score; }
                state = ST_GAMEOVER;
            } else {
                resetPlayer();
            }
        }
        // Still update asteroids and bullets while dead
    }

    // Player controls
    if (pAlive) {
        var TURN_SPEED = 3.2;
        var THRUST     = 140.0;
        var DRAG       = 0.985;

        if (keyLeft())  { pAngle -= TURN_SPEED * dt; }
        if (keyRight()) { pAngle += TURN_SPEED * dt; }
        if (keyUp()) {
            pvx += sin(pAngle) * THRUST * dt;
            pvy -= cos(pAngle) * THRUST * dt;
        }

        pvx *= DRAG;
        pvy *= DRAG;
        px  += pvx * dt;
        py  += pvy * dt;

        // Wrap player
        if (px < 0.0)  { px = W; }
        if (px > W)    { px = 0.0; }
        if (py < 0.0)  { py = H; }
        if (py > H)    { py = 0.0; }

        if (keyFireNow()) { fireBullet(); }
    }

    // Update bullets
    for (var i = 0; i < bullets.size(); i++) { bullets[i].update(dt); }
    for (var i = bullets.size() - 1; i >= 0; i--) {
        if (!bullets[i].alive) { bullets.erase(i); }
    }

    // Update asteroids
    for (var i = 0; i < asteroids.size(); i++) { asteroids[i].update(dt); }

    // Bullet-asteroid collisions
    for (var bi = bullets.size() - 1; bi >= 0; bi--) {
        for (var ai = asteroids.size() - 1; ai >= 0; ai--) {
            if (!bullets[bi].alive || !asteroids[ai].alive) { continue; }
            var dx = bullets[bi].x - asteroids[ai].x;
            var dy = bullets[bi].y - asteroids[ai].y;
            var dist = sqrt(dx*dx + dy*dy);
            if (dist < asteroids[ai].radius) {
                bullets[bi].alive = false;
                var pts = 0;
                if (asteroids[ai].size == 3) { pts = 20; }
                else if (asteroids[ai].size == 2) { pts = 50; }
                else { pts = 100; }
                score += pts;
                splitAsteroid(ai);
            }
        }
    }

    // Player-asteroid collisions
    if (pAlive) {
        for (var ai = 0; ai < asteroids.size(); ai++) {
            if (!asteroids[ai].alive) { continue; }
            var dx = px - asteroids[ai].x;
            var dy = py - asteroids[ai].y;
            var dist = sqrt(dx*dx + dy*dy);
            if (dist < asteroids[ai].radius + 7.0) {
                pAlive = false;
                pRespawn = 2.0;
                lives -= 1;
                spawnExplosion(px, py, 100, 200, 255);
                startShake(0.4, 8.0);
            }
        }
    }

    // Remove dead asteroids
    for (var i = asteroids.size() - 1; i >= 0; i--) {
        if (!asteroids[i].alive) { asteroids.erase(i); }
    }

    // Next level
    if (asteroids.size() == 0) {
        level += 1;
        spawnAsteroids(3 + level);
    }
}

// ============================================================
// DRAW
// ============================================================

def drawShip(x, y, angle, thrusting) {
    var tip_x = x + sin(angle) * 10.0;
    var tip_y = y - cos(angle) * 10.0;
    var left_x = x + sin(angle - 2.4) * 7.0;
    var left_y = y - cos(angle - 2.4) * 7.0;
    var right_x = x + sin(angle + 2.4) * 7.0;
    var right_y = y - cos(angle + 2.4) * 7.0;
    var back_x = x + sin(angle + PI) * 4.0;
    var back_y = y - cos(angle + PI) * 4.0;

    love.graphics.setColor(100, 200, 255, 255);
    love.graphics.line(tip_x, tip_y, left_x, left_y);
    love.graphics.line(tip_x, tip_y, right_x, right_y);
    love.graphics.line(left_x, left_y, back_x, back_y);
    love.graphics.line(right_x, right_y, back_x, back_y);

    // Thruster flame
    if (thrusting) {
        love.graphics.setColor(255, 140, 0, 200);
        var flame_x = x + sin(angle + PI) * 10.0;
        var flame_y = y - cos(angle + PI) * 10.0;
        love.graphics.line(back_x, back_y, flame_x, flame_y);
    }
}

def drawHUD() {
    love.graphics.setColor(200, 200, 220, 255);
    love.graphics.print("SCORE " + to_string(score), 8.0, 8.0);
    love.graphics.print("LVL " + to_string(level), 8.0, 22.0);
    love.graphics.print("HI " + to_string(hiScore), W - 80.0, 8.0);

    // Lives as small ship icons
    for (var i = 0; i < lives; i++) {
        drawShip(W - 20.0 - to_float(i) * 18.0, 30.0, 0.0, false);
    }
}

def drawScanlines() {
    love.graphics.setColor(0, 0, 0, 30);
    var y = 0;
    while (y < to_int(H)) {
        love.graphics.rectangle("fill", 0, y, to_int(W), 1);
        y += 3;
    }
}

def drawTitle() {
    love.graphics.clear(5, 5, 15);

    // Title
    love.graphics.setColor(100, 200, 255, 255);
    love.graphics.print("V O I D R I F T", 90.0, 70.0);
    love.graphics.setColor(160, 160, 180, 255);
    love.graphics.print("ASTEROID HUNTER", 98.0, 90.0);

    // Demo asteroids silhouette
    love.graphics.setColor(60, 60, 80, 255);
    love.graphics.circle("fill", 80.0, 160.0, 20.0);
    love.graphics.circle("fill", 220.0, 150.0, 14.0);
    love.graphics.circle("fill", 155.0, 170.0, 10.0);

    love.graphics.setColor(80, 255, 80, 255);
    love.graphics.print("PRESS START", 110.0, 130.0);

    love.graphics.setColor(120, 120, 140, 255);
    love.graphics.print("ARROW KEYS: STEER", 90.0, 196.0);
    love.graphics.print("UP: THRUST   Z: FIRE", 84.0, 210.0);

    drawScanlines();
}

def drawGameOver() {
    love.graphics.clear(5, 0, 0);

    love.graphics.setColor(255, 60, 60, 255);
    love.graphics.print("S H I P   L O S T", 96.0, 80.0);

    love.graphics.setColor(200, 200, 200, 255);
    love.graphics.print("FINAL SCORE: " + to_string(score), 100.0, 110.0);

    if (score >= hiScore && score > 0) {
        love.graphics.setColor(255, 220, 0, 255);
        love.graphics.print("NEW HIGH SCORE!", 103.0, 128.0);
    }

    love.graphics.setColor(80, 255, 80, 255);
    love.graphics.print("PRESS START", 110.0, 160.0);

    drawScanlines();
}

def draw() {
    love.graphics.clear();

    if (state == ST_TITLE) {
        drawTitle();
        return;
    }

    if (state == ST_GAMEOVER) {
        drawGameOver();
        return;
    }

    // --- Draw playing state ---
    // Apply screen shake offset to a translated space manually
    // (ChaiLove has no push/pop transform, so we pass offsets to draw calls
    //  or just apply shake only to the playfield background)

    // Stars (static — draw without shake)
    love.graphics.setColor(255, 255, 255, 80);
    // Fixed star pattern using predictable positions
    love.graphics.point(45.0, 32.0);  love.graphics.point(130.0, 18.0);
    love.graphics.point(210.0, 55.0); love.graphics.point(290.0, 22.0);
    love.graphics.point(75.0, 120.0); love.graphics.point(185.0, 95.0);
    love.graphics.point(250.0, 180.0);love.graphics.point(30.0, 200.0);

    // Asteroids
    for (var i = 0; i < asteroids.size(); i++) {
        asteroids[i].draw();
    }

    // Bullets
    for (var i = 0; i < bullets.size(); i++) {
        bullets[i].draw();
    }

    // Player ship
    if (pAlive) {
        var thrusting = keyUp();
        drawShip(px + shakeX, py + shakeY, pAngle, thrusting);
    } else if (pRespawn > 1.0) {
        // Show "RESPAWNING" text
        love.graphics.setColor(100, 200, 255, 180);
        love.graphics.print("RESPAWNING...", 115.0, 110.0);
    }

    // Particles
    for (var i = 0; i < particles.size(); i++) {
        particles[i].draw();
    }

    drawHUD();
    drawScanlines();
}
```

### What the Code Demonstrates

Reading through `VOIDRIFT` end to end, you can trace every pattern from Part 4:

- **State machine** — `ST_TITLE`, `ST_PLAYING`, `ST_GAMEOVER` with clean transitions in `update()` and `draw()`.
- **Classes** — `Asteroid`, `Bullet`, `Particle` with constructors and `update()`/`draw()` methods.
- **Vectors with backwards erase** — every cleanup loop iterates `size()-1` down to `0`.
- **Input edge detection** — `keyFireNow()` and `keyStartNow()` track previous frame state.
- **Procedural drawing** — ship drawn with `line()` calls, asteroids with a polygon loop.
- **Particles** — `spawnExplosion()` scatters velocity particles with alpha fade.
- **Screen shake** — `startShake()` / `updateShake()` with magnitude falloff.
- **Dual input** — keyboard and joystick both checked in every input helper.
- **Wrap-around** — asteroids, bullets, and player all wrap at screen edges.
- **Level progression** — asteroid count increases with `level`, resetting when the field is clear.

### Next Steps

Once `VOIDRIFT` runs, extend it:

- **Sound** — if your ChaiLove build includes audio, add `love.audio.newSource("fire.wav")` in `load()` and `.play()` on fire/explosion.
- **High score persistence** — ChaiLove's `love.filesystem` can write text files to a save directory. Read and write the high score to `"hiscore.txt"`.
- **Animated sprites** — add an `animTimer` field to entities, advance it in `update()`, and select a different sprite row from your sprite definition based on `to_int(animTimer * FPS) % frameCount`.
- **Power-ups** — add a `PowerUp` class, spawn on large asteroid destruction, check overlap with player, apply effect (triple shot, shield, speed boost).
- **Story screen** — add a `ST_STORY` state with a typewriter effect (see Part 4) that plays between levels. "SECTOR CLEAR. DEEPER SIGNALS DETECTED. PROCEEDING."

---

## Quick Reference Card

```
SETUP
  def conf(t)          window config
  def load()           startup init
  def update(dt)       logic, dt=seconds
  def draw()           render

GRAPHICS
  love.graphics.setColor(r,g,b,a)     0-255 range
  love.graphics.clear()
  love.graphics.rectangle("fill"|"line", x,y,w,h)
  love.graphics.circle("fill"|"line", x,y,r)
  love.graphics.line(x1,y1,x2,y2)
  love.graphics.print(text, x, y)
  love.graphics.getWidth() / getHeight()

INPUT
  love.keyboard.isDown("left"|"right"|"up"|"down"|"space"|"z"|"return")
  love.joystick.isDown(0, "a"|"b"|"x"|"y"|"start"|"select"|"dpup"...)

MATH
  sin(r) cos(r) sqrt(x) abs(x) floor(x) ceil(x)
  to_int(x) to_float(x) to_string(x)
  love.math.random() love.math.random(max) love.math.random(min,max)
  love.math.rad(deg)

CONTAINERS
  var v = Vector()
  v.push_back(x)       add element
  v.size()             length
  v[i]                 access / assign
  v.erase(i)           remove by index
  for (item : v) {}    range loop (copy)

GOTCHAS
  - No switch — use if/else if chains
  - "text" + to_string(num) — always convert numbers
  - Range-for gives copies — use index loop to modify in place
  - Integer division: 7/2 == 3, use 7.0/2 for float
  - No PI constant — define: global PI = 3.14159265358979;
  - Erase while iterating — go backwards (size-1 down to 0)

DEPLOY
  zip -r game.chailove main.chai assets/
  scp game.chailove root@miyoo:/mnt/SDCARD/Roms/CHAILOVE/
```
