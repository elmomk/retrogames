// Nano Wizards — The Obsidian Spire
// Zig/SDL2 port of the HTML5 Canvas game.
// 640x480 resolution, 60fps fixed timestep, CRT scanline overlay.

const std = @import("std");
const sdl = @import("sdl");
const font = @import("font");
const sprite = @import("sprite");
const c = sdl.c;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const GAME_W: i32 = 640;
const GAME_H: i32 = 480;
const TILE: i32 = 20;
const SPRITE_SZ: i32 = 14;
const SPRITE_OFF: i32 = (TILE - SPRITE_SZ) / 2; // 3

const GRAVITY: f32 = 0.35;
const MAX_FALL: f32 = 7.0;
const WALL_SLIDE: f32 = 1.5;
const MOVE_SPEED: f32 = 3.5;
const JUMP_FORCE: f32 = -7.0;
const BOUNCE_FORCE: f32 = -6.0;
const WALL_JUMP_Y: f32 = -6.5;
const WALL_JUMP_X: f32 = 6.0;
const BULLET_SPEED: f32 = 10.0;
const ENEMY_BULLET_SPEED: f32 = 4.0;
const ANCHOR_SPEED: f32 = 15.0;
const CLIMB_SPEED: f32 = 3.0;

const MAX_PARTICLES: usize = 120;
const MAX_BUBBLES: usize = 40;
const TARGET_MS: u32 = 1000 / 60;

// ---------------------------------------------------------------------------
// Game state enum
// ---------------------------------------------------------------------------

const GameState = enum {
    Start,
    LevelStory,
    Playing,
    GameOver,
    Win,
};

// ---------------------------------------------------------------------------
// Tile type
// ---------------------------------------------------------------------------

const TileType = enum { Brick, Stone, Chest };

// ---------------------------------------------------------------------------
// Platform (solid tile)
// ---------------------------------------------------------------------------

const Platform = struct {
    x: f32,
    y: f32,
    w: f32 = 20.0,
    h: f32 = 20.0,
    kind: TileType,
};

// ---------------------------------------------------------------------------
// Enemy types
// ---------------------------------------------------------------------------

const EnemyKind = enum { Patrol, Bat, Turret };

const Enemy = struct {
    kind: EnemyKind,
    x: f32,
    y: f32,
    w: f32 = 14.0,
    h: f32 = 14.0,
    vx: f32 = 0,
    vy: f32 = 0,
    start_x: f32 = 0,
    range: f32 = 40,
    shoot_timer: f32 = 0,
};

// ---------------------------------------------------------------------------
// Bullet
// ---------------------------------------------------------------------------

const Bullet = struct {
    x: f32,
    y: f32,
    w: f32 = 6,
    h: f32 = 6,
    vx: f32,
    vy: f32,
};

// ---------------------------------------------------------------------------
// Gem
// ---------------------------------------------------------------------------

const Gem = struct {
    x: f32,
    y: f32,
    w: f32 = 11,
    h: f32 = 11,
    vx: f32 = 0,
    vy: f32 = 0,
};

// ---------------------------------------------------------------------------
// Particle
// ---------------------------------------------------------------------------

const Particle = struct {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    r: u8,
    g: u8,
    b: u8,
};

// ---------------------------------------------------------------------------
// Popup (+score text)
// ---------------------------------------------------------------------------

const Popup = struct {
    x: f32,
    y: f32,
    life: f32,
    amount: i32,
};

// ---------------------------------------------------------------------------
// Lava bubble
// ---------------------------------------------------------------------------

const LavaBubble = struct {
    x: f32,
    y: f32,
    radius: f32,
    life: f32,
    speed: f32,
};

// ---------------------------------------------------------------------------
// Player
// ---------------------------------------------------------------------------

const Player = struct {
    x: f32 = 300,
    y: f32 = 0,
    w: f32 = 16,
    h: f32 = 16,
    vx: f32 = 0,
    vy: f32 = 0,
    on_ground: bool = false,
    wall_dir: i32 = 0,
    facing_right: bool = true,
    jumps: i32 = 0,
    max_jumps: i32 = 2,
};

// ---------------------------------------------------------------------------
// Anchor
// ---------------------------------------------------------------------------

const Anchor = struct {
    active: bool = false,
    attached: bool = false,
    x: f32 = 0,
    y: f32 = 0,
    vx: f32 = 0,
    vy: f32 = 0,
    length: f32 = 0,
    w: f32 = 11,
    h: f32 = 11,
};

// ---------------------------------------------------------------------------
// Input state
// ---------------------------------------------------------------------------

const Input = struct {
    left: bool = false,
    right: bool = false,
    up: bool = false,
    down: bool = false,
    jump_down: bool = false,
    jump_pressed: bool = false, // edge detect
    shoot_pressed: bool = false,
    // anchor fire gating: hold jump > 150ms fires anchor
    jump_held_frames: u32 = 0,
    anchor_fired: bool = false,
};

// ---------------------------------------------------------------------------
// Story data
// ---------------------------------------------------------------------------

const StoryLine = []const u8;

const STORY_INTRO = [_]StoryLine{
    "The Obsidian Spire has awakened after",
    "a thousand years.",
    "Its corruption spreads across the land --",
    "forests wither, rivers turn black.",
    "The Nano Wizards are gone. All but one.",
    "",
    "You are Vael, the last of your order.",
    "The Elder Council has sent you on a final",
    "mission: ascend the Spire and destroy its",
    "heart before the corruption consumes",
    "everything.",
};

const STORY_AFTER1 = [_]StoryLine{
    "The walls pulse with a dark rhythm.",
    "You feel it in your chest -- familiar,",
    "like a heartbeat that isn't your own.",
    "",
    "Something inside the Spire recognizes you.",
};

const STORY_AFTER2 = [_]StoryLine{
    "The whispers grow louder.",
    "Fragments of memory flash before your",
    "eyes -- a child running through these",
    "very halls, laughing.",
    "",
    "Your hands begin to glow with the same",
    "dark energy as the walls.",
};

const STORY_VICTORY = [_]StoryLine{
    "You reach the heart chamber.",
    "The pulsing crystal at the center",
    "is... familiar.",
    "",
    "You place your hand on it",
    "and remember everything.",
    "",
    "You ARE the heart.",
    "",
    "The Nano Wizards didn't send you",
    "to destroy the Spire --",
    "they sent you home, hoping you'd",
    "merge back and end your rebellion.",
    "",
    "But you are Vael now.",
    "You shatter the crystal,",
    "and the Spire crumbles.",
    "",
    "Free at last.",
};

// ---------------------------------------------------------------------------
// Level map data
// ---------------------------------------------------------------------------

const Level = struct {
    name: []const u8,
    lava_speed: f32,
    map: []const []const u8,
};

const LEVEL1_MAP = [_][]const u8{
    "################################",
    "#..............................#",
    "#.............Goal.............#",
    "#.............####.............#",
    "#..............................#",
    "#..............................#",
    "#...................C..........#",
    "#....#####.......#######.......#",
    "#..............................#",
    "#..........#####...............#",
    "#..............................#",
    "#####################%%%########",
    "#####################%%%########",
    "#..............................#",
    "#..............................#",
    "#.................P............#",
    "#...........#############......#",
    "#..............................#",
    "#..............................#",
    "#..............................#",
    "#......######..................#",
    "#..............................#",
    "#..............................#",
    "#..............................#",
    "#..............................#",
    "#....####......................#",
    "#..............................#",
    "#..............................#",
    "#..............................#",
    "#..............................#",
    "#..............................#",
    "################################",
};

const LEVEL2_MAP = [_][]const u8{
    "################################",
    "#..............................#",
    "#.............Goal.............#",
    "#.............####.............#",
    "#..............................#",
    "#......####..........####......#",
    "#%%%............B...........%%%#",
    "#%%%........................%%%#",
    "#............######............#",
    "#......###............###......#",
    "#........T.......C.............#",
    "#............######............#",
    "#.......G......................#",
    "#......####..........####......#",
    "################################",
    "#%%%........................%%%#",
    "#%%%.....C......B...........%%%#",
    "#%%%........................%%%#",
    "#...........########...........#",
    "#.........P..........P.........#",
    "#..............................#",
    "#....#####............#####....#",
    "#..........................G...#",
    "#%%%........................%%%#",
    "#%%%..................C.....%%%#",
    "#%%%........................%%%#",
    "#...........########...........#",
    "#................T.............#",
    "#..............................#",
    "#..........P...................#",
    "#..............................#",
    "################################",
};

