use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::color::Color;

/// Parameters for draw_texture_ex, mirroring macroquad's DrawTextureParams.
pub struct DrawTextureParams {
    /// Override rendered size in pixels. None = use texture's natural size.
    pub dest_size: Option<(f32, f32)>,
    /// Rotation in radians (clockwise).
    pub rotation: f32,
    /// Flip horizontally.
    pub flip_x: bool,
    /// Flip vertically.
    pub flip_y: bool,
    /// Source sub-rectangle within the texture. None = full texture.
    pub source: Option<sdl2::rect::Rect>,
}

impl Default for DrawTextureParams {
    fn default() -> Self {
        Self {
            dest_size: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            source: None,
        }
    }
}

/// Wraps an SDL2 Canvas<Window> and its TextureCreator.
/// All draw calls take f32 coordinates for API compatibility with macroquad.
pub struct GameRenderer {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
}

impl GameRenderer {
    /// Create a window and hardware-accelerated renderer with nearest-neighbor scaling.
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        eprintln!("[retro-sdl2] SDL_Init...");
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;
        eprintln!("[retro-sdl2] Video subsystem OK, driver: {}", video.current_video_driver());

        // Use fullscreen on embedded targets (Miyoo mmiyoo driver requires it).
        let is_embedded = std::env::var("SDL_VIDEODRIVER")
            .map(|v| v == "mmiyoo")
            .unwrap_or(false);
        eprintln!("[retro-sdl2] embedded={}, creating {}x{} window", is_embedded, width, height);

        let mut wb = video.window(title, width, height);
        if is_embedded {
            wb.fullscreen();
        } else {
            wb.position_centered();
        }
        let window = wb.build().map_err(|e| e.to_string())?;
        eprintln!("[retro-sdl2] Window created OK");

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;
        eprintln!("[retro-sdl2] Renderer created OK");

        // Nearest-neighbor scaling for crisp pixels.
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        // SDL_HINT_RENDER_SCALE_QUALITY is set to "0" (nearest) by default in SDL2.
        // Logical size ensures the game renders at native resolution on any window size.
        canvas
            .set_logical_size(width, height)
            .map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();

