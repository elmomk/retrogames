use macroquad::prelude::*;

// ---------------------------------------------------------------------------
// Constants (matched exactly to the JS prototype)
// ---------------------------------------------------------------------------
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
const SCREEN_W: f32 = 640.0;
const SCREEN_H: f32 = 480.0;
const TIME_STEP: f64 = 1.0 / 60.0;
const COYOTE_MAX: i32 = 6;
const JUMP_BUFFER_MAX: i32 = 6;

// ---------------------------------------------------------------------------
// Sprite art data  (8x8, '.' = transparent, digits index into colour palette)
// ---------------------------------------------------------------------------
const MAGE_ART: [&str; 8] = [
    "..1111..",
    ".122221.",
    "13122131",
    "13322331",
    ".122221.",
    "..1111..",
    ".121121.",
    "12211221",
];
const MAGE_COLORS: [Color; 3] = [BLACK, Color::new(0.0, 1.0, 1.0, 1.0), WHITE];

const BRICK_ART: [&str; 8] = [
    "22222221",
    "33333331",
    "33333331",
    "11111111",
    "22212222",
    "33313333",
    "33313333",
    "11111111",
];
const BRICK_COLORS: [Color; 3] = [
    Color::new(0.118, 0.118, 0.196, 1.0),
    Color::new(0.392, 0.392, 0.549, 1.0),
    Color::new(0.275, 0.275, 0.392, 1.0),
];

const STONE_ART: [&str; 8] = [
    "12222221",
    "23313332",
    "23133132",
    "21333312",
    "23313332",
    "23133132",
    "23333332",
    "12222221",
];
const STONE_COLORS: [Color; 3] = [
    Color::new(0.2, 0.2, 0.2, 1.0),
    Color::new(0.333, 0.333, 0.333, 1.0),
    Color::new(0.467, 0.467, 0.467, 1.0),
];

const CHEST_ART: [&str; 8] = [
    "11111111",
    "12222221",
    "12122121",
    "12222221",
    "11111111",
    "12222221",
    "12222221",
    "11111111",
];
const CHEST_COLORS: [Color; 2] = [BLACK, Color::new(0.545, 0.271, 0.075, 1.0)];

const BG_ART: [&str; 8] = [
    "1.......",
    "......2.",
    "...1....",
    "........",
    "......1.",
    ".2......",
    "........",
    "....1...",
];
const BG_COLORS: [Color; 2] = [Color::new(0.157, 0.157, 0.235, 1.0), Color::new(0.235, 0.235, 0.314, 1.0)];

const BULLET_ART: [&str; 8] = [
    "........",
    "...11...",
    "..1221..",
    ".123321.",
    "..1221..",
    "...11...",
    "........",
    "........",
];
const BULLET_COLORS: [Color; 3] = [
    Color::new(1.0, 0.0, 0.0, 1.0),
    Color::new(1.0, 0.667, 0.0, 1.0),
    WHITE,
];

const ENEMY_BULLET_ART: [&str; 8] = [
    "........",
    "...11...",
    "..1221..",
    ".122221.",
    "..1221..",
    "...11...",
    "........",
    "........",
];
const ENEMY_BULLET_COLORS: [Color; 2] = [Color::new(0.667, 0.0, 1.0, 1.0), WHITE];

const ANCHOR_ART: [&str; 8] = [
    "...11...",
    "..1221..",
    ".122221.",
    "12222221",
    ".122221.",
    "..1221..",
    "...11...",
    "........",
];
const ANCHOR_COLORS: [Color; 2] = [WHITE, Color::new(0.667, 0.667, 0.667, 1.0)];

const PATROL_ART: [&str; 8] = [
    "..1111..",
    ".122221.",
    "12322321",
    "12222221",
    "12211221",
    "121..121",
    "11....11",
    "........",
];
const PATROL_COLORS: [Color; 3] = [BLACK, Color::new(1.0, 0.0, 1.0, 1.0), WHITE];

const BAT_ART: [&str; 8] = [
    "1......1",
    "11....11",
    "121..121",
    ".111111.",
    "..1111..",
    ".1.11.1.",
    "1......1",
    "........",
];
const BAT_COLORS: [Color; 2] = [BLACK, Color::new(1.0, 0.133, 0.133, 1.0)];

const TURRET_ART: [&str; 8] = [
    "..1111..",
    ".122221.",
    ".131131.",
    "11333311",
    ".122221.",
    ".122221.",
    ".111111.",
    "11111111",
];
const TURRET_COLORS: [Color; 3] = [BLACK, Color::new(0.133, 1.0, 0.133, 1.0), Color::new(1.0, 0.0, 0.0, 1.0)];

const GOAL_ART: [&str; 8] = [
    "...11...",
    "..1221..",
    ".123321.",
    "12333321",
    "12333321",
    ".123321.",
    "..1221..",
    "...11...",
];
const GOAL_COLORS: [Color; 3] = [
    Color::new(1.0, 0.667, 0.0, 1.0),
    Color::new(1.0, 1.0, 0.0, 1.0),
    WHITE,
];

const GEM_ART: [&str; 8] = [
    "........",
    ".111111.",
    "11222211",
    "12233221",
    ".122221.",
    "..1221..",
    "...11...",
    "........",
];
const GEM_COLORS: [Color; 3] = [BLACK, Color::new(0.0, 1.0, 1.0, 1.0), WHITE];

