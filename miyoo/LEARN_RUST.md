# Learn Rust for Game Development
## From C to Production Game Code on Miyoo Mini Plus

This guide takes you from zero Rust knowledge to writing real game code for an embedded
ARM device. You already know C and you're porting SDL2 games to the Miyoo Mini Plus.
Every section builds toward that goal.

---

## Table of Contents

1. [Part 1: Rust Fundamentals](#part-1-rust-fundamentals)
2. [Part 2: Rust's Unique Powers](#part-2-rusts-unique-powers)
3. [Part 3: Patterns for Game Development](#part-3-patterns-for-game-development)
4. [Part 4: SDL2 in Rust — Complete Reference](#part-4-sdl2-in-rust--complete-reference)
5. [Part 5: Cross-Compilation for Miyoo Mini Plus](#part-5-cross-compilation-for-miyoo-mini-plus)
6. [Part 6: Complete Game Example](#part-6-complete-game-example)

---

# Part 1: Rust Fundamentals

## Why Rust (vs C, vs Zig)

**Versus C:**

C gives you total control and zero overhead — which also means every bug is your problem.
Use-after-free, dangling pointers, buffer overflows, data races: the compiler doesn't stop
you. On embedded targets like the Miyoo, a crash takes down the whole game session with no
debugger attached. Rust gives you the same zero-overhead control but makes the entire
class of memory-safety bugs compile errors instead of runtime surprises.

The tradeoff is real: Rust's borrow checker is a new mental model you have to internalize.
The first two weeks feel like fighting the compiler. After that it feels like the compiler
is pair-programming with you.

**Versus Zig:**

Zig is an excellent C replacement — simpler than C++, explicit allocators, comptime.
Zig has fewer "magic" rules than Rust. However, Zig's safety guarantees are weaker:
it catches many bugs at runtime in debug mode, not at compile time. Rust's ownership
system catches them at compile time in all modes. For a long-lived codebase with multiple
contributors, that matters.

**Honest tradeoffs with Rust:**

- Compile times are slow. A cold `cargo build` on a project with many dependencies takes
  minutes. `cargo check` (no codegen, just type-check) is fast and what you should run
  constantly.
- Lifetimes are hard. Most code never needs explicit lifetime annotations, but when you
  do need them, the learning curve is steep.
- Async Rust is complex. We don't use it for games; ignore it for now.
- The borrow checker sometimes rejects code that is actually safe. There are escape hatches
  (see Part 2). You will need them for game dev.
- The ecosystem is excellent. `crates.io` has high-quality crates for almost everything.

**The bottom line for game dev on embedded ARM:** you get C-level performance, no GC
pauses, no runtime crashes from memory bugs, and a package manager that actually works.
For a game that runs unattended on a handheld, that combination is worth learning.

---

## Installing and Toolchain

```bash
# Install rustup (manages Rust versions and targets)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload your shell, then verify
rustc --version
cargo --version

# Add the ARM cross-compilation target (needed for Miyoo)
rustup target add armv7-unknown-linux-gnueabihf

# Install cargo-zigbuild for glibc version targeting
cargo install cargo-zigbuild
```

**The core commands you will use every day:**

```bash
# Create a new binary project
cargo new my_game
cd my_game

# Type-check only (fast — use this constantly)
cargo check

# Build debug binary (fast compile, slow runtime, includes debug symbols)
cargo build

# Build release binary (slow compile, fast runtime, stripped)
cargo build --release

# Build and run
cargo run

# Run tests
cargo test

# Add a dependency
cargo add fastrand
```

The project structure `cargo new` creates:

```
my_game/
├── Cargo.toml    # Package manifest and dependencies
├── Cargo.lock    # Exact locked dependency versions (commit this for binaries)
└── src/
    └── main.rs   # Entry point
```

**Cargo.toml anatomy:**

```toml
[package]
name = "micro_miyoo"
version = "0.1.0"
edition = "2024"          # Use the latest edition

[dependencies]
retro-sdl2 = { path = "../retro-sdl2" }   # Local path dependency
sdl2 = { version = "0.37", features = [] }
fastrand = "2"

[profile.release]
lto = true       # Link-time optimization — smaller, faster binary
strip = true     # Strip debug symbols — much smaller binary for the device
```

---

## Variables: let, let mut, Type Inference, Shadowing

In C, mutability is the default. In Rust, immutability is the default.

```rust
fn main() {
    // Immutable by default. This is a binding, not a declaration.
    let x = 5;
    // x = 6;  // ERROR: cannot assign twice to immutable variable

    // Mutable requires explicit opt-in
    let mut score = 0;
    score += 10;

    // Type inference: Rust figures out the type from context
    let speed = 3.5_f32;    // f32 literal suffix
    let hp = 100_u32;       // u32 literal suffix (underscores are ignored, just readability)
    let alive = true;       // bool

    // Explicit type annotation (sometimes required)
    let angle: f32 = 0.0;

    // Shadowing: redeclare the same name in the same scope
    // This is different from mutation — it creates a new binding
    let level = "1";
    let level = level.parse::<u32>().unwrap(); // now level is a u32, not &str
    println!("Level: {}", level);
}
```

Shadowing is useful for type transformations. In C you'd use a different variable name;
in Rust you can reuse the name cleanly.

---

## Types

**Integers:**

```rust
// Signed
let a: i8  = -128;       // 8-bit
let b: i16 = -32768;
let c: i32 = -2147483648;  // default integer type
let d: i64 = -9223372036854775808;

// Unsigned
let e: u8  = 255;
let f: u16 = 65535;
let g: u32 = 4294967295;
let h: u64 = 18446744073709551615;

// Platform-sized (pointer width, like size_t in C)
let i: usize = 42;   // use for array indices and lengths
let j: isize = -1;   // signed equivalent

// Integer literals
let hex = 0xFF_u8;
let binary = 0b1010_0101_u8;
let big = 1_000_000_i32;    // underscores improve readability
```

**Floats:**

```rust
let x: f32 = 3.14;    // 32-bit float — use this in games (matches GPU, faster on ARM)
let y: f64 = 3.14;    // 64-bit float — default for float literals

// IMPORTANT: this is a common footgun in game dev
// let angle = fastrand::f32() * std::f32::consts::PI;  // fine
// let angle = some_func_returning_untyped_float();
// angle.cos()   // ERROR: the method exists on both f32 and f64 — ambiguous
// Fix: annotate explicitly
let angle: f32 = fastrand::f32() * std::f32::consts::PI;
let _ = angle.cos();   // now unambiguous
```

**Booleans, chars, tuples, arrays:**

```rust
let alive: bool = true;
let ch: char = 'A';        // char is a 4-byte Unicode scalar, not a byte like C's char

// Tuples: fixed-size, mixed types
let pos: (f32, f32) = (100.0, 200.0);
let (x, y) = pos;          // destructuring
println!("x={}, y={}", pos.0, pos.1);   // field access by index

// Arrays: fixed-size, single type (on the stack)
let palette: [u8; 4] = [255, 0, 128, 255];   // RGBA
let zeros = [0u8; 64];                         // 64 zeros
println!("first: {}", palette[0]);
println!("length: {}", palette.len());

// Arrays are NOT like C arrays. They carry their length and panic on OOB access.
// For heap-allocated dynamic arrays, use Vec<T> (covered later).
```

---

## Ownership: THE Concept

This is the core of Rust. Everything else makes sense once you understand this.

**The rules:**
1. Every value has exactly one owner.
2. When the owner goes out of scope, the value is dropped (memory freed).
3. There can be any number of immutable references OR exactly one mutable reference — never both simultaneously.

**Why this exists:**

In C, you manually manage memory. `malloc`/`free`, careful pointer lifetime tracking.
The compiler won't stop you from freeing memory twice, using it after it's freed, or
accessing out-of-bounds memory. These bugs cause crashes, corrupted state, and security
vulnerabilities.

Garbage-collected languages (Go, Java, Python) solve this with a runtime GC. No manual
memory management, but you pay with GC pauses and overhead — unacceptable for a game
running on a 1 GHz ARM Cortex-A7.

Rust solves it statically, at compile time, with zero runtime overhead.

### Move Semantics

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;    // s1 is MOVED into s2. s1 no longer exists.
    // println!("{}", s1);  // ERROR: value borrowed here after move

    // String is heap-allocated. In C this would copy the pointer (shallow copy),
    // and you'd have a double-free bug. Rust prevents it at compile time.

    // To actually copy, use clone()
    let s3 = String::from("hello");
    let s4 = s3.clone();   // deep copy — allocates new memory
    println!("{} {}", s3, s4);  // both valid

    // Types that implement Copy are implicitly copied (no move)
    // All primitives implement Copy: i32, f32, bool, char, etc.
    let a = 42i32;
    let b = a;         // copied, not moved
    println!("{} {}", a, b);  // both valid
}
```

### Borrowing: &T and &mut T

References let you use a value without taking ownership.

```rust
fn print_pos(pos: &(f32, f32)) {    // takes an immutable reference
    println!("({}, {})", pos.0, pos.1);
}

fn move_right(pos: &mut (f32, f32), speed: f32) {   // takes a mutable reference
    pos.0 += speed;
}

fn main() {
    let mut player_pos = (100.0_f32, 200.0_f32);

    print_pos(&player_pos);          // borrow immutably
    move_right(&mut player_pos, 5.0); // borrow mutably
    print_pos(&player_pos);

    // The borrow checker enforces: while a mut reference is alive,
    // no other references (mutable or immutable) can exist.
    let r1 = &player_pos;
    let r2 = &player_pos;        // fine — multiple immutable refs
    println!("{:?} {:?}", r1, r2);

    let rm = &mut player_pos;    // fine — r1 and r2 are no longer used (NLL)
    rm.0 += 1.0;

    // In older Rust this would fail because r1/r2 lifetimes overlapped with rm.
    // Non-Lexical Lifetimes (NLL) in modern Rust is smarter — lifetimes end at
    // the last use, not at the end of the scope.
}
```

### Common Ownership Errors and Fixes

**Error: borrow after move**

```rust
fn consume(s: String) { println!("{}", s); }

fn bad() {
    let name = String::from("Nano Wizard");
    consume(name);
    // consume(name);  // ERROR: use of moved value: `name`
}

// Fix 1: clone (allocates)
fn fix1() {
    let name = String::from("Nano Wizard");
    consume(name.clone());
    consume(name);
}

// Fix 2: borrow instead of moving
fn consume_ref(s: &str) { println!("{}", s); }
fn fix2() {
    let name = String::from("Nano Wizard");
    consume_ref(&name);
    consume_ref(&name);
}
```

**Error: cannot borrow as mutable because it is also borrowed as immutable**

```rust
// This is the classic game dev problem — covered in depth in Part 2
fn bad() {
    let mut enemies: Vec<String> = vec!["goblin".into(), "orc".into()];
    let first = &enemies[0];       // immutable borrow
    enemies.push("dragon".into()); // mutable borrow — ERROR
    println!("{}", first);
}

// Fix: don't hold the reference across the mutation
fn fixed() {
    let mut enemies: Vec<String> = vec!["goblin".into(), "orc".into()];
    {
        let first = &enemies[0];
        println!("{}", first);
    }  // first's borrow ends here
    enemies.push("dragon".into());  // now fine
}
```

### When to use .clone() vs references

- Use references (`&T`, `&mut T`) whenever possible — zero cost.
- Use `.clone()` when you genuinely need an independent copy.
- For small `Copy` types (i32, f32, bool, small structs with `#[derive(Copy)]`),
  just pass by value — the compiler copies them for free.
- Avoid `.clone()` inside hot game loops. Clone allocates.

---

## Strings: String vs &str

```rust
// &str — a string slice: an immutable reference to string data
// Can point into a String, a string literal, or any UTF-8 buffer
let s1: &str = "hello world";   // string literal, &'static str

// String — owned heap-allocated string
let s2: String = String::from("hello world");
let s3: String = "hello world".to_string();
let s4: String = format!("hello {}", "world");   // like sprintf, returns String

// Slicing
let first_word: &str = &s2[0..5];  // "hello"

// String vs &str in function signatures
// Prefer &str for read-only string parameters — works with both String and &str
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let owned = String::from("wizard");
    greet(&owned);    // String coerces to &str with &
    greet("knight");  // string literal is already &str
}

// Formatting
let score = 9001;
let msg = format!("Score: {}", score);
let msg2 = format!("Pos: ({:.1}, {:.1})", 3.14159_f32, 2.71828_f32);
// "Pos: (3.1, 2.7)"

// Printing
println!("{}", msg);         // Display trait
println!("{:?}", (1, 2, 3)); // Debug trait — prints tuples, vecs, etc.
eprintln!("error: {}", msg); // prints to stderr
```

---

## Structs

```rust
// Definition
struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    hp: i32,
    alive: bool,
}

// Implementation block — methods and associated functions
impl Player {
    // Associated function (no self) — like a static method or constructor in C++
    fn new(x: f32, y: f32) -> Player {
        Player {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            hp: 5,
            alive: true,
        }
    }

    // Method: takes &self (immutable), returns without modifying
    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    // Method: takes &mut self (mutable)
    fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
        if self.hp <= 0 {
            self.alive = false;
        }
    }

    // Method that consumes self (rare in game dev)
    fn destroy(self) -> String {
        format!("Player at ({}, {}) destroyed", self.x, self.y)
    }
}

// Tuple struct — like a named tuple
struct Vec2(f32, f32);

impl Vec2 {
    fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
}

fn main() {
    let mut p = Player::new(100.0, 200.0);
    println!("alive: {}", p.is_alive());
    p.take_damage(3);
    println!("hp: {}", p.hp);

    let v = Vec2(3.0, 4.0);
    println!("length: {}", v.length());  // 5.0
}
```

**Struct update syntax** (useful for spawning entities):

```rust
let template = Player::new(0.0, 0.0);
let p2 = Player { x: 50.0, y: 100.0, ..template };
// p2 has x=50, y=100 and all other fields copied from template
```

---

## Enums and Pattern Matching

Enums in Rust are sum types (tagged unions), not just integer constants like in C.
This is the foundation of game state machines.

```rust
// Simple enum — like C enum
#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// Enum with data in variants — this is where Rust shines
#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Start,
    Playing,
    Paused,
    GameOver { score: u32, level: u32 },  // named fields
    Win(u32),                              // tuple-style
}

fn describe_state(state: GameState) -> &'static str {
    match state {
        GameState::Start => "Press Start",
        GameState::Playing => "Playing",
        GameState::Paused => "Paused",
        GameState::GameOver { score, .. } => {
            // We can bind the score field here but ignore others with ..
            // Note: we can't return a formatted String from here easily
            // because we'd need to return a &str. In real code you'd
            // use format!() and return String instead.
            if score > 1000 { "GAME OVER — Great score!" } else { "GAME OVER" }
        }
        GameState::Win(level) => {
            if level >= 5 { "YOU WIN — All levels cleared!" } else { "Level clear!" }
        }
    }
}

fn main() {
    let state = GameState::GameOver { score: 9001, level: 3 };
    println!("{}", describe_state(state));

    // Match must be EXHAUSTIVE — if you add a variant to the enum,
    // every match block in the codebase will fail to compile until you handle it.
    // This is exactly what you want for game states.

    // Match with guards
    let hp = 15_i32;
    let status = match hp {
        0 => "dead",
        1..=10 => "critical",
        11..=50 => "wounded",
        _ => "healthy",    // _ is the wildcard "else" arm
    };
    println!("Status: {}", status);
}
```

### Option and Result

These are the two most important enums in Rust.

```rust
// Option<T> — either Some(value) or None. Replaces NULL in C.
// You cannot accidentally dereference a null pointer; the type system prevents it.

fn find_enemy(enemies: &[String], name: &str) -> Option<usize> {
    for (i, e) in enemies.iter().enumerate() {
        if e == name {
            return Some(i);
        }
    }
    None
}

fn main() {
    let enemies = vec!["goblin".to_string(), "orc".to_string()];

    match find_enemy(&enemies, "orc") {
        Some(idx) => println!("Found at index {}", idx),
        None => println!("Not found"),
    }

    // Shortcuts
    let idx = find_enemy(&enemies, "orc").unwrap();      // panics if None
    let idx = find_enemy(&enemies, "orc").unwrap_or(0);  // default value
    let idx = find_enemy(&enemies, "troll").unwrap_or_else(|| {
        println!("troll not found, using 0");
        0
    });

    // if let — when you only care about the Some case
    if let Some(i) = find_enemy(&enemies, "goblin") {
        println!("goblin at {}", i);
    }
}
```

```rust
// Result<T, E> — either Ok(value) or Err(error). Replaces errno in C.

use std::fs;
use std::num::ParseIntError;

fn parse_level(s: &str) -> Result<u32, ParseIntError> {
    s.trim().parse::<u32>()
}

fn main() {
    match parse_level("  3  ") {
        Ok(level) => println!("Level: {}", level),
        Err(e) => println!("Parse error: {}", e),
    }

    // The ? operator — propagate errors up the call stack
    // (only works in functions that return Result or Option)
    fn load_and_parse(s: &str) -> Result<u32, ParseIntError> {
        let n = s.trim().parse::<u32>()?;  // returns Err early if parse fails
        Ok(n * 2)
    }

    // unwrap() in game code
    // In initialization code (loading sprites, creating renderer), unwrap() or
    // expect() is fine — if setup fails, there's nothing to recover from.
    // In the game loop, handle errors gracefully.
    let level = parse_level("5").expect("level data must be valid");
}
```

---

## Error Handling

For game initialization code, `expect("message")` is appropriate:

```rust
let renderer = GameRenderer::new("My Game", 640, 480)
    .expect("failed to create SDL2 renderer");
```

For library code, return `Result`. For quick prototyping, use the `anyhow` crate:

```toml
[dependencies]
anyhow = "1"
```

```rust
use anyhow::{Context, Result};

fn init_game() -> Result<GameRenderer> {
    let renderer = GameRenderer::new("My Game", 640, 480)
        .context("failed to create SDL2 renderer")?;
    Ok(renderer)
}

fn main() -> Result<()> {
    let renderer = init_game()?;
    // ...
    Ok(())
}
```

---

## Collections

### Vec<T>

`Vec<T>` is Rust's dynamic array, equivalent to a heap-allocated flexible array in C.
It stores elements contiguously in memory — cache-friendly, like a C array.

```rust
fn main() {
    // Create
    let mut enemies: Vec<String> = Vec::new();
    let mut scores: Vec<i32> = vec![100, 200, 300];   // vec! macro
    let zeros = vec![0i32; 64];                         // 64 zeros

    // Add/remove
    enemies.push("goblin".to_string());
    enemies.push("orc".to_string());
    let last = enemies.pop();          // Option<String>

    // Access
    let first = &enemies[0];          // panics if OOB
    let safe = enemies.get(5);        // Option<&String> — safe access

    // Length
    println!("{}", enemies.len());
    println!("empty: {}", enemies.is_empty());

    // Iterate
    for e in &enemies {               // immutable references
        println!("{}", e);
    }
    for e in &mut enemies {           // mutable references
        e.push_str("!"); 
    }

    // Remove element by index (order-preserving, O(n))
    enemies.remove(0);

    // Remove by swapping with last (O(1), changes order)
    if !enemies.is_empty() {
        enemies.swap_remove(0);
    }

    // Retain elements matching a condition (in-place filter)
    scores.retain(|&s| s > 150);      // keeps 200, 300

    // Reserve capacity upfront (important for performance in game loops)
    let mut bullets: Vec<(f32, f32)> = Vec::with_capacity(256);
}
```

### HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut high_scores: HashMap<String, u32> = HashMap::new();

    high_scores.insert("Alice".to_string(), 9000);
    high_scores.insert("Bob".to_string(), 7500);

    // Access
    let score = high_scores.get("Alice");   // Option<&u32>
    println!("{:?}", score);

    // Default if missing
    let score = high_scores.get("Charlie").copied().unwrap_or(0);

    // Entry API — insert if not present
    high_scores.entry("Charlie".to_string()).or_insert(5000);

    // Iterate
    for (name, score) in &high_scores {
        println!("{}: {}", name, score);
    }
}
```

### Iterators

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // map: transform each element
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();

    // filter: keep elements matching predicate
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();

    // chain
    let result: Vec<i32> = numbers.iter()
        .filter(|&&x| x > 3)
        .map(|&x| x * 10)
        .collect();     // [40, 50, 60, 70, 80]

    // sum, max, min, count
    let total: i32 = numbers.iter().sum();
    let max = numbers.iter().max();

    // enumerate: (index, value) pairs
    for (i, &n) in numbers.iter().enumerate() {
        println!("[{}] = {}", i, n);
    }

    // any / all
    let has_big = numbers.iter().any(|&x| x > 7);
    let all_pos = numbers.iter().all(|&x| x > 0);

    // find
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);

    // position
    let pos = numbers.iter().position(|&x| x == 5);
}
```

**In game dev, iterators are often cleaner than index loops for read-only operations.
For mutations that need to call methods on the game struct (`self`), use index loops.**
This is explained in depth in Part 2.

---

## Control Flow

```rust
fn main() {
    // if/else — same as C but no parentheses required
    let hp = 50;
    if hp <= 0 {
        println!("dead");
    } else if hp < 25 {
        println!("critical");
    } else {
        println!("ok");
    }

    // if as expression
    let status = if hp > 0 { "alive" } else { "dead" };

    // loop — infinite loop (use break to exit, continue to skip)
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;   // loop can return a value!
        }
    };

    // while
    let mut n = 1;
    while n < 100 {
        n *= 2;
    }

    // for over a range
    for i in 0..10 {       // 0, 1, ..., 9 (exclusive end)
        print!("{} ", i);
    }
    for i in 0..=10 {      // 0, 1, ..., 10 (inclusive end)
        print!("{} ", i);
    }

    // for over a collection
    let names = vec!["wizard", "knight", "rogue"];
    for name in &names {
        println!("{}", name);
    }

    // match — covered in enums section, but also works on integers
    let x = 5u32;
    match x {
        0 => println!("zero"),
        1 | 2 | 3 => println!("small"),
        4..=10 => println!("medium"),
        _ => println!("large"),
    }
}
```

---

## Functions, Closures, and Fn Traits

```rust
// Basic function
fn add(a: i32, b: i32) -> i32 {
    a + b   // last expression without semicolon is the return value
}

// Multiple return values via tuple
fn split_velocity(speed: f32, angle: f32) -> (f32, f32) {
    (speed * angle.cos(), speed * angle.sin())
}

// Closures: anonymous functions that capture their environment
fn main() {
    let multiplier = 3;
    let triple = |x: i32| x * multiplier;  // captures multiplier
    println!("{}", triple(5));  // 15

    // Closures as arguments
    let numbers = vec![1, 2, 3, 4, 5];
    let evens: Vec<i32> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();

    // Storing a closure in a struct (for callbacks)
    // Use Box<dyn Fn()> for type-erased closures
    struct Button {
        label: String,
        on_click: Box<dyn Fn()>,
    }

    let btn = Button {
        label: "Start".to_string(),
        on_click: Box::new(|| println!("Start clicked!")),
    };
    (btn.on_click)();
}
```

**The Fn trait family:**

- `Fn` — can be called multiple times, doesn't mutate captures
- `FnMut` — can be called multiple times, may mutate captures
- `FnOnce` — can only be called once, consumes captures

```rust
fn call_once(f: impl FnOnce()) { f(); }
fn call_repeat(f: impl Fn()) { f(); f(); }
fn call_mutating(mut f: impl FnMut()) { f(); f(); }

fn main() {
    let name = String::from("wizard");
    // FnOnce — consumes name (moves it into the closure)
    call_once(move || println!("{}", name));

    let prefix = "HP:";
    // Fn — borrows prefix immutably, can be called multiple times
    call_repeat(|| println!("{} 100", prefix));

    let mut counter = 0;
    // FnMut — mutates counter
    call_mutating(|| { counter += 1; println!("count: {}", counter); });
}
```

---

## Traits

Traits are like interfaces in other languages, or abstract base classes in C++.
They define behavior that types can implement.

```rust
// Define a trait
trait Drawable {
    fn draw(&self, x: f32, y: f32);
    fn bounds(&self) -> (f32, f32, f32, f32);  // x, y, w, h

    // Default implementation
    fn center(&self) -> (f32, f32) {
        let (x, y, w, h) = self.bounds();
        (x + w / 2.0, y + h / 2.0)
    }
}

struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Drawable for Circle {
    fn draw(&self, ox: f32, oy: f32) {
        println!("draw circle at ({}, {})", self.x + ox, self.y + oy);
    }
    fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x - self.r, self.y - self.r, self.r * 2.0, self.r * 2.0)
    }
}

impl Drawable for Rect {
    fn draw(&self, ox: f32, oy: f32) {
        println!("draw rect at ({}, {})", self.x + ox, self.y + oy);
    }
    fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }
}

// Trait bounds: accept any type that implements Drawable
fn draw_at_origin(d: &impl Drawable) {
    d.draw(0.0, 0.0);
}

// Alternative syntax with where clause (needed for complex bounds)
fn draw_pair<T>(a: &T, b: &T) where T: Drawable {
    a.draw(0.0, 0.0);
    b.draw(10.0, 0.0);
}

// Dynamic dispatch: store different types in the same collection
fn draw_all(items: &[Box<dyn Drawable>]) {
    for item in items {
        item.draw(0.0, 0.0);
    }
}
```

**Common standard library traits:**

```rust
// Debug — enables {:?} formatting
#[derive(Debug)]
struct Point { x: f32, y: f32 }

// Clone — enables .clone()
// Copy — enables implicit copy on assignment (for small types)
// PartialEq — enables == operator
// Default — enables Point::default() which returns zeroed fields
#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Vec2 { x: f32, y: f32 }

// Display — enables {} formatting
use std::fmt;
impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

fn main() {
    let v = Vec2 { x: 1.0, y: 2.0 };
    println!("{:?}", v);    // debug:   Vec2 { x: 1.0, y: 2.0 }
    println!("{}", v);      // display: (1.00, 2.00)
    let v2 = v;             // copy, not move (because of Copy)
    let v3 = v.clone();     // explicit clone
    assert_eq!(v, v2);      // works because of PartialEq
}
```

---

## Generics

```rust
// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic struct
struct Pool<T> {
    items: Vec<T>,
    active: Vec<bool>,
}

impl<T: Default> Pool<T> {
    fn new(capacity: usize) -> Self {
        Pool {
            items: (0..capacity).map(|_| T::default()).collect(),
            active: vec![false; capacity],
        }
    }

    fn allocate(&mut self) -> Option<usize> {
        self.active.iter().position(|&a| !a).map(|i| {
            self.active[i] = true;
            i
        })
    }

    fn free(&mut self, index: usize) {
        if index < self.active.len() {
            self.active[index] = false;
        }
    }
}

fn main() {
    let nums = vec![34, 50, 25, 100, 65];
    println!("largest: {}", largest(&nums));

    let mut bullet_pool: Pool<(f32, f32, f32, f32)> = Pool::new(256);
    if let Some(idx) = bullet_pool.allocate() {
        bullet_pool.items[idx] = (100.0, 200.0, 5.0, 0.0); // x, y, vx, vy
    }
}
```

---

## Modules

```rust
// In src/main.rs or src/lib.rs:
mod math {
    // Everything in a module is private by default
    // Use pub to expose it
    pub struct Vec2 {
        pub x: f32,
        pub y: f32,
    }

    impl Vec2 {
        pub fn new(x: f32, y: f32) -> Self {
            Self { x, y }
        }

        pub fn length(&self) -> f32 {
            (self.x * self.x + self.y * self.y).sqrt()
        }

        // Private method — not accessible from outside this module
        fn squared_length(&self) -> f32 {
            self.x * self.x + self.y * self.y
        }
    }

    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }
}

// File-based modules: put code in src/math.rs or src/math/mod.rs
// then declare: mod math;
// The compiler looks for src/math.rs or src/math/mod.rs automatically.

use math::Vec2;
use math::lerp;

fn main() {
    let v = Vec2::new(3.0, 4.0);
    println!("length: {}", v.length());  // 5.0
    println!("lerp: {}", lerp(0.0, 10.0, 0.5));  // 5.0
}
```

---

# Part 2: Rust's Unique Powers

## The Borrow Checker in Depth

### Lifetimes

A lifetime is a label for how long a reference is valid. The compiler tracks them
automatically in most cases. You only need to write lifetime annotations when the compiler
can't figure it out on its own.

```rust
// This works — compiler infers that the returned reference lives as long as the input
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// This requires explicit lifetimes — the compiler doesn't know
// whether the result borrows from x or y
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// 'a says: the output reference lives as long as the SHORTER of x and y

// Structs with references need lifetime annotations
struct TextDisplay<'a> {
    text: &'a str,   // this struct cannot outlive the &str it borrows
}

impl<'a> TextDisplay<'a> {
    fn new(text: &'a str) -> Self {
        TextDisplay { text }
    }
    fn show(&self) { println!("{}", self.text); }
}
```

**Lifetime elision rules** (when you don't need to write `'a`):

1. Each reference parameter gets its own lifetime.
2. If there's exactly one input lifetime, it's assigned to all output lifetimes.
3. If one parameter is `&self` or `&mut self`, its lifetime is assigned to all outputs.

In game dev, you rarely need explicit lifetimes. The main case is storing a reference
in a struct, which you should usually avoid — own the data instead.

### Interior Mutability: Cell and RefCell

Sometimes the borrow checker is too strict. You have a `&self` method but need to
mutate something inside. Two escape hatches exist for single-threaded code:

```rust
use std::cell::Cell;
use std::cell::RefCell;

// Cell<T>: for Copy types (i32, f32, bool)
// Mutation through a shared reference, no runtime overhead
struct Enemy {
    x: f32,
    y: f32,
    shoot_timer: Cell<f32>,   // can be mutated via &self
}

impl Enemy {
    fn tick(&self, dt: f32) {
        let t = self.shoot_timer.get() - dt;
        self.shoot_timer.set(t);
    }
}

// RefCell<T>: for non-Copy types
// Runtime borrow checking (panics if you borrow incorrectly)
// Use sparingly — it's an escape hatch, not a design pattern
struct GameWorld {
    log: RefCell<Vec<String>>,
}

impl GameWorld {
    fn log_event(&self, msg: &str) {
        self.log.borrow_mut().push(msg.to_string());
    }
    fn print_log(&self) {
        for line in self.log.borrow().iter() {
            println!("{}", line);
        }
    }
}
```

### Index-Based Loops: The Game Dev Escape Hatch

This is the most important practical pattern in this entire guide.

**The problem:**

```rust
struct Game {
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
}

impl Game {
    fn update(&mut self) {
        // This fails to compile:
        for enemy in &mut self.enemies {
            if enemy.should_shoot() {
                self.spawn_bullet(enemy.x, enemy.y);  // ERROR
                // Cannot borrow `*self` as mutable because it is also borrowed
                // as mutable (via the for loop over self.enemies)
            }
        }
    }

    fn spawn_bullet(&mut self, x: f32, y: f32) {
        self.bullets.push(Bullet { x, y, vx: 5.0, vy: 0.0 });
    }
}
```

The compiler sees: you have `&mut self.enemies` (via the for loop) AND you're trying
to call `self.spawn_bullet` which takes `&mut self`. Two mutable borrows of `self`.
Even though they access different fields, the compiler doesn't allow it at this level
of analysis.

**The fix — use an index loop:**

```rust
impl Game {
    fn update(&mut self) {
        for i in 0..self.enemies.len() {
            if self.enemies[i].should_shoot() {
                let (x, y) = (self.enemies[i].x, self.enemies[i].y);
                self.spawn_bullet(x, y);   // fine — no active borrow of self.enemies
            }
        }
    }
}
```

The index loop `for i in 0..self.enemies.len()` computes the range before the loop
starts and doesn't hold a borrow on `self.enemies`. Inside the loop body, each access
`self.enemies[i]` is a fresh borrow that ends immediately after the expression.
So `self.spawn_bullet()` can borrow `self` mutably without conflicting.

We use this pattern in every game in this repo.

**Collecting new items to add after the loop (another common pattern):**

```rust
impl Game {
    fn update_enemies(&mut self) {
        let mut new_bullets = Vec::new();

        for i in 0..self.enemies.len() {
            if self.enemies[i].shoot_timer <= 0.0 {
                new_bullets.push(Bullet {
                    x: self.enemies[i].x,
                    y: self.enemies[i].y,
                    vx: 5.0,
                    vy: 0.0,
                });
                self.enemies[i].shoot_timer = 60.0;
            }
            self.enemies[i].shoot_timer -= 1.0;
        }

        // Now it's safe to push — no borrows active
        self.bullets.extend(new_bullets);
    }
}
```

---

## Pattern Matching Mastery

```rust
// Destructuring structs
struct Point { x: f32, y: f32 }

let p = Point { x: 3.0, y: 7.0 };
let Point { x, y } = p;   // x = 3.0, y = 7.0

// In match
match p {
    Point { x: 0.0, y } => println!("on y-axis at {}", y),
    Point { x, y: 0.0 } => println!("on x-axis at {}", x),
    Point { x, y } => println!("({}, {})", x, y),
}

// Destructuring enums
enum Message {
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
    Quit,
}

let msg = Message::Move { x: 10, y: -5 };
match msg {
    Message::Move { x, y } => println!("move to ({}, {})", x, y),
    Message::Write(text) => println!("write: {}", text),
    Message::Color(r, g, b) => println!("color: #{:02X}{:02X}{:02X}", r, g, b),
    Message::Quit => println!("quit"),
}

// Match guards
let n = 42i32;
match n {
    x if x < 0 => println!("negative"),
    x if x % 2 == 0 => println!("positive even"),
    _ => println!("positive odd"),
}

// if let — match exactly one arm
let config: Option<String> = Some("debug".to_string());
if let Some(mode) = config {
    println!("mode: {}", mode);
}

// while let — loop while pattern matches
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);   // 3, 2, 1
}

// Nested patterns
let point_pair = ((0.0f32, 5.0f32), (3.0f32, 0.0f32));
let ((ax, ay), (bx, by)) = point_pair;

// Binding with @ 
let n = 15u32;
match n {
    x @ 1..=10 => println!("small: {}", x),
    x @ 11..=20 => println!("medium: {}", x),
    x => println!("large: {}", x),
}
```

---

## Iterators and Functional Style

Iterators in Rust are zero-cost abstractions — the compiler optimizes chains of
`.map().filter().collect()` into the same machine code as a handwritten loop.

```rust
fn main() {
    let enemies = vec![
        ("goblin", 10, true),
        ("orc", 50, false),
        ("dragon", 200, true),
        ("slime", 5, true),
    ];

    // Active enemies sorted by HP
    let mut active: Vec<_> = enemies.iter()
        .filter(|(_, _, alive)| *alive)
        .collect();
    active.sort_by(|a, b| a.1.cmp(&b.1));

    // Total HP of living enemies
    let total_hp: i32 = enemies.iter()
        .filter(|(_, _, alive)| *alive)
        .map(|(_, hp, _)| hp)
        .sum();

    // enumerate: index + value
    for (i, (name, hp, _)) in enemies.iter().enumerate() {
        println!("[{}] {} ({}HP)", i, name, hp);
    }

    // zip: pair two iterators
    let names = vec!["Alice", "Bob", "Carol"];
    let scores = vec![100, 200, 150];
    for (name, score) in names.iter().zip(scores.iter()) {
        println!("{}: {}", name, score);
    }

    // windows: sliding window of size n
    let positions = vec![1.0f32, 2.0, 4.0, 7.0, 11.0];
    for window in positions.windows(2) {
        println!("delta: {}", window[1] - window[0]);
    }

    // chunks: non-overlapping groups
    let tile_data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9];
    for chunk in tile_data.chunks(3) {
        println!("{:?}", chunk);   // [1,2,3], [4,5,6], [7,8,9]
    }

    // flat_map: map then flatten
    let level_enemies = vec![vec!["goblin", "goblin"], vec!["orc"]];
    let all: Vec<&str> = level_enemies.into_iter().flatten().collect();
}
```

**When to use iterators vs index loops in game dev:**

- Read-only transforms over a single collection: use iterators (cleaner, same speed)
- Mutations that need `self.method()`: use index loops (avoids borrow checker issues)
- Processing multiple collections that interact: use index loops (safer)
- Spawning new entities from existing ones: collect into a temp Vec, extend after

---

## Macros

```rust
// derive macros — generate trait implementations automatically
#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Vec2 {
    x: f32,
    y: f32,
}

// macro_rules! — declarative macros (patterns)
macro_rules! clamp {
    ($val:expr, $min:expr, $max:expr) => {
        if $val < $min { $min }
        else if $val > $max { $max }
        else { $val }
    }
}

// Usage
let speed = clamp!(player_speed, 0.0_f32, 10.0_f32);

// Common built-in macros
let v: Vec<i32> = vec![1, 2, 3];        // create Vec
assert_eq!(2 + 2, 4);                    // equality assertion (panics on failure)
assert!(v.len() > 0);                    // bool assertion
todo!("implement this");                  // panics with "not yet implemented"
unreachable!("should never reach here"); // panics
dbg!(v.len());                           // prints file/line/value, returns the value
```

---

## Unsafe Rust

Unsafe is the escape hatch for when Rust's rules are too strict. Use it rarely,
contain it, document why it's correct.

```rust
unsafe fn dangerous() {
    // Raw pointer operations
    let mut x = 42i32;
    let raw = &mut x as *mut i32;  // create raw pointer (safe)

    // Dereferencing raw pointers requires unsafe
    *raw = 100;
    println!("{}", *raw);
}

fn main() {
    unsafe { dangerous(); }

    // FFI: calling C functions
    // This is the main use case in games — calling SDL2 C API directly
    extern "C" {
        fn abs(x: i32) -> i32;
    }
    let n = unsafe { abs(-5) };
    println!("{}", n);  // 5
}
```

In practice, the `sdl2` crate wraps all unsafe SDL2 calls for you. You'll rarely
write `unsafe` directly unless you're interfacing with a custom C library.

---

# Part 3: Patterns for Game Development

## Game Loop Pattern

```rust
fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let window = video.window("Game", 640, 480).build().map_err(|e| e.to_string())?;
    let canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl.event_pump()?;

    let mut renderer = GameRenderer::from_parts(canvas, texture_creator);
    let mut input = Input::new(event_pump);
    let mut clock = GameClock::new(60.0);
    let mut game = Game::new();

    'main: loop {
        // 1. Poll input
        input.poll();
        if input.should_quit() { break 'main; }

        // 2. Update (fixed timestep)
        while clock.should_update() {
            game.update(&input, clock.fixed_dt() as f32);
        }

        // 3. Draw
        renderer.clear(BLACK);
        game.draw(&mut renderer);
        renderer.present();

        // 4. Sleep until next frame (paces CPU usage)
        clock.wait_for_next_frame();
    }

    Ok(())
}
```

## Fixed Timestep with Accumulator

The `GameClock` in `retro-sdl2` handles this for you, but understanding it is important:

```rust
// The accumulator pattern prevents physics from running faster on fast hardware
// and maintains determinism regardless of render frame rate.