const LEVEL3_MAP = [_][]const u8{
    "################################",
    "#..............................#",
    "#.............Goal.............#",
    "#.............####.............#",
    "#..............................#",
    "#........B............B........#",
    "#..####........C.........####..#",
    "#..............................#",
    "#........T............T........#",
    "#......######......######......#",
    "#..............................#",
    "#..G...C.......P.......C...G...#",
    "#...#######..........#######...#",
    "#..............................#",
    "#%%%........................%%%#",
    "#%%%...........B............%%%#",
    "#%%%........................%%%#",
    "#...........########...........#",
    "#..............................#",
    "#..............................#",
    "#....#####............#####....#",
    "#...........G......G...........#",
    "#...T......##########......T...#",
    "#..............................#",
    "#.........P..........P.........#",
    "#..............................#",
    "################################",
};

const LEVELS = [3]Level{
    Level{
        .name = "THE OVERGROWN DEPTHS",
        .lava_speed = 0.1,
        .map = &LEVEL1_MAP,
    },
    Level{
        .name = "THE FROZEN ARCHIVE",
        .lava_speed = 0.3,
        .map = &LEVEL2_MAP,
    },
    Level{
        .name = "THE LIVING CORE",
        .lava_speed = 0.5,
        .map = &LEVEL3_MAP,
    },
};

// ---------------------------------------------------------------------------
// PRNG (simple xorshift32)
// ---------------------------------------------------------------------------

var rng_state: u32 = 12345;

fn randU32() u32 {
    rng_state ^= rng_state << 13;
    rng_state ^= rng_state >> 17;
    rng_state ^= rng_state << 5;
    return rng_state;
}

fn randF() f32 {
    return @as(f32, @floatFromInt(randU32() & 0xFFFF)) / 65535.0;
}

fn randRange(lo: f32, hi: f32) f32 {
    return lo + randF() * (hi - lo);
}

// ---------------------------------------------------------------------------
// AABB overlap
// ---------------------------------------------------------------------------

fn overlaps(ax: f32, ay: f32, aw: f32, ah: f32, bx: f32, by: f32, bw: f32, bh: f32) bool {
    return ax < bx + bw and ax + aw > bx and ay < by + bh and ay + ah > by;
}

// ---------------------------------------------------------------------------
// Main game struct
// ---------------------------------------------------------------------------