// ---------------------------------------------------------------------------
// Sprite builder  (string art -> Texture2D)
// ---------------------------------------------------------------------------
fn create_sprite(art: &[&str], colors: &[Color]) -> Texture2D {
    let width = art[0].len() as u16;
    let height = art.len() as u16;
    let mut img = Image::gen_image_color(width, height, BLANK);

    for (y, row) in art.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch != '.' {
                if let Some(digit) = ch.to_digit(10) {
                    let idx = (digit as usize).wrapping_sub(1);
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

// ---------------------------------------------------------------------------
// Data structures
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Start,
    Story,
    LevelStory,
    Playing,
    GameOver,
    Win,
}

#[derive(Clone, Copy, PartialEq)]
enum TileKind {
    Brick,
    Stone,
    Chest,
}

#[derive(Clone, Copy, PartialEq)]
enum EnemyKind {
    Patrol,
    Bat,
    Turret,
}

struct Platform {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    kind: TileKind,
}

struct Enemy {
    kind: EnemyKind,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    vx: f32,
    vy: f32,
    start_x: f32,
    range: f32,
    shoot_timer: f32,
}

struct Bullet {
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
    size: f32,
}

struct DustMote {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    size: f32,
    alpha: f32,
}

struct Popup {
    text: String,
    x: f32,
    y: f32,
    life: f32,
}

struct LavaBubble {
    x: f32,
    y: f32,
    r: f32,
    life: f32,
    speed: f32,
}

struct TitleEmber {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    size: f32,
}

struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    vx: f32,
    vy: f32,
    on_ground: bool,
    wall_dir: i8,
    facing_right: bool,
    jumps: u8,
    max_jumps: u8,
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

// Level text signs shown in tutorial
struct LevelText {
    col: f32,
    row: f32,
    text: &'static str,
    ghost: bool,
}

struct LevelDef {
    name: &'static str,
    lava_speed: f32,
    map: &'static [&'static str],
    texts: &'static [LevelText],
}

// ---------------------------------------------------------------------------
// Story data (matches web version)
// ---------------------------------------------------------------------------
static STORY_INTRO: &[&str] = &[
    "The Obsidian Spire has awakened after a thousand years.",
    "Its corruption spreads across the land --",
    "forests wither, rivers turn black.",
    "The Crystal Mages are gone. All but one.",
    "",
    "You are Vael, the last of your order.",
    "The Elder Council has sent you on a final mission:",
    "ascend the Spire and destroy its heart",
    "before the corruption consumes everything.",
];

static STORY_AFTER_LEVEL_1: &[&str] = &[
    "The walls pulse with a dark rhythm.",
    "You feel it in your chest -- familiar,",
    "like a heartbeat that isn't your own.",
    "",
    "Something inside the Spire recognizes you.",
];

static STORY_AFTER_LEVEL_2: &[&str] = &[
    "The whispers grow louder.",
    "Fragments of memory flash before your eyes --",
    "a child running through these very halls, laughing.",
    "",
    "Your hands begin to glow with the same",
    "dark energy as the walls.",
];

static STORY_VICTORY: &[&str] = &[
    "You reach the heart chamber.",
    "The pulsing crystal at the center is... familiar.",
    "",
    "You place your hand on it",
    "and remember everything.",
    "",
    "You ARE the heart.",
    "",
    "The Crystal Mages didn't send you",
    "to destroy the Spire --",
    "they sent you home, hoping you'd merge back",
    "and end your rebellion.",
    "",
    "But you are Vael now.",
    "You shatter the crystal,",
    "and the Spire crumbles.",
    "",
    "Free at last.",
];

struct Sprites {
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

// ---------------------------------------------------------------------------
// Level definitions (exact mirror of JS)
// ---------------------------------------------------------------------------
static LEVEL_1_MAP: &[&str] = &[
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

static LEVEL_1_TEXTS: &[LevelText] = &[
    LevelText { col: 2.0, row: 28.0, text: "The corruption starts here. Twisted vines choke the stone.", ghost: false },
    LevelText { col: 12.0, row: 26.0, text: "Arrow Keys/D-Pad to Move", ghost: false },
    LevelText { col: 14.0, row: 24.0, text: "Z/Space to Jump", ghost: false },
    LevelText { col: 4.0, row: 18.0, text: "Press Jump again in mid-air to Double Jump", ghost: false },
    LevelText { col: 6.0, row: 14.0, text: "Press X to shoot crystal shards", ghost: false },
    LevelText { col: 2.0, row: 10.0, text: "Hold Jump to fire your Anchor and mine Stone (%)", ghost: false },
    LevelText { col: 2.0, row: 5.0, text: "The Anchor attaches to Bricks (#). Swing!", ghost: false },
    LevelText { col: 16.0, row: 5.0, text: "Mine Chests (C) for Gems", ghost: false },
];

static LEVEL_2_MAP: &[&str] = &[
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

static LEVEL_3_MAP: &[&str] = &[
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

static LEVEL_DEFS: [LevelDef; 3] = [
    LevelDef { name: "THE OVERGROWN DEPTHS", lava_speed: 0.1, map: LEVEL_1_MAP, texts: LEVEL_1_TEXTS },
    LevelDef { name: "THE FROZEN ARCHIVE", lava_speed: 0.3, map: LEVEL_2_MAP, texts: LEVEL_2_TEXTS },
    LevelDef { name: "THE LIVING CORE", lava_speed: 0.5, map: LEVEL_3_MAP, texts: LEVEL_3_TEXTS },
];

static LEVEL_2_TEXTS: &[LevelText] = &[
    LevelText { col: 3.0, row: 29.0, text: "We tried to contain it...", ghost: true },
    LevelText { col: 14.0, row: 22.0, text: "The heart was never destroyed...", ghost: true },
    LevelText { col: 5.0, row: 16.0, text: "it escaped...", ghost: true },
    LevelText { col: 10.0, row: 9.0, text: "It took the form of a child...", ghost: true },
    LevelText { col: 4.0, row: 4.0, text: "The archive remembers what you have forgotten.", ghost: true },
];

static LEVEL_3_TEXTS: &[LevelText] = &[
    LevelText { col: 2.0, row: 25.0, text: "The walls breathe. The Spire is alive.", ghost: true },
    LevelText { col: 6.0, row: 14.0, text: "You feel the pull growing stronger...", ghost: true },
    LevelText { col: 3.0, row: 4.0, text: "TURN BACK, CHILD. YOU CANNOT DESTROY WHAT YOU ARE.", ghost: true },
];

fn get_level(idx: usize) -> &'static LevelDef {
    &LEVEL_DEFS[idx.min(2)]
}

// ---------------------------------------------------------------------------
// AABB overlap
// ---------------------------------------------------------------------------
fn overlaps(ax: f32, ay: f32, aw: f32, ah: f32, bx: f32, by: f32, bw: f32, bh: f32) -> bool {
    ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
}

// ---------------------------------------------------------------------------
// Input state (polled each tick)
// ---------------------------------------------------------------------------
struct Input {
    right: bool,
    left: bool,
    up: bool,
    down: bool,
    jump_buffer: i32,
    shoot_pressed: bool,
    b_down: bool,
    b_down_frames: i32,
    anchor_fired: bool,
}

impl Input {
    fn new() -> Self {
        Self {
            right: false,
            left: false,
            up: false,
            down: false,
            jump_buffer: 0,
            shoot_pressed: false,
            b_down: false,
            b_down_frames: 0,
            anchor_fired: false,
        }
    }

    fn poll(&mut self) {
        self.right = is_key_down(KeyCode::Right);
        self.left = is_key_down(KeyCode::Left);
        self.up = is_key_down(KeyCode::Up);
        self.down = is_key_down(KeyCode::Down);

        // B button (jump / anchor)
        let b_now = is_key_down(KeyCode::Space) || is_key_down(KeyCode::Z);
        if b_now && !self.b_down {
            self.b_down = true;
            self.b_down_frames = 0;
            self.jump_buffer = JUMP_BUFFER_MAX;
        }
        if b_now {
            self.b_down_frames += 1;
        }
        if !b_now && self.b_down {
            self.b_down = false;
        }

        // A button (shoot)
        if is_key_pressed(KeyCode::X) {
            self.shoot_pressed = true;
        }
    }

    fn get_trajectory(&self, speed: f32, facing_right: bool) -> (f32, f32) {
        if self.up && self.right {
            (speed, -speed)
        } else if self.up && self.left {
            (-speed, -speed)
        } else if self.down && self.right {
            (speed, speed)
        } else if self.down && self.left {
            (-speed, speed)
        } else if self.up {
            (0.0, -speed)
        } else if self.down {
            (0.0, speed)
        } else if self.right {
            (speed, 0.0)
        } else if self.left {
            (-speed, 0.0)
        } else if facing_right {
            (speed, 0.0)
        } else {
            (-speed, 0.0)
        }
    }
}

// ---------------------------------------------------------------------------
// World (all mutable game state)
// ---------------------------------------------------------------------------
struct World {
    state: GameState,
    score: i32,
    current_level: usize,
    level_lava_speed: f32,

    player: Player,
    anchor: Anchor,
    bullets: Vec<Bullet>,
    enemy_bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    platforms: Vec<Platform>,
    gems: Vec<Gem>,
    particles: Vec<Particle>,
    popups: Vec<Popup>,
    text_popups: Vec<Popup>,
    lava_bubbles: Vec<LavaBubble>,
    title_embers: Vec<TitleEmber>,

    lava_y: f32,
    camera_y: f32,
    coyote_frames: i32,

    shake_magnitude: f32,
    screen_shake_x: f32,
    screen_shake_y: f32,
    damage_flash_timer: f32,

    time_counter: f64, // for lava wave animation
    frame_count: u64,
    hit_stop_frames: i32,
    dust_motes: Vec<DustMote>,
    trail_particles: Vec<Particle>,

    // Story typewriter state
    story_lines: &'static [&'static str],
    story_line_index: usize,
    story_char_index: usize,
    story_frame_counter: u64,
    story_full_text: String,
    story_is_victory: bool,
}

impl World {
    fn new() -> Self {
        let mut w = Self {
            state: GameState::Start,
            score: 0,
            current_level: 0,
            level_lava_speed: 0.1,
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
            platforms: Vec::new(),
            gems: Vec::new(),
            particles: Vec::new(),
            popups: Vec::new(),
            text_popups: Vec::new(),
            lava_bubbles: Vec::new(),
            title_embers: Vec::new(),
            lava_y: 1000.0,
            camera_y: 0.0,
            coyote_frames: 0,
            shake_magnitude: 0.0,
            screen_shake_x: 0.0,
            screen_shake_y: 0.0,
            damage_flash_timer: 0.0,
            time_counter: 0.0,
            frame_count: 0,
            hit_stop_frames: 0,
            dust_motes: Vec::new(),
            trail_particles: Vec::new(),

            story_lines: &[],
            story_line_index: 0,
            story_char_index: 0,
            story_frame_counter: 0,
            story_full_text: String::new(),
            story_is_victory: false,
        };
        // Initialize ambient dust motes
        for _ in 0..20 {
            w.dust_motes.push(DustMote {
                x: rand::gen_range(0.0_f32, SCREEN_W),
                y: rand::gen_range(0.0_f32, SCREEN_H),
                vx: rand::gen_range(-0.2_f32, 0.2),
                vy: rand::gen_range(-0.3_f32, -0.05),
                size: rand::gen_range(1.0_f32, 2.0),
                alpha: rand::gen_range(0.1_f32, 0.2),
            });
        }
        w.reset_game(true);
        w.state = GameState::Start;
        w
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
        self.anchor = Anchor {
            active: false, is_attached: false,
            x: 0.0, y: 0.0, vx: 0.0, vy: 0.0,
            w: 16.0, h: 16.0, length: 0.0,
        };
        self.bullets.clear();
        self.enemy_bullets.clear();
        self.particles.clear();
        self.popups.clear();
        self.text_popups.clear();
        self.gems.clear();
        self.lava_bubbles.clear();
        self.trail_particles.clear();
        self.hit_stop_frames = 0;
        self.damage_flash_timer = 0.0;
        self.shake_magnitude = 0.0;
        self.screen_shake_x = 0.0;
        self.screen_shake_y = 0.0;
        self.coyote_frames = 0;

        self.platforms.clear();
        self.enemies.clear();

        let level = get_level(self.current_level);
        self.level_lava_speed = level.lava_speed;
        let map = level.map;
        let map_height = map.len() as f32;
        let start_y = -(map_height * TILE_SIZE) + SCREEN_H;

        for (row, line) in map.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let px = col as f32 * TILE_SIZE;
                let py = start_y + row as f32 * TILE_SIZE;

                match ch {
                    '#' => self.platforms.push(Platform { x: px, y: py, w: TILE_SIZE, h: TILE_SIZE, kind: TileKind::Brick }),
                    '%' => self.platforms.push(Platform { x: px, y: py, w: TILE_SIZE, h: TILE_SIZE, kind: TileKind::Stone }),
                    'C' => self.platforms.push(Platform { x: px, y: py, w: TILE_SIZE, h: TILE_SIZE, kind: TileKind::Chest }),
                    'P' => self.enemies.push(Enemy {
                        kind: EnemyKind::Patrol, x: px, y: py, w: TILE_SIZE, h: TILE_SIZE,
                        vx: 1.5, vy: 0.0, start_x: px, range: 40.0, shoot_timer: 0.0,
                    }),
                    'B' => self.enemies.push(Enemy {
                        kind: EnemyKind::Bat, x: px, y: py, w: TILE_SIZE, h: TILE_SIZE,
                        vx: 0.0, vy: 0.0, start_x: px, range: 0.0, shoot_timer: 0.0,
                    }),
                    'T' => self.enemies.push(Enemy {
                        kind: EnemyKind::Turret, x: px, y: py, w: TILE_SIZE, h: TILE_SIZE,
                        vx: 0.0, vy: 0.0, start_x: px, range: 0.0,
                        shoot_timer: rand::gen_range(0.0_f32, 60.0),
                    }),
                    'G' => self.gems.push(Gem {
                        x: px + 2.0, y: py + 2.0, w: 16.0, h: 16.0, vx: 0.0, vy: 0.0,
                    }),
                    _ => {}
                }
            }
        }

        let map_pixel_h = map_height * TILE_SIZE;
        self.player.y = start_y + map_pixel_h - 60.0;
        self.lava_y = self.player.y + 400.0;
        self.camera_y = self.player.y - 200.0;
        self.state = GameState::Playing;
    }

    fn trigger_shake(&mut self, mag: f32) {
        if mag > self.shake_magnitude {
            self.shake_magnitude = mag;
        }
    }

    fn init_story_screen(&mut self, lines: &'static [&'static str], is_victory: bool) {
        self.story_lines = lines;
        self.story_line_index = 0;
        self.story_char_index = 0;
        self.story_frame_counter = 0;
        self.story_full_text = String::new();
        self.story_is_victory = is_victory;
        self.state = GameState::LevelStory;
    }

    fn add_score(&mut self, amount: i32, x: f32, y: f32) {
        self.score += amount;
        self.popups.push(Popup {
            text: format!("+{}", amount),
            x, y, life: 40.0,
        });
    }

    // ------------------------------------------------------------------
    // Physics tick (one 1/60s step)
    // ------------------------------------------------------------------
    fn update(&mut self, input: &mut Input) {
        self.time_counter += TIME_STEP;
        self.frame_count += 1;

        // Hit stop: freeze game logic briefly on enemy kill for impact
        if self.hit_stop_frames > 0 {
            self.hit_stop_frames -= 1;
            return;
        }

        // Update dust motes (ambient particles)
        for mote in &mut self.dust_motes {
            mote.x += mote.vx;
            mote.y += mote.vy;
            if mote.y < -5.0 { mote.y = SCREEN_H + 5.0; }
            if mote.y > SCREEN_H + 5.0 { mote.y = -5.0; }
            if mote.x < -5.0 { mote.x = SCREEN_W + 5.0; }
            if mote.x > SCREEN_W + 5.0 { mote.x = -5.0; }
        }

        // Player trail effect: spawn afterimage when moving fast
        let vel_mag = (self.player.vx * self.player.vx + self.player.vy * self.player.vy).sqrt();
        if vel_mag > 2.0 && self.frame_count % 2 == 0 {
            self.trail_particles.push(Particle {
                x: self.player.x + self.player.w / 2.0,
                y: self.player.y + self.player.h / 2.0,
                vx: 0.0,
                vy: 0.0,
                life: 8.0,
                color: Color::new(0.0, 1.0, 1.0, 0.3),
                size: self.player.w * 0.8,
            });
        }
        // Update trail particles
        {
            let mut i = self.trail_particles.len();
            while i > 0 {
                i -= 1;
                self.trail_particles[i].life -= 1.0;
                if self.trail_particles[i].life <= 0.0 {
                    self.trail_particles.remove(i);
                }
            }
        }

        // Coyote time
        if self.player.on_ground {
            self.coyote_frames = COYOTE_MAX;
        } else {
            self.coyote_frames -= 1;
        }
        if input.jump_buffer > 0 {
            input.jump_buffer -= 1;
        }

        // ----- ANCHOR LOGIC -----
        // Fire anchor if B held for >9 frames (~150ms) and not already active
        if input.b_down && !self.anchor.active && !input.anchor_fired && input.b_down_frames > 9 {
            let (avx, avy) = input.get_trajectory(ANCHOR_SPEED, self.player.facing_right);
            self.anchor.active = true;
            self.anchor.is_attached = false;
            input.anchor_fired = true;
            self.anchor.x = self.player.x + self.player.w / 2.0 - self.anchor.w / 2.0;
            self.anchor.y = self.player.y + self.player.h / 2.0 - self.anchor.h / 2.0;
            self.anchor.vx = avx;
            self.anchor.vy = avy;
        }
        if !input.b_down {
            input.anchor_fired = false;
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

        // Move anchor projectile
        if self.anchor.active && !self.anchor.is_attached {
            self.anchor.x += self.anchor.vx;
            self.anchor.y += self.anchor.vy;

            let mut destroyed = false;
            let mut i = self.platforms.len();
            while i > 0 {
                i -= 1;
                let p = &self.platforms[i];
                if overlaps(self.anchor.x, self.anchor.y, self.anchor.w, self.anchor.h,
                            p.x, p.y, p.w, p.h)
                {
                    match p.kind {
                        TileKind::Stone | TileKind::Chest => {
                            let px = p.x;
                            let py = p.y;
                            let is_chest = p.kind == TileKind::Chest;
                            let pcolor = if is_chest {
                                color_u8!(139, 69, 19, 255)
                            } else {
                                color_u8!(119, 119, 119, 255)
                            };
                            if is_chest {
                                self.gems.push(Gem {
                                    x: px + 2.0, y: py, w: 16.0, h: 16.0,
                                    vx: rand::gen_range(-2.0_f32, 2.0),
                                    vy: -5.0,
                                });
                            }
                            self.platforms.remove(i);
                            for _ in 0..6 {
                                self.particles.push(Particle {
                                    x: px + 10.0, y: py + 10.0,
                                    vx: rand::gen_range(-4.0_f32, 4.0),
                                    vy: rand::gen_range(-4.0_f32, 4.0),
                                    life: 15.0 + rand::gen_range(0.0_f32, 15.0),
                                    color: pcolor,
                                    size: 4.0,
                                });
                            }
                            self.anchor.active = false;
                            destroyed = true;
                            break;
                        }
                        TileKind::Brick => {
                            self.anchor.is_attached = true;
                            self.trigger_shake(4.0);
                            let dx = (self.player.x + self.player.w / 2.0) - self.anchor.x;
                            let dy = (self.player.y + self.player.h / 2.0) - self.anchor.y;
                            self.anchor.length = (dx * dx + dy * dy).sqrt();
                            break;
                        }
                    }
                }
            }

            if !destroyed && !self.anchor.is_attached {
                let dx = (self.player.x + self.player.w / 2.0) - self.anchor.x;
                let dy = (self.player.y + self.player.h / 2.0) - self.anchor.y;
                if (dx * dx + dy * dy).sqrt() > 300.0 {
                    self.anchor.active = false;
                }
            }
        }

        // ----- PARTICLES -----
        {
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
        }

        // ----- POPUPS -----
        {
            let mut i = self.popups.len();
            while i > 0 {
                i -= 1;
                self.popups[i].y -= 1.0;
                self.popups[i].life -= 1.0;
                if self.popups[i].life <= 0.0 {
                    self.popups.remove(i);
                }
            }
        }

        // ----- GEMS physics + pickup -----
        {
            let mut i = self.gems.len();
            while i > 0 {
                i -= 1;
                self.gems[i].vy += GRAVITY;
                self.gems[i].x += self.gems[i].vx;
                self.gems[i].y += self.gems[i].vy;

                // Bounce on platforms
                for p in &self.platforms {
                    if p.kind != TileKind::Stone && p.kind != TileKind::Chest {
                        let g = &self.gems[i];
                        if overlaps(g.x, g.y, g.w, g.h, p.x, p.y, p.w, p.h) {
                            if self.gems[i].vy > 0.0 && self.gems[i].y < p.y {
                                self.gems[i].y = p.y - self.gems[i].h;
                                self.gems[i].vy = -self.gems[i].vy * 0.5;
                                self.gems[i].vx *= 0.8;
                            }
                        }
                    }
                }

                let g = &self.gems[i];
                if overlaps(self.player.x, self.player.y, self.player.w, self.player.h,
                            g.x, g.y, g.w, g.h)
                {
                    let gx = g.x;
                    let gy = g.y;
                    self.add_score(50, gx, gy);
                    self.text_popups.push(Popup { text: "+100".into(), x: gx, y: gy, life: 30.0 });
                    self.gems.remove(i);
                }
            }
        }

        // ----- PLAYER MOVEMENT -----
        let mut target_vx: f32 = 0.0;
        if input.right { target_vx = MOVE_SPEED; self.player.facing_right = true; }
        if input.left  { target_vx = -MOVE_SPEED; self.player.facing_right = false; }

        if self.anchor.is_attached {
            self.player.vx += target_vx * 0.05;
            if input.up && self.anchor.length > 20.0 { self.anchor.length -= CLIMB_SPEED; }
            if input.down && self.anchor.length < 300.0 { self.anchor.length += CLIMB_SPEED; }
        } else if self.player.on_ground {
            self.player.vx = target_vx;
        } else {
            self.player.vx = self.player.vx * 0.8 + target_vx * 0.2;
        }

        // X sweep
        self.player.x += self.player.vx;
        self.player.wall_dir = 0;
        for p in &self.platforms {
            if overlaps(self.player.x, self.player.y, self.player.w, self.player.h,
                        p.x, p.y, p.w, p.h)
            {
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
        // Screen wrap
        if self.player.x > SCREEN_W { self.player.x = -self.player.w; }
        if self.player.x < -self.player.w { self.player.x = SCREEN_W; }

        // Gravity
        self.player.vy += GRAVITY;
        if self.player.wall_dir != 0 && self.player.vy > 0.0 && !self.anchor.is_attached {
            if self.player.vy > WALL_SLIDE_SPEED { self.player.vy = WALL_SLIDE_SPEED; }
        } else if self.player.vy > MAX_FALL_SPEED {
            self.player.vy = MAX_FALL_SPEED;
        }

        // Y sweep
        self.player.y += self.player.vy;
        self.player.on_ground = false;
        for p in &self.platforms {
            if overlaps(self.player.x, self.player.y, self.player.w, self.player.h,
                        p.x, p.y, p.w, p.h)
            {
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

        // Anchor constraint (Verlet-like)
        if self.anchor.is_attached {
            let dx = (self.player.x + self.player.w / 2.0) - self.anchor.x;
            let dy = (self.player.y + self.player.h / 2.0) - self.anchor.y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist > self.anchor.length && dist > 0.001 {
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

        // ----- JUMPING -----
        if input.jump_buffer > 0 {
            if self.anchor.is_attached {
                // no jump while anchored
            } else if self.coyote_frames > 0 {
                self.player.vy = JUMP_FORCE;
                self.player.jumps = 1;
                self.coyote_frames = 0;
                input.jump_buffer = 0;
            } else if self.player.wall_dir != 0 {
                self.player.vy = WALL_JUMP_Y;
                self.player.vx = -(self.player.wall_dir as f32) * WALL_JUMP_X;
                self.player.facing_right = self.player.wall_dir == -1;
                self.player.jumps = 1;
                input.jump_buffer = 0;
            } else if self.player.jumps < self.player.max_jumps {
                self.player.vy = JUMP_FORCE;
                self.player.jumps += 1;
                input.jump_buffer = 0;
            }
        }

        // ----- SHOOTING -----
        if input.shoot_pressed {
            let (bvx, bvy) = input.get_trajectory(BULLET_SPEED, self.player.facing_right);
            self.bullets.push(Bullet {
                x: self.player.x + self.player.w / 2.0 - 4.0,
                y: self.player.y + self.player.h / 2.0 - 4.0,
                vx: bvx, vy: bvy, w: 8.0, h: 8.0,
            });
            input.shoot_pressed = false;
        }

        // Update bullets
        {
            let mut i = self.bullets.len();
            while i > 0 {
                i -= 1;
                self.bullets[i].x += self.bullets[i].vx;
                self.bullets[i].y += self.bullets[i].vy;
                let b = &self.bullets[i];

                let mut hit_wall = false;
                for p in &self.platforms {
                    if p.kind != TileKind::Stone && p.kind != TileKind::Chest {
                        if overlaps(b.x, b.y, b.w, b.h, p.x, p.y, p.w, p.h) {
                            hit_wall = true;
                            break;
                        }
                    }
                }

                let mut hit_enemy = false;
                if !hit_wall {
                    let bx = b.x; let by = b.y; let bw = b.w; let bh = b.h;
                    let mut j = self.enemies.len();
                    while j > 0 {
                        j -= 1;
                        let e = &self.enemies[j];
                        if overlaps(bx, by, bw, bh, e.x, e.y, e.w, e.h) {
                            let ex = e.x; let ey = e.y;
                            self.enemies.remove(j);
                            hit_enemy = true;
                            self.add_score(100, ex, ey);
                            self.trigger_shake(2.0);
                            self.hit_stop_frames = 3;
                            let death_colors = [
                                Color::new(1.0, 0.5, 0.0, 1.0),  // orange
                                Color::new(1.0, 1.0, 0.0, 1.0),  // yellow
                                Color::new(1.0, 1.0, 1.0, 1.0),  // white
                                Color::new(1.0, 0.1, 0.1, 1.0),  // red
                            ];
                            for k in 0..20 {
                                self.particles.push(Particle {
                                    x: ex + 10.0, y: ey + 10.0,
                                    vx: rand::gen_range(-5.0_f32, 5.0),
                                    vy: rand::gen_range(-5.0_f32, 5.0),
                                    life: 15.0 + rand::gen_range(0.0_f32, 15.0),
                                    color: death_colors[k % 4],
                                    size: rand::gen_range(1.0_f32, 4.0),
                                });
                            }
                            break;
                        }
                    }
                }

                let b = &self.bullets[i];
                let oob = b.x > SCREEN_W || b.x < -b.w
                    || b.y < self.camera_y - 100.0
                    || b.y > self.camera_y + SCREEN_H + 100.0;
                if hit_wall || hit_enemy || oob {
                    self.bullets.remove(i);
                }
            }
        }

        // ----- ENEMY LOGIC -----
        // We need to collect new enemy bullets separately to avoid borrow issues.
        let mut new_enemy_bullets: Vec<Bullet> = Vec::new();
        {
            let px = self.player.x;
            let py = self.player.y;
            let mut i = self.enemies.len();
            while i > 0 {
                i -= 1;
                match self.enemies[i].kind {
                    EnemyKind::Patrol => {
                        self.enemies[i].x += self.enemies[i].vx;
                        let e = &self.enemies[i];
                        if e.x > e.start_x + e.range || e.x < e.start_x - e.range {
                            self.enemies[i].vx *= -1.0;
                        }
                    }
                    EnemyKind::Bat => {
                        let e = &self.enemies[i];
                        let dx = px - e.x;
                        let dy = py - e.y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist < 250.0 && dist > 0.001 {
                            self.enemies[i].x += (dx / dist) * 1.2;
                            self.enemies[i].y += (dy / dist) * 1.2;
                        }
                    }
                    EnemyKind::Turret => {
                        self.enemies[i].shoot_timer += 1.0;
                        if self.enemies[i].shoot_timer > 90.0 {
                            let e = &self.enemies[i];
                            let dx = px - e.x;
                            let dy = py - e.y;
                            let dist = (dx * dx + dy * dy).sqrt();
                            if dist < 300.0 && dist > 0.001 {
                                new_enemy_bullets.push(Bullet {
                                    x: e.x + e.w / 2.0 - 4.0,
                                    y: e.y + e.h / 2.0 - 4.0,
                                    vx: (dx / dist) * ENEMY_BULLET_SPEED,
                                    vy: (dy / dist) * ENEMY_BULLET_SPEED,
                                    w: 8.0, h: 8.0,
                                });
                            }
                            self.enemies[i].shoot_timer = 0.0;
                        }
                    }
                }

                // Player-enemy collision
                let e = &self.enemies[i];
                if overlaps(self.player.x, self.player.y, self.player.w, self.player.h,
                            e.x, e.y, e.w, e.h)
                {
                    if self.player.vy > 0.0
                        && self.player.y + self.player.h < e.y + e.h / 2.0 + 5.0
                        && e.kind != EnemyKind::Turret
                    {
                        self.player.vy = BOUNCE_FORCE;
                        self.player.jumps = 1;
                        let ex = e.x;
                        let ey = e.y;
                        self.enemies.remove(i);
                        self.add_score(100, ex, ey);
                        self.trigger_shake(2.0);
                        self.hit_stop_frames = 3;
                        let death_colors = [
                            Color::new(1.0, 0.5, 0.0, 1.0),
                            Color::new(1.0, 1.0, 0.0, 1.0),
                            Color::new(1.0, 1.0, 1.0, 1.0),
                            Color::new(1.0, 0.1, 0.1, 1.0),
                        ];
                        for k in 0..20 {
                            self.particles.push(Particle {
                                x: ex + 10.0, y: ey + 10.0,
                                vx: rand::gen_range(-5.0_f32, 5.0),
                                vy: rand::gen_range(-5.0_f32, 5.0),
                                life: 15.0 + rand::gen_range(0.0_f32, 15.0),
                                color: death_colors[k % 4],
                                size: rand::gen_range(1.0_f32, 4.0),
                            });
                        }
                    } else {
                        self.state = GameState::GameOver;
                        self.trigger_shake(6.0);
                        self.damage_flash_timer = 6.0;
                    }
                }
            }
        }
        self.enemy_bullets.extend(new_enemy_bullets);

        // Update enemy bullets
        {
            let mut i = self.enemy_bullets.len();
            while i > 0 {
                i -= 1;
                self.enemy_bullets[i].x += self.enemy_bullets[i].vx;
                self.enemy_bullets[i].y += self.enemy_bullets[i].vy;
                let b = &self.enemy_bullets[i];

                let mut hit_wall = false;
                for p in &self.platforms {
                    if p.kind != TileKind::Stone && p.kind != TileKind::Chest {
                        if overlaps(b.x, b.y, b.w, b.h, p.x, p.y, p.w, p.h) {
                            hit_wall = true;
                            break;
                        }
                    }
                }

                let b = &self.enemy_bullets[i];
                if overlaps(b.x, b.y, b.w, b.h,
                            self.player.x, self.player.y, self.player.w, self.player.h)
                {
                    self.state = GameState::GameOver;
                    self.trigger_shake(6.0);
                    self.damage_flash_timer = 6.0;
                }

                let b = &self.enemy_bullets[i];
                let oob = b.x > SCREEN_W || b.x < -b.w
                    || b.y < self.camera_y - 100.0
                    || b.y > self.camera_y + SCREEN_H + 100.0;
                if hit_wall || oob {
                    self.enemy_bullets.remove(i);
                }
            }
        }

        // ----- LAVA -----
        self.lava_y -= self.level_lava_speed;
        if self.player.y + self.player.h > self.lava_y {
            self.state = GameState::GameOver;
            self.trigger_shake(6.0);
            self.damage_flash_timer = 6.0;
        }

        // ----- GOAL CHECK -----
        {
            let level = get_level(self.current_level);
            let map_height = level.map.len() as f32;
            let start_y = -(map_height * TILE_SIZE) + SCREEN_H;
            let goal_x: f32 = 280.0;
            let goal_y: f32 = start_y + 2.0 * TILE_SIZE;
            let goal_w: f32 = 30.0;
            let goal_h: f32 = 30.0;
            if overlaps(self.player.x, self.player.y, self.player.w, self.player.h,
                        goal_x, goal_y, goal_w, goal_h)
            {
                let just_finished = self.current_level;
                self.current_level += 1;
                if self.current_level >= 3 {
                    self.init_story_screen(STORY_VICTORY, true);
                } else if just_finished == 0 {
                    self.init_story_screen(STORY_AFTER_LEVEL_1, false);
                } else if just_finished == 1 {
                    self.init_story_screen(STORY_AFTER_LEVEL_2, false);
                } else {
                    self.reset_game(false);
                }
                return;
            }
        }

        // ----- SCREEN SHAKE -----
        if self.shake_magnitude > 0.1 {
            self.screen_shake_x = rand::gen_range(-1.0_f32, 1.0) * self.shake_magnitude;
            self.screen_shake_y = rand::gen_range(-1.0_f32, 1.0) * self.shake_magnitude;
            self.shake_magnitude *= 0.85;
        } else {
            self.screen_shake_x = 0.0;
            self.screen_shake_y = 0.0;
            self.shake_magnitude = 0.0;
        }

        if self.damage_flash_timer > 0.0 { self.damage_flash_timer -= 1.0; }

        // Text popups
        {
            let mut i = self.text_popups.len();
            while i > 0 {
                i -= 1;
                self.text_popups[i].y -= 1.5;
                self.text_popups[i].life -= 1.0;
                if self.text_popups[i].life <= 0.0 {
                    self.text_popups.remove(i);
                }
            }
        }

        // Lava bubbles
        if rand::gen_range(0.0_f32, 1.0) < 0.3 {
            self.lava_bubbles.push(LavaBubble {
                x: rand::gen_range(0.0_f32, SCREEN_W),
                y: self.lava_y,
                r: 2.0 + rand::gen_range(0.0_f32, 4.0),
                life: 20.0 + rand::gen_range(0.0_f32, 30.0),
                speed: 0.5 + rand::gen_range(0.0_f32, 1.5),
            });
        }
        {
            let mut i = self.lava_bubbles.len();
            while i > 0 {
                i -= 1;
                self.lava_bubbles[i].y -= self.lava_bubbles[i].speed;
                self.lava_bubbles[i].life -= 1.0;
                if self.lava_bubbles[i].life <= 0.0 {
                    self.lava_bubbles.remove(i);
                }
            }
        }

        // Camera
        let target_cam = self.player.y - SCREEN_H * 0.6;
        self.camera_y += (target_cam - self.camera_y) * 0.1;
        if self.camera_y > self.lava_y - SCREEN_H + 100.0 {
            self.camera_y = self.lava_y - SCREEN_H + 100.0;
        }
    }
}

// ---------------------------------------------------------------------------
// Drawing
// ---------------------------------------------------------------------------
fn draw_world(world: &mut World, sprites: &Sprites) {
    clear_background(Color::new(0.059, 0.059, 0.098, 1.0));

    let cam_y = world.camera_y;

    // Parallax background
    let bg_size: f32 = 64.0;
    let start_y_bg = ((cam_y * 0.5) / bg_size).floor() as i32 - 1;
    for y in start_y_bg..(start_y_bg + 12) {
        for x in 0..12 {
            draw_texture_ex(
                &sprites.bg,
                x as f32 * bg_size,
                y as f32 * bg_size - cam_y * 0.5,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(bg_size, bg_size)),
                    ..Default::default()
                },
            );
        }
    }

    // Ambient dust motes (background layer)
    for mote in &world.dust_motes {
        draw_rectangle(mote.x, mote.y, mote.size, mote.size,
            Color::new(1.0, 1.0, 1.0, mote.alpha));
    }

    // Apply screen shake offset for game world drawing
    let sx = world.screen_shake_x;
    let sy = world.screen_shake_y;

    // Tutorial texts
    if world.state == GameState::Playing {
        let level = get_level(world.current_level);
        let map_height = level.map.len() as f32;
        let start_y = -(map_height * TILE_SIZE) + SCREEN_H;
        for t in level.texts {
            let tx = t.col * TILE_SIZE + sx;
            let ty = start_y + t.row * TILE_SIZE - cam_y + sy;
            // Black outline
            let outline = 2.0;
            for &ox in &[-outline, 0.0, outline] {
                for &oy in &[-outline, 0.0, outline] {
                    if ox != 0.0 || oy != 0.0 {
                        draw_text(t.text, tx + ox, ty + oy, 16.0, BLACK);
                    }
                }
            }
            draw_text(t.text, tx, ty, 16.0, WHITE);
        }
    }

    // Platforms
    for p in &world.platforms {
        if p.y > cam_y - TILE_SIZE && p.y < cam_y + SCREEN_H + TILE_SIZE {
            let tex = match p.kind {
                TileKind::Brick => &sprites.brick,
                TileKind::Stone => &sprites.stone,
                TileKind::Chest => &sprites.chest,
            };
            draw_texture_ex(
                tex,
                p.x + sx, p.y - cam_y + sy, WHITE,
                DrawTextureParams { dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)), ..Default::default() },
            );
        }
    }

    // Particles
    for p in &world.particles {
        let alpha = (p.life / 15.0).min(1.0);
        let mut c = p.color;
        c.a *= alpha;
        draw_rectangle(p.x + sx, p.y - cam_y + sy, p.size, p.size, c);
    }

    // Gems (with pulsing glow)
    for g in &world.gems {
        // Pulsing glow behind the gem
        let glow_alpha = 0.15 + 0.1 * (world.frame_count as f32 * 0.08).sin();
        let glow_size = g.w + 8.0 + 2.0 * (world.frame_count as f32 * 0.08).sin();
        draw_circle(
            g.x + g.w / 2.0 + sx,
            g.y + g.h / 2.0 - cam_y + sy,
            glow_size / 2.0,
            Color::new(0.0, 1.0, 1.0, glow_alpha),
        );
        draw_texture_ex(
            &sprites.gem,
            g.x + sx, g.y - cam_y + sy, WHITE,
            DrawTextureParams { dest_size: Some(vec2(g.w, g.h)), ..Default::default() },
        );
    }

    // Player bullets
    for b in &world.bullets {
        let angle = b.vy.atan2(b.vx);
        let cx = b.x + b.w / 2.0 + sx;
        let cy = b.y - cam_y + b.h / 2.0 + sy;
        draw_texture_ex(
            &sprites.bullet,
            cx - b.w / 2.0, cy - b.h / 2.0, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(b.w, b.h)),
                rotation: angle,
                ..Default::default()
            },
        );
    }

    // Enemy bullets
    for b in &world.enemy_bullets {
        let angle = b.vy.atan2(b.vx);
        let cx = b.x + b.w / 2.0 + sx;
        let cy = b.y - cam_y + b.h / 2.0 + sy;
        draw_texture_ex(
            &sprites.enemy_bullet,
            cx - b.w / 2.0, cy - b.h / 2.0, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(b.w, b.h)),
                rotation: angle,
                ..Default::default()
            },
        );
    }

    // Enemies
    for e in &world.enemies {
        let tex = match e.kind {
            EnemyKind::Patrol => &sprites.patrol,
            EnemyKind::Bat => &sprites.bat,
            EnemyKind::Turret => &sprites.turret,
        };
        let flip = match e.kind {
            EnemyKind::Patrol => e.vx > 0.0,
            EnemyKind::Bat => world.player.x > e.x,
            EnemyKind::Turret => false,
        };
        draw_texture_ex(
            tex,
            e.x + sx, e.y - cam_y + sy, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(e.w, e.h)),
                flip_x: flip,
                ..Default::default()
            },
        );
    }

    // Anchor rope + head
    if world.anchor.active || world.anchor.is_attached {
        let px = world.player.x + world.player.w / 2.0 + sx;
        let py = world.player.y + world.player.h / 2.0 - cam_y + sy;
        let ax = world.anchor.x + world.anchor.w / 2.0 + sx;
        let ay = world.anchor.y + world.anchor.h / 2.0 - cam_y + sy;
        draw_line(px, py, ax, ay, 2.0, Color::new(1.0, 1.0, 1.0, 0.5));
        draw_texture_ex(
            &sprites.anchor,
            world.anchor.x + sx, world.anchor.y - cam_y + sy, WHITE,
            DrawTextureParams { dest_size: Some(vec2(world.anchor.w, world.anchor.h)), ..Default::default() },
        );
    }

    // Player trail afterimages
    for tp in &world.trail_particles {
        let alpha = (tp.life / 8.0) * 0.25;
        let trail_size = tp.size;
        draw_rectangle(
            tp.x - trail_size / 2.0 + sx,
            tp.y - trail_size / 2.0 - cam_y + sy,
            trail_size, trail_size,
            Color::new(0.0, 1.0, 1.0, alpha),
        );
    }

    // Player
    if world.state == GameState::Playing || world.state == GameState::Win {
        draw_texture_ex(
            &sprites.mage,
            world.player.x + sx, world.player.y - cam_y + sy, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(world.player.w, world.player.h)),
                flip_x: !world.player.facing_right,
                ..Default::default()
            },
        );
    }

    // Goal portal
    if world.state == GameState::Playing {
        let level = get_level(world.current_level);
        let map_height = level.map.len() as f32;
        let start_y = -(map_height * TILE_SIZE) + SCREEN_H;
        draw_texture_ex(
            &sprites.goal,
            280.0 + sx, start_y + 2.0 * TILE_SIZE - cam_y + sy, WHITE,
            DrawTextureParams { dest_size: Some(vec2(30.0, 30.0)), ..Default::default() },
        );
    }

    // ----- LAVA -----
    let lava_screen_y = world.lava_y - cam_y + sy;
    let now = world.time_counter as f32 * 3.0;

    // Draw lava body as a series of thin rects for the gradient
    let lava_top = lava_screen_y - 8.0; // account for wave amplitude
    if lava_top < SCREEN_H {
        let top = lava_top.max(0.0);
        let height = SCREEN_H - top;
        // Simple gradient: orange -> dark red
        let steps = 8;
        let step_h = height / steps as f32;
        for s in 0..steps {
            let t = s as f32 / steps as f32;
            let r = 1.0 - t * 0.7;
            let g_val = 0.63 * (1.0 - t);
            let b_val = 0.0;
            draw_rectangle(0.0, top + s as f32 * step_h, SCREEN_W, step_h + 1.0,
                Color::new(r, g_val, b_val, 0.9));
        }
    }

    // Wavy top edge highlight
    {
        for wx_i in 0..((SCREEN_W as i32) / 4) {
            let wx = wx_i as f32 * 4.0;
            let wave_y = lava_screen_y
                + (now + wx * 0.02).sin() * 4.0
                + (now * 1.7 + wx * 0.035).sin() * 3.0;
            // Fill column from wave to old lava_screen_y with orange
            let fill_top = wave_y.min(lava_screen_y);
            let fill_h = (wave_y - fill_top).abs() + 4.0;
            draw_rectangle(wx, fill_top, 4.0, fill_h, Color::new(1.0, 0.63, 0.0, 0.95));
            // Bright edge
            draw_rectangle(wx, wave_y, 4.0, 3.0, Color::new(1.0, 1.0, 0.39, 0.7));
        }
    }

    // Lava bubbles
    for lb in &world.lava_bubbles {
        let alpha = (lb.life / 50.0).min(1.0);
        draw_circle(lb.x, lb.y - cam_y + sy, lb.r, Color::new(1.0, 0.78, 0.2, alpha));
    }

    // ----- DAMAGE FLASH -----
    if world.damage_flash_timer > 0.0 {
        let alpha = 0.3 * (world.damage_flash_timer / 6.0);
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.78, 0.0, 0.0, alpha));
    }

    // Text popups (gem pickup)
    for tp in &world.text_popups {
        let alpha = (tp.life / 30.0).min(1.0);
        draw_text(&tp.text, tp.x + 10.0, tp.y - cam_y, 18.0, Color::new(1.0, 1.0, 0.0, alpha));
    }

    // HUD popups (score)
    for p in &world.popups {
        let alpha = (p.life / 30.0).min(1.0);
        draw_text(&p.text, p.x + 10.0, p.y - cam_y, 16.0, Color::new(1.0, 1.0, 1.0, alpha));
    }

    // HUD
    if world.state == GameState::Playing {
        let score_str = format!("SCORE: {}", world.score);
        draw_text(&score_str, 20.0, 40.0, 20.0, WHITE);
        let level_names = ["THE OVERGROWN DEPTHS", "THE FROZEN ARCHIVE", "THE LIVING CORE"];
        let level_str = level_names[world.current_level.min(2) as usize];
        let lw = measure_text(level_str, None, 14, 1.0);
        draw_text(level_str, SCREEN_W - 20.0 - lw.width, 40.0, 14.0,
            Color::new(0.7, 0.6, 1.0, 0.8));
    }

    // ----- SCREEN OVERLAYS -----
    match world.state {
        GameState::Start => {
            draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.0, 0.0, 0.0, 0.8));

            // Title embers
            if rand::gen_range(0.0_f32, 1.0) < 0.4 {
                world.title_embers.push(TitleEmber {
                    x: rand::gen_range(0.0_f32, SCREEN_W),
                    y: SCREEN_H + 5.0,
                    vy: -(1.0 + rand::gen_range(0.0_f32, 2.0)),
                    vx: rand::gen_range(-0.25_f32, 0.25),
                    life: 60.0 + rand::gen_range(0.0_f32, 60.0),
                    size: 2.0 + rand::gen_range(0.0_f32, 3.0),
                });
            }
            let ember_colors = [
                Color::new(1.0, 0.63, 0.0, 1.0),
                Color::new(1.0, 0.78, 0.2, 1.0),
                Color::new(1.0, 0.39, 0.0, 1.0),
            ];
            let mut i = world.title_embers.len();
            while i > 0 {
                i -= 1;
                world.title_embers[i].x += world.title_embers[i].vx;
                world.title_embers[i].y += world.title_embers[i].vy;
                world.title_embers[i].life -= 1.0;
                let e = &world.title_embers[i];
                let alpha = (e.life / 40.0).min(1.0);
                let mut c = ember_colors[i % 3];
                c.a = alpha;
                draw_rectangle(e.x, e.y, e.size, e.size, c);
                if world.title_embers[i].life <= 0.0 {
                    world.title_embers.remove(i);
                }
            }

            // Title
            let title = "THE OBSIDIAN SPIRE";
            let tm = measure_text(title, None, 42, 1.0);
            // Glow effect via slightly offset copies
            for &ox in &[-2.0_f32, 2.0, 0.0] {
                for &oy in &[-2.0_f32, 2.0, 0.0] {
                    draw_text(title, SCREEN_W / 2.0 - tm.width / 2.0 + ox, SCREEN_H / 2.0 - 20.0 + oy, 42.0,
                        Color::new(1.0, 0.53, 0.0, 0.3));
                }
            }
            draw_text(title, SCREEN_W / 2.0 - tm.width / 2.0, SCREEN_H / 2.0 - 40.0, 42.0, WHITE);

            // Subtitle
            let sub = "The last Crystal Mage ascends";
            let subm = measure_text(sub, None, 16, 1.0);
            draw_text(sub, SCREEN_W / 2.0 - subm.width / 2.0, SCREEN_H / 2.0 + 5.0, 16.0,
                Color::new(0.7, 0.6, 1.0, 0.8));

            // Blinking prompt
            let blink = ((world.time_counter * 2.0) as i32) % 2 == 0;
            if blink {
                let prompt = "Press B/Z/Space to Begin";
                let pm = measure_text(prompt, None, 20, 1.0);
                draw_text(prompt, SCREEN_W / 2.0 - pm.width / 2.0, SCREEN_H / 2.0 + 50.0, 20.0,
                    color_u8!(255, 170, 0, 255));
            }
        }
        GameState::Story => {
            draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.02, 0.0, 0.06, 0.95));
            let story_sets: [&[&str]; 4] = [
                &[ // Intro
                    "The Obsidian Spire has awakened after a thousand years.",
                    "Its corruption spreads — forests wither, rivers turn black.",
                    "You are Vael, the last Crystal Mage.",
                    "Ascend the Spire. Destroy its heart.",
                ],
                &[ // After Level 1
                    "The walls pulse with a dark rhythm.",
                    "You feel it in your chest — familiar,",
                    "like a heartbeat that isn't your own.",
                    "Something inside the Spire recognizes you.",
                ],
                &[ // After Level 2
                    "Fragments of memory flash before your eyes —",
                    "a child running through these halls, laughing.",
                    "Your hands glow with the same dark energy",
                    "as the walls. The whispers grow louder.",
                ],
                &[ // Victory
                    "You reach the heart. The pulsing crystal is... familiar.",
                    "You ARE the heart. A fragment that gained free will.",
                    "The Mages sent you home, hoping you'd merge back.",
                    "But you are Vael now. You shatter the crystal.",
                    "Free at last.",
                ],
            ];
            let idx = world.current_level.min(3) as usize;
            let lines = story_sets[idx];
            for (i, line) in lines.iter().enumerate() {
                let m = measure_text(line, None, 18, 1.0);
                draw_text(line, SCREEN_W / 2.0 - m.width / 2.0,
                    SCREEN_H / 2.0 - 60.0 + i as f32 * 36.0, 18.0,
                    Color::new(0.78, 0.7, 1.0, 1.0));
            }
            let prompt = "Press B/Z/Space to continue...";
            let pm = measure_text(prompt, None, 16, 1.0);
            let blink = ((world.time_counter * 2.0) as i32) % 2 == 0;
            if blink {
                draw_text(prompt, SCREEN_W / 2.0 - pm.width / 2.0, SCREEN_H / 2.0 + 140.0, 16.0,
                    color_u8!(255, 170, 0, 255));
            }
        }
        GameState::GameOver => {
            draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.3, 0.0, 0.1, 0.7));
            let title = "THE SPIRE CLAIMS YOU";
            let tm = measure_text(title, None, 40, 1.0);
            draw_text(title, SCREEN_W / 2.0 - tm.width / 2.0, SCREEN_H / 2.0 - 40.0, 40.0,
                Color::new(1.0, 0.2, 0.2, 1.0));
            let sub = "The corruption swallows another soul...";
            let subm = measure_text(sub, None, 14, 1.0);
            draw_text(sub, SCREEN_W / 2.0 - subm.width / 2.0, SCREEN_H / 2.0, 14.0,
                Color::new(0.7, 0.5, 0.5, 0.8));
            let score_str = format!("FINAL SCORE: {}", world.score);
            let sm = measure_text(&score_str, None, 24, 1.0);
            draw_text(&score_str, SCREEN_W / 2.0 - sm.width / 2.0, SCREEN_H / 2.0 + 35.0, 24.0, WHITE);
            let prompt = "Press B/Z to Restart";
            let pm = measure_text(prompt, None, 20, 1.0);
            draw_text(prompt, SCREEN_W / 2.0 - pm.width / 2.0, SCREEN_H / 2.0 + 70.0, 20.0, WHITE);
        }
        GameState::Win => {
            draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.08, 0.0, 0.16, 0.9));
            let title = "FREE AT LAST";
            let tm = measure_text(title, None, 44, 1.0);
            // Purple glow
            for &ox in &[-2.0_f32, 2.0, 0.0] {
                for &oy in &[-2.0_f32, 2.0, 0.0] {
                    draw_text(title, SCREEN_W / 2.0 - tm.width / 2.0 + ox, SCREEN_H / 2.0 - 80.0 + oy, 44.0,
                        Color::new(0.6, 0.3, 1.0, 0.3));
                }
            }
            draw_text(title, SCREEN_W / 2.0 - tm.width / 2.0, SCREEN_H / 2.0 - 80.0, 44.0, WHITE);
            let epilogue = [
                "The Spire crumbles. Its corruption fades.",
                "Vael walks into the dawn — no longer",
                "the heart of darkness, but something new.",
                "Something free.",
            ];
            for (i, line) in epilogue.iter().enumerate() {
                let m = measure_text(line, None, 16, 1.0);
                draw_text(line, SCREEN_W / 2.0 - m.width / 2.0,
                    SCREEN_H / 2.0 - 20.0 + i as f32 * 28.0, 16.0,
                    Color::new(0.78, 0.7, 1.0, 0.9));
            }
            let score_str = format!("FINAL SCORE: {}", world.score);
            let sm = measure_text(&score_str, None, 22, 1.0);
            draw_text(&score_str, SCREEN_W / 2.0 - sm.width / 2.0, SCREEN_H / 2.0 + 80.0, 22.0,
                color_u8!(255, 255, 0, 255));
            let prompt = "Press B/Z to Play Again";
            let pm = measure_text(prompt, None, 18, 1.0);
            draw_text(prompt, SCREEN_W / 2.0 - pm.width / 2.0, SCREEN_H / 2.0 + 120.0, 18.0, WHITE);
        }
        GameState::Playing => {} // already drawn above
    }

    // ----- CRT SCANLINE OVERLAY -----
    {
        let scanline_color = Color::new(0.0, 0.0, 0.0, 0.15);
        let mut y = 0.0_f32;
        while y < SCREEN_H {
            draw_line(0.0, y, SCREEN_W, y, 1.0, scanline_color);
            y += 4.0;
        }
    }

    // ----- VIGNETTE EFFECT -----
    {
        let depth = 60.0_f32;
        let layers = 4_u32;
        for i in 0..layers {
            let t = i as f32 / layers as f32;
            let alpha = 0.25 * (1.0 - t);
            let inset = t * depth;
            let c = Color::new(0.0, 0.0, 0.0, alpha);
            let thickness = depth / layers as f32;
            // Top edge
            draw_rectangle(0.0, inset, SCREEN_W, thickness, c);
            // Bottom edge
            draw_rectangle(0.0, SCREEN_H - inset - thickness, SCREEN_W, thickness, c);
            // Left edge
            draw_rectangle(inset, 0.0, thickness, SCREEN_H, c);
            // Right edge
            draw_rectangle(SCREEN_W - inset - thickness, 0.0, thickness, SCREEN_H, c);
        }
    }
}