struct GameClock {
    fixed_dt: f64,          // 1.0 / target_fps (e.g. 1/60 = 0.01667 s)
    accumulator: f64,       // time bank: not yet consumed by update steps
    last_instant: Instant,
}

impl GameClock {
    fn tick(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_instant).as_secs_f64();
        self.last_instant = now;

        // Death spiral prevention: if the frame took longer than 250ms
        // (e.g. OS preempted us), cap it. Otherwise we'd simulate 10 seconds
        // of physics in one frame to "catch up", which is visible as a
        // massive position jump.
        let capped = elapsed.min(0.250);
        self.accumulator += capped;
    }

    fn should_update(&mut self) -> bool {
        if self.accumulator >= self.fixed_dt {
            self.accumulator -= self.fixed_dt;
            true
        } else {
            false
        }
    }
}

// In the game loop:
clock.tick();
while clock.should_update() {
    game.update(clock.fixed_dt as f32);
}
game.draw();
```

## State Machines with Enums

This is where Rust's exhaustive match shines. Add a new state and every match block
that doesn't handle it becomes a compile error — you can't forget to handle it.

```rust
#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Start,
    Playing,
    LevelStory,     // cutscene between levels
    GameOver,
    Win,
}

struct Game {
    state: GameState,
    player: Player,
    enemies: Vec<Enemy>,
    // ...
}

