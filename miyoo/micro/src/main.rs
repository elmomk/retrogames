use macroquad::prelude::*;

// ── CONSTANTS ──────────────────────────────────────────────────────────────
const GRAVITY: f32 = 0.35;
const MAX_FALL_SPEED: f32 = 7.0;
const WALL_SLIDE_SPEED: f32 = 1.5;
const MOVE_SPEED: f32 = 3.5;
const JUMP_FORCE: f32 = -7.0;
const BOUNCE_FORCE: f32 = -6.0;
const WALL_JUMP_Y: f32 = -6.5;
const WALL_JUMP_X: f32 = 6.0;
const BULLET_SPEED: f32 = 10.0;
const ENEMY_BULLET_SPEED: f32 = 4.0;
const ANCHOR_SPEED: f32 = 15.0;
const CLIMB_SPEED: f32 = 3.0;
const TILE_SIZE: f32 = 20.0;
const CANVAS_WIDTH: f32 = 640.0;
const CANVAS_HEIGHT: f32 = 480.0;
const TIME_STEP: f64 = 1.0 / 60.0;

// ── SPRITE DATA ────────────────────────────────────────────────────────────
const MAGE_ART: [&str; 8] = [
    "..1111..", ".122221.", "13122131", "13322331",
    ".122221.", "..1111..", ".121121.", "12211221",
];
const BRICK_ART: [&str; 8] = [
    "22222221", "33333331", "33333331", "11111111",
    "22212222", "33313333", "33313333", "11111111",
];
const STONE_ART: [&str; 8] = [
    "12222221", "23313332", "23133132", "21333312",
    "23313332", "23133132", "23333332", "12222221",
];
const CHEST_ART: [&str; 8] = [
    "11111111", "12222221", "12122121", "12222221",
    "11111111", "12222221", "12222221", "11111111",
];
const BG_ART: [&str; 8] = [
    "1.......", "......2.", "...1....", "........",
    "......1.", ".2......", "........", "....1...",
];
const BULLET_ART: [&str; 8] = [
    "........", "...11...", "..1221..", ".123321.",
    "..1221..", "...11...", "........", "........",
];
const ENEMY_BULLET_ART: [&str; 8] = [
    "........", "...11...", "..1221..", ".122221.",
    "..1221..", "...11...", "........", "........",
];
const ANCHOR_ART: [&str; 8] = [
    "...11...", "..1221..", ".122221.", "12222221",
    ".122221.", "..1221..", "...11...", "........",
];
const PATROL_ART: [&str; 8] = [
    "..1111..", ".122221.", "12322321", "12222221",
    "12211221", "121..121", "11....11", "........",
];
const BAT_ART: [&str; 8] = [
    "1......1", "11....11", "121..121", ".111111.",
    "..1111..", ".1.11.1.", "1......1", "........",
];
const TURRET_ART: [&str; 8] = [
    "..1111..", ".122221.", ".131131.", "11333311",
    ".122221.", ".122221.", ".111111.", "11111111",
];
const GOAL_ART: [&str; 8] = [
    "...11...", "..1221..", ".123321.", "12333321",
    "12333321", ".123321.", "..1221..", "...11...",
];
const GEM_ART: [&str; 8] = [
    "........", ".111111.", "11222211", "12233221",
    ".122221.", "..1221..", "...11...", "........",
];

fn hex_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    Color::from_rgba(r, g, b, 255)
}

fn create_sprite(art: &[&str], colors: &[Color]) -> Texture2D {
    let width = art[0].len() as u16;
    let height = art.len() as u16;
    let mut img = Image::gen_image_color(width, height, BLANK);
    for (y, row) in art.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch != '.' {
                if let Some(digit) = ch.to_digit(10) {
                    let idx = (digit - 1) as usize;
                    if idx < colors.len() {
                        img.set_pixel(x as u32, y as u32, colors[idx]);
                    }
                }
            }
        }
    }
    let tex = Texture2D::from_image(&img);
    tex.set_filter(FilterMode::Nearest);
    tex
}

// ── DATA STRUCTURES ────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Start,
    Story,
    Playing,
    GameOver,
    Win,
}

#[derive(Clone, Copy, PartialEq)]
enum PlatformType {
    Brick,
    Stone,
    Chest,
}

struct Platform {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    p_type: PlatformType,
}

#[derive(Clone, Copy, PartialEq)]
enum EnemyType {
    Patrol,
    Bat,
    Turret,
}

struct Enemy {
    e_type: EnemyType,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    vx: f32,
    start_x: f32,
    range: f32,
    shoot_timer: f32,
}

struct Projectile {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    vx: f32,
    vy: f32,
}

struct Gem {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    vx: f32,
    vy: f32,
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    color: Color,
}

struct Popup {
    text: String,
    x: f32,
    y: f32,
    life: f32,
}

struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    vx: f32,
    vy: f32,
    on_ground: bool,
    wall_dir: i32,
    facing_right: bool,
    jumps: u32,
    max_jumps: u32,
}

struct Anchor {
    active: bool,
    is_attached: bool,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    w: f32,
    h: f32,
    length: f32,
}

struct LevelText {
    col: usize,
    row: usize,
    text: &'static str,
}

struct LevelDef {
    lava_speed: f32,
    map: &'static [&'static str],
    texts: &'static [LevelText],
}

struct InputState {
    right: bool,
    left: bool,
    up: bool,
    down: bool,
    shoot_pressed: bool,
    b_down: bool,
    b_time: f64,
    anchor_fired: bool,
}

struct Textures {
    mage: Texture2D,
    brick: Texture2D,
    stone: Texture2D,
    chest: Texture2D,
    bg: Texture2D,
    bullet: Texture2D,
    enemy_bullet: Texture2D,
    anchor: Texture2D,
    patrol: Texture2D,
    bat: Texture2D,
    turret: Texture2D,
    goal: Texture2D,
    gem: Texture2D,
}