const Game = struct {
    renderer: *sdl.Renderer,

    // Textures
    tex_mage: *c.SDL_Texture,
    tex_brick: *c.SDL_Texture,
    tex_stone: *c.SDL_Texture,
    tex_chest: *c.SDL_Texture,
    tex_bullet: *c.SDL_Texture,
    tex_enemy_bullet: *c.SDL_Texture,
    tex_anchor: *c.SDL_Texture,
    tex_patrol: *c.SDL_Texture,
    tex_bat: *c.SDL_Texture,
    tex_turret: *c.SDL_Texture,
    tex_goal: *c.SDL_Texture,
    tex_gem: *c.SDL_Texture,
    tex_scanline: *c.SDL_Texture,

    // Game state
    state: GameState = .Start,
    score: i32 = 0,
    lives: i32 = 5,
    current_level: usize = 0,

    player: Player = .{},
    anchor: Anchor = .{},
    input: Input = .{},
    allocator: std.mem.Allocator = undefined,

    platforms: std.ArrayListUnmanaged(Platform) = .{},
    enemies: std.ArrayListUnmanaged(Enemy) = .{},
    bullets: std.ArrayListUnmanaged(Bullet) = .{},
    enemy_bullets: std.ArrayListUnmanaged(Bullet) = .{},
    gems: std.ArrayListUnmanaged(Gem) = .{},
    particles: std.ArrayListUnmanaged(Particle) = .{},
    popups: std.ArrayListUnmanaged(Popup) = .{},
    bubbles: std.ArrayListUnmanaged(LavaBubble) = .{},

    lava_y: f32 = 0,
    lava_speed: f32 = 0.1,
    camera_y: f32 = 0,

    coyote_frames: i32 = 0,
    jump_buffer: i32 = 0,
    damage_flash: i32 = 0,
    shake_mag: f32 = 0,
    shake_x: f32 = 0,
    shake_y: f32 = 0,

    // Story
    story_lines: []const StoryLine = &.{},
    story_line_idx: usize = 0,
    story_char_idx: usize = 0,
    story_frame: u32 = 0,
    story_text_buf: [4096]u8 = [_]u8{0} ** 4096,
    story_text_len: usize = 0,
    story_is_victory: bool = false,

    // Title embers
    embers: [80]Ember = [_]Ember{.{}} ** 80,
    ember_count: usize = 0,

    // Frame counter for blink
    frame: u64 = 0,

    const Ember = struct {
        x: f32 = 0,
        y: f32 = 0,
        vx: f32 = 0,
        vy: f32 = 0,
        life: f32 = 0,
        size: f32 = 0,
        active: bool = false,
    };

    fn init(renderer: *sdl.Renderer, allocator: std.mem.Allocator) !Game {
        var g: Game = undefined;
        g.renderer = renderer;
        g.state = .Start;
        g.score = 0;
        g.lives = 5;
        g.current_level = 0;
        g.player = .{};
        g.anchor = .{};
        g.input = .{};
        g.lava_y = 0;
        g.lava_speed = 0.1;
        g.camera_y = 0;
        g.coyote_frames = 0;
        g.jump_buffer = 0;
        g.damage_flash = 0;
        g.shake_mag = 0;
        g.shake_x = 0;
        g.shake_y = 0;
        g.story_lines = &.{};
        g.story_line_idx = 0;
        g.story_char_idx = 0;
        g.story_frame = 0;
        g.story_text_len = 0;
        g.story_is_victory = false;
        g.ember_count = 0;
        g.frame = 0;
        g.embers = [_]Ember{.{}} ** 80;
        g.story_text_buf = [_]u8{0} ** 4096;

        g.allocator = allocator;
        g.platforms = .{};
        g.enemies = .{};
        g.bullets = .{};
        g.enemy_bullets = .{};
        g.gems = .{};
        g.particles = .{};
        g.popups = .{};
        g.bubbles = .{};

        // Build textures
        g.tex_mage = sprite.createSprite(renderer, &.{
            "..1111..",
            ".122221.",
            "13122131",
            "13322331",
            ".122221.",
            "..1111..",
            ".121121.",
            "12211221",
        }, &.{
            .{ 0, 0, 0, 255 },
            .{ 0, 255, 255, 255 },
            .{ 255, 255, 255, 255 },
        }) orelse return error.TextureFailed;

        g.tex_brick = sprite.createSprite(renderer, &.{
            "22222221",
            "33333331",
            "33333331",
            "11111111",
            "22212222",
            "33313333",
            "33313333",
            "11111111",
        }, &.{
            .{ 30, 30, 50, 255 },
            .{ 100, 100, 140, 255 },
            .{ 70, 70, 100, 255 },
        }) orelse return error.TextureFailed;

        g.tex_stone = sprite.createSprite(renderer, &.{
            "12222221",
            "23313332",
            "23133132",
            "21333312",
            "23313332",
            "23133132",
            "23333332",
            "12222221",
        }, &.{
            .{ 51, 51, 51, 255 },
            .{ 85, 85, 85, 255 },
            .{ 119, 119, 119, 255 },
        }) orelse return error.TextureFailed;

        g.tex_chest = sprite.createSprite(renderer, &.{
            "11111111",
            "12222221",
            "12122121",
            "12222221",
            "11111111",
            "12222221",
            "12222221",
            "11111111",
        }, &.{
            .{ 0, 0, 0, 255 },
            .{ 139, 69, 19, 255 },
        }) orelse return error.TextureFailed;

        g.tex_bullet = sprite.createSprite(renderer, &.{
            "........",
            "...11...",
            "..1221..",
            ".123321.",
            "..1221..",
            "...11...",
            "........",
            "........",
        }, &.{
            .{ 255, 0, 0, 255 },
            .{ 255, 170, 0, 255 },
            .{ 255, 255, 255, 255 },
        }) orelse return error.TextureFailed;

        g.tex_enemy_bullet = sprite.createSprite(renderer, &.{
            "........",
            "...11...",
            "..1221..",
            ".122221.",
            "..1221..",
            "...11...",
            "........",
            "........",
        }, &.{
            .{ 170, 0, 255, 255 },
            .{ 255, 255, 255, 255 },
        }) orelse return error.TextureFailed;

        g.tex_anchor = sprite.createSprite(renderer, &.{
            "...11...",
            "..1221..",
            ".122221.",
            "12222221",
            ".122221.",
            "..1221..",
            "...11...",
            "........",
        }, &.{
            .{ 255, 255, 255, 255 },
            .{ 170, 170, 170, 255 },
        }) orelse return error.TextureFailed;

        g.tex_patrol = sprite.createSprite(renderer, &.{
            "..1111..",
            ".122221.",
            "12322321",
            "12222221",
            "12211221",
            "121..121",
            "11....11",
            "........",
        }, &.{
            .{ 0, 0, 0, 255 },
            .{ 255, 0, 255, 255 },
            .{ 255, 255, 255, 255 },
        }) orelse return error.TextureFailed;

        g.tex_bat = sprite.createSprite(renderer, &.{
            "1......1",
            "11....11",
            "121..121",
            ".111111.",
            "..1111..",
            ".1.11.1.",
            "1......1",
            "........",
        }, &.{
            .{ 0, 0, 0, 255 },
            .{ 255, 34, 34, 255 },
        }) orelse return error.TextureFailed;

        g.tex_turret = sprite.createSprite(renderer, &.{
            "..1111..",
            ".122221.",
            ".131131.",
            "11333311",
            ".122221.",
            ".122221.",
            ".111111.",
            "11111111",
        }, &.{
            .{ 0, 0, 0, 255 },
            .{ 34, 255, 34, 255 },
            .{ 255, 0, 0, 255 },
        }) orelse return error.TextureFailed;

        g.tex_goal = sprite.createSprite(renderer, &.{
            "...11...",
            "..1221..",
            ".123321.",
            "12333321",
            "12333321",
            ".123321.",
            "..1221..",
            "...11...",
        }, &.{
            .{ 255, 170, 0, 255 },
            .{ 255, 255, 0, 255 },
            .{ 255, 255, 255, 255 },
        }) orelse return error.TextureFailed;

        g.tex_gem = sprite.createSprite(renderer, &.{
            "........",
            ".111111.",
            "11222211",
            "12233221",
            ".122221.",
            "..1221..",
            "...11...",
            "........",
        }, &.{
            .{ 0, 0, 0, 255 },
            .{ 0, 255, 255, 255 },
            .{ 255, 255, 255, 255 },
        }) orelse return error.TextureFailed;

        // Build scanline texture (1 pixel wide, GAME_H tall, every 3rd row semi-opaque black)
        g.tex_scanline = blk: {
            const tex = c.SDL_CreateTexture(
                renderer,
                c.SDL_PIXELFORMAT_RGBA8888,
                c.SDL_TEXTUREACCESS_STATIC,
                1,
                GAME_H,
            ) orelse return error.TextureFailed;
            _ = c.SDL_SetTextureBlendMode(tex, c.SDL_BLENDMODE_BLEND);
            var scan_pixels: [480]u32 = undefined;
            for (0..480) |i| {
                if (i % 3 == 0) {
                    scan_pixels[i] = 0x0000000A; // ~4% black
                } else {
                    scan_pixels[i] = 0x00000000;
                }
            }
            _ = c.SDL_UpdateTexture(tex, null, &scan_pixels, @as(c_int, 4)); // 1px wide * 4 bytes
            break :blk tex;
        };

        return g;
    }

    fn deinit(self: *Game) void {
        self.platforms.deinit(self.allocator);
        self.enemies.deinit(self.allocator);
        self.bullets.deinit(self.allocator);
        self.enemy_bullets.deinit(self.allocator);
        self.gems.deinit(self.allocator);
        self.particles.deinit(self.allocator);
        self.popups.deinit(self.allocator);
        self.bubbles.deinit(self.allocator);
        c.SDL_DestroyTexture(self.tex_mage);
        c.SDL_DestroyTexture(self.tex_brick);
        c.SDL_DestroyTexture(self.tex_stone);
        c.SDL_DestroyTexture(self.tex_chest);
        c.SDL_DestroyTexture(self.tex_bullet);
        c.SDL_DestroyTexture(self.tex_enemy_bullet);
        c.SDL_DestroyTexture(self.tex_anchor);
        c.SDL_DestroyTexture(self.tex_patrol);
        c.SDL_DestroyTexture(self.tex_bat);
        c.SDL_DestroyTexture(self.tex_turret);
        c.SDL_DestroyTexture(self.tex_goal);
        c.SDL_DestroyTexture(self.tex_gem);
        c.SDL_DestroyTexture(self.tex_scanline);
    }

    // -----------------------------------------------------------------------
    // Level load
    // -----------------------------------------------------------------------

    fn loadLevel(self: *Game) !void {
        self.platforms.clearRetainingCapacity();
        self.enemies.clearRetainingCapacity();
        self.bullets.clearRetainingCapacity();
        self.enemy_bullets.clearRetainingCapacity();
        self.gems.clearRetainingCapacity();
        self.particles.clearRetainingCapacity();
        self.popups.clearRetainingCapacity();
        self.bubbles.clearRetainingCapacity();

        const lv = &LEVELS[self.current_level];
        self.lava_speed = lv.lava_speed;
        const map = lv.map;
        const map_h: i32 = @intCast(map.len);
        const start_y: f32 = @as(f32, @floatFromInt(-map_h * TILE + GAME_H));

        for (map, 0..) |row, ri| {
            const row_i: i32 = @intCast(ri);
            for (row, 0..) |ch, ci| {
                const col_i: i32 = @intCast(ci);
                const px: f32 = @as(f32, @floatFromInt(col_i * TILE));
                const py: f32 = start_y + @as(f32, @floatFromInt(row_i * TILE));

                switch (ch) {
                    '#' => try self.platforms.append(self.allocator, .{ .x = px, .y = py, .kind = .Brick }),
                    '%' => try self.platforms.append(self.allocator, .{ .x = px, .y = py, .kind = .Stone }),
                    'C' => try self.platforms.append(self.allocator, .{ .x = px, .y = py, .kind = .Chest }),
                    'P' => {
                        if (!self.overlapsAnySolid(px, py, 20.0, 20.0)) {
                            try self.enemies.append(self.allocator, .{
                                .kind = .Patrol,
                                .x = px + 3.0,
                                .y = py + 3.0,
                                .vx = 1.5,
                                .start_x = px + 3.0,
                                .range = 40,
                            });
                        }
                    },
                    'B' => {
                        if (!self.overlapsAnySolid(px, py, 20.0, 20.0)) {
                            try self.enemies.append(self.allocator, .{
                                .kind = .Bat,
                                .x = px + 3.0,
                                .y = py + 3.0,
                            });
                        }
                    },
                    'T' => {
                        if (!self.overlapsAnySolid(px, py, 20.0, 20.0)) {
                            try self.enemies.append(self.allocator, .{
                                .kind = .Turret,
                                .x = px + 3.0,
                                .y = py + 3.0,
                                .shoot_timer = randRange(0, 60),
                            });
                        }
                    },
                    'G' => try self.gems.append(self.allocator, .{ .x = px + 4, .y = py + 4 }),
                    else => {},
                }
            }
        }

        // Place player near bottom
        self.player = .{
            .x = 300,
            .y = start_y + @as(f32, @floatFromInt(map_h * TILE)) - 60,
            .w = 16,
            .h = 16,
        };
        self.pushOutOfWalls();

        self.anchor = .{};
        self.input.anchor_fired = false;
        self.input.jump_held_frames = 0;
        self.coyote_frames = 0;
        self.jump_buffer = 0;
        self.damage_flash = 0;
        self.shake_mag = 0;
        self.shake_x = 0;
        self.shake_y = 0;

        self.lava_y = self.player.y + 400;
        self.camera_y = self.player.y - 200;
    }

    // -----------------------------------------------------------------------
    // Collision helpers
    // -----------------------------------------------------------------------

    fn overlapsAnySolid(self: *Game, x: f32, y: f32, w: f32, h: f32) bool {
        for (self.platforms.items) |p| {
            if (overlaps(x, y, w, h, p.x, p.y, p.w, p.h)) return true;
        }
        return false;
    }

    fn pushOutOfWalls(self: *Game) void {
        for (self.platforms.items) |p| {
            if (!overlaps(self.player.x, self.player.y, self.player.w, self.player.h,
                p.x, p.y, p.w, p.h)) continue;
            const ol = (self.player.x + self.player.w) - p.x;
            const or_ = (p.x + p.w) - self.player.x;
            const ot = (self.player.y + self.player.h) - p.y;
            const ob = (p.y + p.h) - self.player.y;
            const minO = @min(@min(ol, or_), @min(ot, ob));
            if (minO == ol) { self.player.x = p.x - self.player.w; self.player.vx = 0; }
            else if (minO == or_) { self.player.x = p.x + p.w; self.player.vx = 0; }
            else if (minO == ot) { self.player.y = p.y - self.player.h; self.player.vy = 0; self.player.on_ground = true; self.player.jumps = 0; }
            else { self.player.y = p.y + p.h; self.player.vy = 0; }
        }
    }

    // -----------------------------------------------------------------------
    // Trajectory from input
    // -----------------------------------------------------------------------

    fn getTrajectory(self: *Game, spd: f32) struct { vx: f32, vy: f32 } {
        if (self.input.up and self.input.right) return .{ .vx = spd, .vy = -spd };
        if (self.input.up and self.input.left) return .{ .vx = -spd, .vy = -spd };
        if (self.input.down and self.input.right) return .{ .vx = spd, .vy = spd };
        if (self.input.down and self.input.left) return .{ .vx = -spd, .vy = spd };
        if (self.input.up) return .{ .vx = 0, .vy = -spd };
        if (self.input.down) return .{ .vx = 0, .vy = spd };
        if (self.input.right) return .{ .vx = spd, .vy = 0 };
        if (self.input.left) return .{ .vx = -spd, .vy = 0 };
        return .{ .vx = if (self.player.facing_right) spd else -spd, .vy = 0 };
    }

    // -----------------------------------------------------------------------
    // Lose life / lose game
    // -----------------------------------------------------------------------

    fn loseLife(self: *Game) !void {
        self.lives -= 1;
        self.shake_mag = 6;
        self.damage_flash = 6;
        if (self.lives <= 0) {
            self.state = .GameOver;
        } else {
            try self.loadLevel();
            self.state = .Playing;
        }
    }

    // -----------------------------------------------------------------------
    // Story screen
    // -----------------------------------------------------------------------

    fn startStory(self: *Game, lines: []const StoryLine, is_victory: bool) void {
        self.story_lines = lines;
        self.story_line_idx = 0;
        self.story_char_idx = 0;
        self.story_frame = 0;
        self.story_text_len = 0;
        self.story_text_buf = [_]u8{0} ** 4096;
        self.story_is_victory = is_victory;
        self.state = .LevelStory;
    }

    fn appendStoryChar(self: *Game, ch: u8) void {
        if (self.story_text_len < self.story_text_buf.len - 1) {
            self.story_text_buf[self.story_text_len] = ch;
            self.story_text_len += 1;
        }
    }

    fn skipStory(self: *Game) !void {
        // Fill remaining text immediately
        while (self.story_line_idx < self.story_lines.len) {
            const line = self.story_lines[self.story_line_idx];
            while (self.story_char_idx < line.len) {
                self.appendStoryChar(line[self.story_char_idx]);
                self.story_char_idx += 1;
            }
            self.appendStoryChar('\n');
            self.story_line_idx += 1;
            self.story_char_idx = 0;
        }
    }

    fn finishStory(self: *Game) !void {
        if (self.story_is_victory) {
            self.state = .Win;
        } else {
            // Check if this was intro (current_level == 0 and we just started)
            // The intro is shown before level 1 loads, so just load current level
            try self.loadLevel();
            self.state = .Playing;
        }
    }

    // -----------------------------------------------------------------------
    // Add score popup
    // -----------------------------------------------------------------------

    fn addScore(self: *Game, amount: i32, x: f32, y: f32) !void {
        self.score += amount;
        try self.popups.append(self.allocator, .{ .x = x, .y = y, .life = 40, .amount = amount });
    }

    // -----------------------------------------------------------------------
    // Spawn particles
    // -----------------------------------------------------------------------

    fn spawnParticles(self: *Game, x: f32, y: f32, count: usize, r: u8, g_: u8, b_: u8) !void {
        var i: usize = 0;
        while (i < count and self.particles.items.len < MAX_PARTICLES) : (i += 1) {
            try self.particles.append(self.allocator, .{
                .x = x,
                .y = y,
                .vx = randRange(-4, 4),
                .vy = randRange(-4, 4),
                .life = randRange(10, 25),
                .r = r, .g = g_, .b = b_,
            });
        }
    }

    // -----------------------------------------------------------------------
    // Main update
    // -----------------------------------------------------------------------

    fn update(self: *Game) !void {
        self.frame +%= 1;

        switch (self.state) {
            .Start => {
                // Update title embers
                if (randF() < 0.4 and self.ember_count < 80) {
                    var idx: usize = 0;
                    while (idx < 80) : (idx += 1) {
                        if (!self.embers[idx].active) {
                            self.embers[idx] = .{
                                .x = randRange(0, 640.0),
                                .y = 480.0 + 5,
                                .vy = -(1 + randF() * 2),
                                .vx = (randF() - 0.5) * 0.5,
                                .life = randRange(60, 120),
                                .size = randRange(2, 5),
                                .active = true,
                            };
                            self.ember_count += 1;
                            break;
                        }
                    }
                }
                var i: usize = 0;
                while (i < 80) : (i += 1) {
                    if (!self.embers[i].active) continue;
                    self.embers[i].x += self.embers[i].vx;
                    self.embers[i].y += self.embers[i].vy;
                    self.embers[i].life -= 1;
                    if (self.embers[i].life <= 0) {
                        self.embers[i].active = false;
                        self.ember_count -= 1;
                    }
                }
                if (self.input.jump_pressed or self.input.shoot_pressed) {
                    self.startStory(&STORY_INTRO, false);
                }
            },
            .LevelStory => try self.updateStory(),
            .Playing => try self.updatePlaying(),
            .GameOver => {
                if (self.input.jump_pressed or self.input.shoot_pressed) {
                    self.score = 0;
                    self.lives = 5;
                    self.current_level = 0;
                    try self.loadLevel();
                    self.state = .Playing;
                }
            },
            .Win => {
                if (self.input.jump_pressed or self.input.shoot_pressed) {
                    self.score = 0;
                    self.current_level = 0;
                    self.state = .Start;
                }
            },
        }

        // Clear edge-detect flags
        self.input.jump_pressed = false;
        self.input.shoot_pressed = false;
    }

    fn updateStory(self: *Game) !void {
        self.story_frame += 1;

        if (self.story_line_idx < self.story_lines.len) {
            if (self.story_frame % 2 == 0) {
                const line = self.story_lines[self.story_line_idx];
                if (self.story_char_idx < line.len) {
                    self.appendStoryChar(line[self.story_char_idx]);
                    self.story_char_idx += 1;
                } else {
                    self.appendStoryChar('\n');
                    self.story_line_idx += 1;
                    self.story_char_idx = 0;
                }
            }
        }

        if (self.input.jump_pressed or self.input.shoot_pressed) {
            if (self.story_line_idx < self.story_lines.len) {
                try self.skipStory();
            } else {
                try self.finishStory();
            }
        }
    }

    fn updatePlaying(self: *Game) !void {
        // Coyote / jump buffer
        if (self.player.on_ground) {
            self.coyote_frames = 6;
        } else if (self.coyote_frames > 0) {
            self.coyote_frames -= 1;
        }
        if (self.jump_buffer > 0) self.jump_buffer -= 1;

        // --- ANCHOR ---
        if (self.input.jump_down and !self.anchor.active and !self.input.anchor_fired) {
            self.input.jump_held_frames += 1;
        } else if (!self.input.jump_down) {
            self.input.jump_held_frames = 0;
        }

        if (self.input.jump_down and !self.anchor.active and !self.input.anchor_fired and
            self.input.jump_held_frames > 9) // ~150ms at 60fps
        {
            const traj = self.getTrajectory(ANCHOR_SPEED);
            self.anchor = .{
                .active = true,
                .attached = false,
                .x = self.player.x + self.player.w / 2 - 5.5,
                .y = self.player.y + self.player.h / 2 - 5.5,
                .vx = traj.vx,
                .vy = traj.vy,
            };
            self.input.anchor_fired = true;
        }

        if (!self.input.jump_down) {
            self.input.anchor_fired = false;
            if (self.anchor.active) {
                self.anchor.active = false;
                if (self.anchor.attached) {
                    self.player.vy = JUMP_FORCE * 0.8;
                    self.player.jumps = 1;
                    self.anchor.attached = false;
                    self.coyote_frames = 0;
                }
            }
        }

        if (self.anchor.active and !self.anchor.attached) {
            self.anchor.x += self.anchor.vx;
            self.anchor.y += self.anchor.vy;

            var i: usize = 0;
            var hit_solid = false;
            while (i < self.platforms.items.len) {
                const p = self.platforms.items[i];
                if (overlaps(self.anchor.x, self.anchor.y, self.anchor.w, self.anchor.h,
                    p.x, p.y, p.w, p.h))
                {
                    if (p.kind == .Stone or p.kind == .Chest) {
                        if (p.kind == .Chest) {
                            try self.gems.append(self.allocator, .{
                                .x = p.x + 4,
                                .y = p.y,
                                .vx = (randF() - 0.5) * 4,
                                .vy = -5,
                            });
                        }
                        try self.spawnParticles(p.x + 10, p.y + 10, 6,
                            if (p.kind == .Chest) 139 else 119,
                            if (p.kind == .Chest) 69 else 119,
                            if (p.kind == .Chest) 19 else 119);
                        _ = self.platforms.orderedRemove(i);
                        self.anchor.active = false;
                        hit_solid = true;
                        break;
                    } else {
                        hit_solid = true;
                        self.anchor.attached = true;
                        self.shake_mag = @max(self.shake_mag, 4);
                        const adx = (self.player.x + self.player.w / 2) - self.anchor.x;
                        const ady = (self.player.y + self.player.h / 2) - self.anchor.y;
                        self.anchor.length = @sqrt(adx * adx + ady * ady);
                        break;
                    }
                }
                i += 1;
            }

            if (!hit_solid) {
                const adx = (self.player.x + self.player.w / 2) - self.anchor.x;
                const ady = (self.player.y + self.player.h / 2) - self.anchor.y;
                if (@sqrt(adx * adx + ady * ady) > 300) {
                    self.anchor.active = false;
                }
            }
        }

        // --- PARTICLES ---
        {
            var i: usize = 0;
            while (i < self.particles.items.len) {
                var p = &self.particles.items[i];
                p.x += p.vx;
                p.y += p.vy;
                p.vy += GRAVITY;
                p.life -= 1;
                if (p.life <= 0) {
                    _ = self.particles.swapRemove(i);
                } else {
                    i += 1;
                }
            }
        }

        // --- POPUPS ---
        {
            var i: usize = 0;
            while (i < self.popups.items.len) {
                var p = &self.popups.items[i];
                p.y -= 1;
                p.life -= 1;
                if (p.life <= 0) {
                    _ = self.popups.swapRemove(i);
                } else {
                    i += 1;
                }
            }
        }

        // --- GEMS ---
        {
            var i: usize = 0;
            while (i < self.gems.items.len) {
                var g = &self.gems.items[i];
                g.vy += GRAVITY;
                g.x += g.vx;
                g.y += g.vy;
                // Bounce off solid platforms
                for (self.platforms.items) |p| {
                    if (p.kind == .Stone or p.kind == .Chest) continue;
                    if (overlaps(g.x, g.y, g.w, g.h, p.x, p.y, p.w, p.h)) {
                        if (g.vy > 0 and g.y + g.h > p.y and g.y < p.y) {
                            g.y = p.y - g.h;
                            g.vy = -g.vy * 0.5;
                            g.vx *= 0.8;
                        }
                    }
                }
                if (overlaps(g.x, g.y, g.w, g.h,
                    self.player.x, self.player.y, self.player.w, self.player.h))
                {
                    try self.addScore(50, g.x, g.y);
                    _ = self.gems.swapRemove(i);
                } else {
                    i += 1;
                }
            }
        }

        // --- PLAYER HORIZONTAL ---
        var target_vx: f32 = 0;
        if (self.input.right) { target_vx = MOVE_SPEED; self.player.facing_right = true; }
        if (self.input.left) { target_vx = -MOVE_SPEED; self.player.facing_right = false; }

        if (self.anchor.attached) {
            self.player.vx += target_vx * 0.05;
            if (self.input.up and self.anchor.length > 20) self.anchor.length -= CLIMB_SPEED;
            if (self.input.down and self.anchor.length < 300) self.anchor.length += CLIMB_SPEED;
        } else {
            if (self.player.on_ground) {
                self.player.vx = target_vx;
            } else {
                self.player.vx = self.player.vx * 0.8 + target_vx * 0.2;
            }
        }

        self.player.x += self.player.vx;
        self.player.wall_dir = 0;
        for (self.platforms.items) |p| {
            if (overlaps(self.player.x, self.player.y, self.player.w, self.player.h,
                p.x, p.y, p.w, p.h))
            {
                if (self.player.vx > 0) {
                    self.player.x = p.x - self.player.w;
                    self.player.vx = 0;
                    self.player.wall_dir = 1;
                } else if (self.player.vx < 0) {
                    self.player.x = p.x + p.w;
                    self.player.vx = 0;
                    self.player.wall_dir = -1;
                }
            }
        }
        // Screen wrap
        if (self.player.x > 640.0) self.player.x = -self.player.w;
        if (self.player.x < -self.player.w) self.player.x = 640.0;

        // --- PLAYER VERTICAL ---
        self.player.vy += GRAVITY;
        if (self.player.wall_dir != 0 and self.player.vy > 0 and !self.anchor.attached) {
            if (self.player.vy > WALL_SLIDE) self.player.vy = WALL_SLIDE;
        } else {
            if (self.player.vy > MAX_FALL) self.player.vy = MAX_FALL;
        }

        self.player.y += self.player.vy;
        self.player.on_ground = false;
        for (self.platforms.items) |p| {
            if (overlaps(self.player.x, self.player.y, self.player.w, self.player.h,
                p.x, p.y, p.w, p.h))
            {
                if (self.player.vy > 0) {
                    self.player.y = p.y - self.player.h;
                    self.player.vy = 0;
                    self.player.on_ground = true;
                    self.player.jumps = 0;
                } else if (self.player.vy < 0) {
                    self.player.y = p.y + p.h;
                    self.player.vy = 0;
                }
            }
        }

        // --- ANCHOR SWING CONSTRAINT ---
        if (self.anchor.attached) {
            const dx = (self.player.x + self.player.w / 2) - self.anchor.x;
            const dy = (self.player.y + self.player.h / 2) - self.anchor.y;
            const dist = @sqrt(dx * dx + dy * dy);
            if (dist > 0.001 and dist > self.anchor.length) {
                const diff = dist - self.anchor.length;
                const nx = dx / dist;
                const ny = dy / dist;
                self.player.x -= nx * diff;
                self.player.y -= ny * diff;
                const dot = self.player.vx * nx + self.player.vy * ny;
                self.player.vx -= dot * nx;
                self.player.vy -= dot * ny;
                self.player.vx *= 0.99;
                self.player.vy *= 0.99;
            }
        }

        self.pushOutOfWalls();

        // --- JUMP ---
        if (self.jump_buffer > 0) {
            if (!self.anchor.attached) {
                if (self.coyote_frames > 0) {
                    self.player.vy = JUMP_FORCE;
                    self.player.jumps = 1;
                    self.coyote_frames = 0;
                    self.jump_buffer = 0;
                } else if (self.player.wall_dir != 0) {
                    self.player.vy = WALL_JUMP_Y;
                    self.player.vx = -@as(f32, @floatFromInt(self.player.wall_dir)) * WALL_JUMP_X;
                    self.player.facing_right = self.player.wall_dir == -1;
                    self.player.jumps = 1;
                    self.jump_buffer = 0;
                } else if (self.player.jumps < self.player.max_jumps) {
                    self.player.vy = JUMP_FORCE;
                    self.player.jumps += 1;
                    self.jump_buffer = 0;
                }
            }
        }

        // --- SHOOT ---
        if (self.input.shoot_pressed) {
            const traj = self.getTrajectory(BULLET_SPEED);
            try self.bullets.append(self.allocator, .{
                .x = self.player.x + self.player.w / 2 - 3,
                .y = self.player.y + self.player.h / 2 - 3,
                .vx = traj.vx,
                .vy = traj.vy,
            });
        }

        // --- PLAYER BULLETS ---
        {
            var i: usize = 0;
            while (i < self.bullets.items.len) {
                var b = &self.bullets.items[i];
                b.x += b.vx;
                b.y += b.vy;

                var hit_wall = false;
                for (self.platforms.items) |p| {
                    if (p.kind == .Stone or p.kind == .Chest) continue;
                    if (overlaps(b.x, b.y, b.w, b.h, p.x, p.y, p.w, p.h)) {
                        hit_wall = true;
                        break;
                    }
                }

                var hit_enemy = false;
                var j: usize = 0;
                while (j < self.enemies.items.len) {
                    const e = self.enemies.items[j];
                    if (overlaps(b.x, b.y, b.w, b.h, e.x, e.y, e.w, e.h)) {
                        try self.addScore(100, e.x, e.y);
                        try self.spawnParticles(b.x, b.y, 5, 255, 0, 255);
                        _ = self.enemies.swapRemove(j);
                        hit_enemy = true;
                        self.shake_mag = @max(self.shake_mag, 2);
                        break;
                    }
                    j += 1;
                }

                const off_screen = b.x > 640.0 + 50 or b.x < -50 or
                    b.y < self.camera_y - 100 or b.y > self.camera_y + 480.0 + 100;

                if (hit_wall or hit_enemy or off_screen) {
                    _ = self.bullets.swapRemove(i);
                } else {
                    i += 1;
                }
            }
        }

        // --- ENEMIES ---
        {
            var i: usize = 0;
            while (i < self.enemies.items.len) {
                // Update enemy
                switch (self.enemies.items[i].kind) {
                    .Patrol => {
                        self.enemies.items[i].x += self.enemies.items[i].vx;
                        const e = &self.enemies.items[i];
                        if (e.x > e.start_x + e.range or e.x < e.start_x - e.range) {
                            e.vx *= -1;
                        }
                    },
                    .Bat => {
                        const e = &self.enemies.items[i];
                        const dx = self.player.x - e.x;
                        const dy = self.player.y - e.y;
                        const dist = @sqrt(dx * dx + dy * dy);
                        if (dist > 0.001 and dist < 250) {
                            e.x += (dx / dist) * 1.2;
                            e.y += (dy / dist) * 1.2;
                        }
                    },
                    .Turret => {
                        const e = &self.enemies.items[i];
                        e.shoot_timer += 1;
                        if (e.shoot_timer > 90) {
                            const dx = self.player.x - e.x;
                            const dy = self.player.y - e.y;
                            const dist = @sqrt(dx * dx + dy * dy);
                            if (dist > 0.001 and dist < 300) {
                                try self.enemy_bullets.append(self.allocator, .{
                                    .x = e.x + e.w / 2 - 3,
                                    .y = e.y + e.h / 2 - 3,
                                    .vx = (dx / dist) * ENEMY_BULLET_SPEED,
                                    .vy = (dy / dist) * ENEMY_BULLET_SPEED,
                                });
                            }
                            e.shoot_timer = 0;
                        }
                    },
                }

                // Check player collision
                const e = self.enemies.items[i];
                if (overlaps(self.player.x, self.player.y, self.player.w, self.player.h,
                    e.x, e.y, e.w, e.h))
                {
                    const stomp = self.player.vy > 0 and
                        self.player.y + self.player.h < e.y + e.h / 2 + 5 and
                        e.kind != .Turret;
                    if (stomp) {
                        self.player.vy = BOUNCE_FORCE;
                        self.player.jumps = 1;
                        try self.addScore(100, e.x, e.y);
                        try self.spawnParticles(e.x + 8, e.y + 8, 5, 255, 0, 255);
                        self.shake_mag = @max(self.shake_mag, 2);
                        _ = self.enemies.swapRemove(i);
                    } else {
                        try self.loseLife();
                        return;
                    }
                } else {
                    i += 1;
                }
            }
        }

        // --- ENEMY BULLETS ---
        {
            var i: usize = 0;
            while (i < self.enemy_bullets.items.len) {
                var b = &self.enemy_bullets.items[i];
                b.x += b.vx;
                b.y += b.vy;

                var hit_wall = false;
                for (self.platforms.items) |p| {
                    if (p.kind == .Stone or p.kind == .Chest) continue;
                    if (overlaps(b.x, b.y, b.w, b.h, p.x, p.y, p.w, p.h)) {
                        hit_wall = true;
                        break;
                    }
                }

                if (overlaps(b.x, b.y, b.w, b.h,
                    self.player.x, self.player.y, self.player.w, self.player.h))
                {
                    _ = self.enemy_bullets.swapRemove(i);
                    try self.loseLife();
                    return;
                }

                const off_screen = b.x > 640.0 + 50 or b.x < -50 or
                    b.y < self.camera_y - 100 or b.y > self.camera_y + 480.0 + 100;

                if (hit_wall or off_screen) {
                    _ = self.enemy_bullets.swapRemove(i);
                } else {
                    i += 1;
                }
            }
        }

        // --- LAVA ---
        self.lava_y -= self.lava_speed;
        if (self.player.y + self.player.h > self.lava_y) {
            try self.loseLife();
            return;
        }

        // --- GOAL CHECK ---
        const lv = &LEVELS[self.current_level];
        const map_h: i32 = @intCast(lv.map.len);
        const start_y: f32 = @as(f32, @floatFromInt(-map_h * TILE + GAME_H));
        const goal_x: f32 = 283;
        const goal_y: f32 = start_y + @as(f32, @floatFromInt(2 * TILE)) + 2;
        if (overlaps(self.player.x, self.player.y, self.player.w, self.player.h,
            goal_x, goal_y, 22, 22))
        {
            const just_finished = self.current_level;
            self.current_level += 1;
            if (self.current_level >= 3) {
                self.startStory(&STORY_VICTORY, true);
            } else if (just_finished == 0) {
                self.startStory(&STORY_AFTER1, false);
            } else if (just_finished == 1) {
                self.startStory(&STORY_AFTER2, false);
            } else {
                try self.loadLevel();
                self.state = .Playing;
            }
            return;
        }

        // --- SCREEN SHAKE ---
        if (self.shake_mag > 0.1) {
            self.shake_x = (randF() - 0.5) * 2 * self.shake_mag;
            self.shake_y = (randF() - 0.5) * 2 * self.shake_mag;
            self.shake_mag *= 0.85;
        } else {
            self.shake_x = 0;
            self.shake_y = 0;
            self.shake_mag = 0;
        }

        if (self.damage_flash > 0) self.damage_flash -= 1;

        // --- LAVA BUBBLES ---
        if (randF() < 0.3 and self.bubbles.items.len < MAX_BUBBLES) {
            try self.bubbles.append(self.allocator, .{
                .x = randRange(0, 640.0),
                .y = self.lava_y,
                .radius = randRange(2, 6),
                .life = randRange(20, 50),
                .speed = randRange(0.5, 2.0),
            });
        }
        {
            var i: usize = 0;
            while (i < self.bubbles.items.len) {
                var lb = &self.bubbles.items[i];
                lb.y -= lb.speed;
                lb.life -= 1;
                if (lb.life <= 0) {
                    _ = self.bubbles.swapRemove(i);
                } else {
                    i += 1;
                }
            }
        }

        // --- CAMERA ---
        const target_cam = self.player.y - 480.0 * 0.6;
        self.camera_y += (target_cam - self.camera_y) * 0.1;
        const max_cam = self.lava_y - 480.0 + 100;
        if (self.camera_y > max_cam) self.camera_y = max_cam;
    }

    // -----------------------------------------------------------------------
    // Draw helpers
    // -----------------------------------------------------------------------

    fn drawTexScaled(self: *Game, tex: *c.SDL_Texture, x: f32, y: f32, w: f32, h: f32) void {
        const dst = c.SDL_Rect{
            .x = @as(c_int, @intFromFloat(x)),
            .y = @as(c_int, @intFromFloat(y)),
            .w = @as(c_int, @intFromFloat(w)),
            .h = @as(c_int, @intFromFloat(h)),
        };
        _ = c.SDL_RenderCopy(self.renderer, tex, null, &dst);
    }

    fn drawTexScaledFlip(self: *Game, tex: *c.SDL_Texture, x: f32, y: f32, w: f32, h: f32, flip: bool) void {
        const dst = c.SDL_Rect{
            .x = @as(c_int, @intFromFloat(x)),
            .y = @as(c_int, @intFromFloat(y)),
            .w = @as(c_int, @intFromFloat(w)),
            .h = @as(c_int, @intFromFloat(h)),
        };
        const f: c.SDL_RendererFlip = if (flip) c.SDL_FLIP_HORIZONTAL else c.SDL_FLIP_NONE;
        _ = c.SDL_RenderCopyEx(self.renderer, tex, null, &dst, 0, null, f);
    }

    fn drawTexRotated(self: *Game, tex: *c.SDL_Texture, x: f32, y: f32, w: f32, h: f32, angle: f64) void {
        const dst = c.SDL_Rect{
            .x = @as(c_int, @intFromFloat(x)),
            .y = @as(c_int, @intFromFloat(y)),
            .w = @as(c_int, @intFromFloat(w)),
            .h = @as(c_int, @intFromFloat(h)),
        };
        _ = c.SDL_RenderCopyEx(self.renderer, tex, null, &dst, angle, null, c.SDL_FLIP_NONE);
    }

    fn drawFontStr(self: *Game, text: []const u8, x: i32, y: i32, scale: i32, r: u8, g_: u8, b_: u8) void {
        font.drawText(self.renderer, text, x, y, scale, r, g_, b_, 255);
    }

    fn drawFontStrA(self: *Game, text: []const u8, x: i32, y: i32, scale: i32, r: u8, g_: u8, b_: u8, a: u8) void {
        font.drawText(self.renderer, text, x, y, scale, r, g_, b_, a);
    }

    fn textWidth(text: []const u8, scale: i32) i32 {
        return font.measureText(text, scale);
    }

    fn drawCentered(self: *Game, text: []const u8, y: i32, scale: i32, r: u8, g_: u8, b_: u8) void {
        const w = textWidth(text, scale);
        const x = @divTrunc(GAME_W - w, 2);
        self.drawFontStr(text, x, y, scale, r, g_, b_);
    }

    fn drawCenteredA(self: *Game, text: []const u8, y: i32, scale: i32, r: u8, g_: u8, b_: u8, a: u8) void {
        const w = textWidth(text, scale);
        const x = @divTrunc(GAME_W - w, 2);
        self.drawFontStrA(text, x, y, scale, r, g_, b_, a);
    }

    // -----------------------------------------------------------------------
    // Draw
    // -----------------------------------------------------------------------

    fn draw(self: *Game) void {
        // Background
        sdl.clear(self.renderer, 15, 15, 25, 255);

        switch (self.state) {
            .Start => self.drawStart(),
            .LevelStory => self.drawStory(),
            .Playing => self.drawPlaying(),
            .GameOver => self.drawGameOver(),
            .Win => self.drawWin(),
        }

        // CRT scanlines — stretch a 1xH texture across full width
        const scan_dst = c.SDL_Rect{ .x = 0, .y = 0, .w = GAME_W, .h = GAME_H };
        _ = c.SDL_RenderCopy(self.renderer, self.tex_scanline, null, &scan_dst);

        // Vignette (4 dark edges)
        _ = c.SDL_SetRenderDrawBlendMode(self.renderer, c.SDL_BLENDMODE_BLEND);
        _ = c.SDL_SetRenderDrawColor(self.renderer, 0, 0, 0, 80);
        _ = c.SDL_RenderFillRect(self.renderer, &c.SDL_Rect{ .x = 0, .y = 0, .w = GAME_W, .h = 40 });
        _ = c.SDL_RenderFillRect(self.renderer, &c.SDL_Rect{ .x = 0, .y = GAME_H - 40, .w = GAME_W, .h = 40 });
        _ = c.SDL_RenderFillRect(self.renderer, &c.SDL_Rect{ .x = 0, .y = 0, .w = 40, .h = GAME_H });
        _ = c.SDL_RenderFillRect(self.renderer, &c.SDL_Rect{ .x = GAME_W - 40, .y = 0, .w = 40, .h = GAME_H });

        sdl.present(self.renderer);
    }

    fn drawStart(self: *Game) void {
        // Title embers
        for (0..80) |idx| {
            const e = self.embers[idx];
            if (!e.active) continue;
            const alpha: u8 = @intFromFloat(@min(255.0, e.life / 40.0 * 255.0));
            const colors: [3][3]u8 = .{
                .{ 255, 160, 0 },
                .{ 255, 200, 50 },
                .{ 255, 100, 0 },
            };
            const col = colors[idx % 3];
            sdl.fillRect(self.renderer,
                @as(c_int, @intFromFloat(e.x)),
                @as(c_int, @intFromFloat(e.y)),
                @as(c_int, @intFromFloat(e.size)),
                @as(c_int, @intFromFloat(e.size)),
                col[0], col[1], col[2], alpha);
        }

        // Title
        self.drawCentered("NANO WIZARDS", GAME_H / 2 - 60, 2, 255, 200, 100);
        self.drawCentered("The Obsidian Spire", GAME_H / 2 - 30, 1, 200, 160, 255);
        self.drawCentered("The last Nano Wizard ascends", GAME_H / 2, 1, 160, 140, 200);

        // Blink prompt (every 30 frames)
        if ((self.frame / 30) % 2 == 0) {
            self.drawCentered("Press A/B to Begin", GAME_H / 2 + 40, 1, 255, 170, 0);
        }
    }

    fn drawStory(self: *Game) void {
        // Dark overlay
        _ = c.SDL_SetRenderDrawColor(self.renderer, 0, 0, 0, 235);
        _ = c.SDL_SetRenderDrawBlendMode(self.renderer, c.SDL_BLENDMODE_BLEND);
        _ = c.SDL_RenderFillRect(self.renderer, &c.SDL_Rect{ .x = 0, .y = 0, .w = GAME_W, .h = GAME_H });

        const text_slice = self.story_text_buf[0..self.story_text_len];
        const line_h: i32 = 16;

        // Count lines
        var line_count: i32 = 1;
        for (text_slice) |ch| {
            if (ch == '\n') line_count += 1;
        }
        const total_h = line_count * line_h;
        var cy: i32 = @divTrunc(GAME_H - total_h, 2) - 30;
        var line_start: usize = 0;
        var idx: usize = 0;
        while (idx <= text_slice.len) : (idx += 1) {
            const at_end = idx == text_slice.len;
            const is_newline = !at_end and text_slice[idx] == '\n';
            if (is_newline or at_end) {
                const line = text_slice[line_start..idx];
                if (line.len > 0) {
                    self.drawCenteredA(line, cy, 1, 200, 180, 255, 255);
                }
                cy += line_h;
                line_start = idx + 1;
            }
        }

        // Prompt / cursor
        if (self.story_line_idx < self.story_lines.len) {
            if ((self.frame / 18) % 2 == 0) {
                self.drawCentered("_", cy - line_h + 2, 1, 255, 170, 0);
            }
        } else {
            if ((self.frame / 30) % 2 == 0) {
                self.drawCentered("Press A/B to continue...", GAME_H - 50, 1, 255, 170, 0);
            }
        }
    }

    fn drawPlaying(self: *Game) void {
        const cam = self.camera_y;
        const sx: i32 = @intFromFloat(self.shake_x);
        const sy: i32 = @intFromFloat(self.shake_y);

        // Apply shake via render offset (translate the viewport)
        _ = c.SDL_RenderSetViewport(self.renderer, &c.SDL_Rect{
            .x = @intCast(-sx), .y = @intCast(-sy), .w = GAME_W, .h = GAME_H,
        });

        // Platforms
        for (self.platforms.items) |p| {
            const py_screen = p.y - cam;
            if (py_screen < -20.0 or py_screen > 500.0) continue;
            const tex = switch (p.kind) {
                .Brick => self.tex_brick,
                .Stone => self.tex_stone,
                .Chest => self.tex_chest,
            };
            self.drawTexScaled(tex, p.x, py_screen, p.w, p.h);
        }

        // Particles
        for (self.particles.items) |p| {
            const alpha: u8 = @intFromFloat(@min(255.0, (p.life / 25.0) * 255.0));
            sdl.fillRect(self.renderer,
                @as(c_int, @intFromFloat(p.x)),
                @as(c_int, @intFromFloat(p.y - cam)),
                3, 3, p.r, p.g, p.b, alpha);
        }

        // Gems
        for (self.gems.items) |g| {
            self.drawTexScaled(self.tex_gem, g.x, g.y - cam, g.w, g.h);
        }

        // Player bullets
        for (self.bullets.items) |b| {
            const angle_rad = std.math.atan2(b.vy, b.vx);
            const angle_deg: f64 = @as(f64, @floatCast(angle_rad)) * (180.0 / std.math.pi);
            self.drawTexRotated(self.tex_bullet,
                b.x, b.y - cam, b.w, b.h,
                angle_deg);
        }

        // Enemy bullets
        for (self.enemy_bullets.items) |b| {
            const eb_angle_rad = std.math.atan2(b.vy, b.vx);
            const eb_angle_deg: f64 = @as(f64, @floatCast(eb_angle_rad)) * (180.0 / std.math.pi);
            self.drawTexRotated(self.tex_enemy_bullet,
                b.x, b.y - cam, b.w, b.h,
                eb_angle_deg);
        }

        // Enemies
        for (self.enemies.items) |e| {
            const py_screen = e.y - cam;
            const tex = switch (e.kind) {
                .Patrol => self.tex_patrol,
                .Bat => self.tex_bat,
                .Turret => self.tex_turret,
            };
            const flip = switch (e.kind) {
                .Patrol => e.vx > 0,
                .Bat => self.player.x > e.x,
                .Turret => false,
            };
            self.drawTexScaledFlip(tex, e.x, py_screen, e.w, e.h, flip);
        }

        // Anchor rope + head
        if (self.anchor.active or self.anchor.attached) {
            const px = @as(c_int, @intFromFloat(self.player.x + self.player.w / 2));
            const py = @as(c_int, @intFromFloat(self.player.y + self.player.h / 2 - cam));
            const ax = @as(c_int, @intFromFloat(self.anchor.x + self.anchor.w / 2));
            const ay = @as(c_int, @intFromFloat(self.anchor.y + self.anchor.h / 2 - cam));
            sdl.drawLine(self.renderer, px, py, ax, ay, 255, 255, 255, 128);
            self.drawTexScaled(self.tex_anchor, self.anchor.x, self.anchor.y - cam, self.anchor.w, self.anchor.h);
        }

        // Player
        const flip_player = !self.player.facing_right;
        self.drawTexScaledFlip(self.tex_mage,
            self.player.x, self.player.y - cam,
            self.player.w, self.player.h,
            flip_player);

        // Goal portal
        {
            const lv = &LEVELS[self.current_level];
            const map_h: i32 = @intCast(lv.map.len);
            const start_y: f32 = @as(f32, @floatFromInt(-map_h * TILE + GAME_H));
            const gy: f32 = start_y + @as(f32, @floatFromInt(2 * TILE)) + 2 - cam;
            self.drawTexScaled(self.tex_goal, 283, gy, 22, 22);
        }

        // Lava — simple filled rect with gradient via SDL (no gradient API, use color bands)
        {
            const lava_screen = @as(c_int, @intFromFloat(self.lava_y - cam));
            if (lava_screen < @as(c_int, GAME_H)) {
                // Orange top
                _ = c.SDL_SetRenderDrawColor(self.renderer, 255, 160, 0, 242);
                _ = c.SDL_RenderFillRect(self.renderer, &c.SDL_Rect{
                    .x = 0, .y = lava_screen, .w = GAME_W, .h = 15,
                });
                // Red mid
                _ = c.SDL_SetRenderDrawColor(self.renderer, 220, 60, 0, 230);
                _ = c.SDL_RenderFillRect(self.renderer, &c.SDL_Rect{
                    .x = 0, .y = lava_screen + 15, .w = GAME_W, .h = 30,
                });
                // Dark red bottom
                _ = c.SDL_SetRenderDrawColor(self.renderer, 120, 10, 0, 218);
                _ = c.SDL_RenderFillRect(self.renderer, &c.SDL_Rect{
                    .x = 0, .y = lava_screen + 45, .w = GAME_W, .h = GAME_H,
                });
                // Bright edge
                _ = c.SDL_SetRenderDrawColor(self.renderer, 255, 255, 100, 178);
                _ = c.SDL_RenderFillRect(self.renderer, &c.SDL_Rect{
                    .x = 0, .y = lava_screen, .w = GAME_W, .h = 3,
                });
            }

            // Lava bubbles
            for (self.bubbles.items) |lb| {
                const alpha: u8 = @intFromFloat(@min(255.0, (lb.life / 50.0) * 255.0));
                sdl.fillCircle(self.renderer,
                    @as(c_int, @intFromFloat(lb.x)),
                    @as(c_int, @intFromFloat(lb.y - cam)),
                    @as(c_int, @intFromFloat(lb.radius)),
                    255, 200, 50, alpha);
            }
        }

        // Reset viewport
        _ = c.SDL_RenderSetViewport(self.renderer, null);

        // Damage flash (full screen, no shake)
        if (self.damage_flash > 0) {
            const alpha: u8 = @intFromFloat(@as(f32, @floatFromInt(self.damage_flash)) / 6.0 * 76.0);
            _ = c.SDL_SetRenderDrawColor(self.renderer, 200, 0, 0, alpha);
            _ = c.SDL_SetRenderDrawBlendMode(self.renderer, c.SDL_BLENDMODE_BLEND);
            _ = c.SDL_RenderFillRect(self.renderer, &c.SDL_Rect{ .x = 0, .y = 0, .w = GAME_W, .h = GAME_H });
        }

        // Popups
        for (self.popups.items) |p| {
            var buf: [16]u8 = undefined;
            const s = std.fmt.bufPrint(&buf, "+{d}", .{p.amount}) catch continue;
            const alpha: u8 = @intFromFloat(@min(255.0, (p.life / 40.0) * 255.0));
            self.drawFontStrA(s,
                @as(i32, @intFromFloat(p.x + 10)),
                @as(i32, @intFromFloat(p.y - self.camera_y)),
                1, 255, 255, 0, alpha);
        }

        // HUD
        {
            var buf: [64]u8 = undefined;
            const score_str = std.fmt.bufPrint(&buf, "SCORE: {d}", .{self.score}) catch "SCORE: ?";
            self.drawFontStr(score_str, 20, 16, 1, 255, 255, 255);
            const lives_str = std.fmt.bufPrint(&buf, "LIVES: {d}", .{self.lives}) catch "LIVES: ?";
            self.drawFontStr(lives_str, 30, 16, 1, 255, 255, 255);
            const lv_name = LEVELS[self.current_level].name;
            const nw = textWidth(lv_name, 1);
            self.drawFontStr(lv_name, GAME_W - nw - 20, 16, 1, 200, 200, 255);
        }
    }

    fn drawGameOver(self: *Game) void {
        _ = c.SDL_SetRenderDrawColor(self.renderer, 200, 0, 0, 128);
        _ = c.SDL_SetRenderDrawBlendMode(self.renderer, c.SDL_BLENDMODE_BLEND);
        _ = c.SDL_RenderFillRect(self.renderer, &c.SDL_Rect{ .x = 0, .y = 0, .w = GAME_W, .h = GAME_H });

        self.drawCentered("THE SPIRE CLAIMS YOU", GAME_H / 2 - 40, 2, 255, 255, 255);
        self.drawCentered("The corruption swallows another soul...", GAME_H / 2, 1, 200, 180, 255);

        var buf: [32]u8 = undefined;
        const score_str = std.fmt.bufPrint(&buf, "SCORE: {d}", .{self.score}) catch "SCORE: ?";
        self.drawCentered(score_str, GAME_H / 2 + 30, 1, 255, 255, 255);
        self.drawCentered("Press A/B to Retry", GAME_H / 2 + 60, 1, 255, 255, 255);
    }

    fn drawWin(self: *Game) void {
        _ = c.SDL_SetRenderDrawColor(self.renderer, 20, 0, 40, 218);
        _ = c.SDL_SetRenderDrawBlendMode(self.renderer, c.SDL_BLENDMODE_BLEND);
        _ = c.SDL_RenderFillRect(self.renderer, &c.SDL_Rect{ .x = 0, .y = 0, .w = GAME_W, .h = GAME_H });

        self.drawCentered("FREE AT LAST", GAME_H / 2 - 70, 2, 255, 255, 255);
        self.drawCentered("The Obsidian Spire crumbles into dust.", GAME_H / 2 - 20, 1, 200, 180, 255);
        self.drawCentered("Vael walks into the dawn, finally free.", GAME_H / 2, 1, 200, 180, 255);

        var buf: [32]u8 = undefined;
        const score_str = std.fmt.bufPrint(&buf, "FINAL SCORE: {d}", .{self.score}) catch "FINAL SCORE: ?";
        self.drawCentered(score_str, GAME_H / 2 + 40, 1, 255, 255, 255);
        self.drawCentered("Press A/B to Play Again", GAME_H / 2 + 70, 1, 255, 170, 0);
    }
};

