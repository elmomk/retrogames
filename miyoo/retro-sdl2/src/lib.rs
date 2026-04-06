//! `retro-sdl2` — thin SDL2 rendering layer with a macroquad-compatible API surface.
//!
//! Provides the same draw-function signatures, Color type, KeyCode enum, and sprite
//! pipeline used by the macroquad Miyoo ports so that game logic can be shared with
//! minimal changes.

pub mod color;
pub mod effects;
pub mod font;
pub mod input;
pub mod renderer;
pub mod sprite;
pub mod timing;

// Flat re-exports for a macroquad-like `use retro_sdl2::*;` import style.
pub use color::{
    color_u8, Color, BLACK, BLANK, BLUE, GRAY, GREEN, MAGENTA, RED, SKYBLUE, WHITE, YELLOW,
};
pub use effects::{draw_scanlines, draw_vignette};
pub use font::{draw_text, measure_text};
pub use input::{Input, KeyCode};
pub use renderer::{DrawTextureParams, GameRenderer};
pub use sprite::{create_sprite, create_sprite_sdl};
pub use timing::GameClock;
