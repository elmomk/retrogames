// sdl.zig — thin wrapper over SDL2 C API
const std = @import("std");

pub const c = @cImport({
    @cInclude("SDL2/SDL.h");
});

pub const Window = c.SDL_Window;
pub const Renderer = c.SDL_Renderer;
pub const Texture = c.SDL_Texture;

pub const InitResult = struct {
    window: *Window,
    renderer: *Renderer,
};

pub fn init(title: [*:0]const u8, width: c_int, height: c_int) !InitResult {
    // Init video (required); audio is optional — ignore failure on embedded targets
    if (c.SDL_Init(c.SDL_INIT_VIDEO) != 0) {
        return error.SDLInitFailed;
    }
    _ = c.SDL_InitSubSystem(c.SDL_INIT_AUDIO);

    const window = c.SDL_CreateWindow(
        title,
        c.SDL_WINDOWPOS_CENTERED,
        c.SDL_WINDOWPOS_CENTERED,
        width,
        height,
        c.SDL_WINDOW_SHOWN,
    ) orelse return error.WindowCreationFailed;

    const renderer = c.SDL_CreateRenderer(
        window,
        -1,
        c.SDL_RENDERER_ACCELERATED | c.SDL_RENDERER_PRESENTVSYNC,
    ) orelse return error.RendererCreationFailed;

    _ = c.SDL_SetRenderDrawBlendMode(renderer, c.SDL_BLENDMODE_BLEND);

    return InitResult{ .window = window, .renderer = renderer };
}

pub fn quit(window: *Window, renderer: *Renderer) void {
    c.SDL_DestroyRenderer(renderer);
    c.SDL_DestroyWindow(window);
    c.SDL_Quit();
}

pub fn clear(renderer: *Renderer, r: u8, g: u8, b: u8, a: u8) void {
    _ = c.SDL_SetRenderDrawColor(renderer, r, g, b, a);
    _ = c.SDL_RenderClear(renderer);
}

pub fn fillRect(renderer: *Renderer, x: c_int, y: c_int, w: c_int, h: c_int, r: u8, g: u8, b: u8, a: u8) void {
    _ = c.SDL_SetRenderDrawColor(renderer, r, g, b, a);
    const rect = c.SDL_Rect{ .x = x, .y = y, .w = w, .h = h };
    _ = c.SDL_RenderFillRect(renderer, &rect);
}

pub fn drawLine(renderer: *Renderer, x1: c_int, y1: c_int, x2: c_int, y2: c_int, r: u8, g: u8, b: u8, a: u8) void {
    _ = c.SDL_SetRenderDrawColor(renderer, r, g, b, a);
    _ = c.SDL_RenderDrawLine(renderer, x1, y1, x2, y2);
}

pub fn fillCircle(renderer: *Renderer, cx: c_int, cy: c_int, radius: c_int, r: u8, g: u8, b: u8, a: u8) void {
    _ = c.SDL_SetRenderDrawColor(renderer, r, g, b, a);
    var dy: c_int = -radius;
    while (dy <= radius) : (dy += 1) {
        const dx = @as(c_int, @intFromFloat(@sqrt(@as(f32, @floatFromInt(radius * radius - dy * dy)))));
        const rect = c.SDL_Rect{ .x = cx - dx, .y = cy + dy, .w = dx * 2, .h = 1 };
        _ = c.SDL_RenderFillRect(renderer, &rect);
    }
}

pub fn copyTexture(
    renderer: *Renderer,
    texture: *Texture,
    src_rect: ?*const c.SDL_Rect,
    dst_rect: ?*const c.SDL_Rect,
) void {
    _ = c.SDL_RenderCopy(renderer, texture, src_rect, dst_rect);
}

pub fn copyTextureEx(
    renderer: *Renderer,
    texture: *Texture,
    src: ?*const c.SDL_Rect,
    dst: ?*const c.SDL_Rect,
    angle: f64,
    flip: c.SDL_RendererFlip,
) void {
    _ = c.SDL_RenderCopyEx(renderer, texture, src, dst, angle, null, flip);
}

pub fn present(renderer: *Renderer) void {
    c.SDL_RenderPresent(renderer);
}

pub fn getTicks() u32 {
    return c.SDL_GetTicks();
}

pub fn delay(ms: u32) void {
    c.SDL_Delay(ms);
}