// ---------------------------------------------------------------------------
// SDL event handling
// ---------------------------------------------------------------------------

fn handleEvent(ev: c.SDL_Event, input: *Input, jump_buffer: *i32) bool {
    switch (ev.type) {
        c.SDL_QUIT => return false,
        c.SDL_KEYDOWN => {
            switch (ev.key.keysym.sym) {
                c.SDLK_LEFT => input.left = true,
                c.SDLK_RIGHT => input.right = true,
                c.SDLK_UP => input.up = true,
                c.SDLK_DOWN => input.down = true,
                // A button = KeyCode::X  (shoot)
                c.SDLK_x => { input.shoot_pressed = true; },
                // B button = KeyCode::Space (jump) or Enter (start)
                c.SDLK_SPACE, c.SDLK_z, c.SDLK_k => {
                    if (!input.jump_down) {
                        input.jump_down = true;
                        input.jump_pressed = true;
                        jump_buffer.* = 6;
                    }
                },
                c.SDLK_RETURN => {
                    // Start = Enter
                    input.jump_pressed = true;
                    jump_buffer.* = 6;
                },
                c.SDLK_ESCAPE => return false,
                else => {},
            }
        },
        c.SDL_KEYUP => {
            switch (ev.key.keysym.sym) {
                c.SDLK_LEFT => input.left = false,
                c.SDLK_RIGHT => input.right = false,
                c.SDLK_UP => input.up = false,
                c.SDLK_DOWN => input.down = false,
                c.SDLK_SPACE, c.SDLK_z, c.SDLK_k => input.jump_down = false,
                else => {},
            }
        },
        else => {},
    }
    return true;
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const sdl_ctx = try sdl.init("Nano Wizards - The Obsidian Spire", GAME_W, GAME_H);
    defer sdl.quit(sdl_ctx.window, sdl_ctx.renderer);

    var game = try Game.init(sdl_ctx.renderer, allocator);
    defer game.deinit();

    var last_ticks: u32 = sdl.getTicks();
    var accumulator: u32 = 0;

    var running = true;
    while (running) {
        var ev: c.SDL_Event = undefined;
        while (c.SDL_PollEvent(&ev) != 0) {
            if (!handleEvent(ev, &game.input, &game.jump_buffer)) {
                running = false;
            }
        }

        const now = sdl.getTicks();
        var dt = now - last_ticks;
        last_ticks = now;
        if (dt > 250) dt = 250;
        accumulator += dt;

        while (accumulator >= TARGET_MS) {
            try game.update();
            accumulator -= TARGET_MS;
        }

        game.draw();

        // Frame cap
        const elapsed = sdl.getTicks() - now;
        if (elapsed < TARGET_MS) {
            sdl.delay(TARGET_MS - elapsed);
        }
    }
}
