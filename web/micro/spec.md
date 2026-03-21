🛠️ Technical Specification: Nano Wizards Rust Port

1. Overview

This document specifies the technical architecture for porting the "Nano Wizards" HTML5/JS prototype to Rust. The target engine is Macroquad, chosen for its minimal overhead, immediate-mode rendering, and seamless cross-compilation to both WebAssembly (for browser testing) and armv7-unknown-linux-gnueabihf (for the Miyoo Mini Plus).

2. Tech Stack & Dependencies

Language: Rust (Edition 2021)

Engine/Framework: Macroquad 0.4

Target Devices: * Primary: Miyoo Mini Plus (ARM Cortex-A7, 128MB RAM, Linux).

Secondary: Web browser (wasm32-unknown-unknown).

Cargo.toml

[package]
name = "nano_wizards"
version = "0.1.0"
edition = "2021"

[dependencies]
macroquad = "0.4"


3. Core Architecture & Structs

To avoid the overhead of heavy ECS (Entity-Component-System) frameworks like Bevy on constrained hardware, the game will use a Data-Oriented Imperative design, keeping state in flat Vec arrays and simple structs.

3.1. Game State Enum

enum GameState {
    Start,
    Story,
    Playing,
    GameOver,
    Win,
}


3.2. Core Entities

struct Player {
    rect: Rect, // x, y, w, h
    vel: Vec2,
    on_ground: bool,
    wall_dir: i8, // -1 (left), 0 (none), 1 (right)
    facing_right: bool,
    jumps: u8,
    max_jumps: u8,
}

struct Anchor {
    active: bool,
    is_attached: bool,
    rect: Rect,
    vel: Vec2,
    length: f32,
}

enum EnemyType {
    Patrol,
    Bat,
    Turret,
}

struct Enemy {
    e_type: EnemyType,
    rect: Rect,
    vel: Vec2,
    start_x: f32,
    range: f32,
    shoot_timer: f32,
}

struct Platform {
    rect: Rect,
    is_destructible: bool,
    is_chest: bool,
}


4. The Game Loop (Fixed Timestep)

To guarantee deterministic physics between a 144Hz desktop monitor and the Miyoo Mini Plus, a fixed-timestep loop must be manually implemented inside Macroquad's next_frame() await loop.

const TIME_STEP: f64 = 1.0 / 60.0;

#[macroquad::main("Nano Wizards")]
async fn main() {
    let mut accumulator: f64 = 0.0;
    let mut last_time = get_time();

    loop {
        let current_time = get_time();
        let mut frame_time = current_time - last_time;
        last_time = current_time;

        // Prevent spiral of death on lag spikes
        if frame_time > 0.25 { frame_time = 0.25; }
        accumulator += frame_time;

        while accumulator >= TIME_STEP {
            update_physics(); // Advance simulation by 1/60th of a second
            accumulator -= TIME_STEP;
        }

        draw(); // Render as fast as possible, but physics remain steady
        next_frame().await
    }
}


5. Input Mapping (Miyoo Hardware)

The Miyoo Mini Plus registers its physical buttons as standard keyboard inputs via its embedded Linux OS. Macroquad's is_key_down will map exactly to these inputs.

Miyoo Button

Standard Key Map

Macroquad Equivalent

Game Action

D-Pad

Arrow Keys

KeyCode::Up, Down, Left, Right

Move / Aim

A Button

Alt / X

KeyCode::X

Shoot Fireball

B Button

Space / Z

KeyCode::Space, KeyCode::Z

Jump / Hold for Anchor

Implementation Detail: For jump buffering and holding logic, input state must be tracked manually across frames (e.g., logging get_time() when the B button is initially pressed to calculate the hold duration for the Anchor).

6. Procedural Asset Generation

To maintain a single-binary distribution (no external .png files required on the SD card), the createSprite JS function will be ported using Macroquad's Image buffer.

fn create_sprite(art: &[&str], colors: &[Color]) -> Texture2D {
    let width = art[0].len() as u16;
    let height = art.len() as u16;
    let mut img = Image::gen_image_color(width, height, BLANK);

    for (y, row) in art.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch != '.' {
                if let Some(digit) = ch.to_digit(10) {
                    let color_idx = (digit - 1) as usize;
                    if color_idx < colors.len() {
                        img.set_pixel(x as u32, y as u32, colors[color_idx]);
                    }
                }
            }
        }
    }

    let texture = Texture2D::from_image(&img);
    texture.set_filter(FilterMode::Nearest); // CRITICAL for crisp pixel art
    texture
}


7. Physics & Collision Resolution

Collision uses strictly 2D Axis-Aligned Bounding Boxes (AABB), leveraging Macroquad's Rect::overlaps().

Order of Resolution:

Apply X-axis velocity.

Check X-axis collisions and push out (calculate wall direction here).

Apply Y-axis velocity (Gravity + Terminal Velocity clamps).

Check Y-axis collisions and push out (calculate grounded state here).

Apply Rope Constraint (Verlet integration) if the Anchor is attached.

8. Rendering & Camera

Resolution: The logical internal resolution will assume a 640x480 space (matching the Miyoo's 4:3 screen aspect ratio).

Camera Tracking: A manual camera_y floating-point value will track the player's Y position. Every draw_texture_ex call will subtract camera_y from the object's absolute Y coordinate to achieve vertical scrolling without using a complex engine camera struct.

9. Development Roadmap

This section tracks our progress from the initial browser prototype to a polished, native release.

Phase 1: Browser Prototyping (HTML5 / Vanilla JS) 🟢 Completed

[x] Core Physics & Movement: Gravity, terminal velocity, AABB collision, wall-sliding, wall-jumping, coyote time, jump buffering, and double jumps.

[x] Combat & Tools: 8-way directional shooting, grappling anchor, and destructible terrain.

[x] Level Architecture: String-based tilemap parser, entity management, arcade scoring, floating text popups, and level progression/storylines.

[x] Mobile / Web Usability: Virtual joystick and classic A/B button layout.

Phase 2: Engine Port (Rust / Macroquad) 🟡 In Progress

[x] CI/CD Pipeline Setup: GitHub Actions workflow (release.yml), Ubuntu runner with arm-linux-gnueabihf-gcc, and automated GitHub Releases.

[ ] Core Translation: Initialize Cargo project, port the Fixed Timestep loop, Player Physics, AABB Collision, and Tilemap parsing.

[ ] Rendering Pipeline: Implement draw_texture_ex and translate procedural pixel art arrays.

[ ] Miyoo Tweaks: Map Miyoo hardware buttons and lock resolution to native 640x480.

Phase 3: Audio, Visuals & Polish 🔴 Planned

[ ] Sprite & Animation System: Frame animation logic (Idle, Run, Jump, Wall-Slide, Swing) and animated enemy cycles.

[ ] VFX (Visual Effects): Screen shake, gravity-affected sparks, and dust clouds.

[ ] Audio Implementation: Integrate Macroquad audio module, retro 8-bit SFX, and background chiptune tracks.

Phase 4: Advanced Content & Systems 🔴 Planned

[ ] Level Design: Build 10+ distinct vertical shaft levels and integrate external map editors (e.g., Tiled).

[ ] New Hazards & Enemies: Crumbling platforms, mini-boss encounters.

[ ] Save States & High Scores: Local file I/O for saving High Scores.

Phase 5: Distribution & Release 🔴 Planned

[ ] WebAssembly Export: Compile target wasm32-unknown-unknown and deploy to GitHub Pages.

[ ] Onion OS Packaging: Create launch.sh scripts, generate miyoogamelist.xml metadata, and package final ARM binary into a zip release.