// ── LEVEL DEFINITIONS ──────────────────────────────────────────────────────
const LEVEL1_MAP: [&str; 32] = [
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
];

const LEVEL1_TEXTS: [LevelText; 8] = [
    LevelText { col: 2, row: 28, text: "Welcome to the Obsidian Spire." },
    LevelText { col: 12, row: 26, text: "Arrow Keys/D-Pad to Move" },
    LevelText { col: 14, row: 24, text: "Z/Space to Jump" },
    LevelText { col: 4, row: 18, text: "Press Jump again in mid-air to Double Jump" },
    LevelText { col: 6, row: 14, text: "Press X to shoot fireballs" },
    LevelText { col: 2, row: 10, text: "Hold Jump to fire your Anchor and mine Stone (%)" },
    LevelText { col: 2, row: 5, text: "The Anchor attaches to Bricks (#). Swing!" },
    LevelText { col: 16, row: 5, text: "Mine Chests (C) for Gems" },
];

const LEVEL2_MAP: [&str; 32] = [
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
];

const LEVEL3_MAP: [&str; 27] = [
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
];

const LEVEL2_TEXTS: [LevelText; 0] = [];
const LEVEL3_TEXTS: [LevelText; 0] = [];

fn get_levels() -> Vec<LevelDef> {
    vec![
        LevelDef {
            lava_speed: 0.1,
            map: &LEVEL1_MAP,
            texts: &LEVEL1_TEXTS,
        },
        LevelDef {
            lava_speed: 0.3,
            map: &LEVEL2_MAP,
            texts: &LEVEL2_TEXTS,
        },
        LevelDef {
            lava_speed: 0.5,
            map: &LEVEL3_MAP,
            texts: &LEVEL3_TEXTS,
        },
    ]
}

// ── HELPERS ────────────────────────────────────────────────────────────────
fn overlaps(ax: f32, ay: f32, aw: f32, ah: f32, bx: f32, by: f32, bw: f32, bh: f32) -> bool {
    ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
}

fn rand_f32() -> f32 {
    macroquad::rand::gen_range(0.0f32, 1.0f32)
}

// ── GAME WORLD ─────────────────────────────────────────────────────────────
struct Game {
    state: GameState,
    score: i32,
    current_level: usize,
    player: Player,
    anchor: Anchor,
    bullets: Vec<Projectile>,
    enemy_bullets: Vec<Projectile>,
    enemies: Vec<Enemy>,
    gems: Vec<Gem>,
    platforms: Vec<Platform>,
    particles: Vec<Particle>,
    popups: Vec<Popup>,
    lava_y: f32,
    camera_y: f32,
    level_lava_speed: f32,
    coyote_frames: i32,
    jump_buffer_frames: i32,
    keys: InputState,
    start_y: f32,
    map_height: usize,
}

impl Game {
    fn new() -> Self {
        let mut g = Game {
            state: GameState::Start,
            score: 0,
            current_level: 0,
            player: Player {
                x: 300.0, y: 0.0, w: 24.0, h: 24.0,
                vx: 0.0, vy: 0.0,
                on_ground: false, wall_dir: 0,
                facing_right: true, jumps: 0, max_jumps: 2,
            },
            anchor: Anchor {
                active: false, is_attached: false,
                x: 0.0, y: 0.0, vx: 0.0, vy: 0.0,
                w: 16.0, h: 16.0, length: 0.0,
            },
            bullets: Vec::new(),
            enemy_bullets: Vec::new(),
            enemies: Vec::new(),
            gems: Vec::new(),
            platforms: Vec::new(),
            particles: Vec::new(),
            popups: Vec::new(),
            lava_y: 0.0,
            camera_y: 0.0,
            level_lava_speed: 0.3,
            coyote_frames: 0,
            jump_buffer_frames: 0,
            keys: InputState {
                right: false, left: false, up: false, down: false,
                shoot_pressed: false,
                b_down: false, b_time: 0.0, anchor_fired: false,
            },
            start_y: 0.0,
            map_height: 0,
        };
        g.reset_game(true);
        g.state = GameState::Start;
        g
    }

