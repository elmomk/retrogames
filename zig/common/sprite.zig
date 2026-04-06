// sprite.zig — procedural sprite creation from string art
const sdl = @import("sdl");
const c = sdl.c;

/// Create an SDL_Texture from string art.
/// art: slice of 8 strings, each 8 chars. '.' = transparent, '1'-'9' = color index.
/// colors: palette, each entry is [4]u8 { r, g, b, a }.
/// Returns null on failure; caller owns and must SDL_DestroyTexture the result.
pub fn createSprite(
    renderer: *sdl.Renderer,
    art: []const []const u8,
    colors: []const [4]u8,
) ?*c.SDL_Texture {
    const size: c_int = 8;
    const texture = c.SDL_CreateTexture(
        renderer,
        c.SDL_PIXELFORMAT_RGBA8888,
        c.SDL_TEXTUREACCESS_STATIC,
        size,
        size,
    ) orelse return null;

    _ = c.SDL_SetTextureBlendMode(texture, c.SDL_BLENDMODE_BLEND);

    // Build pixel buffer: RGBA8888 = R in high byte
    var pixels: [64]u32 = [_]u32{0} ** 64;

    for (art, 0..) |row, y| {
        if (y >= 8) break;
        for (row, 0..) |ch, x| {
            if (x >= 8) break;
            if (ch == '.' or ch < '1' or ch > '9') {
                pixels[y * 8 + x] = 0x00000000;
            } else {
                const idx = ch - '1';
                if (idx < colors.len) {
                    const col = colors[idx];
                    // RGBA8888: R<<24 | G<<16 | B<<8 | A
                    pixels[y * 8 + x] =
                        (@as(u32, col[0]) << 24) |
                        (@as(u32, col[1]) << 16) |
                        (@as(u32, col[2]) << 8) |
                        @as(u32, col[3]);
                }
            }
        }
    }

    _ = c.SDL_UpdateTexture(texture, null, &pixels, @as(c_int, 8 * 4)); // 8px wide * 4 bytes per pixel
    return texture;
}