impl Game {
    fn update(&mut self, input: &Input, dt: f32) {
        match self.state {
            GameState::Start => {
                if input.is_key_pressed(KeyCode::Return) {
                    self.state = GameState::Playing;
                }
            }
            GameState::Playing => {
                self.update_gameplay(input, dt);
            }
            GameState::LevelStory => {
                self.update_story(input);
            }
            GameState::GameOver => {
                if input.is_key_pressed(KeyCode::Return) {
                    self.reset();
                    self.state = GameState::Start;
                }
            }
            GameState::Win => {
                if input.is_key_pressed(KeyCode::Return) {
                    self.state = GameState::Start;
                }
            }
            // If you add a new variant to GameState and forget to add it here,
            // the compiler gives: error[E0004]: non-exhaustive patterns
            // and points to this match block. You cannot ship broken state machines.
        }
    }

    fn draw(&self, renderer: &mut GameRenderer) {
        // Every state must be handled in EVERY match block
        match self.state {
            GameState::Start => self.draw_start_screen(renderer),
            GameState::Playing => self.draw_gameplay(renderer),
            GameState::LevelStory => self.draw_story(renderer),
            GameState::GameOver => self.draw_game_over(renderer),
            GameState::Win => self.draw_win(renderer),
        }
    }
}
```

## Entity Storage: Vec with Index-Based Access

We use flat `Vec` arrays with index-based access throughout all the games.
This is the pattern from our codebase:

```rust
// DON'T do this — you'll fight the borrow checker constantly
struct GameBad {
    enemies: Vec<Box<dyn Enemy>>,  // trait objects + heap = cache-unfriendly
}

