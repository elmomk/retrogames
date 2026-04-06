use sdl2::pixels::{Color as SdlColor, PixelFormatEnum};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::surface::Surface;

use crate::color::{Color, BLANK};

/// Parse string-art pixel data and bake it into an SDL2 Texture.
///
/// Each string in `art` is one row of pixels.
/// Character meanings:
///   '.' → transparent (alpha = 0)
///   '1' … '9' → index into the `colors` slice (1-based)
///
/// The texture uses `RGBA8888` format and nearest-neighbor filtering
/// (SDL2 defaults to nearest when the hint is not overridden).
pub fn create_sprite<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    art: &[&str],
    colors: &[Color],
) -> Result<Texture<'a>, String> {
    let height = art.len() as u32;
    let width = art.iter().map(|row| row.len()).max().unwrap_or(0) as u32;

    if width == 0 || height == 0 {
        return Err("Sprite art is empty".to_string());
    }

    // Build a pixel buffer: 4 bytes per pixel (R, G, B, A).
    let mut pixels: Vec<u8> = vec![0u8; (width * height * 4) as usize];

    for (row_idx, row) in art.iter().enumerate() {
        for (col_idx, ch) in row.chars().enumerate() {
            let base = (row_idx as u32 * width + col_idx as u32) as usize * 4;
            let color: Color = if ch == '.' {
                BLANK
            } else {
                let idx = ch.to_digit(10).unwrap_or(0) as usize;
                if idx > 0 && idx <= colors.len() {
                    colors[idx - 1]
                } else {
                    BLANK
                }
            };

            let sdl_c = color.to_sdl();
            pixels[base]     = sdl_c.r;
            pixels[base + 1] = sdl_c.g;
            pixels[base + 2] = sdl_c.b;
            pixels[base + 3] = sdl_c.a;
        }
    }

    // Create a Surface from the raw pixel data.
    // RGBA8888 means bytes are laid out R, G, B, A in memory (pitch = width * 4).
    let pitch = width * 4;
    // ABGR8888 = bytes in memory are R, G, B, A on little-endian (ARM).
    // RGBA8888 would interpret as 0xRRGGBBAA (packed), which is wrong for byte-order layout.
    let surface = Surface::from_data(
        &mut pixels,
        width,
        height,
        pitch,
        PixelFormatEnum::ABGR8888,
    )
    .map_err(|e| e.to_string())?;

    // Convert surface to texture. SDL2 uses nearest-neighbor by default.
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    Ok(texture)
}

/// Convenience: create a sprite from art + colors given already-borrowed texture creator.
/// Identical to `create_sprite` but accepts `colors` as a slice of `sdl2::pixels::Color`
/// for callers working directly with SDL types.
pub fn create_sprite_sdl<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    art: &[&str],
    colors: &[SdlColor],
) -> Result<Texture<'a>, String> {
    let mapped: Vec<Color> = colors
        .iter()
        .map(|c| Color::new(
            c.r as f32 / 255.0,
            c.g as f32 / 255.0,
            c.b as f32 / 255.0,
            c.a as f32 / 255.0,
        ))
        .collect();
    create_sprite(texture_creator, art, &mapped)
}
