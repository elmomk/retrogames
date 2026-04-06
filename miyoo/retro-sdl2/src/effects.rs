use crate::color::Color;
use crate::renderer::GameRenderer;
use sdl2::rect::Rect;

/// Draw horizontal scanlines over the whole screen for a CRT effect.
///
/// - `line_height`: height of each dark scanline bar in pixels.
/// - `spacing`: gap (lit pixels) between scanline bars.
/// - `alpha`: opacity of the scanline bars (0.0 = invisible, 1.0 = fully black).
pub fn draw_scanlines(
    renderer: &mut GameRenderer,
    width: f32,
    height: f32,
    line_height: f32,
    spacing: f32,
    alpha: f32,
) {
    let a = alpha.clamp(0.0, 1.0);
    let overlay_color = Color::new(0.0, 0.0, 0.0, a);
    renderer.canvas_mut().set_draw_color(overlay_color.to_sdl());

    let lh = line_height.max(1.0) as i32;
    let sp = spacing.max(0.0) as i32;
    let step = lh + sp;
    if step <= 0 {
        return;
    }

    let w = width as u32;
    let h = height as i32;
    let mut y = 0i32;
    while y < h {
        let bar_h = lh.min(h - y).max(0) as u32;
        if bar_h > 0 {
            let _ = renderer.canvas_mut().fill_rect(Rect::new(0, y, w, bar_h));
        }
        y += step;
    }
}

/// Draw a vignette (dark corners fading to transparent centre) by drawing
/// `layers` concentric filled rectangles with increasing alpha near the edges.
///
/// - `depth`: controls the maximum edge darkness (0.0 = no effect, 1.0 = heavy).
/// - `layers`: number of gradient steps (8–16 recommended).
/// - `max_alpha`: the alpha of the outermost rectangle.
pub fn draw_vignette(
    renderer: &mut GameRenderer,
    width: f32,
    height: f32,
    depth: f32,
    layers: u32,
    max_alpha: f32,
) {
    if layers == 0 {
        return;
    }
    let canvas = renderer.canvas_mut();
    let depth = depth.clamp(0.0, 1.0);
    let max_alpha = max_alpha.clamp(0.0, 1.0);

    for i in 0..layers {
        // t goes from 1.0 (outermost) to ~0 (innermost).
        let t = 1.0 - i as f32 / layers as f32;
        let a = max_alpha * depth * t * t; // quadratic falloff

        let inset_x = (i as f32 * width / (2.0 * layers as f32)) as i32;
        let inset_y = (i as f32 * height / (2.0 * layers as f32)) as i32;
        let rect_w = (width as i32 - 2 * inset_x).max(0) as u32;
        let rect_h = (height as i32 - 2 * inset_y).max(0) as u32;

        if rect_w == 0 || rect_h == 0 {
            continue;
        }

        let color = Color::new(0.0, 0.0, 0.0, a);
        canvas.set_draw_color(color.to_sdl());

        // Draw only the border strip (outline) of this rectangle so we build up
        // the vignette gradient without filling the interior.
        let thickness = ((width / (2.0 * layers as f32)).ceil() as i32).max(1);
        let x = inset_x;
        let y = inset_y;
        let w = rect_w as i32;
        let h = rect_h as i32;

        // Top strip
        let top_h = thickness.min(h).max(0) as u32;
        if top_h > 0 {
            let _ = canvas.fill_rect(Rect::new(x, y, rect_w, top_h));
        }
        // Bottom strip
        let bot_y = y + h - thickness;
        if bot_y > y && top_h > 0 {
            let _ = canvas.fill_rect(Rect::new(x, bot_y, rect_w, top_h));
        }
        // Left strip (excluding corners)
        let inner_y = y + thickness;
        let inner_h = (h - 2 * thickness).max(0) as u32;
        let left_w = (thickness.min(w)).max(0) as u32;
        if inner_h > 0 && left_w > 0 {
            let _ = canvas.fill_rect(Rect::new(x, inner_y, left_w, inner_h));
        }
        // Right strip
        let right_x = x + w - thickness;
        if right_x > x && inner_h > 0 && left_w > 0 {
            let _ = canvas.fill_rect(Rect::new(right_x, inner_y, left_w, inner_h));
        }
    }
}