// DO this — flat Vec, index-based access, struct-of-arrays style
struct Game {
    // All enemies stored in flat arrays for cache locality
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    particles: Vec<Particle>,
    // ...
}

impl Game {
    fn update_enemies(&mut self, dt: f32) {
        // Index loop — allows calling self methods inside the loop
        for i in 0..self.enemies.len() {
            // Update position
            self.enemies[i].x += self.enemies[i].vx * dt;
            self.enemies[i].y += self.enemies[i].vy * dt;

            // Enemy fires a bullet — needs self.bullets
            self.enemies[i].shoot_timer -= dt;
            if self.enemies[i].shoot_timer <= 0.0 {
                self.enemies[i].shoot_timer = 2.0;
                let bx = self.enemies[i].x;
                let by = self.enemies[i].y;
                // Safe: we're not iterating self.bullets
                self.bullets.push(Bullet { x: bx, y: by, vx: -5.0, vy: 0.0 });
            }
        }

        // Remove dead enemies (iterate backwards to preserve indices)
        for i in (0..self.enemies.len()).rev() {
            if self.enemies[i].hp <= 0 {
                self.spawn_particles(self.enemies[i].x, self.enemies[i].y);
                self.enemies.swap_remove(i);   // O(1) removal, changes order
            }
        }
    }

