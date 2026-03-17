🛠️ Technical Specification: Micro (Miyoo Mini Plus Rust Port)

1. Overview

This document specifies the technical architecture for the native Miyoo Mini Plus version of "Micro", located in the miyoo/micro/ directory. The game is written in Rust using the Macroquad framework. It translates the exact physics, logic, and arrays from the HTML5 browser prototype (specifically using web/micro/index.html as the foundational base) into a highly optimized, compiled ARM binary.

2. Tech Stack & Target Environment

Language: Rust (Edition 2021)

Engine/Framework: Macroquad 0.4

Target Architecture: armv7-unknown-linux-gnueabihf (Miyoo Mini Plus - ARM Cortex-A7)

OS Environment: Onion OS / Miyoo Linux

Cargo.toml Blueprint

[package]
name = "micro_miyoo"
version = "0.1.0"
edition = "2021"

[dependencies]
macroquad = "0.4"


3. Game Loop (Fixed Timestep Implementation)

To guarantee identical physics behavior across a 144Hz desktop testing environment and the 60Hz Miyoo display, the game uses a fixed-timestep loop decoupled from rendering.

const TIME_STEP: f64 = 1.0 / 60.0;

#[macroquad::main("Micro")]
async fn main() {
    let mut accumulator: f64 = 0.0;
    let mut last_time = get_time();

    loop {
        let current_time = get_time();
        let mut frame_time = current_time - last_time;
        last_time = current_time;

        // Death Spiral Prevention
        if frame_time > 0.25 { frame_time = 0.25; }
        accumulator += frame_time;

        while accumulator >= TIME_STEP {
            update_physics(); // Advance simulation exactly 16.66ms
            accumulator -= TIME_STEP;
        }

        draw(); 
        next_frame().await
    }
}


4. Hardware Input Mapping (Miyoo Mini Plus)

The Miyoo Mini Linux OS maps its physical buttons directly to standard keyboard scan codes. Macroquad captures these natively.

| Physical Miyoo Button | Macroquad KeyCode | Game Action |
|---|---|---|
| D-Pad | KeyCode::Up, Down, Left, Right | Movement & Aiming |
| A Button | KeyCode::X | Shoot Fireball |
| B Button | KeyCode::Space or KeyCode::Z | Jump / Hold for Anchor |
| Start Button | KeyCode::Enter | Pause / Start |
| Select Button | KeyCode::RightShift | Back / Menu |

Implementation Detail: For jump buffering and holding logic, input state must be tracked manually across frames (e.g., logging get_time() when the B button is initially pressed to calculate the hold duration for the Anchor).

5. Procedural Asset Generation (Memory Management)

To ensure the Miyoo binary is a single, easily distributable file without complex asset folders, sprites are generated procedurally into Texture2D memory buffers on boot.

// Converts an array of string rows into a Macroquad Texture2D
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
    texture.set_filter(FilterMode::Nearest); // Enforces crisp pixel art scaling
    texture
}


6. Core Data Structures (Data-Oriented Design)

To maximize performance on the ARM CPU, the game avoids complex ECS abstractions in favor of flat Vec structs and standard primitive types.

6.1. Game State Enum

enum GameState {
    Start,
    Story,
    Playing,
    GameOver,
    Win,
}

6.2. Core Entities

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
    length: f32, // Tension constraint limit
}

struct Projectile {
    rect: Rect,
    vel: Vec2,
}

enum PlatformType {
    Brick,
    Stone, // Destructible via anchor
    Chest, // Spawns gems when destroyed
}

struct Platform {
    rect: Rect,
    p_type: PlatformType,
}

6.3. Enemy Types

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


7. Physics & Collision Pipeline

Collision strictly utilizes 2D Axis-Aligned Bounding Boxes (AABB) using Rect::overlaps(&other_rect).

X-Axis Sweep: Apply Player.vel.x. Resolve overlaps by pushing the player back. Set wall_dir to enable wall-jumping.

Y-Axis Sweep: Apply Player.vel.y (Gravity). Clamp to MAX_FALL_SPEED or WALL_SLIDE_SPEED. Resolve overlaps by pushing the player back. Set on_ground.

Verlet Anchor Constraint: If anchor.is_attached is true, calculate the Euclidean distance between Player and Anchor. If distance > anchor.length, apply a pull vector to restrict the player to the radius, damping velocity to simulate swing tension.

8. Rendering & Camera

Resolution: The logical internal resolution is 640x480 (matching the Miyoo Mini Plus 4:3 screen aspect ratio).

Camera Tracking: A manual camera_y floating-point value will track the player's Y position. Every draw_texture_ex call will subtract camera_y from the object's absolute Y coordinate to achieve vertical scrolling without using a complex engine camera struct.

9. Build & Deployment Pipeline

The artifact is built automatically via a GitHub Actions CI/CD pipeline.

Toolchain: rust-toolchain stable, armv7-unknown-linux-gnueabihf target.

Linker: Requires Ubuntu package gcc-arm-linux-gnueabihf configured in .cargo/config.toml.

Output: The resulting compiled binary is automatically attached to GitHub Releases for direct installation onto the Miyoo Mini Plus SD card.

10. Development Roadmap

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

[ ] Onion OS Packaging: Create launch.sh scripts, generate miyoogamelist.xml metadata, and package final ARM binary into a zip release.