        Ok(Self {
            canvas,
            texture_creator,
        })
    }

    /// Access the SDL EventPump by initialising SDL again (caller manages sdl_context).
    /// This constructor variant accepts an already-built canvas and texture_creator
    /// for callers that manage the SDL context externally.
    pub fn from_parts(
        canvas: Canvas<Window>,
        texture_creator: TextureCreator<WindowContext>,
    ) -> Self {
        Self {
            canvas,
            texture_creator,
        }
    }

    /// Returns a reference to the TextureCreator so callers can create textures.
    pub fn texture_creator(&self) -> &TextureCreator<WindowContext> {
        &self.texture_creator
    }

    /// Returns a mutable reference to the underlying canvas for advanced use.
    pub fn canvas_mut(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }

    /// Clear the entire canvas to `color`.
    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color.to_sdl());
        self.canvas.clear();
    }

    /// Draw a filled rectangle.
    pub fn draw_rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.canvas.set_draw_color(color.to_sdl());
        let _ = self.canvas.fill_rect(Rect::new(
            x as i32,
            y as i32,
            w.max(0.0) as u32,
            h.max(0.0) as u32,
        ));
    }

    /// Draw a rectangle outline of the given `thickness` (drawn inward).
    pub fn draw_rectangle_lines(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        thickness: f32,
        color: Color,
    ) {
        let t = thickness.max(1.0) as i32;
        let xi = x as i32;
        let yi = y as i32;
        let wi = w.max(0.0) as i32;
        let hi = h.max(0.0) as i32;

        // Top, bottom, left, right strips.
        self.canvas.set_draw_color(color.to_sdl());
        let _ = self
            .canvas
            .fill_rect(Rect::new(xi, yi, wi as u32, t as u32)); // top
        let _ = self
            .canvas
            .fill_rect(Rect::new(xi, yi + hi - t, wi as u32, t as u32)); // bottom
        let _ = self
            .canvas
            .fill_rect(Rect::new(xi, yi, t as u32, hi as u32)); // left
        let _ = self
            .canvas
            .fill_rect(Rect::new(xi + wi - t, yi, t as u32, hi as u32)); // right
    }

    /// Draw a line of the given `thickness` between two points.
    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) {
        let t = thickness.max(1.0) as i32;
        self.canvas.set_draw_color(color.to_sdl());

        if t <= 1 {
            let _ = self
                .canvas
                .draw_line(Point::new(x1 as i32, y1 as i32), Point::new(x2 as i32, y2 as i32));
        } else {
            // Approximate thick line as a filled rectangle aligned to major axis.
            let dx = x2 - x1;
            let dy = y2 - y1;
            if dx.abs() >= dy.abs() {
                // Horizontal-ish: vary in x, thicken in y.
                let half = t / 2;
                let lx = x1.min(x2) as i32;
                let rx = x1.max(x2) as i32;
                let _ = self.canvas.fill_rect(Rect::new(
                    lx,
                    y1 as i32 - half,
                    (rx - lx).max(1) as u32,
                    t as u32,
                ));
            } else {
                // Vertical-ish: vary in y, thicken in x.
                let half = t / 2;
                let ty = y1.min(y2) as i32;
                let by = y1.max(y2) as i32;
                let _ = self.canvas.fill_rect(Rect::new(
                    x1 as i32 - half,
                    ty,
                    t as u32,
                    (by - ty).max(1) as u32,
                ));
            }
        }
    }

    /// Draw a filled circle using the midpoint (Bresenham) algorithm.
    pub fn draw_circle(&mut self, cx: f32, cy: f32, r: f32, color: Color) {
        self.canvas.set_draw_color(color.to_sdl());
        let cx = cx as i32;
        let cy = cy as i32;
        let r = r as i32;

        let mut x = 0i32;
        let mut y = r;
        let mut d = 1 - r;

        while x <= y {
            // Fill horizontal spans for each pair of symmetric points.
            let spans: [(i32, i32, i32); 4] = [
                (cx - y, cx + y, cy + x),
                (cx - y, cx + y, cy - x),
                (cx - x, cx + x, cy + y),
                (cx - x, cx + x, cy - y),
            ];
            for (lx, rx, row_y) in spans {
                let w = (rx - lx).max(0) as u32;
                if w > 0 {
                    let _ = self.canvas.fill_rect(Rect::new(lx, row_y, w, 1));
                }
            }
            if d < 0 {
                d += 2 * x + 3;
            } else {
                d += 2 * (x - y) + 5;
                y -= 1;
            }
            x += 1;
        }
    }

    /// Draw a texture at (x, y) with optional transform overrides.
    pub fn draw_texture_ex(&mut self, texture: &Texture, x: f32, y: f32, params: DrawTextureParams) {
        let query = texture.query();
        let src = params.source;

        let (src_w, src_h) = match src {
            Some(r) => (r.width(), r.height()),
            None => (query.width, query.height),
        };

        let (dst_w, dst_h) = match params.dest_size {
            Some((dw, dh)) => (dw as u32, dh as u32),
            None => (src_w, src_h),
        };

        let dst = Rect::new(x as i32, y as i32, dst_w, dst_h);

        // SDL2 rotation centre is relative to the destination rect.
        let center = sdl2::rect::Point::new((dst_w / 2) as i32, (dst_h / 2) as i32);
        let angle_deg = params.rotation.to_degrees() as f64;

        let _ = self.canvas.copy_ex(
            texture,
            src,
            Some(dst),
            angle_deg,
            Some(center),
            params.flip_x,
            params.flip_y,
        );
    }

    /// Swap the back buffer (SDL_RenderPresent).
    pub fn present(&mut self) {
        self.canvas.present();
    }

    /// Create a blank RGBA texture of the given size for render-to-texture usage.
    pub fn create_render_texture(&self, width: u32, height: u32) -> Result<Texture<'_>, String> {
        self.texture_creator
            .create_texture_target(PixelFormatEnum::RGBA8888, width, height)
            .map_err(|e| e.to_string())
    }
}