    fn spawn_particles(&mut self, x: f32, y: f32) {
        for _ in 0..8 {
            let angle: f32 = fastrand::f32() * std::f32::consts::TAU;
            let speed: f32 = 1.0 + fastrand::f32() * 3.0;
            self.particles.push(Particle {
                x, y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                life: 30.0,
            });
        }
    }
}
```

**Removing entities during iteration — use reverse index loop:**

```rust
// Forward iteration with removal is tricky because indices shift.
// Backward iteration is safe and simple.
fn remove_dead(items: &mut Vec<Bullet>) {
    for i in (0..items.len()).rev() {
        if items[i].out_of_bounds() {
            items.swap_remove(i);  // O(1) — swaps with last element, truncates
        }
    }
}
```

## Component Pattern Without ECS

For small games, parallel flat arrays (struct-of-arrays layout) give you the
cache efficiency of an ECS without the complexity:

```rust
// Instead of Vec<Enemy> where Enemy has all fields:
struct EnemySystem {
    x:     Vec<f32>,
    y:     Vec<f32>,
    vx:    Vec<f32>,
    vy:    Vec<f32>,
    hp:    Vec<i32>,
    kind:  Vec<EnemyKind>,
    alive: Vec<bool>,
    count: usize,
}

impl EnemySystem {
    fn spawn(&mut self, x: f32, y: f32, kind: EnemyKind) {
        if self.count < self.x.len() {
            let i = self.count;
            self.x[i] = x;
            self.y[i] = y;
            self.vx[i] = 0.0;
            self.vy[i] = 0.0;
            self.hp[i] = 10;
            self.kind[i] = kind;
            self.alive[i] = true;
            self.count += 1;
        }
    }
}
```

In practice, for games of our scale (hundreds of entities), a simple `Vec<Enemy>` with
a plain struct is more readable and fast enough. Use struct-of-arrays for performance
tuning if needed.

## Sprite System: String Art to Pixel Buffer to Texture

From our `retro-sdl2` crate:

```rust
// Define sprite art as string arrays
// '.' = transparent, '1'-'9' = index into color palette (1-based)
const PLAYER_ART: [&str; 8] = [
    "..1111..",
    ".122221.",
    "13122131",
    "13322331",
    ".122221.",
    "..1111..",
    ".121121.",
    "12211221",
];
const PLAYER_COLORS: [Color; 3] = [
    BLACK,                                    // color 1
    Color::new(0.0, 1.0, 1.0, 1.0),          // color 2: cyan
    WHITE,                                    // color 3
];

// At startup, compile art + colors into an SDL2 Texture
let player_tex = create_sprite(&renderer.texture_creator(), &PLAYER_ART, &PLAYER_COLORS)
    .expect("failed to create player sprite");

// In the draw loop, render the texture
renderer.draw_texture_ex(&player_tex, player.x, player.y, DrawTextureParams {
    dest_size: Some((32.0, 32.0)),    // scale up to 32x32
    flip_x: player.facing_left,
    ..Default::default()
});
```

## Random Numbers with fastrand

```rust
// Cargo.toml: fastrand = "2"
// fastrand is no-std compatible, has no dependencies, and is fast enough for games.

use fastrand;

fn main() {
    // Float in [0.0, 1.0)
    let r: f32 = fastrand::f32();

    // IMPORTANT: annotate the type explicitly to avoid ambiguity with f64
    let angle: f32 = fastrand::f32() * std::f32::consts::TAU;
    let vx = angle.cos() * 5.0;   // unambiguous now that angle is typed
    let vy = angle.sin() * 5.0;

    // Integer in range
    let die = fastrand::u32(1..=6);

    // Bool
    let flip = fastrand::bool();

    // Shuffle a slice
    let mut items = vec![1, 2, 3, 4, 5];
    fastrand::shuffle(&mut items);

    // Pick a random element
    let pick = items[fastrand::usize(0..items.len())];
}
```

---

# Part 4: SDL2 in Rust — Complete Reference

## Setup

```toml
# Cargo.toml
[dependencies]
sdl2 = { version = "0.37", features = [] }
```

For dynamic linking (needed for Miyoo — links against the device's libSDL2.so):

```toml
[dependencies]
sdl2 = { version = "0.37", features = ["use_pkgconfig"] }
```

Or use the `retro-sdl2` wrapper crate from this repo, which sets it up correctly.

## Initializing SDL2

```rust
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() -> Result<(), String> {
    // SDL2 context manages library lifetime
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("My Game", 640, 480)
        .position_centered()
        // .fullscreen()           // for embedded devices
        // .borderless()
        .build()
        .map_err(|e| e.to_string())?;

    // Canvas is the renderer — wraps the window for 2D drawing
    let mut canvas = window
        .into_canvas()
        .accelerated()             // use GPU acceleration
        .present_vsync()           // sync to display refresh
        .build()
        .map_err(|e| e.to_string())?;

    // For pixel-perfect scaling (nearest-neighbor interpolation)
    canvas.set_logical_size(640, 480).map_err(|e| e.to_string())?;
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump()?;

    // Main loop
    'running: loop {
        // Event polling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        // Clear
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw
        canvas.set_draw_color(Color::RGB(255, 64, 64));
        canvas.fill_rect(sdl2::rect::Rect::new(100, 100, 64, 64))?;

        // Present (swap buffers)
        canvas.present();

        // Cap to 60fps if vsync isn't available
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
```

## Drawing Primitives

```rust
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;

fn draw_frame(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
    // Filled rectangle
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.fill_rect(Rect::new(10, 10, 100, 50))?;   // x, y, w, h

    // Rectangle outline
    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.draw_rect(Rect::new(10, 10, 100, 50))?;

    // Line
    canvas.set_draw_color(Color::RGB(255, 255, 0));
    canvas.draw_line(Point::new(0, 0), Point::new(640, 480))?;

    // Multiple lines (more efficient than individual draw_line calls)
    let points: Vec<Point> = vec![
        Point::new(0, 0),
        Point::new(100, 100),
        Point::new(200, 50),
    ];
    canvas.draw_lines(points.as_slice())?;

    // Point
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_point(Point::new(320, 240))?;

    Ok(())
}
```

## Textures and Sprites

```rust
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

fn make_sprite_texture<'a>(
    creator: &'a TextureCreator<WindowContext>,
    pixel_data: &[u8],   // RGBA bytes
    width: u32,
    height: u32,
) -> Result<sdl2::render::Texture<'a>, String> {
    // Surface is a CPU-side pixel buffer
    let pitch = width * 4;  // bytes per row = width * 4 bytes per pixel (RGBA)
    let mut pixels = pixel_data.to_vec();
    let surface = Surface::from_data(
        &mut pixels,
        width,
        height,
        pitch,
        PixelFormatEnum::RGBA8888,
    ).map_err(|e| e.to_string())?;

    // Convert to GPU texture
    creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
}