    fn reset_game(&mut self, full_reset: bool) {
        if full_reset {
            self.score = 0;
            self.current_level = 0;
        }

        self.player = Player {
            x: 300.0, y: 0.0, w: 24.0, h: 24.0,
            vx: 0.0, vy: 0.0,
            on_ground: false, wall_dir: 0,
            facing_right: true, jumps: 0, max_jumps: 2,
        };
        self.bullets.clear();
        self.enemy_bullets.clear();
        self.particles.clear();
        self.popups.clear();
        self.gems.clear();
        self.anchor = Anchor {
            active: false, is_attached: false,
            x: 0.0, y: 0.0, vx: 0.0, vy: 0.0,
            w: 16.0, h: 16.0, length: 0.0,
        };
        self.keys.anchor_fired = false;
        self.platforms.clear();
        self.enemies.clear();

        let levels = get_levels();
        let level_data = &levels[self.current_level];
        self.level_lava_speed = level_data.lava_speed;
        let map = level_data.map;
        let map_height = map.len();
        self.map_height = map_height;
        let start_y = -(map_height as f32 * TILE_SIZE) + CANVAS_HEIGHT;
        self.start_y = start_y;

        for (row, line) in map.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let px = col as f32 * TILE_SIZE;
                let py = start_y + row as f32 * TILE_SIZE;
                match ch {
                    '#' => self.platforms.push(Platform {
                        x: px, y: py, w: TILE_SIZE, h: TILE_SIZE,
                        p_type: PlatformType::Brick,
                    }),
                    '%' => self.platforms.push(Platform {
                        x: px, y: py, w: TILE_SIZE, h: TILE_SIZE,
                        p_type: PlatformType::Stone,
                    }),
                    'C' => self.platforms.push(Platform {
                        x: px, y: py, w: TILE_SIZE, h: TILE_SIZE,
                        p_type: PlatformType::Chest,
                    }),
                    'P' => self.enemies.push(Enemy {
                        e_type: EnemyType::Patrol,
                        x: px, y: py, w: TILE_SIZE, h: TILE_SIZE,
                        vx: 1.5,
                        start_x: px, range: 40.0, shoot_timer: 0.0,
                    }),
                    'B' => self.enemies.push(Enemy {
                        e_type: EnemyType::Bat,
                        x: px, y: py, w: TILE_SIZE, h: TILE_SIZE,
                        vx: 0.0,
                        start_x: px, range: 0.0, shoot_timer: 0.0,
                    }),
                    'T' => self.enemies.push(Enemy {
                        e_type: EnemyType::Turret,
                        x: px, y: py, w: TILE_SIZE, h: TILE_SIZE,
                        vx: 0.0,
                        start_x: px, range: 0.0,
                        shoot_timer: rand_f32() * 60.0,
                    }),
                    'G' => self.gems.push(Gem {
                        x: px + 2.0, y: py + 2.0,
                        w: 16.0, h: 16.0, vx: 0.0, vy: 0.0,
                    }),
                    _ => {}
                }
            }
        }

        self.player.y = start_y + map_height as f32 * TILE_SIZE - 60.0;
        self.lava_y = self.player.y + 400.0;
        self.camera_y = self.player.y - 200.0;
        self.coyote_frames = 0;
        self.jump_buffer_frames = 0;
        self.state = GameState::Playing;
    }

    fn add_score(&mut self, amount: i32, x: f32, y: f32) {
        self.score += amount;
        self.popups.push(Popup {
            text: format!("+{}", amount),
            x, y, life: 40.0,
        });
    }

    fn get_trajectory(&self, speed: f32) -> (f32, f32) {
        let k = &self.keys;
        if k.up && k.right {
            (speed, -speed)
        } else if k.up && k.left {
            (-speed, -speed)
        } else if k.down && k.right {
            (speed, speed)
        } else if k.down && k.left {
            (-speed, speed)
        } else if k.up {
            (0.0, -speed)
        } else if k.down {
            (0.0, speed)
        } else if k.right {
            (speed, 0.0)
        } else if k.left {
            (-speed, 0.0)
        } else {
            (if self.player.facing_right { speed } else { -speed }, 0.0)
        }
    }

    fn update_physics(&mut self) {
        // Coyote time
        if self.player.on_ground {
            self.coyote_frames = 6;
        } else {
            self.coyote_frames -= 1;
        }
        if self.jump_buffer_frames > 0 {
            self.jump_buffer_frames -= 1;
        }

        // ── ANCHOR LOGIC ───────────────────────────────────────────────
        if self.keys.b_down
            && !self.anchor.active
            && !self.keys.anchor_fired
            && (get_time() - self.keys.b_time > 0.150)
        {
            let (tvx, tvy) = self.get_trajectory(ANCHOR_SPEED);
            self.anchor.active = true;
            self.anchor.is_attached = false;
            self.keys.anchor_fired = true;
            self.anchor.x = self.player.x + self.player.w / 2.0 - self.anchor.w / 2.0;
            self.anchor.y = self.player.y + self.player.h / 2.0 - self.anchor.h / 2.0;
            self.anchor.vx = tvx;
            self.anchor.vy = tvy;
        }
        if !self.keys.b_down {
            self.keys.anchor_fired = false;
            if self.anchor.active {
                self.anchor.active = false;
                if self.anchor.is_attached {
                    self.player.vy = JUMP_FORCE * 0.8;
                    self.player.jumps = 1;
                    self.anchor.is_attached = false;
                    self.coyote_frames = 0;
                }
            }
        }

        // Flying anchor
        if self.anchor.active && !self.anchor.is_attached {
            self.anchor.x += self.anchor.vx;
            self.anchor.y += self.anchor.vy;
            let mut deactivate = false;
            let mut i = self.platforms.len();
            while i > 0 {
                i -= 1;
                let p = &self.platforms[i];
                if overlaps(
                    self.anchor.x, self.anchor.y, self.anchor.w, self.anchor.h,
                    p.x, p.y, p.w, p.h,
                ) {
                    if p.p_type == PlatformType::Stone || p.p_type == PlatformType::Chest {
                        if p.p_type == PlatformType::Chest {
                            self.gems.push(Gem {
                                x: p.x + 2.0, y: p.y,
                                w: 16.0, h: 16.0,
                                vx: (rand_f32() - 0.5) * 4.0,
                                vy: -5.0,
                            });
                        }
                        let px = p.x;
                        let py = p.y;
                        let pt = p.p_type;
                        self.platforms.remove(i);
                        let col = if pt == PlatformType::Chest {
                            hex_color("8B4513")
                        } else {
                            hex_color("777777")
                        };
                        for _ in 0..6 {
                            self.particles.push(Particle {
                                x: px + 10.0, y: py + 10.0,
                                vx: (rand_f32() - 0.5) * 8.0,
                                vy: (rand_f32() - 0.5) * 8.0,
                                life: 15.0 + rand_f32() * 15.0,
                                color: col,
                            });
                        }
                        deactivate = true;
                        break;
                    } else {
                        // Brick – attach
                        self.anchor.is_attached = true;
                        let dx = (self.player.x + self.player.w / 2.0) - self.anchor.x;
                        let dy = (self.player.y + self.player.h / 2.0) - self.anchor.y;
                        self.anchor.length = (dx * dx + dy * dy).sqrt();
                        break;
                    }
                }
            }
            if deactivate {
                self.anchor.active = false;
            }
            // Too far check
            if self.anchor.active && !self.anchor.is_attached {
                let dx = (self.player.x + self.player.w / 2.0) - self.anchor.x;
                let dy = (self.player.y + self.player.h / 2.0) - self.anchor.y;
                if (dx * dx + dy * dy).sqrt() > 300.0 {
                    self.anchor.active = false;
                }
            }
        }

        // ── PARTICLES ──────────────────────────────────────────────────
        let mut i = self.particles.len();
        while i > 0 {
            i -= 1;
            self.particles[i].x += self.particles[i].vx;
            self.particles[i].y += self.particles[i].vy;
            self.particles[i].vy += GRAVITY;
            self.particles[i].life -= 1.0;
            if self.particles[i].life <= 0.0 {
                self.particles.remove(i);
            }
        }

        // ── POPUPS ─────────────────────────────────────────────────────
        let mut i = self.popups.len();
        while i > 0 {
            i -= 1;
            self.popups[i].y -= 1.0;
            self.popups[i].life -= 1.0;
            if self.popups[i].life <= 0.0 {
                self.popups.remove(i);
            }
        }

        // ── GEMS ───────────────────────────────────────────────────────
        let px = self.player.x;
        let py = self.player.y;
        let pw = self.player.w;
        let ph = self.player.h;
        let mut gem_scores: Vec<(i32, f32, f32)> = Vec::new();

        let mut i = self.gems.len();
        while i > 0 {
            i -= 1;
            self.gems[i].vy += GRAVITY;
            self.gems[i].x += self.gems[i].vx;
            self.gems[i].y += self.gems[i].vy;

            let gx = self.gems[i].x;
            let gy = self.gems[i].y;
            let gw = self.gems[i].w;
            let gh = self.gems[i].h;
            let gvy = self.gems[i].vy;

            for p in &self.platforms {
                if p.p_type != PlatformType::Stone && p.p_type != PlatformType::Chest
                    && overlaps(gx, gy, gw, gh, p.x, p.y, p.w, p.h)
                {
                    if gvy > 0.0 && gy < p.y {
                        self.gems[i].y = p.y - gh;
                        self.gems[i].vy = -self.gems[i].vy * 0.5;
                        self.gems[i].vx *= 0.8;
                    }
                }
            }

            if overlaps(px, py, pw, ph, self.gems[i].x, self.gems[i].y, gw, gh) {
                gem_scores.push((50, self.gems[i].x, self.gems[i].y));
                self.gems.remove(i);
            }
        }
        for (amt, gx, gy) in gem_scores {
            self.add_score(amt, gx, gy);
        }

        // ── PLAYER MOVEMENT ────────────────────────────────────────────
        let mut target_vx: f32 = 0.0;
        if self.keys.right {
            target_vx = MOVE_SPEED;
            self.player.facing_right = true;
        }
        if self.keys.left {
            target_vx = -MOVE_SPEED;
            self.player.facing_right = false;
        }

        if self.anchor.is_attached {
            self.player.vx += target_vx * 0.05;
            if self.keys.up && self.anchor.length > 20.0 {
                self.anchor.length -= CLIMB_SPEED;
            }
            if self.keys.down && self.anchor.length < 300.0 {
                self.anchor.length += CLIMB_SPEED;
            }
        } else if self.player.on_ground {
            self.player.vx = target_vx;
        } else {
            self.player.vx = self.player.vx * 0.8 + target_vx * 0.2;
        }

        // X movement + collision
        self.player.x += self.player.vx;
        self.player.wall_dir = 0;
        for p in &self.platforms {
            if overlaps(
                self.player.x, self.player.y, self.player.w, self.player.h,
                p.x, p.y, p.w, p.h,
            ) {
                if self.player.vx > 0.0 {
                    self.player.x = p.x - self.player.w;
                    self.player.vx = 0.0;
                    self.player.wall_dir = 1;
                } else if self.player.vx < 0.0 {
                    self.player.x = p.x + p.w;
                    self.player.vx = 0.0;
                    self.player.wall_dir = -1;
                }
            }
        }

        // Horizontal wrapping
        if self.player.x > CANVAS_WIDTH {
            self.player.x = -self.player.w;
        }
        if self.player.x < -self.player.w {
            self.player.x = CANVAS_WIDTH;
        }

        // Gravity
        self.player.vy += GRAVITY;
        if self.player.wall_dir != 0 && self.player.vy > 0.0 && !self.anchor.is_attached {
            if self.player.vy > WALL_SLIDE_SPEED {
                self.player.vy = WALL_SLIDE_SPEED;
            }
        } else if self.player.vy > MAX_FALL_SPEED {
            self.player.vy = MAX_FALL_SPEED;
        }

        // Y movement + collision
        self.player.y += self.player.vy;
        self.player.on_ground = false;
        for p in &self.platforms {
            if overlaps(
                self.player.x, self.player.y, self.player.w, self.player.h,
                p.x, p.y, p.w, p.h,
            ) {
                if self.player.vy > 0.0 {
                    self.player.y = p.y - self.player.h;
                    self.player.vy = 0.0;
                    self.player.on_ground = true;
                    self.player.jumps = 0;
                } else if self.player.vy < 0.0 {
                    self.player.y = p.y + p.h;
                    self.player.vy = 0.0;
                }
            }
        }

        // Anchor Verlet constraint
        if self.anchor.is_attached {
            let dx = (self.player.x + self.player.w / 2.0) - self.anchor.x;
            let dy = (self.player.y + self.player.h / 2.0) - self.anchor.y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist > self.anchor.length {
                let diff = dist - self.anchor.length;
                let nx = dx / dist;
                let ny = dy / dist;
                self.player.x -= nx * diff;
                self.player.y -= ny * diff;
                let dot = self.player.vx * nx + self.player.vy * ny;
                self.player.vx -= dot * nx;
                self.player.vy -= dot * ny;
                self.player.vx *= 0.99;
                self.player.vy *= 0.99;
            }
        }

        // ── JUMP LOGIC ─────────────────────────────────────────────────
        if self.jump_buffer_frames > 0 {
            if self.anchor.is_attached {
                // Skip – anchor overrides jump
            } else if self.coyote_frames > 0 {
                self.player.vy = JUMP_FORCE;
                self.player.jumps = 1;
                self.coyote_frames = 0;
                self.jump_buffer_frames = 0;
            } else if self.player.wall_dir != 0 {
                self.player.vy = WALL_JUMP_Y;
                self.player.vx = -(self.player.wall_dir as f32) * WALL_JUMP_X;
                self.player.facing_right = self.player.wall_dir == -1;
                self.player.jumps = 1;
                self.jump_buffer_frames = 0;
            } else if self.player.jumps < self.player.max_jumps {
                self.player.vy = JUMP_FORCE;
                self.player.jumps += 1;
                self.jump_buffer_frames = 0;
            }
        }

        // ── SHOOTING ───────────────────────────────────────────────────
        if self.keys.shoot_pressed {
            let (tvx, tvy) = self.get_trajectory(BULLET_SPEED);
            self.bullets.push(Projectile {
                x: self.player.x + self.player.w / 2.0 - 4.0,
                y: self.player.y + self.player.h / 2.0 - 4.0,
                vx: tvx, vy: tvy, w: 8.0, h: 8.0,
            });
            self.keys.shoot_pressed = false;
        }

        // Update bullets
        let camera_y = self.camera_y;
        let mut i = self.bullets.len();
        while i > 0 {
            i -= 1;
            self.bullets[i].x += self.bullets[i].vx;
            self.bullets[i].y += self.bullets[i].vy;
            let bx = self.bullets[i].x;
            let by = self.bullets[i].y;
            let bw = self.bullets[i].w;
            let bh = self.bullets[i].h;

            let mut hit = false;
            for p in &self.platforms {
                if p.p_type != PlatformType::Stone && p.p_type != PlatformType::Chest
                    && overlaps(bx, by, bw, bh, p.x, p.y, p.w, p.h)
                {
                    hit = true;
                    break;
                }
            }

            let mut hit_enemy = false;
            let mut j = self.enemies.len();
            while j > 0 {
                j -= 1;
                let ex = self.enemies[j].x;
                let ey = self.enemies[j].y;
                let ew = self.enemies[j].w;
                let eh = self.enemies[j].h;
                if overlaps(bx, by, bw, bh, ex, ey, ew, eh) {
                    let e = self.enemies.remove(j);
                    hit_enemy = true;
                    self.score += 100;
                    self.popups.push(Popup {
                        text: "+100".to_string(),
                        x: e.x, y: e.y, life: 40.0,
                    });
                    for _ in 0..5 {
                        self.particles.push(Particle {
                            x: bx, y: by,
                            vx: (rand_f32() - 0.5) * 6.0,
                            vy: (rand_f32() - 0.5) * 6.0,
                            life: 15.0,
                            color: hex_color("ff00ff"),
                        });
                    }
                    break;
                }
            }

            if hit || hit_enemy
                || bx > CANVAS_WIDTH || bx < -bw
                || by < camera_y - 100.0
                || by > camera_y + CANVAS_HEIGHT + 100.0
            {
                self.bullets.remove(i);
            }
        }

        // ── ENEMY LOGIC ────────────────────────────────────────────────
        let player_x = self.player.x;
        let player_y = self.player.y;
        let player_w = self.player.w;
        let player_h = self.player.h;

        let mut new_enemy_bullets: Vec<Projectile> = Vec::new();
        let mut enemy_scores: Vec<(i32, f32, f32)> = Vec::new();
        let mut player_bounce = false;
        let mut game_over = false;

        let mut i = self.enemies.len();
        while i > 0 {
            i -= 1;
            match self.enemies[i].e_type {
                EnemyType::Patrol => {
                    self.enemies[i].x += self.enemies[i].vx;
                    if self.enemies[i].x > self.enemies[i].start_x + self.enemies[i].range
                        || self.enemies[i].x < self.enemies[i].start_x - self.enemies[i].range
                    {
                        self.enemies[i].vx *= -1.0;
                    }
                }
                EnemyType::Bat => {
                    let dx = player_x - self.enemies[i].x;
                    let dy = player_y - self.enemies[i].y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < 250.0 {
                        self.enemies[i].x += (dx / dist) * 1.2;
                        self.enemies[i].y += (dy / dist) * 1.2;
                    }
                }
                EnemyType::Turret => {
                    self.enemies[i].shoot_timer += 1.0;
                    if self.enemies[i].shoot_timer > 90.0 {
                        let dx = player_x - self.enemies[i].x;
                        let dy = player_y - self.enemies[i].y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist < 300.0 {
                            let ex = self.enemies[i].x + self.enemies[i].w / 2.0 - 4.0;
                            let ey = self.enemies[i].y + self.enemies[i].h / 2.0 - 4.0;
                            new_enemy_bullets.push(Projectile {
                                x: ex, y: ey,
                                vx: (dx / dist) * ENEMY_BULLET_SPEED,
                                vy: (dy / dist) * ENEMY_BULLET_SPEED,
                                w: 8.0, h: 8.0,
                            });
                        }
                        self.enemies[i].shoot_timer = 0.0;
                    }
                }
            }

            let ex = self.enemies[i].x;
            let ey = self.enemies[i].y;
            let ew = self.enemies[i].w;
            let eh = self.enemies[i].h;

            if overlaps(player_x, player_y, player_w, player_h, ex, ey, ew, eh) {
                if self.player.vy > 0.0
                    && player_y + player_h < ey + eh / 2.0 + 5.0
                    && self.enemies[i].e_type != EnemyType::Turret
                {
                    player_bounce = true;
                    let e = self.enemies.remove(i);
                    enemy_scores.push((100, e.x, e.y));
                    for _ in 0..5 {
                        self.particles.push(Particle {
                            x: ex + 10.0, y: ey + 10.0,
                            vx: (rand_f32() - 0.5) * 6.0,
                            vy: (rand_f32() - 0.5) * 6.0,
                            life: 15.0,
                            color: hex_color("ff00ff"),
                        });
                    }
                } else {
                    game_over = true;
                }
            }
        }

        self.enemy_bullets.extend(new_enemy_bullets);

        if player_bounce {
            self.player.vy = BOUNCE_FORCE;
            self.player.jumps = 1;
        }
        for (amt, ex, ey) in enemy_scores {
            self.add_score(amt, ex, ey);
        }
        if game_over {
            self.state = GameState::GameOver;
            return;
        }

        // Update enemy bullets
        let mut i = self.enemy_bullets.len();
        while i > 0 {
            i -= 1;
            self.enemy_bullets[i].x += self.enemy_bullets[i].vx;
            self.enemy_bullets[i].y += self.enemy_bullets[i].vy;
            let bx = self.enemy_bullets[i].x;
            let by = self.enemy_bullets[i].y;
            let bw = self.enemy_bullets[i].w;
            let bh = self.enemy_bullets[i].h;

            let mut hit_wall = false;
            for p in &self.platforms {
                if p.p_type != PlatformType::Stone && p.p_type != PlatformType::Chest
                    && overlaps(bx, by, bw, bh, p.x, p.y, p.w, p.h)
                {
                    hit_wall = true;
                    break;
                }
            }

            if overlaps(bx, by, bw, bh, player_x, player_y, player_w, player_h) {
                self.state = GameState::GameOver;
                return;
            }

            if hit_wall
                || bx > CANVAS_WIDTH || bx < -bw
                || by < camera_y - 100.0
                || by > camera_y + CANVAS_HEIGHT + 100.0
            {
                self.enemy_bullets.remove(i);
            }
        }

        // ── LAVA ───────────────────────────────────────────────────────
        self.lava_y -= self.level_lava_speed;
        if self.player.y + self.player.h > self.lava_y {
            self.state = GameState::GameOver;
            return;
        }

        // ── GOAL COLLISION ─────────────────────────────────────────────
        let levels = get_levels();
        let map_height = levels[self.current_level].map.len();
        let start_y = -(map_height as f32 * TILE_SIZE) + CANVAS_HEIGHT;
        let goal_x = 280.0;
        let goal_y = start_y + 2.0 * TILE_SIZE;
        let goal_w = 30.0;
        let goal_h = 30.0;

        if overlaps(
            self.player.x, self.player.y, self.player.w, self.player.h,
            goal_x, goal_y, goal_w, goal_h,
        ) {
            self.current_level += 1;
            if self.current_level >= levels.len() {
                self.state = GameState::Win;
            } else {
                self.reset_game(false);
            }
            return;
        }

        // ── CAMERA ─────────────────────────────────────────────────────
        let target_camera_y = self.player.y - CANVAS_HEIGHT * 0.6;
        self.camera_y += (target_camera_y - self.camera_y) * 0.1;
        if self.camera_y > self.lava_y - CANVAS_HEIGHT + 100.0 {
            self.camera_y = self.lava_y - CANVAS_HEIGHT + 100.0;
        }
    }

    fn process_input(&mut self) {
        // Direction keys
        self.keys.right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
        self.keys.left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
        self.keys.up = is_key_down(KeyCode::Up) || is_key_down(KeyCode::W);
        self.keys.down = is_key_down(KeyCode::Down) || is_key_down(KeyCode::S);

        // B button (jump/anchor): Space, Z, K
        let b_pressed = is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Z)
            || is_key_pressed(KeyCode::K);
        let b_released = is_key_released(KeyCode::Space)
            || is_key_released(KeyCode::Z)
            || is_key_released(KeyCode::K);

        if b_pressed && !self.keys.b_down {
            self.keys.b_down = true;
            self.keys.b_time = get_time();
            self.jump_buffer_frames = 6;
        }
        if b_released {
            self.keys.b_down = false;
        }

        // A button (shoot): X, J, LeftShift
        if is_key_pressed(KeyCode::X)
            || is_key_pressed(KeyCode::J)
            || is_key_pressed(KeyCode::LeftShift)
        {
            self.keys.shoot_pressed = true;
        }
    }
}

