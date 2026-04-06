/// A color with f32 components in the 0.0–1.0 range, matching macroquad's Color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Create a color from f32 components (0.0–1.0).
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Convert to sdl2::pixels::Color (u8 components).
    pub fn to_sdl(&self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGBA(
            (self.r.clamp(0.0, 1.0) * 255.0) as u8,
            (self.g.clamp(0.0, 1.0) * 255.0) as u8,
            (self.b.clamp(0.0, 1.0) * 255.0) as u8,
            (self.a.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }
}

/// Construct a Color from u8 components (0–255).
pub fn color_u8(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    )
}

pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
pub const BLANK: Color = Color::new(0.0, 0.0, 0.0, 0.0);
pub const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
pub const GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
pub const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
pub const YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
pub const MAGENTA: Color = Color::new(1.0, 0.0, 1.0, 1.0);
pub const GRAY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
pub const SKYBLUE: Color = Color::new(0.4, 0.75, 1.0, 1.0);