fn draw_texture(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    texture: &sdl2::render::Texture,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    angle_degrees: f64,
    flip_horizontal: bool,
) -> Result<(), String> {
    let dst = Rect::new(x, y, w, h);
    let center = sdl2::rect::Point::new((w / 2) as i32, (h / 2) as i32);

    canvas.copy_ex(
        texture,
        None,              // source rect (None = whole texture)
        Some(dst),         // destination rect
        angle_degrees,     // rotation
        Some(center),      // rotation center (relative to dst)
        flip_horizontal,   // flip_x
        false,             // flip_y
    )?;
    Ok(())
}
```

## Keyboard Input

```rust
use sdl2::keyboard::Scancode;

// Snapshot approach: query all keys at once (prefer this for games)
fn handle_input(event_pump: &sdl2::EventPump, player: &mut Player) {
    let kb = event_pump.keyboard_state();

    if kb.is_scancode_pressed(Scancode::Left) || kb.is_scancode_pressed(Scancode::A) {
        player.vx = -MOVE_SPEED;
    } else if kb.is_scancode_pressed(Scancode::Right) || kb.is_scancode_pressed(Scancode::D) {
        player.vx = MOVE_SPEED;
    } else {
        player.vx = 0.0;
    }

    if kb.is_scancode_pressed(Scancode::Space) && player.on_ground {
        player.vy = JUMP_FORCE;
    }
}

// Edge detection (pressed this frame, not held):
// Requires comparing current vs previous frame state.
// The Input struct in retro-sdl2 handles this for you.
```

## Timing with std::time::Instant

```rust
use std::time::Instant;

fn main() {
    let start = Instant::now();

    // ... do work ...

    let elapsed = start.elapsed();
    println!("elapsed: {:.3}s", elapsed.as_secs_f64());
    println!("elapsed ms: {}", elapsed.as_millis());

    // Frame timing
    let frame_start = Instant::now();
    // ... render ...
    let frame_time = frame_start.elapsed();
    let target = std::time::Duration::from_millis(16);
    if frame_time < target {
        std::thread::sleep(target - frame_time);
    }
}
```

## The retro-sdl2 Crate: Macroquad-Compatible API

Our `retro-sdl2` crate wraps SDL2 with function signatures that mirror macroquad.
This means game logic written for macroquad ports needs minimal changes.

```rust
use retro_sdl2::*;

// Color — f32 RGBA components
let red = Color::new(1.0, 0.0, 0.0, 1.0);
let cyan = Color::new(0.0, 1.0, 1.0, 1.0);
let semi_white = Color::new(1.0, 1.0, 1.0, 0.5);

// Built-in color constants
let c = BLACK;   // Color::new(0,0,0,1)
let c = WHITE;
let c = RED;
let c = GREEN;
let c = BLUE;

// GameRenderer wraps Canvas<Window>
let mut renderer = GameRenderer::new("Title", 640, 480)?;

// Draw calls (all take f32 coordinates)
renderer.clear(BLACK);
renderer.draw_rectangle(10.0, 20.0, 100.0, 50.0, RED);
renderer.draw_rectangle_lines(10.0, 20.0, 100.0, 50.0, 2.0, WHITE);
renderer.draw_circle(320.0, 240.0, 30.0, YELLOW);
renderer.draw_line(0.0, 0.0, 640.0, 480.0, 1.0, GREEN);

// Text rendering (embedded 8x8 bitmap font)
draw_text(&mut renderer, "GAME OVER", 240.0, 200.0, 16.0, WHITE);
// font_size=8 → 1:1, font_size=16 → 2x scale, font_size=24 → 3x scale

// Measure text width before drawing (for centering)
let w = measure_text("SCORE: 1000", 16.0);
let x = (640.0 - w) / 2.0;
draw_text(&mut renderer, "SCORE: 1000", x, 240.0, 16.0, WHITE);

// Textures
let player_tex = create_sprite(
    renderer.texture_creator(),
    &PLAYER_ART,
    &PLAYER_COLORS
).expect("sprite");

renderer.draw_texture_ex(&player_tex, 100.0, 200.0, DrawTextureParams {
    dest_size: Some((32.0, 32.0)),
    flip_x: facing_left,
    rotation: 0.0,
    ..Default::default()
});

// CRT effects
draw_scanlines(&mut renderer, 640.0, 480.0, 1.0, 1.0, 0.15);
draw_vignette(&mut renderer, 640.0, 480.0, 1.0, 12, 0.6);

renderer.present();

// Input
let mut input = Input::new(event_pump);
input.poll();                               // call once per frame
let left = input.is_key_down(KeyCode::Left);
let jump = input.is_key_pressed(KeyCode::Space);  // edge detection
let quit = input.should_quit();

// Timing
let mut clock = GameClock::new(60.0);
clock.tick();
while clock.should_update() {
    game.update(clock.fixed_dt() as f32);
}
clock.wait_for_next_frame();
```

---

# Part 5: Cross-Compilation for Miyoo Mini Plus

## The Miyoo Environment

- **CPU**: ARM Cortex-A7, ARMv7 hardfloat
- **OS**: Linux (buildroot-based), glibc 2.28
- **SDL2**: custom `mmiyoo` driver provided by the parasyte runtime
  (built as `libSDL2.so`, dropped into the game directory)
- **Display**: 640×480 framebuffer, accessed through SDL2
- **Input**: D-Pad + face buttons mapped to keyboard scancodes
- **Audio**: SDL2 audio via the mmiyoo driver (optional)

The key constraint: the device's libc is glibc 2.28. If you compile against a newer
glibc on your host, the binary will fail to start on the device with a "version
GLIBC_2.29 not found" error. Hence `cargo-zigbuild`.

## cargo-zigbuild and glibc Targeting

`cargo zigbuild` replaces the linker with zig's bundled linker, which allows specifying
a minimum glibc version. This ensures the binary only uses symbols available in glibc 2.28.

```bash
# Install
cargo install cargo-zigbuild

# You also need zig itself
# On Arch: sudo pacman -S zig
# On Ubuntu: snap install zig --classic --beta
# Or from ziglang.org/download/

# Add the target
rustup target add armv7-unknown-linux-gnueabihf

# Build with glibc 2.28 targeting
cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.28 --release

# Binary is at:
# target/armv7-unknown-linux-gnueabihf/release/<crate-name>
```

## Dynamic Linking to parasyte SDL2

The Miyoo games use dynamic linking (`features = []` in sdl2 dependency — no
`bundled` or `static` features). This means the binary links against `libSDL2.so`
at runtime. On the device, the parasyte runtime provides this library.

Your game directory on the device should contain:
```
MyGame/
├── MyGame          # the ELF binary
├── launch.sh       # shell script to set env and run the binary
└── libSDL2.so      # SDL2 library from the parasyte runtime
```

**launch.sh:**
```bash
#!/bin/sh
cd "$(dirname "$0")"
export SDL_VIDEODRIVER=mmiyoo
export SDL_AUDIODRIVER=mmiyoo
./MyGame
```

`SDL_VIDEODRIVER=mmiyoo` tells SDL2 to use the Miyoo-specific video backend instead
of trying to connect to an X11 display (which doesn't exist on the device).

## Build Command Summary

```bash
# From the game directory (e.g. miyoo/micro/)

# Check types compile (fast, use constantly during development)
cargo check

# Debug build for desktop testing
cargo build
./target/debug/micro_miyoo

# Release build for desktop testing
cargo build --release
./target/release/micro_miyoo

# Cross-compile for Miyoo Mini Plus
cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.28 --release
# Output: target/armv7-unknown-linux-gnueabihf/release/micro_miyoo
```

## Testing Workflow

```bash
# Build
cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.28 --release

# Copy to device (assumes SSH access via USB networking or WiFi)
scp target/armv7-unknown-linux-gnueabihf/release/micro_miyoo miyoo@192.168.1.x:~/MyGame/

# SSH in and test
ssh miyoo@192.168.1.x
cd ~/MyGame
./micro_miyoo

# Or use the launch script
./launch.sh
```

## CI/CD with GitHub Actions

From `.github/workflows/build-and-publish-release.yml`:

```yaml
jobs:
  build:
    strategy:
      matrix:
        game: [micro, space, shadow, arena, dragon]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: armv7-unknown-linux-gnueabihf
      - uses: mlugg/setup-zig@v1
      - run: cargo install cargo-zigbuild
      - name: Build
        working-directory: miyoo/${{ matrix.game }}
        run: cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.28 --release
      - name: Upload
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.game }}-miyoo
          path: miyoo/${{ matrix.game }}/target/armv7-unknown-linux-gnueabihf/release/