// ---------------------------------------------------------------------------
// Main entry point
// ---------------------------------------------------------------------------
fn window_conf() -> Conf {
    Conf {
        window_title: "Micro Mages".to_owned(),
        window_width: SCREEN_W as i32,
        window_height: SCREEN_H as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Build all sprites at boot
    let sprites = Sprites {
        mage: create_sprite(&MAGE_ART, &MAGE_COLORS),
        brick: create_sprite(&BRICK_ART, &BRICK_COLORS),
        stone: create_sprite(&STONE_ART, &STONE_COLORS),
        chest: create_sprite(&CHEST_ART, &CHEST_COLORS),
        bg: create_sprite(&BG_ART, &BG_COLORS),
        bullet: create_sprite(&BULLET_ART, &BULLET_COLORS),
        enemy_bullet: create_sprite(&ENEMY_BULLET_ART, &ENEMY_BULLET_COLORS),
        anchor: create_sprite(&ANCHOR_ART, &ANCHOR_COLORS),
        patrol: create_sprite(&PATROL_ART, &PATROL_COLORS),
        bat: create_sprite(&BAT_ART, &BAT_COLORS),
        turret: create_sprite(&TURRET_ART, &TURRET_COLORS),
        goal: create_sprite(&GOAL_ART, &GOAL_COLORS),
        gem: create_sprite(&GEM_ART, &GEM_COLORS),
    };

    let mut world = World::new();
    let mut input = Input::new();

    let mut accumulator: f64 = 0.0;
    let mut last_time = get_time();

    loop {
        let current_time = get_time();
        let mut frame_time = current_time - last_time;
        last_time = current_time;

        // Death spiral prevention
        if frame_time > 0.25 { frame_time = 0.25; }
        accumulator += frame_time;

        // Poll input once per render frame
        input.poll();

        while accumulator >= TIME_STEP {
            match world.state {
                GameState::Playing => {
                    world.update(&mut input);
                }
                _ => {
                    // In non-playing states, advance time counter for animations
                    world.time_counter += TIME_STEP;

                    if input.jump_buffer > 0 || input.shoot_pressed {
                        match world.state {
                            GameState::Start => { world.state = GameState::Story; }
                            GameState::Story => { world.reset_game(true); }
                            GameState::GameOver => { world.reset_game(true); }
                            GameState::Win => { world.reset_game(true); }
                            _ => {}
                        }
                        input.jump_buffer = 0;
                        input.shoot_pressed = false;
                    }
                }
            }
            accumulator -= TIME_STEP;
        }

        draw_world(&mut world, &sprites);
        next_frame().await;
    }
}