// ── DRAWING ────────────────────────────────────────────────────────────────
fn draw_game(game: &Game, tex: &Textures) {
    // 1. Clear
    clear_background(Color::from_rgba(15, 15, 25, 255));

    let cam = game.camera_y;

    // 2. Background tiles (parallax 0.5)
    let bg_size = 64.0f32;
    let start_y_bg = ((cam * 0.5) / bg_size).floor() as i32 - 1;
    for y in start_y_bg..start_y_bg + 12 {
        for x in 0..12 {
            draw_texture_ex(
                &tex.bg,
                x as f32 * bg_size,
                y as f32 * bg_size - cam * 0.5,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(bg_size, bg_size)),
                    ..Default::default()
                },
            );
        }
    }

    // 3. Tutorial texts
    if game.state == GameState::Playing {
        let levels = get_levels();
        let level = &levels[game.current_level];
        let map_height = level.map.len();
        let start_y = -(map_height as f32 * TILE_SIZE) + CANVAS_HEIGHT;

        for t in level.texts {
            let tx = t.col as f32 * TILE_SIZE;
            let ty = start_y + t.row as f32 * TILE_SIZE - cam;

            // Black outline
            let font_size = 16;
            for ox in [-1.5f32, 0.0, 1.5] {
                for oy in [-1.5f32, 0.0, 1.5] {
                    draw_text(t.text, tx + ox, ty + oy, font_size as f32, BLACK);
                }
            }
            // White fill
            draw_text(t.text, tx, ty, font_size as f32, WHITE);
        }
    }

    // 4. Platforms
    for p in &game.platforms {
        if p.y > cam - TILE_SIZE && p.y < cam + CANVAS_HEIGHT + TILE_SIZE {
            let t = match p.p_type {
                PlatformType::Brick => &tex.brick,
                PlatformType::Stone => &tex.stone,
                PlatformType::Chest => &tex.chest,
            };
            draw_texture_ex(
                t, p.x, p.y - cam, WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
            );
        }
    }

    // 5. Particles
    for p in &game.particles {
        draw_rectangle(p.x, p.y - cam, 4.0, 4.0, p.color);
    }

    // 6. Gems
    for g in &game.gems {
        draw_texture_ex(
            &tex.gem, g.x, g.y - cam, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(g.w, g.h)),
                ..Default::default()
            },
        );
    }

    // 7. Player bullets (rotated)
    for b in &game.bullets {
        let cx = b.x + b.w / 2.0;
        let cy = b.y - cam + b.h / 2.0;
        let angle = b.vy.atan2(b.vx);
        draw_texture_ex(
            &tex.bullet, cx - b.w / 2.0, cy - b.h / 2.0, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(b.w, b.h)),
                rotation: angle,
                pivot: Some(vec2(cx, cy)),
                ..Default::default()
            },
        );
    }

    // 8. Enemy bullets (rotated)
    for b in &game.enemy_bullets {
        let cx = b.x + b.w / 2.0;
        let cy = b.y - cam + b.h / 2.0;
        let angle = b.vy.atan2(b.vx);
        draw_texture_ex(
            &tex.enemy_bullet, cx - b.w / 2.0, cy - b.h / 2.0, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(b.w, b.h)),
                rotation: angle,
                pivot: Some(vec2(cx, cy)),
                ..Default::default()
            },
        );
    }

    // 9. Enemies (flipped)
    for e in &game.enemies {
        let cx = e.x + e.w / 2.0;
        let cy = e.y - cam + e.h / 2.0;
        let t = match e.e_type {
            EnemyType::Patrol => &tex.patrol,
            EnemyType::Bat => &tex.bat,
            EnemyType::Turret => &tex.turret,
        };
        let flip = match e.e_type {
            EnemyType::Bat => game.player.x > e.x,
            _ => e.vx > 0.0,
        };
        draw_texture_ex(
            t, cx - e.w / 2.0, cy - e.h / 2.0, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(e.w, e.h)),
                flip_x: flip,
                ..Default::default()
            },
        );
    }

    // 10. Anchor line + anchor sprite
    if game.anchor.active || game.anchor.is_attached {
        let px = game.player.x + game.player.w / 2.0;
        let py = game.player.y + game.player.h / 2.0 - cam;
        let ax = game.anchor.x + game.anchor.w / 2.0;
        let ay = game.anchor.y + game.anchor.h / 2.0 - cam;
        draw_line(px, py, ax, ay, 2.0, Color::from_rgba(255, 255, 255, 128));
        draw_texture_ex(
            &tex.anchor,
            game.anchor.x, game.anchor.y - cam, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(game.anchor.w, game.anchor.h)),
                ..Default::default()
            },
        );
    }

    // 11. Player (flipped)
    if game.state == GameState::Playing || game.state == GameState::Win {
        let cx = game.player.x + game.player.w / 2.0;
        let cy = game.player.y - cam + game.player.h / 2.0;
        draw_texture_ex(
            &tex.mage,
            cx - game.player.w / 2.0,
            cy - game.player.h / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(game.player.w, game.player.h)),
                flip_x: !game.player.facing_right,
                ..Default::default()
            },
        );
    }

    // 12. Goal portal
    if game.state == GameState::Playing {
        let levels = get_levels();
        let map_height = levels[game.current_level].map.len();
        let start_y = -(map_height as f32 * TILE_SIZE) + CANVAS_HEIGHT;
        draw_texture_ex(
            &tex.goal,
            280.0,
            start_y + 2.0 * TILE_SIZE - cam,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(30.0, 30.0)),
                ..Default::default()
            },
        );
    }

    // 13. Lava
    draw_rectangle(
        0.0,
        game.lava_y - cam,
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        Color::from_rgba(255, 60, 0, 216),
    );
    draw_rectangle(
        0.0,
        game.lava_y - cam,
        CANVAS_WIDTH,
        10.0,
        Color::from_rgba(255, 150, 0, 229),
    );

    // 14. Popups
    for p in &game.popups {
        let alpha = (p.life / 30.0).min(1.0).max(0.0);
        let col = Color::from_rgba(255, 255, 255, (alpha * 255.0) as u8);
        draw_text(&p.text, p.x + 10.0, p.y - cam, 16.0, col);
    }

    // 15. HUD
    if game.state == GameState::Playing {
        let score_text = format!("SCORE: {}", game.score);
        draw_text(&score_text, 20.0, 40.0, 20.0, WHITE);
        let level_text = format!("LEVEL: {}", game.current_level + 1);
        let m = measure_text(&level_text, None, 20, 1.0);
        draw_text(&level_text, CANVAS_WIDTH - 20.0 - m.width, 40.0, 20.0, WHITE);
    }

    // 16. Screen overlays
    match game.state {
        GameState::Start => {
            draw_rectangle(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT, Color::from_rgba(0, 0, 0, 204));
            let title = "THE OBSIDIAN SPIRE";
            let m = measure_text(title, None, 42, 1.0);
            draw_text(title, (CANVAS_WIDTH - m.width) / 2.0, CANVAS_HEIGHT / 2.0 - 20.0, 42.0, WHITE);
            let sub = "Press B/Z/Space to Begin";
            let m2 = measure_text(sub, None, 20, 1.0);
            draw_text(sub, (CANVAS_WIDTH - m2.width) / 2.0, CANVAS_HEIGHT / 2.0 + 30.0, 20.0, WHITE);
        }
        GameState::Story => {
            draw_rectangle(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT, Color::from_rgba(0, 0, 0, 229));
            let lines = [
                "The Archmage has stolen the Kingdom's Mana Gems.",
                "He hides at the top of the volcanic Obsidian Spire.",
                "As a Micro Mage, you must scale the spire,",
                "reclaim the gems, and outrun the rising lava.",
            ];
            let cy = CANVAS_HEIGHT / 2.0;
            for (idx, line) in lines.iter().enumerate() {
                let m = measure_text(line, None, 20, 1.0);
                draw_text(
                    line,
                    (CANVAS_WIDTH - m.width) / 2.0,
                    cy - 60.0 + idx as f32 * 40.0,
                    20.0,
                    WHITE,
                );
            }
            let prompt = "Press B/Z/Space to enter the Spire...";
            let m = measure_text(prompt, None, 20, 1.0);
            draw_text(
                prompt,
                (CANVAS_WIDTH - m.width) / 2.0,
                cy + 130.0,
                20.0,
                Color::from_rgba(255, 170, 0, 255),
            );
        }
        GameState::GameOver => {
            draw_rectangle(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT, Color::from_rgba(200, 0, 0, 128));
            let title = "YOU BURNED";
            let m = measure_text(title, None, 48, 1.0);
            draw_text(title, (CANVAS_WIDTH - m.width) / 2.0, CANVAS_HEIGHT / 2.0 - 20.0, 48.0, WHITE);
            let score_text = format!("FINAL SCORE: {}", game.score);
            let m2 = measure_text(&score_text, None, 24, 1.0);
            draw_text(&score_text, (CANVAS_WIDTH - m2.width) / 2.0, CANVAS_HEIGHT / 2.0 + 30.0, 24.0, WHITE);
            let restart = "Press B/Z to Restart";
            let m3 = measure_text(restart, None, 24, 1.0);
            draw_text(restart, (CANVAS_WIDTH - m3.width) / 2.0, CANVAS_HEIGHT / 2.0 + 70.0, 24.0, WHITE);
        }
        GameState::Win => {
            draw_rectangle(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT, Color::from_rgba(0, 200, 0, 128));
            let title = "YOU ESCAPED THE SPIRE!";
            let m = measure_text(title, None, 48, 1.0);
            draw_text(title, (CANVAS_WIDTH - m.width) / 2.0, CANVAS_HEIGHT / 2.0 - 20.0, 48.0, WHITE);
            let score_text = format!("FINAL SCORE: {}", game.score);
            let m2 = measure_text(&score_text, None, 24, 1.0);
            draw_text(&score_text, (CANVAS_WIDTH - m2.width) / 2.0, CANVAS_HEIGHT / 2.0 + 30.0, 24.0, WHITE);
            let restart = "Press B/Z to Play Again";
            let m3 = measure_text(restart, None, 24, 1.0);
            draw_text(restart, (CANVAS_WIDTH - m3.width) / 2.0, CANVAS_HEIGHT / 2.0 + 70.0, 24.0, WHITE);
        }
        GameState::Playing => {}
    }
}