```

Games are auto-discovered from `miyoo/*/` subdirectories. Adding a new game is just
creating the directory — no CI changes needed.

## Common Pitfalls

**glibc version mismatch:**
```
./micro_miyoo: /lib/libm.so.6: version GLIBC_2.29 not found
```
Solution: Use `cargo zigbuild` with `.2.28` suffix. Never use `cargo build` for
the Miyoo target — it links against your host glibc.

**Missing libSDL2.so:**
```
./micro_miyoo: error while loading shared libraries: libSDL2.so: cannot open shared object file
```
Solution: Copy the parasyte `libSDL2.so` into the same directory as the binary.
Set `LD_LIBRARY_PATH=.` in your launch script if needed.

**Wrong video driver:**
```
Couldn't connect to display "0.0"
```
Solution: Set `SDL_VIDEODRIVER=mmiyoo` before running. The device has no X11.

**Binary is too large:**
Make sure `strip = true` and `lto = true` in `[profile.release]`.
Also use `opt-level = "z"` for minimum size (slightly slower) or `opt-level = 3`
for maximum speed.

---

# Part 6: Complete Game Example

A fully playable Breakout clone in ~350 lines, using `retro-sdl2`. Heavily commented.
Every concept from the tutorial appears here.

```rust
//! breakout.rs — Complete Breakout clone using retro-sdl2
//! Demonstrates: state machines, fixed timestep, collision, sprites,
//!               index-based loops, text rendering, CRT effects.
//!
//! Controls: Left/Right to move paddle, Space to launch ball, Escape to quit.

use retro_sdl2::*;

// ─── Constants ────────────────────────────────────────────────────────────────

const SCREEN_W: f32 = 640.0;
const SCREEN_H: f32 = 480.0;
const PADDLE_W: f32 = 80.0;
const PADDLE_H: f32 = 12.0;
const PADDLE_SPEED: f32 = 5.0;
const BALL_SIZE: f32 = 10.0;
const BALL_SPEED: f32 = 5.0;
const BRICK_COLS: usize = 10;
const BRICK_ROWS: usize = 6;
const BRICK_W: f32 = 56.0;
const BRICK_H: f32 = 20.0;
const BRICK_MARGIN: f32 = 4.0;
const BRICKS_START_X: f32 = 12.0;
const BRICKS_START_Y: f32 = 60.0;

// ─── Sprite art ───────────────────────────────────────────────────────────────

// Paddle: 8 rows × 8 cols characters, scaled up at draw time
const PADDLE_ART: [&str; 4] = [
    "11111111",
    "12222221",
    "12222221",
    "11111111",
];
const PADDLE_COLORS: [Color; 2] = [
    Color::new(0.2, 0.6, 1.0, 1.0),   // border blue
    Color::new(0.4, 0.8, 1.0, 1.0),   // fill light-blue
];

const BALL_ART: [&str; 8] = [
    "..111...",
    ".12221..",
    "1222221.",
    "1222221.",
    "1222221.",
    ".12221..",
    "..111...",
    "........",
];
const BALL_COLORS: [Color; 2] = [
    Color::new(1.0, 0.8, 0.0, 1.0),   // orange border
    Color::new(1.0, 1.0, 0.6, 1.0),   // yellow fill
];

// ─── Game state enum ──────────────────────────────────────────────────────────

// Copy + PartialEq lets us compare and copy game states freely.
#[derive(Clone, Copy, PartialEq)]
enum State {
    Title,
    Playing,
    GameOver,
    Win,
}

// ─── Entity structs ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, Default)]
struct Paddle {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy)]
struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    launched: bool,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            x: SCREEN_W / 2.0,
            y: SCREEN_H - 80.0,
            vx: BALL_SPEED,
            vy: -BALL_SPEED,
            launched: false,
        }
    }
}

#[derive(Clone, Copy)]
struct Brick {
    x: f32,
    y: f32,
    alive: bool,
    // HP controls color — high-HP bricks need more hits
    hp: i32,
}

// ─── Particle system (simple, flat Vec) ──────────────────────────────────────

#[derive(Clone, Copy)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    max_life: f32,
    color: Color,
}

// ─── Main game struct ─────────────────────────────────────────────────────────

struct Game {
    state: State,
    paddle: Paddle,
    ball: Ball,
    bricks: Vec<Brick>,
    particles: Vec<Particle>,
    score: u32,
    lives: i32,
}

impl Game {
    fn new() -> Self {
        let mut g = Game {
            state: State::Title,
            paddle: Paddle::default(),
            ball: Ball::default(),
            bricks: Vec::new(),
            particles: Vec::with_capacity(256),
            score: 0,
            lives: 3,
        };
        g.reset_level();
        g
    }

    fn reset_level(&mut self) {
        // Position paddle in the centre-bottom
        self.paddle.x = (SCREEN_W - PADDLE_W) / 2.0;
        self.paddle.y = SCREEN_H - 40.0;

        // Reset ball to sitting on paddle
        self.ball = Ball::default();

        // Build brick grid
        self.bricks.clear();
        // Brick row colours correspond to HP: 3 = red, 2 = orange, 1 = green
        let row_hp = [3, 3, 2, 2, 1, 1];
        for row in 0..BRICK_ROWS {
            for col in 0..BRICK_COLS {
                let x = BRICKS_START_X + col as f32 * (BRICK_W + BRICK_MARGIN);
                let y = BRICKS_START_Y + row as f32 * (BRICK_H + BRICK_MARGIN);
                self.bricks.push(Brick {
                    x,
                    y,
                    alive: true,
                    hp: row_hp[row],
                });
            }
        }
    }

    // ── Update ────────────────────────────────────────────────────────────────

    fn update(&mut self, input: &Input, _dt: f32) {
        // State machine
        match self.state {
            State::Title => {
                if input.is_key_pressed(KeyCode::Return) || input.is_key_pressed(KeyCode::Space) {
                    self.state = State::Playing;
                }
            }
            State::Playing => self.update_play(input),
            State::GameOver | State::Win => {
                if input.is_key_pressed(KeyCode::Return) {
                    self.score = 0;
                    self.lives = 3;
                    self.reset_level();
                    self.state = State::Title;
                }
            }
        }

        // Always update particles regardless of game state
        self.update_particles();
    }

    fn update_play(&mut self, input: &Input) {
        // ── Paddle movement ──────────────────────────────────────────────────
        if input.is_key_down(KeyCode::Left) {
            self.paddle.x -= PADDLE_SPEED;
        }
        if input.is_key_down(KeyCode::Right) {
            self.paddle.x += PADDLE_SPEED;
        }
        // Clamp paddle to screen
        self.paddle.x = self.paddle.x.clamp(0.0, SCREEN_W - PADDLE_W);

        // ── Launch ball ──────────────────────────────────────────────────────
        if !self.ball.launched && input.is_key_pressed(KeyCode::Space) {
            self.ball.launched = true;
        }

        if !self.ball.launched {
            // Ball sits on paddle until launched
            self.ball.x = self.paddle.x + PADDLE_W / 2.0 - BALL_SIZE / 2.0;
            self.ball.y = self.paddle.y - BALL_SIZE;
            return;
        }

        // ── Ball movement ────────────────────────────────────────────────────
        self.ball.x += self.ball.vx;
        self.ball.y += self.ball.vy;

        // Wall collisions (left, right, top)
        if self.ball.x <= 0.0 {
            self.ball.x = 0.0;
            self.ball.vx = self.ball.vx.abs();
        }
        if self.ball.x + BALL_SIZE >= SCREEN_W {
            self.ball.x = SCREEN_W - BALL_SIZE;
            self.ball.vx = -self.ball.vx.abs();
        }
        if self.ball.y <= 0.0 {
            self.ball.y = 0.0;
            self.ball.vy = self.ball.vy.abs();
        }

        // Ball falls off bottom
        if self.ball.y > SCREEN_H {
            self.lives -= 1;
            if self.lives <= 0 {
                self.state = State::GameOver;
            } else {
                // Reset ball, keep bricks
                self.ball = Ball::default();
            }
            return;
        }

        // ── Paddle collision ─────────────────────────────────────────────────
        if aabb(self.ball.x, self.ball.y, BALL_SIZE, BALL_SIZE,
                self.paddle.x, self.paddle.y, PADDLE_W, PADDLE_H)
            && self.ball.vy > 0.0  // only bounce when moving downward
        {
            self.ball.vy = -self.ball.vy.abs();

            // Add horizontal spin based on where the ball hits the paddle
            let hit_pos = (self.ball.x + BALL_SIZE / 2.0 - self.paddle.x) / PADDLE_W;
            // hit_pos in [0, 1]; map to [-1, 1] for direction influence
            let curve = (hit_pos - 0.5) * 2.0;
            self.ball.vx = curve * BALL_SPEED * 1.5;

            // Preserve speed (re-normalise)
            let speed = (self.ball.vx * self.ball.vx + self.ball.vy * self.ball.vy).sqrt();
            if speed > 0.0 {
                self.ball.vx = self.ball.vx / speed * BALL_SPEED;
                self.ball.vy = self.ball.vy / speed * BALL_SPEED;
            }
        }

        // ── Brick collision (index loop — so we can call self.spawn_particles) ─
        for i in 0..self.bricks.len() {
            if !self.bricks[i].alive { continue; }

            if aabb(self.ball.x, self.ball.y, BALL_SIZE, BALL_SIZE,
                    self.bricks[i].x, self.bricks[i].y, BRICK_W, BRICK_H)
            {
                self.bricks[i].hp -= 1;

                // Bounce direction: which face was hit?
                // Simple approach: pick the axis of minimum overlap
                let ball_cx = self.ball.x + BALL_SIZE / 2.0;
                let ball_cy = self.ball.y + BALL_SIZE / 2.0;
                let brick_cx = self.bricks[i].x + BRICK_W / 2.0;
                let brick_cy = self.bricks[i].y + BRICK_H / 2.0;
                let dx = ball_cx - brick_cx;
                let dy = ball_cy - brick_cy;

                if dx.abs() / BRICK_W > dy.abs() / BRICK_H {
                    self.ball.vx = -self.ball.vx;
                } else {
                    self.ball.vy = -self.ball.vy;
                }

                if self.bricks[i].hp <= 0 {
                    self.bricks[i].alive = false;
                    self.score += 100;
                    let (bx, by) = (self.bricks[i].x, self.bricks[i].y);
                    // Safe: not iterating self.bricks or self.particles here
                    self.spawn_brick_particles(bx + BRICK_W / 2.0, by + BRICK_H / 2.0);
                }

                break; // only one brick per frame
            }
        }

        // ── Win condition ────────────────────────────────────────────────────
        if self.bricks.iter().all(|b| !b.alive) {
            self.state = State::Win;
        }
    }

    fn spawn_brick_particles(&mut self, x: f32, y: f32) {
        for _ in 0..12 {
            // IMPORTANT: annotate type to avoid f32/f64 ambiguity with .cos()/.sin()
            let angle: f32 = fastrand::f32() * std::f32::consts::TAU;
            let speed: f32 = 1.0 + fastrand::f32() * 4.0;
            let life = 20.0 + fastrand::f32() * 20.0;
            let r = fastrand::f32();
            let g = fastrand::f32();
            let b = fastrand::f32();
            self.particles.push(Particle {
                x,
                y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                life,
                max_life: life,
                color: Color::new(r, g, b, 1.0),
            });
        }
    }

    fn update_particles(&mut self) {
        // Reverse index loop for in-place removal
        for i in (0..self.particles.len()).rev() {
            self.particles[i].x += self.particles[i].vx;
            self.particles[i].y += self.particles[i].vy;
            self.particles[i].vy += 0.1;  // gravity
            self.particles[i].life -= 1.0;
            if self.particles[i].life <= 0.0 {
                self.particles.swap_remove(i);
            }
        }
    }

    // ── Draw ──────────────────────────────────────────────────────────────────

    fn draw(
        &self,
        renderer: &mut GameRenderer,
        paddle_tex: &sdl2::render::Texture,
        ball_tex: &sdl2::render::Texture,
    ) {
        renderer.clear(Color::new(0.05, 0.05, 0.1, 1.0));

        match self.state {
            State::Title => self.draw_title(renderer),
            State::Playing | State::GameOver | State::Win => {
                self.draw_gameplay(renderer, paddle_tex, ball_tex);
                match self.state {
                    State::GameOver => self.draw_overlay(renderer, "GAME OVER", "Press Enter to restart"),
                    State::Win =>      self.draw_overlay(renderer, "YOU WIN!",  "Press Enter to play again"),
                    _ => {}
                }
            }
        }

        // CRT effects — always on top
        draw_scanlines(renderer, SCREEN_W, SCREEN_H, 1.0, 1.0, 0.12);
        draw_vignette(renderer, SCREEN_W, SCREEN_H, 1.0, 12, 0.5);
    }

    fn draw_title(&self, renderer: &mut GameRenderer) {
        let title = "BREAKOUT";
        let tw = measure_text(title, 32.0);
        draw_text(renderer, title, (SCREEN_W - tw) / 2.0, 180.0, 32.0, WHITE);

        let sub = "Press Start";
        let sw = measure_text(sub, 16.0);
        draw_text(renderer, sub, (SCREEN_W - sw) / 2.0, 260.0, 16.0,
            Color::new(0.7, 0.7, 0.7, 1.0));
    }

    fn draw_gameplay(
        &self,
        renderer: &mut GameRenderer,
        paddle_tex: &sdl2::render::Texture,
        ball_tex: &sdl2::render::Texture,
    ) {
        // Draw bricks
        for brick in &self.bricks {
            if !brick.alive { continue; }
            let color = match brick.hp {
                3 => Color::new(1.0, 0.2, 0.2, 1.0),  // red
                2 => Color::new(1.0, 0.6, 0.1, 1.0),  // orange
                _ => Color::new(0.2, 0.9, 0.3, 1.0),  // green
            };
            renderer.draw_rectangle(brick.x, brick.y, BRICK_W, BRICK_H, color);
            renderer.draw_rectangle_lines(brick.x, brick.y, BRICK_W, BRICK_H, 1.0,
                Color::new(0.0, 0.0, 0.0, 0.5));
        }

        // Draw particles
        for p in &self.particles {
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            let c = Color::new(p.color.r, p.color.g, p.color.b, alpha);
            renderer.draw_circle(p.x, p.y, 3.0, c);
        }

        // Draw paddle (texture, scaled)
        renderer.draw_texture_ex(paddle_tex, self.paddle.x, self.paddle.y, DrawTextureParams {
            dest_size: Some((PADDLE_W, PADDLE_H)),
            ..Default::default()
        });

        // Draw ball (texture)
        renderer.draw_texture_ex(ball_tex, self.ball.x, self.ball.y, DrawTextureParams {
            dest_size: Some((BALL_SIZE, BALL_SIZE)),
            ..Default::default()
        });

        // HUD
        let score_text = format!("SCORE: {}", self.score);
        draw_text(renderer, &score_text, 10.0, 10.0, 16.0, WHITE);

        let lives_text = format!("LIVES: {}", self.lives);
        let lw = measure_text(&lives_text, 16.0);
        draw_text(renderer, &lives_text, SCREEN_W - lw - 10.0, 10.0, 16.0, WHITE);
    }

    fn draw_overlay(&self, renderer: &mut GameRenderer, title: &str, subtitle: &str) {
        // Semi-transparent overlay
        renderer.draw_rectangle(0.0, 160.0, SCREEN_W, 160.0,
            Color::new(0.0, 0.0, 0.0, 0.7));

        let tw = measure_text(title, 32.0);
        draw_text(renderer, title, (SCREEN_W - tw) / 2.0, 200.0, 32.0, WHITE);

        let sw = measure_text(subtitle, 16.0);
        draw_text(renderer, subtitle, (SCREEN_W - sw) / 2.0, 260.0, 16.0,
            Color::new(0.6, 0.6, 0.6, 1.0));

        let score_text = format!("Final score: {}", self.score);
        let ssw = measure_text(&score_text, 16.0);
        draw_text(renderer, &score_text, (SCREEN_W - ssw) / 2.0, 290.0, 16.0,
            Color::new(1.0, 0.8, 0.2, 1.0));
    }
}

// ─── Collision helper ─────────────────────────────────────────────────────────

/// Axis-Aligned Bounding Box overlap test.
fn aabb(ax: f32, ay: f32, aw: f32, ah: f32,
        bx: f32, by: f32, bw: f32, bh: f32) -> bool {
    ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
}

// ─── Entry point ──────────────────────────────────────────────────────────────

fn main() -> Result<(), String> {
    // SDL2 boilerplate — must keep sdl_context alive for the whole program
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;

    let window = video
        .window("Breakout", SCREEN_W as u32, SCREEN_H as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let event_pump = sdl_context.event_pump()?;

    let mut renderer = GameRenderer::from_parts(canvas, texture_creator);

    // Load sprites once at startup — unwrap() is appropriate here;
    // if assets fail to load we can't run the game anyway.
    let paddle_tex = create_sprite(renderer.texture_creator(), &PADDLE_ART, &PADDLE_COLORS)
        .expect("paddle sprite");
    let ball_tex = create_sprite(renderer.texture_creator(), &BALL_ART, &BALL_COLORS)
        .expect("ball sprite");

    let mut input = Input::new(event_pump);
    let mut clock = GameClock::new(60.0);
    let mut game = Game::new();

    // ── Main loop ────────────────────────────────────────────────────────────
    loop {
        // 1. Input
        input.poll();
        if input.should_quit() { break; }

        // 2. Fixed timestep update (with death spiral prevention)
        clock.tick();
        while clock.should_update() {
            game.update(&input, clock.fixed_dt() as f32);
        }

        // 3. Draw
        game.draw(&mut renderer, &paddle_tex, &ball_tex);
        renderer.present();

        // 4. Sleep until next frame (saves CPU on desktop, essential on embedded)
        clock.wait_for_next_frame();
    }

    Ok(())
}
```

---

## What the Example Demonstrates

Every pattern from this guide appears in the Breakout example:

| Concept | Where |
|---|---|
| Enums for game state | `State` enum, `match self.state` in `update` and `draw` |
| Exhaustive match | Add a `State::Paused` and watch both match blocks fail to compile |
| Structs + impl | `Paddle`, `Ball`, `Brick`, `Particle`, `Game` |
| Vec with index loop | `update_play` brick collision, `update_particles` removal |
| Index loop for self methods | `for i in 0..self.bricks.len()` calling `self.spawn_brick_particles` |
| Reverse index removal | `for i in (0..self.particles.len()).rev()` with `swap_remove` |
| Float type annotation | `let angle: f32 = fastrand::f32() * ...` before `.cos()` / `.sin()` |
| AABB collision | `fn aabb(...)` — free function, clean and testable |
| Sprite art to texture | `create_sprite` with string-art arrays |
| `DrawTextureParams` | `..Default::default()` struct update syntax |
| `format!` for dynamic text | Score and lives HUD display |
| `measure_text` for centering | Title and overlay text positioning |
| Fixed timestep | `GameClock`, `clock.tick()`, `while clock.should_update()` |
| CRT effects | `draw_scanlines` + `draw_vignette` at the end of `draw` |

---

## Quick Reference: Common Compiler Errors

**E0382: use of moved value**
```
error[E0382]: use of moved value: `name`
  --> src/main.rs:5:20
   |
3  |     let name = String::from("wizard");
4  |     consume(name);
   |             ---- value moved here
5  |     println!("{}", name);
   |                    ^^^^ value used here after move
```
Fix: use `.clone()` before the move, or change `consume` to take `&str`.

**E0502: cannot borrow as mutable because it is also borrowed as immutable**
```
error[E0502]: cannot borrow `self.enemies` as mutable because it is also borrowed as immutable
  --> src/main.rs:42:13
   |
40 |         for enemy in &self.enemies {
   |                       ------------ immutable borrow occurs here
41 |             if enemy.should_shoot() {
42 |                 self.bullets.push(...);
   |                 ^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
```
Fix: switch to an index-based loop.

**E0004: non-exhaustive patterns**
```
error[E0004]: non-exhaustive patterns: `GameState::Paused` not covered
  --> src/main.rs:55:15
   |
55 |         match self.state {
   |               ^^^^^^^^^^ pattern `GameState::Paused` not covered
```
Fix: add the missing arm to the match block. This error is your friend — it prevents
forgotten state transitions from shipping.

**Float method ambiguity**
```
error[E0689]: can't call method `cos` on ambiguous numeric type `{float}`
  --> src/main.rs:12:21
   |
11 |     let angle = fastrand::f32() * 3.14;
12 |     let x = angle.cos();
   |                   ^^^
   |
help: you must specify a concrete type for this numeric value, like `f32`
   --> src/main.rs:11:9
   |
11 |     let angle: f32 = fastrand::f32() * 3.14;
```
Fix: add `: f32` annotation on the binding.

**E0499: cannot borrow as mutable more than once**
```
error[E0499]: cannot borrow `self` as mutable more than once at a time
  --> src/main.rs:38:17
   |
36 |         for enemy in &mut self.enemies {
   |                           ------------ first mutable borrow occurs here
37 |             if enemy.should_shoot() {
38 |                 self.spawn_bullet(enemy.x, enemy.y);
   |                 ^^^^ second mutable borrow occurs here
```
Fix: switch to an index-based loop. Save the values you need before calling the method.

---

## Where to Go Next

- **The Rust Book**: https://doc.rust-lang.org/book/ — the official guide, free online.
  Chapters 4 (ownership), 6 (enums), 10 (generics/traits), 15 (smart pointers).

- **Rustlings**: https://github.com/rust-lang/rustlings — small exercises that match
  exactly the concepts in this guide.

- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/ — code-first,
  less prose than the book. Good as a reference.

- **The Rustonomicon**: https://doc.rust-lang.org/nomicon/ — advanced, covers unsafe.
  Read this when you need to write FFI.

- **Our codebase**: Read `miyoo/micro/src/main.rs`. It's the most complete example
  of all these patterns applied to a real game. Every pattern in this guide was
  extracted directly from that code.

---

*Last updated: April 2026. Rust edition 2024, SDL2 crate 0.37, retro-sdl2 0.1.*
