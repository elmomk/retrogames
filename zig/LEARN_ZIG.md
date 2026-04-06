# Learning Zig for Game Development
## From Zero to SDL2 Games on the Miyoo Mini Plus

**Prerequisites:** You know Rust and C. You want to write games in Zig.
**Target:** Zig 0.13+ | ARM Cortex-A7 | SDL2 | 640x480 | 60fps

---

## Table of Contents

1. [Why Zig](#part-1-zig-fundamentals)
2. [Fundamentals](#variables-types-and-memory-model)
3. [Zig's Unique Powers](#part-2-zigs-unique-powers)
4. [Game Dev Patterns](#part-3-patterns-for-game-development)
5. [SDL2 in Zig](#part-4-sdl2-in-zig--complete-reference)
6. [Cross-Compilation for Miyoo](#part-5-cross-compilation-for-miyoo-mini-plus)
7. [Complete Pong Example](#part-6-complete-game-example)

---

# Part 1: Zig Fundamentals

## Why Zig (Honest Tradeoffs)

Zig sits in a specific niche. Here is what it actually buys you versus your alternatives:

**vs C:**
- No implicit undefined behavior — you still get UB but it's explicit (`undefined` is a value you assign)
- Error handling is first-class (`!T` instead of checking return codes)
- The build system is part of the language (no CMake, no Makefile madness)
- Cross-compilation is a first-class feature, not a hack
- Comptime replaces preprocessor macros with real code
- Null safety with optionals (`?T`)

**vs Rust:**
- No borrow checker. You manage memory manually. This is both freedom and responsibility.
- No trait system, no lifetimes, no generics syntax — comptime handles all of it
- C interop is trivial: `@cImport` and you're done. In Rust you write bindgen and pray.
- Compile times are dramatically faster
- The language is smaller — the spec fits in your head
- LSP/IDE support is worse (ZLS lags the compiler)
- No memory safety guarantees. Use-after-free is possible. Zig trusts you.
- Ecosystem is tiny. If it's not in the stdlib, you write it yourself.

**The honest pitch:** If you're shipping to an embedded device with a known SDL2 ABI and you want to call C libraries without FFI ceremony, Zig is excellent. For a project where you're the only developer and you know the memory layout, the lack of borrow checker is a feature, not a bug.

**Where Zig is genuinely worse than Rust:**
- No fearless concurrency guarantees
- Compiler error messages are improving but still lose to Rust
- IDE completion is unreliable with ZLS on complex comptime
- The ecosystem is pre-1.0. APIs change. `std.ArrayList` changed calling conventions between 0.11 and 0.13.
- Stack traces on embedded are less helpful than Rust's `backtrace`

---

## Installing and Toolchain

```bash
# Download from https://ziglang.org/download/
# Pick the 0.13.0 tarball for your host (x86_64-linux in our case)
wget https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz
tar xf zig-linux-x86_64-0.13.0.tar.xz
export PATH="$PATH:$PWD/zig-linux-x86_64-0.13.0"

zig version  # should print 0.13.0
```

**Key commands:**

```bash
zig run src/main.zig              # compile and run (no build.zig needed)
zig build                         # use build.zig in current dir
zig build run                     # build and run the default exe
zig build test                    # run all test blocks
zig fmt src/main.zig              # format in-place (like rustfmt)
zig build -Dtarget=arm-linux-gnueabihf  # cross-compile
```

**Project layout:**

```
my_game/
  build.zig          # build script (written in Zig)
  src/
    main.zig         # entry point
    game.zig         # game logic
    sprites.zig      # sprite data
```

---

## Variables, Types, and Memory Model

### const and var

In Rust, `let` is immutable and `let mut` is mutable. In Zig: `const` and `var`.

```zig
const x: i32 = 42;         // immutable, type explicit
const y = 42;               // immutable, type inferred (comptime_int)
var score: u32 = 0;         // mutable
var lives: u8 = undefined;  // mutable, NOT initialized — reading this is illegal
```

`undefined` in Zig is not zero. In debug builds, it fills memory with `0xaa` bytes to catch reads. In release, it's whatever is in memory. Never read `undefined` — it is a deliberate crash on debug builds.

In Rust you'd write `let mut lives: u8;` and the compiler stops you from reading it. Zig relies on discipline and debug mode traps.

### Integer Types

```zig
// Signed
i8, i16, i32, i64, i128
// Unsigned
u8, u16, u32, u64, u128
// Pointer-sized (like Rust's isize/usize)
isize, usize
// Arbitrary width (Zig-specific, really useful)
u1, u3, u7, u12, u24  // any bit width you want
```

Integer overflow is a compile error for constants, a runtime panic in debug mode, and **silent wraparound** in release mode unless you use checked ops. This is different from Rust where you choose at call site. In Zig you must be deliberate:

```zig
const a: u8 = 200;
const b: u8 = 100;

// These panic in debug, silent wraparound in release:
const bad = a + b;  // 300 doesn't fit in u8

// Safe alternatives:
const checked = @addWithOverflow(a, b);  // returns struct { result, overflow_bit }
const saturated = @min(a + b, 255);      // manual saturation
const wrapped = a +% b;                  // explicit wrapping add (like Rust's wrapping_add)
```

### Float Types

```zig
const pi: f32 = 3.14159;
const pi64: f64 = 3.141592653589793;
```

Zig has `f16`, `f32`, `f64`, `f80`, `f128`. For games on ARM, use `f32` everywhere — the Miyoo's VFP handles it natively.

### Casting

Zig has no implicit casting. Period. This is stricter than C and as strict as Rust.

```zig
const x: i32 = 42;
const y: f32 = @floatFromInt(x);   // i32 -> f32
const z: i32 = @intFromFloat(y);   // f32 -> i32 (truncates, not rounds)
const w: u8 = @intCast(x);         // narrowing cast, panics if x > 255 in debug
const v: i64 = @as(i64, x);        // widening, always safe, @as is Zig's "as" keyword
```

The common builtins for casting:
- `@floatFromInt(x)` — integer to float
- `@intFromFloat(x)` — float to integer (truncates toward zero)
- `@intCast(x)` — integer to integer, panics on overflow in debug
- `@truncate(x)` — integer narrowing, silently drops high bits
- `@as(T, x)` — type coercion where safe

In Rust you'd write `x as f32` for everything. In Zig you pick the right builtin. The verbosity is intentional: it forces you to think about what conversion you want.

### bool and void

```zig
const alive: bool = true;
const result: void = {};  // void is a real type with one value: {}
```

`void` as a function return type means "returns nothing." `void` as a value is `{}`.

---

## Optionals: ?T

Zig's nullable type. Like Rust's `Option<T>` but with terser syntax.

```zig
// Declaration
var maybe_texture: ?*SDL_Texture = null;
var player_target: ?Vec2 = null;

// Unwrap with if-capture (like Rust's if let Some(x) = ...)
if (maybe_texture) |tex| {
    SDL_RenderCopy(renderer, tex, null, null);
} else {
    // tex is null here
}

// orelse: provide a default if null (like Rust's unwrap_or)
const tex = maybe_texture orelse return;        // return from function
const tex2 = maybe_texture orelse default_tex;  // use fallback value
const tex3 = maybe_texture orelse unreachable;  // assert non-null (crashes on null)

// .? shorthand: unwrap or unreachable (like Rust's .unwrap())
const tex4 = maybe_texture.?;  // panics in debug if null
```

Null pointer crashes are a major source of bugs in C. Zig forces you to handle the null case — you cannot pass `?*T` where `*T` is expected without unwrapping.

---

## Error Handling: !T — THE Killer Feature

This is the biggest reason to choose Zig over C for systems code.

In C you check return values manually and forget. In Rust you have `Result<T, E>`. In Zig, errors are first-class but simpler than Rust.

```zig
// An error set
const SDLError = error{
    InitFailed,
    WindowCreationFailed,
    RendererCreationFailed,
};

// A function that can fail
fn createWindow(title: [*:0]const u8, w: i32, h: i32) !*SDL_Window {
    //                                                 ^ "!T" = "anyerror!T"
    const window = SDL_CreateWindow(title, 0, 0, w, h, 0) orelse {
        return error.WindowCreationFailed;
    };
    return window;
}

// Calling it
pub fn main() !void {
    const win = try createWindow("Game", 640, 480);
    //          ^ try = "return err if this fails"
    defer SDL_DestroyWindow(win);
    // ...
}
```

**`try`** propagates the error up the call stack. It is exactly `return if (result) |val| val else |err| err`. In Rust this is `?`.

**`catch`** handles errors inline:

```zig
// catch with a block
const win = createWindow("Game", 640, 480) catch |err| {
    std.log.err("Failed to create window: {}", .{err});
    return;
};

// catch with a default value
const value = parseInt(str) catch 0;

// catch unreachable — "this should never fail" (panics if it does)
const win = createWindow("Game", 640, 480) catch unreachable;
```

**Error types are inferred from the function body.** You declare `!T` and Zig figures out the error set. You can also name it:

```zig
fn loadFile(path: []const u8) (std.fs.File.OpenError || error{TooLarge})![]u8 {
    // ...
}
```

**Comparison to Rust:**
- Rust: `Result<T, E>` where E is a type you choose (often a boxed trait object)
- Zig: `E!T` where E is an error set (global namespace, no allocation needed)
- `try` in both languages propagates errors up
- Zig error sets are more lightweight — they're u16 values under the hood, no heap allocation
- Rust's `?` can coerce between error types with `From`. Zig requires explicit conversion or `anyerror`.

---

## Strings: []const u8

Zig has no string type. Strings are slices of bytes.

```zig
const greeting: []const u8 = "Hello, world!";
const greeting_z: [*:0]const u8 = "Hello, world!";  // null-terminated (for C APIs)

// String literals are []const u8 with a known comptime length
// They live in read-only memory (like C string literals)

// Concatenate at comptime
const full = "Hello" ++ ", " ++ "world!";

// Repeat at comptime
const bar = "-" ** 40;  // "----------------------------------------"
```

**Slices vs C strings:**
- `[]const u8` — fat pointer: pointer + length. No null terminator needed. Cannot be passed to C.
- `[*:0]const u8` — C-style null-terminated pointer. Needed for C functions like `SDL_CreateWindow`.
- `[*]const u8` — many-item pointer, no length info (like C's `char*`). Dangerous.

Converting between them:

```zig
// Zig string literal to null-terminated (safe, it IS null terminated at compile time)
const title: [*:0]const u8 = "My Game";

// Slice to null-terminated at runtime (requires allocator)
const c_str = try std.fmt.allocPrintZ(allocator, "{s}", .{zig_slice});
defer allocator.free(c_str);

// Null-terminated to slice (if you know the length)
const slice = std.mem.sliceTo(c_ptr, 0);  // scans for null byte
```

---

## Arrays and Slices

```zig
// Fixed-size array — size is part of the type
const tiles: [32]u8 = [_]u8{0} ** 32;  // zero-initialized, [_] infers length
var positions: [100]f32 = undefined;    // uninitialized

// Array literal
const palette = [_][3]u8{
    .{ 255, 0, 0 },
    .{ 0, 255, 0 },
    .{ 0, 0, 255 },
};

// Slice — pointer + length, does NOT own the memory
const all: []const u8 = &tiles;     // coerce array to slice
const part: []const u8 = tiles[4..8]; // slice of 4 elements
```

**Slices are safe.** Indexing out of bounds panics in debug mode (with a useful message), unlike C where it silently corrupts memory.

```zig
// This panics in debug with "index out of bounds"
const x = tiles[99];  // tiles is 32 elements

// Safe iteration
for (tiles) |tile| {
    // tile is a copy of each element
}

for (tiles, 0..) |tile, i| {
    // i is the index
}

// Mutable iteration
for (&positions) |*pos| {
    pos.* += 1.0;
}
```

---

## Structs

Structs are the backbone of game data layout. Zig structs are value types (copied on assignment unless you use pointers), like C, unlike Rust where you have to think about moves.

```zig
const Vec2 = struct {
    x: f32,
    y: f32,

    // Methods are just functions in the namespace
    pub fn add(self: Vec2, other: Vec2) Vec2 {
        return .{ .x = self.x + other.x, .y = self.y + other.y };
    }

    pub fn length(self: Vec2) f32 {
        return @sqrt(self.x * self.x + self.y * self.y);
    }

    // Mutable method — takes pointer
    pub fn normalize(self: *Vec2) void {
        const len = self.length();
        if (len > 0.0) {
            self.x /= len;
            self.y /= len;
        }
    }
};

// Default values in struct fields
const Player = struct {
    x: f32 = 0.0,
    y: f32 = 0.0,
    vx: f32 = 0.0,
    vy: f32 = 0.0,
    lives: u8 = 3,
    score: u32 = 0,
    alive: bool = true,
};

// Instantiation
const p1 = Player{};                    // all defaults
const p2 = Player{ .x = 100, .y = 50 }; // partial init, rest is defaults
var p3 = Player{ .x = 200, .y = 100, .lives = 5 };

// Struct update syntax (like Rust's ..player)
// Zig doesn't have this — you must copy manually or use a function
```

**Anonymous struct literals** (used everywhere in Zig):

```zig
// When the type is known from context, you can omit the name
const point: Vec2 = .{ .x = 1.0, .y = 2.0 };

// In function calls
draw(renderer, .{ .x = 100, .y = 50 });
```

---

## Enums and Tagged Unions

```zig
// Simple enum (like C enum or Rust enum without data)
const GameState = enum {
    MainMenu,
    Playing,
    Paused,
    GameOver,
    Victory,
};

// Enums with methods
const Direction = enum {
    North, South, East, West,

    pub fn opposite(self: Direction) Direction {
        return switch (self) {
            .North => .South,
            .South => .North,
            .East  => .West,
            .West  => .East,
        };
    }
};

// Tagged union — like Rust enums with data
// This is THE pattern for game state machines
const UIEvent = union(enum) {
    key_down: u32,           // carries a keycode
    mouse_click: struct {    // carries an anonymous struct
        x: i32,
        y: i32,
        button: u8,
    },
    window_resize: struct { w: i32, h: i32 },
    quit,                    // no payload
};

// Using a tagged union
fn handleEvent(event: UIEvent) void {
    switch (event) {
        .key_down  => |key| processKey(key),
        .mouse_click => |click| processClick(click.x, click.y),
        .window_resize => |size| resize(size.w, size.h),
        .quit => std.process.exit(0),
    }
    // switch on tagged unions MUST be exhaustive
    // missing a case is a compile error
}
```

**This is the key difference from C unions.** Zig tagged unions know which field is active and won't let you read the wrong one (it panics in debug). In C you have to track this yourself.

---

## Control Flow

```zig
// if/else — condition must be bool (no implicit integer truthiness like C)
if (lives > 0) {
    // ...
} else if (lives == 0) {
    // ...
} else {
    // ...
}

// if as expression
const msg = if (score > 1000) "High score!" else "Keep trying";

// while
var i: u32 = 0;
while (i < 10) : (i += 1) {
    // : (i += 1) is the "continue expression" — runs before each iteration check
    // This is Zig's version of for(;;) init/condition/increment
}

// while with else (runs if condition was never true)
while (queue.pop()) |item| {
    process(item);
} else {
    std.debug.print("Queue was empty\n", .{});
}

// for — iterates over a slice or range
for (items) |item| { _ = item; }
for (items, 0..) |item, idx| { _ = idx; _ = item; }

// Range (Zig 0.12+)
for (0..100) |i| {
    _ = i;
}

// switch — MUST cover all cases, no fallthrough
switch (state) {
    .MainMenu => drawMenu(renderer),
    .Playing  => updateGame(),
    .Paused   => drawPauseScreen(renderer),
    .GameOver,
    .Victory  => drawEndScreen(renderer),  // multiple cases share one branch
}

// switch with capture
switch (event.type) {
    SDL_KEYDOWN => |ev| handleKey(ev.keysym.sym),
    else => {},  // catch-all
}
```

**Labeled break and continue** — Zig's answer to goto for nested loops:

```zig
outer: for (map, 0..) |row, y| {
    for (row, 0..) |cell, x| {
        if (cell == TARGET) {
            found_x = x;
            found_y = y;
            break :outer;  // break the OUTER loop by name
        }
    }
}

// Labeled block — returns a value
const clamped = clamp: {
    if (x < 0) break :clamp 0;
    if (x > 100) break :clamp 100;
    break :clamp x;
};
```

---

## Functions

```zig
// Basic function
fn add(a: i32, b: i32) i32 {
    return a + b;
}

// Void return (no return statement needed if void)
fn drawBullet(renderer: *Renderer, x: i32, y: i32) void {
    fillRect(renderer, x, y, 4, 4, 255, 255, 0, 255);
}

// Error union return
fn loadTexture(renderer: *Renderer, path: []const u8) !*Texture {
    _ = path;
    return error.NotImplemented;
}

// Optional return
fn findEnemy(enemies: []Enemy, id: u32) ?*Enemy {
    for (enemies) |*enemy| {
        if (enemy.id == id) return enemy;
    }
    return null;
}

// Comptime parameter (covered in Part 2)
fn makeArray(comptime T: type, comptime n: usize) [n]T {
    return [_]T{undefined} ** n;
}

// Export for C interop
export fn game_init() void {
    // callable from C as game_init()
}

// Inline (like C's inline keyword)
inline fn lerp(a: f32, b: f32, t: f32) f32 {
    return a + (b - a) * t;
}
```

**Functions are not closures.** Zig has no closures. If you need to capture state, pass it explicitly or use a struct with a method. This is intentional — no hidden allocations, no surprise heap use.

---

## Pointers

Zig has several pointer types. Get this right and C interop becomes natural.

```zig
// Single-item pointer — points to exactly one T
const ptr: *i32 = &my_int;
ptr.* = 42;             // dereference with .*

// Const pointer — cannot modify through the pointer
const cptr: *const i32 = &my_int;
// cptr.* = 42;  // compile error

// Many-item pointer — like C's T*, no length info
// DANGEROUS, but needed for some C APIs
const many: [*]u8 = &buffer;
const byte = many[0];     // indexing is allowed but unchecked

// Null-terminated many-item pointer — C string
const cstr: [*:0]const u8 = "hello";

// Optional pointer (nullable) — like C's nullable T*
var opt_ptr: ?*i32 = null;
opt_ptr = &my_int;
if (opt_ptr) |p| {
    p.* += 1;
}
```

**Pointer arithmetic** — rare in Zig, but available:

```zig
// Advance a many-item pointer
const start: [*]u8 = &buffer;
const next: [*]u8 = start + 1;

// Convert pointer to integer (for packed bit manipulation)
const addr: usize = @intFromPtr(ptr);
const back: *u8 = @ptrFromInt(addr);

// Reinterpret cast (like C's type-punning)
const bytes: *[4]u8 = @ptrCast(&my_f32);
```

---

## Memory: Allocators

This is where Zig diverges most from both C and Rust. **Every allocation is explicit, and you choose the allocator.**

Rust has a global allocator and smart pointers that handle deallocation. C has `malloc`/`free` with no tracking. Zig has allocator interfaces passed explicitly to functions that need them.

```zig
const std = @import("std");
const Allocator = std.mem.Allocator;

// The four allocators you'll use:

// 1. Page allocator — direct mmap/VirtualAlloc, no overhead
//    Use for: long-lived allocations, top-level init
var page = std.heap.page_allocator;

// 2. General Purpose Allocator — tracks leaks and use-after-free in debug
//    Use for: development, catching bugs
var gpa = std.heap.GeneralPurposeAllocator(.{}){};
defer _ = gpa.deinit();  // reports leaks on exit in debug mode
const alloc = gpa.allocator();

// 3. Arena allocator — bulk-free everything at once
//    Use for: per-frame allocations, parsing, anything with a clear lifetime
var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
defer arena.deinit();
const arena_alloc = arena.allocator();
// All allocations freed when arena.deinit() is called

// 4. Fixed Buffer Allocator — allocates from a stack buffer, no heap
//    Use for: embedded, no-heap contexts, small temp allocations
var buf: [4096]u8 = undefined;
var fba = std.heap.FixedBufferAllocator.init(&buf);
const fba_alloc = fba.allocator();
```

**Allocating memory:**

```zig
// Single item
const player = try alloc.create(Player);  // returns *Player
defer alloc.destroy(player);              // free single item

// Slice
const buf = try alloc.alloc(u8, 1024);    // returns []u8
defer alloc.free(buf);

// Resize
buf = try alloc.realloc(buf, 2048);

// Zeroed allocation
const buf_zero = try alloc.allocSentinel(u8, 1024, 0);  // null-terminated
```

**`defer` — your cleanup tool:**

```zig
// defer runs when the current scope exits (including on error)
fn loadAssets(alloc: Allocator) !Assets {
    const sprite_buf = try alloc.alloc(u8, 1024);
    errdefer alloc.free(sprite_buf);  // only runs on ERROR exit
    
    const sound_buf = try alloc.alloc(u8, 512);
    defer alloc.free(sound_buf);      // always runs
    
    // If any `try` below fails, errdefer cleans up sprite_buf
    const sprites = try loadSprites(sprite_buf);
    return Assets{ .sprites = sprites };
}
```

`errdefer` is like Rust's `Drop` triggered on error. `defer` is like Go's defer or a destructor.

**No hidden allocations** — this is the contract Zig makes with you. If a function takes no allocator, it will not allocate. You can audit every allocation site. This matters enormously on embedded hardware.

---

# Part 2: Zig's Unique Powers

## Comptime: Compile-Time Code Execution

This is Zig's most distinctive feature. `comptime` is not a separate template language like C++ templates — it is ordinary Zig code that runs at compile time. Types are first-class values.

### Comptime Parameters

```zig
// Generic function — T is known at compile time
fn max(comptime T: type, a: T, b: T) T {
    return if (a > b) a else b;
}

const x = max(i32, 5, 10);      // T=i32 at compile time
const y = max(f32, 3.14, 2.72); // T=f32 at compile time
```

Compare to Rust: `fn max<T: PartialOrd>(a: T, b: T) -> T`. Zig has no trait bounds syntax — you just use the type and the compiler tells you if it doesn't support the operations you used.

### Comptime Blocks

```zig
// Code that runs at compile time
const lookup = comptime blk: {
    var table: [256]u8 = undefined;
    for (&table, 0..) |*entry, i| {
        entry.* = if (i < 128) @intCast(i) else 0;
    }
    break :blk table;
};
// `lookup` is a compile-time constant [256]u8
```

### Type as a Value

```zig
// Type reflection at compile time
fn printTypeInfo(comptime T: type) void {
    const info = @typeInfo(T);
    switch (info) {
        .Struct => |s| {
            std.debug.print("Struct with {} fields\n", .{s.fields.len});
            inline for (s.fields) |field| {
                std.debug.print("  {s}: {}\n", .{field.name, field.type});
            }
        },
        .Int => |i| std.debug.print("Int: {}bit {s}\n", .{i.bits, @tagName(i.signedness)}),
        else => std.debug.print("Other type\n", .{}),
    }
}
```

### Compile-Time Sprite Atlas Generation

This is a real use case from the codebase. Instead of loading sprite sheets from disk, generate them at compile time from string art:

```zig
// Define sprites as string art at comptime
// '.' = transparent, '1'-'9' = color index
const PLAYER_SPRITE = [8][]const u8{
    "...11...",
    "..1221..",
    ".122221.",
    "11222211",
    "11222211",
    ".122221.",
    "..1..1..",
    "..1..1..",
};

// Compile-time color palette validation
fn validateSprite(comptime art: []const []const u8) void {
    comptime {
        if (art.len != 8) @compileError("Sprite must be 8 rows");
        for (art) |row| {
            if (row.len != 8) @compileError("Sprite row must be 8 columns");
        }
    }
}

// Generate pixel data at compile time
fn spriteToPixels(
    comptime art: []const []const u8,
    comptime palette: []const [4]u8,
) [64]u32 {
    comptime {
        var pixels: [64]u32 = [_]u32{0} ** 64;
        for (art, 0..) |row, y| {
            for (row, 0..) |ch, x| {
                if (ch != '.' and ch >= '1' and ch <= '9') {
                    const idx = ch - '1';
                    const col = palette[idx];
                    pixels[y * 8 + x] =
                        (@as(u32, col[0]) << 24) |
                        (@as(u32, col[1]) << 16) |
                        (@as(u32, col[2]) << 8) |
                        @as(u32, col[3]);
                }
            }
        }
        return pixels;
    }
}

// This array is a compile-time constant — no runtime cost
const PLAYER_PIXELS = spriteToPixels(
    &PLAYER_SPRITE,
    &.{
        .{ 100, 150, 200, 255 },  // color 1: blue
        .{ 200, 220, 255, 255 },  // color 2: light blue
    },
);
```

### Compile-Time Lookup Tables

```zig
// Sin table for fast trig on ARM (avoids FPU call overhead)
const SIN_TABLE: [360]f32 = blk: {
    @setEvalBranchQuota(100000);  // comptime has a branch limit, raise it if needed
    var table: [360]f32 = undefined;
    for (&table, 0..) |*entry, i| {
        const angle: f32 = @as(f32, @floatFromInt(i)) * std.math.pi / 180.0;
        entry.* = @sin(angle);
    }
    break :blk table;
};

fn fastSin(deg: i32) f32 {
    const idx: usize = @intCast(@mod(deg, 360));
    return SIN_TABLE[idx];
}
```

---

## @cImport and C Interop — Zig's Superpower

This is where Zig leaves Rust in the dust for SDL2 game dev. No bindgen. No wrapper crates. Direct C header parsing.

```zig
// One block imports and translates C headers
const c = @cImport({
    @cInclude("SDL2/SDL.h");
    @cInclude("SDL2/SDL_mixer.h");  // add more as needed
});

// Now use C types and functions directly:
var window: *c.SDL_Window = undefined;
var renderer: *c.SDL_Renderer = undefined;

_ = c.SDL_Init(c.SDL_INIT_VIDEO | c.SDL_INIT_AUDIO);
window = c.SDL_CreateWindow("Game", 0, 0, 640, 480, c.SDL_WINDOW_SHOWN).?;
```

**C pointer types in Zig:**

| C type | Zig type |
|--------|----------|
| `T*` | `*T` or `?*T` |
| `const T*` | `*const T` or `?*const T` |
| `T**` | `**T` |
| `void*` | `*anyopaque` |
| `const void*` | `*const anyopaque` |
| `T[]` (as param) | `[*]T` |
| `char*` (string) | `[*:0]u8` or `[*:0]const u8` |
| `NULL` | `null` (for optional pointers) |

**The `orelse` pattern for C functions that return NULL on failure:**

```zig
// SDL_CreateWindow returns ?*SDL_Window (Zig makes nullable automatically)
const window = c.SDL_CreateWindow("Game", 0, 0, 640, 480, 0)
    orelse return error.WindowCreationFailed;
```

**Handling C error codes:**

```zig
// C functions returning 0=success, negative=error
if (c.SDL_Init(c.SDL_INIT_VIDEO) != 0) {
    const err = c.SDL_GetError();  // returns [*:0]const u8
    std.log.err("SDL_Init failed: {s}", .{err});
    return error.SDLInitFailed;
}
```

**C structs:**

```zig
// C structs work directly — field access is the same
var rect = c.SDL_Rect{
    .x = 100,
    .y = 100,
    .w = 64,
    .h = 64,
};
_ = c.SDL_RenderFillRect(renderer, &rect);

// Passing null for optional struct pointers
_ = c.SDL_RenderCopy(renderer, texture, null, &dst_rect);
```

**Type casting from C to Zig:**

```zig
// C's int to Zig's i32 is automatic for c_int
const x: c_int = 42;
const y: i32 = @intCast(x);

// Working with SDL_Color
const color = c.SDL_Color{ .r = 255, .g = 0, .b = 0, .a = 255 };
```

---

## Build System (build.zig)

Zig's build system is a Zig program. This is not CMake generating Makefiles — it's just Zig.

```zig
// build.zig — minimal SDL2 game
const std = @import("std");

pub fn build(b: *std.Build) void {
    // Standard options — supports -Dtarget= and -Doptimize=
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "my_game",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Link SDL2 (system library)
    exe.linkSystemLibrary("SDL2");
    exe.linkLibC();  // needed when linking any C library

    // Add include path (for @cImport to find headers)
    exe.addIncludePath(.{ .cwd_relative = "/usr/include" });

    // Install the binary
    b.installArtifact(exe);

    // Add a "run" step: `zig build run`
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    const run_step = b.step("run", "Run the game");
    run_step.dependOn(&run_cmd.step);

    // Add a test step
    const unit_tests = b.addTest(.{
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });
    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&b.addRunArtifact(unit_tests).step);
}
```

**Cross-compilation** — this is why Zig is excellent for the Miyoo:

```zig
// From the command line:
// zig build -Dtarget=arm-linux-gnueabihf -Doptimize=ReleaseFast

// Or hardcode in build.zig for a dedicated miyoo target:
const miyoo_target = b.resolveTargetQuery(.{
    .cpu_arch = .arm,
    .os_tag = .linux,
    .abi = .gnueabihf,
    .cpu_model = .{ .explicit = &std.Target.arm.cpu.cortex_a7 },
});

const miyoo_exe = b.addExecutable(.{
    .name = "game_miyoo",
    .root_source_file = b.path("src/main.zig"),
    .target = miyoo_target,
    .optimize = .ReleaseFast,
});
```

**Zig ships its own cross-compilation toolchain.** No arm-linux-gnueabihf-gcc needed. The Zig compiler contains LLVM and can produce ARM binaries from any host. This is the biggest practical advantage over Rust's cross-compilation story.

---

## Testing

```zig
// Tests live inline in the source file
const std = @import("std");
const testing = std.testing;

fn clamp(x: f32, lo: f32, hi: f32) f32 {
    return @max(lo, @min(hi, x));
}

test "clamp basics" {
    try testing.expectEqual(@as(f32, 5.0), clamp(5.0, 0.0, 10.0));
    try testing.expectEqual(@as(f32, 0.0), clamp(-3.0, 0.0, 10.0));
    try testing.expectEqual(@as(f32, 10.0), clamp(15.0, 0.0, 10.0));
}

test "Vec2 add" {
    const a = Vec2{ .x = 1.0, .y = 2.0 };
    const b = Vec2{ .x = 3.0, .y = 4.0 };
    const result = a.add(b);
    try testing.expectApproxEqAbs(@as(f32, 4.0), result.x, 0.0001);
    try testing.expectApproxEqAbs(@as(f32, 6.0), result.y, 0.0001);
}

// Run with: zig build test
// or: zig test src/main.zig
```

---

## @import for Modules

```zig
// Import standard library
const std = @import("std");

// Import your own file (path relative to current file)
const game = @import("game.zig");
const sprites = @import("sprites.zig");

// Import from a package defined in build.zig
const sdl = @import("sdl");  // if build.zig adds this as a module

// Everything in a file is public by default (unlike Rust's pub)
// Use `pub` to explicitly mark public items (for documentation/convention)
// Use `const x = ...` at file scope for module-level declarations
```

---

# Part 3: Patterns for Game Development

## Game Loop Pattern

The canonical 60fps fixed timestep loop in Zig:

```zig
const TARGET_FPS: u32 = 60;
const TARGET_MS: u32 = 1000 / TARGET_FPS;   // 16ms per frame

pub fn main() !void {
    // ... init SDL2 ...

    var running = true;
    while (running) {
        const frame_start = c.SDL_GetTicks();

        // 1. Process input
        var event: c.SDL_Event = undefined;
        while (c.SDL_PollEvent(&event) != 0) {
            switch (event.type) {
                c.SDL_QUIT => running = false,
                c.SDL_KEYDOWN => handleKey(event.key.keysym.sym, &input),
                c.SDL_KEYUP   => handleKeyUp(event.key.keysym.sym, &input),
                else => {},
            }
        }

        // 2. Update game state
        update(&game, &input);

        // 3. Render
        _ = c.SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
        _ = c.SDL_RenderClear(renderer);
        draw(renderer, &game);
        c.SDL_RenderPresent(renderer);

        // 4. Cap framerate (death spiral prevention)
        const frame_time = c.SDL_GetTicks() - frame_start;
        if (frame_time < TARGET_MS) {
            c.SDL_Delay(TARGET_MS - frame_time);
        }
        // If frame_time > TARGET_MS, we just run the next frame immediately.
        // Don't try to "catch up" — that causes spiral to 0 FPS.
    }
}
```

---

## State Machines with Tagged Unions

Game states are a perfect use case for tagged unions. The compiler ensures you handle every state.

```zig
const GameState = union(enum) {
    menu: struct {
        selected: u8 = 0,
    },
    story: struct {
        lines: []const []const u8,
        current_line: usize = 0,
        char_timer: f32 = 0,
        chars_shown: usize = 0,
        done: bool = false,
    },
    playing: struct {
        level: u8,
        player: Player,
        enemies: std.ArrayList(Enemy),
    },
    paused,
    game_over: struct {
        final_score: u32,
        timer: f32 = 0,
    },
};

fn update(state: *GameState, input: *const Input, dt: f32) void {
    switch (state.*) {
        .menu => |*menu| updateMenu(menu, input),
        .story => |*story| {
            // Typewriter effect
            story.char_timer += dt;
            if (story.char_timer > 0.04) {
                story.char_timer = 0;
                if (story.chars_shown < currentLine(story).len) {
                    story.chars_shown += 1;
                }
            }
            if (input.confirm_pressed and story.chars_shown >= currentLine(story).len) {
                story.current_line += 1;
                story.chars_shown = 0;
                if (story.current_line >= story.lines.len) {
                    story.done = true;
                }
            }
        },
        .playing => |*play| updatePlaying(play, input, dt),
        .paused => if (input.pause_pressed) state.* = .{ .playing = ... },
        .game_over => |*go| {
            go.timer += dt;
            if (go.timer > 3.0 or input.confirm_pressed) {
                state.* = .{ .menu = .{} };
            }
        },
    }
}
```

The `switch` on a tagged union captures the payload. Missing a case is a **compile error**, not a silent bug. This is one of the most valuable correctness guarantees Zig offers.

---

## Entity Storage: ArrayList vs Fixed Arrays

On the Miyoo Mini Plus, **avoid dynamic allocation in the game loop**. Pre-allocate everything.

```zig
// APPROACH 1: Fixed-size arrays with active flags (preferred for embedded)
const MAX_BULLETS = 64;
const MAX_ENEMIES = 32;
const MAX_PARTICLES = 128;

const GameObjects = struct {
    bullets: [MAX_BULLETS]Bullet = undefined,
    bullet_active: [MAX_BULLETS]bool = [_]bool{false} ** MAX_BULLETS,
    bullet_count: usize = 0,

    enemies: [MAX_ENEMIES]Enemy = undefined,
    enemy_count: usize = 0,
};

// Spawn a bullet (find first inactive slot)
fn spawnBullet(objects: *GameObjects, x: f32, y: f32, vx: f32, vy: f32) void {
    for (&objects.bullet_active, 0..) |*active, i| {
        if (!active.*) {
            active.* = true;
            objects.bullets[i] = .{ .x = x, .y = y, .vx = vx, .vy = vy };
            return;
        }
    }
    // No free slot — just drop the bullet silently
}

// Update bullets (iterate only active ones)
fn updateBullets(objects: *GameObjects) void {
    for (&objects.bullets, 0..) |*bullet, i| {
        if (!objects.bullet_active[i]) continue;
        bullet.x += bullet.vx;
        bullet.y += bullet.vy;
        if (bullet.x < 0 or bullet.x > 640 or bullet.y < 0 or bullet.y > 480) {
            objects.bullet_active[i] = false;
        }
    }
}

// APPROACH 2: ArrayList (when count varies wildly and you have an allocator)
var enemies = std.ArrayList(Enemy).init(allocator);
defer enemies.deinit();

try enemies.append(Enemy{ .x = 100, .y = 50 });

// Remove element without preserving order (swap with last = O(1))
_ = enemies.swapRemove(idx);

// Iterate
for (enemies.items) |*enemy| {
    enemy.x += enemy.vx;
}
```

For a Miyoo game with bounded entity counts, fixed arrays are better:
- Zero allocator overhead
- Predictable memory (fits entirely in cache)
- No deinit needed
- Works with `FixedBufferAllocator` or no allocator at all

---

## Collision Detection

AABB collision — the workhorse for most 2D games:

```zig
const Rect = struct {
    x: f32,
    y: f32,
    w: f32,
    h: f32,

    pub fn overlaps(self: Rect, other: Rect) bool {
        return self.x < other.x + other.w and
               self.x + self.w > other.x and
               self.y < other.y + other.h and
               self.y + self.h > other.y;
    }

    pub fn center(self: Rect) struct { x: f32, y: f32 } {
        return .{ .x = self.x + self.w / 2, .y = self.y + self.h / 2 };
    }
};

// Check bullet vs enemies, destroy on hit
fn checkBulletCollisions(
    bullets: []Bullet,
    bullet_active: []bool,
    enemies: []Enemy,
    enemy_active: []bool,
    score: *u32,
) void {
    for (bullets, 0..) |*bullet, bi| {
        if (!bullet_active[bi]) continue;
        const br = Rect{ .x = bullet.x, .y = bullet.y, .w = 6, .h = 6 };
        for (enemies, 0..) |*enemy, ei| {
            if (!enemy_active[ei]) continue;
            const er = Rect{ .x = enemy.x, .y = enemy.y, .w = 16, .h = 16 };
            if (br.overlaps(er)) {
                bullet_active[bi] = false;
                enemy_active[ei] = false;
                score.* += 100;
                break;
            }
        }
    }
}
```

---

## Input Handling

Edge detection is crucial — buttons fire once per press, not every frame:

```zig
const Input = struct {
    // Current state (held)
    left: bool = false,
    right: bool = false,
    up: bool = false,
    down: bool = false,
    fire: bool = false,
    pause: bool = false,

    // Edge detect (pressed this frame only)
    fire_pressed: bool = false,
    pause_pressed: bool = false,
    jump_pressed: bool = false,

    pub fn clearEdges(self: *Input) void {
        self.fire_pressed = false;
        self.pause_pressed = false;
        self.jump_pressed = false;
    }
};

fn handleKeyDown(key: c.SDL_Keycode, input: *Input) void {
    switch (key) {
        c.SDLK_LEFT, c.SDLK_a  => input.left  = true,
        c.SDLK_RIGHT, c.SDLK_d => input.right = true,
        c.SDLK_UP, c.SDLK_w    => input.up    = true,
        c.SDLK_DOWN, c.SDLK_s  => input.down  = true,
        c.SDLK_z, c.SDLK_SPACE => { input.fire = true; input.fire_pressed = true; },
        c.SDLK_x               => { input.fire = true; input.jump_pressed = true; },
        c.SDLK_RETURN          => input.pause_pressed = true,
        else => {},
    }
}

fn handleKeyUp(key: c.SDL_Keycode, input: *Input) void {
    switch (key) {
        c.SDLK_LEFT, c.SDLK_a  => input.left  = false,
        c.SDLK_RIGHT, c.SDLK_d => input.right = false,
        c.SDLK_UP, c.SDLK_w    => input.up    = false,
        c.SDLK_DOWN, c.SDLK_s  => input.down  = false,
        c.SDLK_z, c.SDLK_SPACE => input.fire  = false,
        else => {},
    }
}

// In the main loop, clear edge detects AFTER processing:
// input.clearEdges(); // at end of frame, before polling new events
```

---

## Random Number Generation

```zig
const std = @import("std");

// Initialize once at startup
var rng = std.Random.DefaultPrng.init(blk: {
    var seed: u64 = undefined;
    std.posix.getrandom(std.mem.asBytes(&seed)) catch {
        seed = @bitCast(std.time.milliTimestamp());
    };
    break :blk seed;
});
const rand = rng.random();

// Usage
const enemy_x = rand.float(f32) * 620.0;           // 0.0 to 620.0
const speed = rand.intRangeAtMost(i32, 1, 5);       // 1 to 5 inclusive
const should_spawn = rand.boolean();                 // true or false
const direction = rand.enumValue(Direction);         // random enum variant
```

---

## String Formatting

Zig uses `std.fmt` for formatting. The format string syntax is similar to Python's `{}`.

```zig
const std = @import("std");

// Print to stderr (debug output)
std.debug.print("Score: {d}, Lives: {d}\n", .{score, lives});

// Format to a fixed buffer (for HUD text that goes to SDL renderer)
var buf: [64]u8 = undefined;
const text = std.fmt.bufPrint(&buf, "SCORE: {d:0>6}", .{score}) catch "SCORE: ------";
// text is []const u8, pointing into buf

// Format allocating (when you need arbitrary length)
const text2 = try std.fmt.allocPrint(allocator, "Wave {d} of {d}", .{wave, max_wave});
defer allocator.free(text2);

// Null-terminated version for C APIs
var buf2: [64]u8 = undefined;
const ctext = try std.fmt.bufPrintZ(&buf2, "FPS: {d}", .{fps});
// ctext is [:0]u8 — null terminated, compatible with [*:0]u8

// Format specifiers
// {d}     — decimal integer
// {x}     — hex integer
// {d:0>6} — decimal, zero-padded to width 6
// {s}     — string slice []const u8
// {any}   — any type, uses Zig's default formatter
// {.2}    — float with 2 decimal places
```

---

## Performance: Cache-Friendly Data Layout

For the Miyoo (ARM Cortex-A7, small L1 cache), data layout matters.

**Prefer struct-of-arrays over array-of-structs for hot paths:**

```zig
// SLOW: Array-of-structs — position and everything else interleaved
const EnemySlow = struct {
    x: f32, y: f32,          // 8 bytes
    vx: f32, vy: f32,        // 8 bytes
    health: i32,             // 4 bytes
    sprite_id: u8,           // 1 byte
    shoot_timer: f32,        // 4 bytes
    patrol_start: f32,       // 4 bytes
    // ... 29 bytes per enemy, 29*64=1856 bytes, doesn't fit in cache well
};

// FAST: Struct-of-arrays — position data is contiguous
const Enemies = struct {
    x: [64]f32,           // 256 bytes — fits in one cache line prefetch
    y: [64]f32,
    vx: [64]f32,
    vy: [64]f32,
    health: [64]i32,
    active: [64]bool,
    sprite_id: [64]u8,
    shoot_timer: [64]f32,
    count: usize = 0,
};

// Now the movement update touches only x, y, vx, vy — all contiguous
fn updatePositions(enemies: *Enemies) void {
    for (0..enemies.count) |i| {
        enemies.x[i] += enemies.vx[i];
        enemies.y[i] += enemies.vy[i];
    }
}
```

---

# Part 4: SDL2 in Zig — Complete Reference

## Initializing SDL2

```zig
const std = @import("std");
const c = @cImport({
    @cInclude("SDL2/SDL.h");
});

pub fn main() !void {
    // Initialize subsystems
    if (c.SDL_Init(c.SDL_INIT_VIDEO | c.SDL_INIT_AUDIO | c.SDL_INIT_GAMECONTROLLER) != 0) {
        std.log.err("SDL_Init: {s}", .{c.SDL_GetError()});
        return error.SDLInitFailed;
    }
    defer c.SDL_Quit();

    // Create window
    const window = c.SDL_CreateWindow(
        "My Game",                  // title
        c.SDL_WINDOWPOS_CENTERED,   // x
        c.SDL_WINDOWPOS_CENTERED,   // y
        640, 480,                   // width, height
        c.SDL_WINDOW_SHOWN,         // flags
    ) orelse {
        std.log.err("SDL_CreateWindow: {s}", .{c.SDL_GetError()});
        return error.WindowFailed;
    };
    defer c.SDL_DestroyWindow(window);

    // Create renderer (hardware accelerated, vsync on)
    const renderer = c.SDL_CreateRenderer(
        window, -1,
        c.SDL_RENDERER_ACCELERATED | c.SDL_RENDERER_PRESENTVSYNC,
    ) orelse {
        std.log.err("SDL_CreateRenderer: {s}", .{c.SDL_GetError()});
        return error.RendererFailed;
    };
    defer c.SDL_DestroyRenderer(renderer);

    // Enable alpha blending globally
    _ = c.SDL_SetRenderDrawBlendMode(renderer, c.SDL_BLENDMODE_BLEND);
}
```

---

## Drawing Primitives

```zig
// Clear screen to black
_ = c.SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
_ = c.SDL_RenderClear(renderer);

// Draw filled rectangle
_ = c.SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255);  // red
var rect = c.SDL_Rect{ .x = 100, .y = 100, .w = 64, .h = 32 };
_ = c.SDL_RenderFillRect(renderer, &rect);

// Draw outline rectangle
_ = c.SDL_RenderDrawRect(renderer, &rect);

// Draw line
_ = c.SDL_SetRenderDrawColor(renderer, 0, 255, 0, 255);
_ = c.SDL_RenderDrawLine(renderer, 0, 0, 640, 480);

// Draw a single pixel
_ = c.SDL_RenderDrawPoint(renderer, 320, 240);

// Semi-transparent overlay (alpha blending must be enabled)
_ = c.SDL_SetRenderDrawColor(renderer, 0, 0, 0, 128);  // 50% black
var overlay = c.SDL_Rect{ .x = 0, .y = 0, .w = 640, .h = 480 };
_ = c.SDL_RenderFillRect(renderer, &overlay);

// Present (swap buffers)
c.SDL_RenderPresent(renderer);
```

---

## Textures: Creating and Rendering

**Creating a texture from pixel data** (procedural sprites — no image files needed):

```zig
fn createTexture(renderer: *c.SDL_Renderer, pixels: []const u32, w: c_int, h: c_int) ?*c.SDL_Texture {
    const tex = c.SDL_CreateTexture(
        renderer,
        c.SDL_PIXELFORMAT_RGBA8888,  // R in high byte
        c.SDL_TEXTUREACCESS_STATIC,  // uploaded once, then GPU-resident
        w, h,
    ) orelse return null;

    _ = c.SDL_SetTextureBlendMode(tex, c.SDL_BLENDMODE_BLEND);
    _ = c.SDL_UpdateTexture(
        tex,
        null,                     // update whole texture
        pixels.ptr,               // pixel data
        @intCast(w * 4),          // pitch: bytes per row
    );
    return tex;
}

// Render the texture
fn drawSprite(
    renderer: *c.SDL_Renderer,
    texture: *c.SDL_Texture,
    x: i32, y: i32,
    scale: i32,
    flip_h: bool,
) void {
    const dst = c.SDL_Rect{
        .x = @intCast(x),
        .y = @intCast(y),
        .w = 8 * scale,
        .h = 8 * scale,
    };
    const flip_flag: c.SDL_RendererFlip = if (flip_h)
        c.SDL_FLIP_HORIZONTAL
    else
        c.SDL_FLIP_NONE;

    _ = c.SDL_RenderCopyEx(
        renderer, texture,
        null,       // src_rect: null = whole texture
        &dst,       // dst_rect
        0.0,        // angle
        null,       // center (null = center of dst)
        flip_flag,
    );
}
```

**Texture color modulation** (tint/flash effects):

```zig
// Flash white on hit
_ = c.SDL_SetTextureColorMod(texture, 255, 255, 255);  // override color
// Restore normal color
_ = c.SDL_SetTextureColorMod(texture, 255, 128, 128);  // red tint

// Fade out
_ = c.SDL_SetTextureAlphaMod(texture, 128);  // 50% transparent
```

---

## Event Handling

```zig
var event: c.SDL_Event = undefined;
while (c.SDL_PollEvent(&event) != 0) {
    switch (event.type) {
        c.SDL_QUIT => running = false,

        c.SDL_KEYDOWN => {
            const sym = event.key.keysym.sym;
            const repeat = event.key.repeat != 0;
            if (!repeat) {
                // First press only
                handleKeyDown(sym, &input);
            }
        },

        c.SDL_KEYUP => handleKeyUp(event.key.keysym.sym, &input),

        c.SDL_MOUSEBUTTONDOWN => {
            const mb = event.button;
            std.debug.print("Click at ({d}, {d})\n", .{mb.x, mb.y});
        },

        // Gamepad (for Miyoo via SDL2 game controller API)
        c.SDL_CONTROLLERBUTTONDOWN => {
            const btn = event.cbutton.button;
            switch (btn) {
                c.SDL_CONTROLLER_BUTTON_DPAD_LEFT  => input.left  = true,
                c.SDL_CONTROLLER_BUTTON_DPAD_RIGHT => input.right = true,
                c.SDL_CONTROLLER_BUTTON_A          => input.fire_pressed = true,
                else => {},
            }
        },

        else => {},
    }
}
```

---

## Timing with SDL_GetTicks

```zig
// Get milliseconds since SDL_Init
const now: u32 = c.SDL_GetTicks();

// Delta time in seconds (for physics integration)
var last_ticks: u32 = c.SDL_GetTicks();

// In the game loop:
const current = c.SDL_GetTicks();
const delta_ms = current - last_ticks;
const dt: f32 = @as(f32, @floatFromInt(delta_ms)) / 1000.0;
last_ticks = current;

// Cap dt to prevent spiral when window is dragged / system is slow
const capped_dt = @min(dt, 0.05);  // max 50ms = min 20fps effective
```

---

## Full Minimal Game Skeleton

```zig
const std = @import("std");
const c = @cImport({
    @cInclude("SDL2/SDL.h");
});

const SCREEN_W = 640;
const SCREEN_H = 480;
const TARGET_MS: u32 = 1000 / 60;

const Vec2 = struct { x: f32, y: f32 };

const Player = struct {
    pos: Vec2 = .{ .x = 320, .y = 240 },
    vel: Vec2 = .{ .x = 0, .y = 0 },
    speed: f32 = 200.0,  // pixels per second
};

const Input = struct {
    left: bool = false,
    right: bool = false,
    up: bool = false,
    down: bool = false,
    quit: bool = false,
};

pub fn main() !void {
    if (c.SDL_Init(c.SDL_INIT_VIDEO) != 0) return error.SDLInit;
    defer c.SDL_Quit();

    const window = c.SDL_CreateWindow("Skeleton", c.SDL_WINDOWPOS_CENTERED,
        c.SDL_WINDOWPOS_CENTERED, SCREEN_W, SCREEN_H, c.SDL_WINDOW_SHOWN)
        orelse return error.Window;
    defer c.SDL_DestroyWindow(window);

    const renderer = c.SDL_CreateRenderer(window, -1,
        c.SDL_RENDERER_ACCELERATED | c.SDL_RENDERER_PRESENTVSYNC)
        orelse return error.Renderer;
    defer c.SDL_DestroyRenderer(renderer);

    var player = Player{};
    var input = Input{};

    while (!input.quit) {
        const frame_start = c.SDL_GetTicks();

        // Input
        var event: c.SDL_Event = undefined;
        while (c.SDL_PollEvent(&event) != 0) {
            switch (event.type) {
                c.SDL_QUIT => input.quit = true,
                c.SDL_KEYDOWN => switch (event.key.keysym.sym) {
                    c.SDLK_ESCAPE => input.quit = true,
                    c.SDLK_LEFT  => input.left  = true,
                    c.SDLK_RIGHT => input.right = true,
                    c.SDLK_UP    => input.up    = true,
                    c.SDLK_DOWN  => input.down  = true,
                    else => {},
                },
                c.SDL_KEYUP => switch (event.key.keysym.sym) {
                    c.SDLK_LEFT  => input.left  = false,
                    c.SDLK_RIGHT => input.right = false,
                    c.SDLK_UP    => input.up    = false,
                    c.SDLK_DOWN  => input.down  = false,
                    else => {},
                },
                else => {},
            }
        }

        // Update (fixed 60fps, so dt is 1/60)
        const dt: f32 = 1.0 / 60.0;
        player.vel.x = 0;
        player.vel.y = 0;
        if (input.left)  player.vel.x = -player.speed;
        if (input.right) player.vel.x =  player.speed;
        if (input.up)    player.vel.y = -player.speed;
        if (input.down)  player.vel.y =  player.speed;
        player.pos.x = @max(0, @min(SCREEN_W - 16, player.pos.x + player.vel.x * dt));
        player.pos.y = @max(0, @min(SCREEN_H - 16, player.pos.y + player.vel.y * dt));

        // Render
        _ = c.SDL_SetRenderDrawColor(renderer, 20, 20, 30, 255);
        _ = c.SDL_RenderClear(renderer);

        _ = c.SDL_SetRenderDrawColor(renderer, 100, 200, 100, 255);
        var rect = c.SDL_Rect{
            .x = @intFromFloat(player.pos.x),
            .y = @intFromFloat(player.pos.y),
            .w = 16, .h = 16,
        };
        _ = c.SDL_RenderFillRect(renderer, &rect);

        c.SDL_RenderPresent(renderer);

        // Framerate cap
        const elapsed = c.SDL_GetTicks() - frame_start;
        if (elapsed < TARGET_MS) c.SDL_Delay(TARGET_MS - elapsed);
    }
}
```

---

# Part 5: Cross-Compilation for Miyoo Mini Plus

## The Miyoo Environment

- **CPU:** ARM Cortex-A7 (armv7-a), VFPv4, no NEON by default in SDL
- **OS:** Linux 4.9 (Buildroot-based), glibc 2.28
- **Display:** 640x480 framebuffer via SDL2 mmiyoo driver
- **SDL2:** Custom "parasyte" runtime, loaded via `SDL_VIDEODRIVER=mmiyoo`
- **Input:** D-pad + 6 buttons + 2 shoulder + Start/Select
- **Storage:** SD card, typical path `/mnt/SDCARD/`

The Miyoo doesn't run X11. SDL2 talks to the framebuffer directly through the mmiyoo driver. Your game must set the environment variable before SDL_Init.

---

## build.zig for ARM Cross-Compilation

```zig
const std = @import("std");

pub fn build(b: *std.Build) void {
    // Desktop target (for development)
    const native_target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Miyoo target — ARM Cortex-A7, hard-float ABI
    const miyoo_target = b.resolveTargetQuery(.{
        .cpu_arch = .arm,
        .os_tag = .linux,
        .abi = .gnueabihf,
        .cpu_model = .{ .explicit = &std.Target.arm.cpu.cortex_a7 },
    });

    // Build the desktop version
    const desktop_exe = b.addExecutable(.{
        .name = "game",
        .root_source_file = b.path("src/main.zig"),
        .target = native_target,
        .optimize = optimize,
    });
    desktop_exe.linkSystemLibrary("SDL2");
    desktop_exe.linkLibC();
    b.installArtifact(desktop_exe);

    // Build the Miyoo version
    const miyoo_exe = b.addExecutable(.{
        .name = "game_miyoo",
        .root_source_file = b.path("src/main.zig"),
        .target = miyoo_target,
        .optimize = .ReleaseFast,
    });
    miyoo_exe.linkLibC();

    // Link the Miyoo SDL2 libraries from the parasyte SDK
    // These are ARM .so files from the Miyoo toolchain
    miyoo_exe.addLibraryPath(.{ .cwd_relative = "/opt/miyoo/libs" });
    miyoo_exe.linkSystemLibrary("SDL2");

    // Add include path for SDL2 headers
    miyoo_exe.addIncludePath(.{ .cwd_relative = "/opt/miyoo/include" });

    // Install to zig-out/bin/
    const miyoo_install = b.addInstallArtifact(miyoo_exe, .{});

    const miyoo_step = b.step("miyoo", "Build for Miyoo Mini Plus");
    miyoo_step.dependOn(&miyoo_install.step);
}
```

Build commands:

```bash
# Desktop (development)
zig build run

# Miyoo
zig build miyoo

# Or use the target flag directly:
zig build -Dtarget=arm-linux-gnueabihf -Doptimize=ReleaseFast
```

---

## Dynamic Linking to the Parasyte Runtime

The Miyoo uses a custom SDL2 that supports the mmiyoo video driver. You link against the prebuilt `.so` files from the toolchain.

The game binary should be dynamically linked. On the device:

```bash
# The device has these in the parasyte SDK paths:
# /mnt/SDCARD/.tmp_update/lib/libSDL2.so
# /mnt/SDCARD/.tmp_update/lib/libSDL2_mixer.so (if using audio)

# Set rpath so the binary finds libs at runtime
# In build.zig:
miyoo_exe.addRPath(.{ .cwd_relative = "/mnt/SDCARD/.tmp_update/lib" });
```

Alternatively, keep the SDL2 `.so` next to your binary and set `LD_LIBRARY_PATH`.

---

## Environment Variables and Launch Script

Your game needs a launch wrapper script on the Miyoo:

```sh
#!/bin/sh
# launch.sh

# Set SDL to use the Miyoo framebuffer driver
export SDL_VIDEODRIVER=mmiyoo
export SDL_AUDIODRIVER=mmiyoo

# Disable screen saver (Miyoo can blank the display)
export SDL_VIDEO_ALLOW_SCREENSAVER=0

# Path to your game binary
cd /mnt/SDCARD/Roms/PORTS/MyGame/
./game_miyoo
```

In your Zig code, you can also set this at startup before `SDL_Init`:

```zig
// Set environment variables before SDL_Init
// Required for Miyoo to use the framebuffer driver
_ = c.SDL_setenv("SDL_VIDEODRIVER", "mmiyoo", 1);
_ = c.SDL_setenv("SDL_AUDIODRIVER", "mmiyoo", 1);
```

---

## Testing Workflow

```bash
# 1. Build
zig build -Dtarget=arm-linux-gnueabihf -Doptimize=ReleaseFast

# 2. Copy to Miyoo over network (if SSH is available via miyoo-mini-toolchain setup)
scp zig-out/bin/game_miyoo root@192.168.1.123:/mnt/SDCARD/Ports/MyGame/

# 3. Run via SSH
ssh root@192.168.1.123 "cd /mnt/SDCARD/Ports/MyGame && SDL_VIDEODRIVER=mmiyoo ./game_miyoo"

# OR: copy to SD card, insert, launch from Miyoo file browser
```

---

## Common Pitfalls

**Pitfall 1: Undefined symbols at link time**

```
error: undefined reference to 'SDL_CreateWindow'
```

Fix: Make sure `exe.linkLibC()` is called in addition to `linkSystemLibrary("SDL2")`. Without libc, the C runtime startup code is missing.

**Pitfall 2: Wrong float ABI**

If you see garbled float behavior on-device, you might be using soft-float ABI. The Miyoo needs `gnueabihf` (hard-float) not `gnueabi`.

**Pitfall 3: Stack size**

The Miyoo's default thread stack is small. Large stack arrays (like `[640*480]u32`) will crash silently. Move big buffers to heap or file-scope globals.

```zig
// BAD: This is on the stack — may crash on Miyoo
fn render() void {
    var pixel_buf: [640 * 480]u32 = undefined;  // 1.2MB on stack!
}

// GOOD: File-scope global (BSS segment)
var pixel_buf: [640 * 480]u32 = undefined;
fn render() void {
    // use pixel_buf
}
```

**Pitfall 4: Zig's safety checks in release**

`ReleaseSafe` includes bounds checks and overflow detection. `ReleaseFast` removes them for maximum speed. For shipping, start with `ReleaseSafe` and only switch to `ReleaseFast` when you've verified correctness.

**Pitfall 5: @cImport fails to find headers**

```
error: 'SDL2/SDL.h' file not found
```

Fix: Add the path explicitly in `build.zig`:
```zig
exe.addIncludePath(.{ .cwd_relative = "/usr/include" });
// or for cross-compiling:
exe.addIncludePath(.{ .cwd_relative = "/opt/miyoo/include" });
```

---

# Part 6: Complete Game Example

## Pong in Zig + SDL2

This is a complete, working Pong implementation. Every concept from the tutorial appears here. ~280 lines.

```zig
// pong.zig — Complete Pong clone in Zig + SDL2
// Demonstrates: game states, tagged unions, fixed timestep, collision,
//               text rendering, SDL2 primitives, error handling.

const std = @import("std");
const c = @cImport({
    @cInclude("SDL2/SDL.h");
});

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const W = 640;
const H = 480;
const PADDLE_W = 12;
const PADDLE_H = 80;
const BALL_SIZE = 10;
const PADDLE_SPEED = 300.0;   // pixels/sec
const BALL_SPEED_INIT = 250.0;
const BALL_SPEED_MAX = 500.0;
const BALL_ACCEL = 20.0;      // speed increase per paddle hit
const SCORE_WIN = 7;
const TARGET_MS: u32 = 1000 / 60;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

const Vec2 = struct {
    x: f32 = 0,
    y: f32 = 0,
};

const Paddle = struct {
    pos: Vec2,
    vel: Vec2 = .{},
    score: u32 = 0,
};

const Ball = struct {
    pos: Vec2,
    vel: Vec2,
    speed: f32 = BALL_SPEED_INIT,
};

// Game states as a tagged union — compiler enforces exhaustive handling
const GameState = union(enum) {
    title,
    playing: struct {
        left: Paddle,
        right: Paddle,
        ball: Ball,
        serve_timer: f32 = 1.5,   // countdown before ball launches
        serving: bool = true,
    },
    goal: struct {
        left_score: u32,
        right_score: u32,
        scored: enum { left, right },
        timer: f32 = 2.0,
    },
    victory: struct {
        winner: enum { left, right },
        timer: f32 = 0,
    },
};

const Input = struct {
    up: bool = false,
    down: bool = false,
    w: bool = false,
    s: bool = false,
    fire_pressed: bool = false,

    pub fn clearEdges(self: *Input) void {
        self.fire_pressed = false;
    }
};

// ---------------------------------------------------------------------------
// 8x8 Bitmap Font (minimal, embedded inline)
// ---------------------------------------------------------------------------
// Full font is in font.zig in the real project — here we inline a minimal one
// just to show the technique.

// Draw text using SDL_RenderFillRect per pixel
fn drawChar(renderer: *c.SDL_Renderer, ch: u8, x: i32, y: i32, scale: i32) void {
    // Minimal 5x7 font data for digits and letters needed for Pong HUD
    // Using a placeholder here — real code uses the full 96-char font in font.zig
    // Format: 5 bytes per char, one per column, bit 0 = top
    const MINI_FONT = [_][5]u8{
        // 0: '0' through '9' and a few letters
        .{ 0x3E, 0x41, 0x41, 0x41, 0x3E },  // 0
        .{ 0x00, 0x42, 0x7F, 0x40, 0x00 },  // 1
        .{ 0x42, 0x61, 0x51, 0x49, 0x46 },  // 2
        .{ 0x21, 0x41, 0x45, 0x4B, 0x31 },  // 3
        .{ 0x18, 0x14, 0x12, 0x7F, 0x10 },  // 4
        .{ 0x27, 0x45, 0x45, 0x45, 0x39 },  // 5
        .{ 0x3C, 0x4A, 0x49, 0x49, 0x30 },  // 6
        .{ 0x01, 0x71, 0x09, 0x05, 0x03 },  // 7
        .{ 0x36, 0x49, 0x49, 0x49, 0x36 },  // 8
        .{ 0x06, 0x49, 0x49, 0x29, 0x1E },  // 9
    };

    const idx: usize = switch (ch) {
        '0'...'9' => ch - '0',
        else => return,
    };
    const glyph = MINI_FONT[idx];

    for (glyph, 0..) |col, gx| {
        var bit: u3 = 0;
        while (bit < 7) : (bit += 1) {
            if (col & (@as(u8, 1) << bit) != 0) {
                var r = c.SDL_Rect{
                    .x = x + @as(i32, @intCast(gx)) * scale,
                    .y = y + @as(i32, @intCast(bit)) * scale,
                    .w = scale,
                    .h = scale,
                };
                _ = c.SDL_RenderFillRect(renderer, &r);
            }
        }
    }
}

fn drawNumber(renderer: *c.SDL_Renderer, n: u32, x: i32, y: i32, scale: i32) void {
    // Draw up to 2 digits
    if (n >= 10) drawChar(renderer, '0' + @as(u8, @intCast(n / 10)), x, y, scale);
    drawChar(renderer, '0' + @as(u8, @intCast(n % 10)), x + 6 * scale, y, scale);
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

fn fillRect(renderer: *c.SDL_Renderer, x: i32, y: i32, w: i32, h: i32,
            r: u8, g: u8, b: u8) void {
    _ = c.SDL_SetRenderDrawColor(renderer, r, g, b, 255);
    var rect = c.SDL_Rect{ .x = x, .y = y, .w = w, .h = h };
    _ = c.SDL_RenderFillRect(renderer, &rect);
}

fn drawDashedLine(renderer: *c.SDL_Renderer) void {
    _ = c.SDL_SetRenderDrawColor(renderer, 60, 60, 60, 255);
    var y: i32 = 0;
    while (y < H) : (y += 20) {
        var rect = c.SDL_Rect{ .x = W / 2 - 1, .y = y, .w = 2, .h = 12 };
        _ = c.SDL_RenderFillRect(renderer, &rect);
    }
}

fn drawState(renderer: *c.SDL_Renderer, state: *const GameState) void {
    // Clear
    _ = c.SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
    _ = c.SDL_RenderClear(renderer);

    switch (state.*) {
        .title => {
            // Title screen — just white text in the center
            _ = c.SDL_SetRenderDrawColor(renderer, 255, 255, 255, 255);
            // Draw "PONG" in large pixels (simplified)
            fillRect(renderer, W/2 - 40, H/2 - 30, 80, 8, 255, 255, 255);
            fillRect(renderer, W/2 - 40, H/2 - 10, 80, 4, 180, 180, 180);
            // "PRESS FIRE" hint
            fillRect(renderer, W/2 - 30, H/2 + 20, 60, 4, 100, 100, 100);
        },

        .playing => |play| {
            drawDashedLine(renderer);

            // Scores
            _ = c.SDL_SetRenderDrawColor(renderer, 200, 200, 200, 255);
            drawNumber(renderer, play.left.score,  W/2 - 60, 20, 3);
            drawNumber(renderer, play.right.score, W/2 + 20, 20, 3);

            // Paddles
            const lp = play.left.pos;
            const rp = play.right.pos;
            fillRect(renderer, @intFromFloat(lp.x), @intFromFloat(lp.y),
                     PADDLE_W, PADDLE_H, 80, 160, 255);
            fillRect(renderer, @intFromFloat(rp.x), @intFromFloat(rp.y),
                     PADDLE_W, PADDLE_H, 255, 100, 80);

            // Ball (only when not serving)
            if (!play.serving) {
                const bp = play.ball.pos;
                fillRect(renderer, @intFromFloat(bp.x), @intFromFloat(bp.y),
                         BALL_SIZE, BALL_SIZE, 255, 255, 255);
            }

            // Serve countdown flash
            if (play.serving) {
                const alpha: u8 = @intFromFloat(@min(255.0,
                    (1.0 - play.serve_timer / 1.5) * 255.0));
                _ = c.SDL_SetRenderDrawColor(renderer, 255, 255, 255, alpha);
                var dot = c.SDL_Rect{
                    .x = W/2 - BALL_SIZE/2,
                    .y = H/2 - BALL_SIZE/2,
                    .w = BALL_SIZE, .h = BALL_SIZE,
                };
                _ = c.SDL_RenderFillRect(renderer, &dot);
            }
        },

        .goal => |g| {
            _ = c.SDL_SetRenderDrawColor(renderer, 255, 220, 50, 255);
            drawNumber(renderer, g.left_score,  W/2 - 60, H/2 - 20, 4);
            drawNumber(renderer, g.right_score, W/2 + 24, H/2 - 20, 4);
            // Flash message
            const flash: u8 = @intFromFloat(@abs(@sin(g.timer * 4.0)) * 255.0);
            _ = c.SDL_SetRenderDrawColor(renderer, flash, flash, 0, 255);
            var bar = c.SDL_Rect{ .x = W/2 - 40, .y = H/2 + 20, .w = 80, .h = 6 };
            _ = c.SDL_RenderFillRect(renderer, &bar);
        },

        .victory => |v| {
            const color: struct { r: u8, g: u8, b: u8 } = switch (v.winner) {
                .left  => .{ .r = 80, .g = 160, .b = 255 },
                .right => .{ .r = 255, .g = 100, .b = 80 },
            };
            _ = c.SDL_SetRenderDrawColor(renderer, color.r, color.g, color.b, 255);
            var top = c.SDL_Rect{ .x = 0, .y = 0, .w = W, .h = H };
            _ = c.SDL_RenderFillRect(renderer, &top);
            // Big "7" for the winning score
            _ = c.SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
            drawNumber(renderer, SCORE_WIN, W/2 - 30, H/2 - 40, 6);
        },
    }

    c.SDL_RenderPresent(renderer);
}

// ---------------------------------------------------------------------------
// Update
// ---------------------------------------------------------------------------

fn initPlaying() GameState {
    return .{ .playing = .{
        .left  = .{ .pos = .{ .x = 20, .y = H/2 - PADDLE_H/2 } },
        .right = .{ .pos = .{ .x = W - 20 - PADDLE_W, .y = H/2 - PADDLE_H/2 } },
        .ball  = .{
            .pos = .{ .x = W/2 - BALL_SIZE/2, .y = H/2 - BALL_SIZE/2 },
            .vel = .{ .x = 0, .y = 0 },
        },
        .serving = true,
        .serve_timer = 1.5,
    }};
}

fn updateState(state: *GameState, input: *const Input, dt: f32) void {
    switch (state.*) {
        .title => {
            if (input.fire_pressed) {
                state.* = initPlaying();
            }
        },

        .playing => |*play| {
            const dt_capped = @min(dt, 0.05);

            // Serve countdown
            if (play.serving) {
                play.serve_timer -= dt_capped;
                if (play.serve_timer <= 0) {
                    play.serving = false;
                    // Launch ball toward right player, slight downward angle
                    const rng = std.crypto.random;
                    const angle: f32 = (rng.float(f32) - 0.5) * 0.8;
                    play.ball.vel = .{
                        .x = play.ball.speed,
                        .y = play.ball.speed * angle,
                    };
                }
                // Paddle input even during serve
            }

            // Left paddle: W/S keys
            play.left.vel.y = 0;
            if (input.w) play.left.vel.y = -PADDLE_SPEED;
            if (input.s) play.left.vel.y =  PADDLE_SPEED;

            // Right paddle: up/down arrows
            play.right.vel.y = 0;
            if (input.up)   play.right.vel.y = -PADDLE_SPEED;
            if (input.down) play.right.vel.y =  PADDLE_SPEED;

            // Move paddles
            inline for (.{ &play.left, &play.right }) |paddle| {
                paddle.pos.y += paddle.vel.y * dt_capped;
                paddle.pos.y = @max(0, @min(H - PADDLE_H, paddle.pos.y));
            }

            if (!play.serving) {
                // Move ball
                play.ball.pos.x += play.ball.vel.x * dt_capped;
                play.ball.pos.y += play.ball.vel.y * dt_capped;

                // Top/bottom wall bounce
                if (play.ball.pos.y <= 0) {
                    play.ball.pos.y = 0;
                    play.ball.vel.y = @abs(play.ball.vel.y);
                }
                if (play.ball.pos.y + BALL_SIZE >= H) {
                    play.ball.pos.y = H - BALL_SIZE;
                    play.ball.vel.y = -@abs(play.ball.vel.y);
                }

                // Paddle collision helper
                const ballHitsPaddle = struct {
                    fn check(ball: *Ball, paddle: *const Paddle, dir: f32) bool {
                        const bx = ball.pos.x;
                        const by = ball.pos.y;
                        const px = paddle.pos.x;
                        const py = paddle.pos.y;
                        if (bx < px + PADDLE_W and bx + BALL_SIZE > px and
                            by < py + PADDLE_H and by + BALL_SIZE > py) {
                            // Reflect and accelerate
                            ball.speed = @min(BALL_SPEED_MAX, ball.speed + BALL_ACCEL);
                            const center_y = py + PADDLE_H / 2;
                            const offset = (by + BALL_SIZE / 2 - center_y) / (PADDLE_H / 2);
                            const angle: f32 = offset * 0.9; // max ~52 deg
                            ball.vel.x = dir * ball.speed * @cos(angle);
                            ball.vel.y = ball.speed * @sin(angle);
                            return true;
                        }
                        return false;
                    }
                }.check;

                _ = ballHitsPaddle(&play.ball, &play.left,   1.0);
                _ = ballHitsPaddle(&play.ball, &play.right, -1.0);

                // Score
                if (play.ball.pos.x + BALL_SIZE < 0) {
                    // Right scores
                    play.right.score += 1;
                    if (play.right.score >= SCORE_WIN) {
                        state.* = .{ .victory = .{ .winner = .right } };
                        return;
                    }
                    state.* = .{ .goal = .{
                        .left_score = play.left.score,
                        .right_score = play.right.score,
                        .scored = .right,
                    }};
                    return;
                }
                if (play.ball.pos.x > W) {
                    play.left.score += 1;
                    if (play.left.score >= SCORE_WIN) {
                        state.* = .{ .victory = .{ .winner = .left } };
                        return;
                    }
                    state.* = .{ .goal = .{
                        .left_score = play.left.score,
                        .right_score = play.right.score,
                        .scored = .left,
                    }};
                    return;
                }
            }
        },

        .goal => |*g| {
            g.timer -= dt;
            if (g.timer <= 0 or input.fire_pressed) {
                // Restart with scores preserved
                var next = initPlaying();
                next.playing.left.score  = g.left_score;
                next.playing.right.score = g.right_score;
                state.* = next;
            }
        },

        .victory => |*v| {
            v.timer += dt;
            if (v.timer > 3.0 or input.fire_pressed) {
                state.* = .title;
            }
        },
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

pub fn main() !void {
    if (c.SDL_Init(c.SDL_INIT_VIDEO) != 0) {
        std.log.err("SDL_Init failed: {s}", .{c.SDL_GetError()});
        return error.SDLInit;
    }
    defer c.SDL_Quit();

    const window = c.SDL_CreateWindow(
        "PONG",
        c.SDL_WINDOWPOS_CENTERED, c.SDL_WINDOWPOS_CENTERED,
        W, H,
        c.SDL_WINDOW_SHOWN,
    ) orelse {
        std.log.err("SDL_CreateWindow: {s}", .{c.SDL_GetError()});
        return error.Window;
    };
    defer c.SDL_DestroyWindow(window);

    const renderer = c.SDL_CreateRenderer(window, -1,
        c.SDL_RENDERER_ACCELERATED | c.SDL_RENDERER_PRESENTVSYNC)
    orelse {
        std.log.err("SDL_CreateRenderer: {s}", .{c.SDL_GetError()});
        return error.Renderer;
    };
    defer c.SDL_DestroyRenderer(renderer);

    var state: GameState = .title;
    var input = Input{};
    var last_ticks: u32 = c.SDL_GetTicks();

    while (true) {
        const now = c.SDL_GetTicks();
        const dt: f32 = @as(f32, @floatFromInt(now - last_ticks)) / 1000.0;
        last_ticks = now;
        const frame_start = now;

        // Process events
        var event: c.SDL_Event = undefined;
        while (c.SDL_PollEvent(&event) != 0) {
            switch (event.type) {
                c.SDL_QUIT => return,
                c.SDL_KEYDOWN => {
                    if (event.key.repeat != 0) continue;
                    switch (event.key.keysym.sym) {
                        c.SDLK_ESCAPE => return,
                        c.SDLK_UP     => input.up   = true,
                        c.SDLK_DOWN   => input.down = true,
                        c.SDLK_w      => input.w    = true,
                        c.SDLK_s      => input.s    = true,
                        c.SDLK_RETURN, c.SDLK_SPACE, c.SDLK_z => {
                            input.fire_pressed = true;
                        },
                        else => {},
                    }
                },
                c.SDL_KEYUP => switch (event.key.keysym.sym) {
                    c.SDLK_UP   => input.up   = false,
                    c.SDLK_DOWN => input.down = false,
                    c.SDLK_w    => input.w    = false,
                    c.SDLK_s    => input.s    = false,
                    else => {},
                },
                else => {},
            }
        }

        updateState(&state, &input, dt);
        input.clearEdges();
        drawState(renderer, &state);

        // Framerate cap (in case vsync is off or unavailable)
        const elapsed = c.SDL_GetTicks() - frame_start;
        if (elapsed < TARGET_MS) c.SDL_Delay(TARGET_MS - elapsed);
    }
}
```

---

## What This Example Demonstrates

Every major concept appears in this ~280-line game:

| Concept | Where |
|---------|-------|
| `@cImport` + SDL2 types | Top of file |
| `const`/`var`, struct defaults | `Paddle`, `Ball`, `Input` |
| Tagged union game state | `GameState` union |
| Exhaustive `switch` | `drawState`, `updateState` — add a variant and the compiler errors |
| `orelse return error.X` | SDL init in `main` |
| `defer` cleanup | `SDL_DestroyWindow`, `SDL_Quit` |
| Edge-detect input | `fire_pressed`, `clearEdges` |
| Fixed-timestep with cap | `@min(dt, 0.05)` |
| `@intFromFloat`, `@floatFromInt` | Rect positioning |
| `@abs`, `@cos`, `@sin` | Ball physics |
| `@min`, `@max` | Paddle clamping, speed cap |
| `inline for` over tuple | Paddle update loop |
| Nested function (in struct) | `ballHitsPaddle` |
| Struct update via pointer | `state.* = .{ .goal = ... }` |

---

## Where to Go From Here

**Immediate next steps:**

1. Add SDL_mixer for audio: `@cInclude("SDL2/SDL_mixer.h")`, link `SDL2_mixer`
2. Replace the minimal font with the full 96-char `font.zig` from this codebase
3. Add the procedural sprite system from `sprite.zig` for textured graphics
4. Port to Miyoo: change build target, add the launch script, test over SSH

**Study the existing codebase:**

- `/home/mo/data/Documents/git/retrogames/zig/common/sdl.zig` — thin SDL2 wrapper showing the full C interop pattern
- `/home/mo/data/Documents/git/retrogames/zig/common/font.zig` — complete 8x8 bitmap font
- `/home/mo/data/Documents/git/retrogames/zig/common/sprite.zig` — string-art to SDL_Texture
- `/home/mo/data/Documents/git/retrogames/zig/micro/src/main.zig` — full platformer showing all patterns together

**Zig resources:**

- `ziglang.org/documentation/master/` — the official language reference (read it, it's short)
- `ziglearn.org` — structured tutorial
- `github.com/zigtools/zls` — LSP for editor support

**Honest final note on ecosystem maturity:**

Zig 0.13 is pre-1.0. The stdlib API has changed between every minor version. Before starting a project, pin your Zig version and don't upgrade mid-project. The language itself is stable enough for shipping, but packages and tooling are in flux. For this codebase — a single developer, no external Zig packages, just `@cImport("SDL2/SDL.h")` — this doesn't matter. You're leaning on the SDL2 ecosystem, which is rock-solid, and using Zig purely as a better C.
