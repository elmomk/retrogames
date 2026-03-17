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

Physical Miyoo Button

Macroquad KeyCode

Game Action

D-Pad

KeyCode::Up, Down, Left, Right

Movement & Aiming

A Button

KeyCode::X

Shoot Fireball

B Button

KeyCode::Space or KeyCode::Z

Jump / Hold for Anchor

Start Button

KeyCode::Enter

Pause / Start

Select Button

KeyCode::RightShift

Back / Menu

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


7. Physics & Collision Pipeline

Collision strictly utilizes 2D Axis-Aligned Bounding Boxes (AABB) using Rect::overlaps(&other_rect).

X-Axis Sweep: Apply Player.vel.x. Resolve overlaps by pushing the player back. Set wall_dir to enable wall-jumping.

Y-Axis Sweep: Apply Player.vel.y (Gravity). Clamp to MAX_FALL_SPEED or WALL_SLIDE_SPEED. Resolve overlaps by pushing the player back. Set on_ground.

Verlet Anchor Constraint: If anchor.is_attached is true, calculate the Euclidean distance between Player and Anchor. If distance > anchor.length, apply a pull vector to restrict the player to the radius, damping velocity to simulate swing tension.

8. Build & Deployment Pipeline

The artifact is built automatically via a GitHub Actions CI/CD pipeline.

Toolchain: rust-toolchain stable, armv7-unknown-linux-gnueabihf target.

Linker: Requires Ubuntu package gcc-arm-linux-gnueabihf configured in .cargo/config.toml.

Output: The resulting compiled binary is automatically attached to GitHub Releases for direct installation onto the Miyoo Mini Plus SD card.