// ── MAIN ───────────────────────────────────────────────────────────────────
fn window_conf() -> Conf {
    Conf {
        window_title: "Micro".to_owned(),
        window_width: CANVAS_WIDTH as i32,
        window_height: CANVAS_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Create textures
    let textures = Textures {
        mage: create_sprite(&MAGE_ART, &[BLACK, hex_color("00FFFF"), WHITE]),
        brick: create_sprite(
            &BRICK_ART,
            &[
                Color::from_rgba(30, 30, 50, 255),
                Color::from_rgba(100, 100, 140, 255),
                Color::from_rgba(70, 70, 100, 255),
            ],
        ),
        stone: create_sprite(
            &STONE_ART,
            &[hex_color("333333"), hex_color("555555"), hex_color("777777")],
        ),
        chest: create_sprite(&CHEST_ART, &[BLACK, hex_color("8B4513")]),
        bg: create_sprite(
            &BG_ART,
            &[
                Color::from_rgba(40, 40, 60, 255),
                Color::from_rgba(60, 60, 80, 255),
            ],
        ),
        bullet: create_sprite(
            &BULLET_ART,
            &[hex_color("ff0000"), hex_color("ffaa00"), WHITE],
        ),
        enemy_bullet: create_sprite(
            &ENEMY_BULLET_ART,
            &[hex_color("aa00ff"), WHITE],
        ),
        anchor: create_sprite(&ANCHOR_ART, &[WHITE, hex_color("aaaaaa")]),
        patrol: create_sprite(
            &PATROL_ART,
            &[BLACK, hex_color("ff00ff"), WHITE],
        ),
        bat: create_sprite(&BAT_ART, &[BLACK, hex_color("ff2222")]),
        turret: create_sprite(
            &TURRET_ART,
            &[BLACK, hex_color("22ff22"), hex_color("ff0000")],
        ),
        goal: create_sprite(
            &GOAL_ART,
            &[hex_color("ffaa00"), hex_color("ffff00"), WHITE],
        ),
        gem: create_sprite(&GEM_ART, &[BLACK, hex_color("00ffff"), WHITE]),
    };

    let mut game = Game::new();
    let mut accumulator: f64 = 0.0;
    let mut last_time = get_time();

    loop {
        // Input
        game.process_input();

        // Fixed timestep
        let current_time = get_time();
        let mut frame_time = current_time - last_time;
        last_time = current_time;
        if frame_time > 0.25 {
            frame_time = 0.25;
        }
        accumulator += frame_time;

        while accumulator >= TIME_STEP {
            if game.state == GameState::Playing {
                game.update_physics();
            } else if game.jump_buffer_frames > 0 || game.keys.shoot_pressed {
                match game.state {
                    GameState::Start => game.state = GameState::Story,
                    GameState::Story => game.reset_game(true),
                    GameState::GameOver | GameState::Win => {
                        game.reset_game(true);
                    }
                    _ => {}
                }
                game.jump_buffer_frames = 0;
                game.keys.shoot_pressed = false;
            }
            accumulator -= TIME_STEP;
        }

        // Draw
        draw_game(&game, &textures);

        next_frame().await;
    }
}
