// Pixel Knight - Mario-like Action Platformer for Miyoo Mini Plus
// Rust/Macroquad port -- 800x600, 60fps fixed timestep
// Story: "The Crystal Kingdom"
// Synced with web version: growth system, sword/fireball, lives, boss charge

use macroquad::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------
const SCREEN_W: f32 = 800.0;
const SCREEN_H: f32 = 600.0;
const TIME_STEP: f64 = 1.0 / 60.0;
const TILE: f32 = 32.0;

const GRAVITY: f32 = 0.42;
const JUMP_FORCE: f32 = -8.2;
const JUMP_HOLD_GRAVITY: f32 = 0.22;
const MOVE_SPEED: f32 = 3.0;
const MOVE_ACCEL: f32 = 0.35;
const MOVE_DECEL: f32 = 0.25;
const MAX_FALL: f32 = 9.0;
const STOMP_BOUNCE: f32 = -6.5;
const COYOTE_MAX: i32 = 6;
const JUMP_BUFFER_MAX: i32 = 6;
const FIREBALL_SPEED: f32 = 5.0;
const INVULN_MAX: i32 = 90;
const BOSS_HP: i32 = 12;

// Player sizes
const SMALL_W: f32 = 8.0;
const SMALL_H: f32 = 12.0;
const BIG_W: f32 = 14.0;
const BIG_H: f32 = 24.0;

// Enemy sizes
const ENEMY_W: f32 = 16.0;
const ENEMY_H: f32 = 16.0;
const BOSS_W: f32 = 32.0;
const BOSS_H: f32 = 32.0;

// ---------------------------------------------------------------------------
// Sprite art data
// ---------------------------------------------------------------------------
// Small knight (8x12)
const KNIGHT_SMALL_ART: [&str; 12] = [
    "..3333..",
    ".333333.",
    ".335533.",
    ".351153.",
    ".355553.",
    "..3333..",
    "..4424..",
    ".444244.",
    ".442244.",
    "..44.44.",
    "..22.22.",
    "..22.22.",
];
const KNIGHT_SMALL_COLORS: [Color; 5] = [
    Color::new(0.88, 0.69, 0.56, 1.0), // 1: skin
    Color::new(0.13, 0.27, 0.67, 1.0), // 2: dark blue
    Color::new(0.75, 0.75, 0.75, 1.0), // 3: silver
    Color::new(0.27, 0.40, 0.80, 1.0), // 4: medium blue
    Color::new(1.0, 0.80, 0.27, 1.0),  // 5: gold belt
];

// Big knight (16x21)
const KNIGHT_BIG_ART: [&str; 21] = [
    "....33333.......",
    "...3333333......",
    "...3355533......",
    "..33511153......",
    "..33511153......",
    "..33555553......",
    "...3333333......",
    "...6666666......",
    "..666626666.....",
    "..666622666.....",
    ".6667766766.....",
    ".6666666666.....",
    "..666666666.....",
    "..666626666.....",
    "..6666.6666.....",
    ".6666..6666.....",
    "..66....66......",
    "..66....66......",
    "..22....22......",
    "..222...222.....",
    "..222...222.....",
];
const KNIGHT_BIG_COLORS: [Color; 7] = [
    Color::new(0.88, 0.69, 0.56, 1.0), // 1: skin
    Color::new(0.13, 0.27, 0.67, 1.0), // 2: dark blue
    Color::new(0.75, 0.75, 0.75, 1.0), // 3: silver
    Color::new(0.27, 0.40, 0.80, 1.0), // 4: medium blue
    Color::new(1.0, 0.80, 0.27, 1.0),  // 5: gold belt
    Color::new(0.67, 0.13, 0.13, 1.0), // 6: red armor
    Color::new(0.87, 0.87, 0.27, 1.0), // 7: yellow trim
];

// Sword slash arc (8x8)
const SWORD_ART: [&str; 8] = [
    "......12",
    ".....122",
    "....1223",
    "...12233",
    "..11223.",
    ".1112...",
    "111.....",
    "11......",
];
const SWORD_COLORS: [Color; 3] = [
    Color::new(0.80, 0.80, 0.80, 1.0), // 1: silver
    WHITE,                               // 2: white
    Color::new(1.0, 1.0, 0.67, 1.0),   // 3: glow
];

// Slime enemy (16x16)
const SLIME_ART: [&str; 16] = [
    "................",
    "................",
    "................",
    "................",
    "......1111......",
    "....11111111....",
    "...1112112111...",
    "..111211211111..",
    "..111111111111..",
    ".11111111111111.",
    ".11111111111111.",
    "1111111111111111",
    "1111111111111111",
    "1111111111111111",
    ".111..1111..111.",
    "................",
];
const SLIME_COLORS: [Color; 2] = [
    Color::new(0.27, 0.73, 0.27, 1.0), // 1: green
    WHITE,                               // 2: eyes
];

// Bat enemy (16x16)
const BAT_ART_1: [&str; 16] = [
    "................",
    "................",
    "1..........1....",
    "11........11....",
    "111......111....",
    "1111....1111....",
    "11111..11111....",
    "111111111111....",
    ".1112112111.....",
    "..11111111......",
    "...111111.......",
    "....1111........",
    "....1..1........",
    "................",
    "................",
    "................",
];
const BAT_COLORS: [Color; 2] = [
    Color::new(0.53, 0.27, 0.53, 1.0), // 1: purple
    Color::new(1.0, 0.27, 0.27, 1.0),  // 2: red eyes
];

// Boss dark knight (16x16, drawn at 2x = 32x32)
const BOSS_ART: [&str; 16] = [
    "....33333.......",
    "...3333333......",
    "...3322233......",
    "..33211123......",
    "..33211123......",
    "..33222223......",
    "...3333333......",
    "....44444.......",
    "...4444444......",
    "..444444444.....",
    "..444444444.....",
    ".4444..4444.....",
    "..44....44......",
    "..33....33......",
    "..333...333.....",
    "..333...333.....",
];
const BOSS_COLORS: [Color; 5] = [
    Color::new(1.0, 0.13, 0.13, 1.0),   // 1: bright red
    Color::new(0.53, 0.0, 0.0, 1.0),    // 2: dark red
    Color::new(0.13, 0.13, 0.13, 1.0),  // 3: near-black
    Color::new(0.27, 0.27, 0.27, 1.0),  // 4: dark gray
    Color::new(0.67, 0.13, 0.13, 1.0),  // 5: medium red
];

// Crystal/coin (8x8)
const CRYSTAL_ART: [&str; 8] = [
    "...11...",
    "..1221..",
    ".122221.",
    "12233221",
    "12233221",
    ".122221.",
    "..1221..",
    "...11...",
];
const CRYSTAL_COLORS: [Color; 3] = [
    Color::new(0.0, 0.67, 1.0, 1.0),  // 1: blue
    Color::new(0.27, 0.87, 1.0, 1.0), // 2: light blue
    WHITE,                              // 3: sparkle
];

// Ground tile (8x8)
const GROUND_ART: [&str; 8] = [
    "11111111",
    "12222221",
    "23333332",
    "23333332",
    "23333332",
    "23333332",
    "23333332",
    "12222221",
];
const GROUND_COLORS: [Color; 3] = [
    Color::new(0.13, 0.53, 0.20, 1.0), // 1
    Color::new(0.33, 0.67, 0.33, 1.0), // 2
    Color::new(0.27, 0.53, 0.27, 1.0), // 3
];

// Cave tile
const CAVE_ART: [&str; 8] = [
    "11111111",
    "12222221",
    "23332332",
    "23233332",
    "23333232",
    "23323332",
    "23333332",
    "12222221",
];
const CAVE_COLORS: [Color; 3] = [
    Color::new(0.20, 0.20, 0.27, 1.0),
    Color::new(0.33, 0.33, 0.40, 1.0),
    Color::new(0.27, 0.27, 0.33, 1.0),
];

// Tower tile
const TOWER_ART: [&str; 8] = [
    "11111111",
    "12222221",
    "23333332",
    "23133132",
    "23333332",
    "23133132",
    "23333332",
    "12222221",
];
const TOWER_COLORS: [Color; 3] = [
    Color::new(0.13, 0.13, 0.13, 1.0),
    Color::new(0.23, 0.23, 0.29, 1.0),
    Color::new(0.17, 0.17, 0.23, 1.0),
];

// Breakable block (8x8)
const BREAKABLE_ART: [&str; 8] = [
    "11111111",
    "12221221",
    "23312331",
    "23312331",
    "11111111",
    "12212221",
    "23312331",
    "11111111",
];
const BREAKABLE_COLORS: [Color; 3] = [
    Color::new(0.53, 0.40, 0.13, 1.0),
    Color::new(0.67, 0.53, 0.27, 1.0),
    Color::new(0.47, 0.40, 0.20, 1.0),
];

// Spike tile (8x8)
const SPIKE_ART: [&str; 8] = [
    "........",
    "........",
    "...1...1",
    "..111.11",
    ".1111111",
    "11111111",
    "11111111",
    "11111111",
];
const SPIKE_COLORS: [Color; 1] = [
    Color::new(0.53, 0.53, 0.53, 1.0),
];

// Powerup crystal shard (8x8)
const SHARD_ART: [&str; 8] = [
    "...11...",
    "..1221..",
    ".123321.",
    "12333321",
    "12333321",
    ".123321.",
    "..1221..",
    "...11...",
];
const SHARD_COLORS: [Color; 3] = [
    Color::new(1.0, 0.53, 0.0, 1.0),
    Color::new(1.0, 0.80, 0.0, 1.0),
    WHITE,
];

// Fire crystal (8x8)
const FIRE_CRYSTAL_ART: [&str; 8] = [
    "...11...",
    "..1221..",
    ".123321.",
    "12333321",
    "12333321",
    ".123321.",
    "..1221..",
    "...11...",
];
const FIRE_CRYSTAL_COLORS: [Color; 3] = [
    Color::new(1.0, 0.13, 0.0, 1.0),
    Color::new(1.0, 0.40, 0.0, 1.0),
    Color::new(1.0, 1.0, 0.0, 1.0),
];

// Fireball (8x8)
const FIREBALL_ART: [&str; 8] = [
    "........",
    "..111...",
    ".12221..",
    ".12321..",
    ".12321..",
    ".12221..",
    "..111...",
    "........",
];
const FIREBALL_COLORS: [Color; 3] = [
    Color::new(1.0, 0.27, 0.0, 1.0),
    Color::new(1.0, 0.53, 0.0, 1.0),
    Color::new(1.0, 1.0, 0.0, 1.0),
];

// ---------------------------------------------------------------------------
// Sprite builder
// ---------------------------------------------------------------------------
fn create_sprite(art: &[&str], colors: &[Color]) -> Texture2D {
    let width = art[0].len() as u16;
    let height = art.len() as u16;
    let mut img = Image::gen_image_color(width, height, BLANK);
    for (y, row) in art.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch != '.' {
                if let Some(digit) = ch.to_digit(16) {
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

fn create_flipped_sprite(art: &[&str], colors: &[Color]) -> Texture2D {
    let width = art[0].len() as u16;
    let height = art.len() as u16;
    let mut img = Image::gen_image_color(width, height, BLANK);
    for (y, row) in art.iter().enumerate() {
        let chars: Vec<char> = row.chars().collect();
        let w = chars.len();
        for (x, &ch) in chars.iter().enumerate() {
            if ch != '.' {
                if let Some(digit) = ch.to_digit(16) {
                    let idx = (digit as usize).wrapping_sub(1);
                    if idx < colors.len() {
                        img.set_pixel((w - 1 - x) as u32, y as u32, colors[idx]);
                    }
                }
            }
        }
    }
    let tex = Texture2D::from_image(&img);
    tex.set_filter(FilterMode::Nearest);
    tex
}

struct Sprites {
    knight_small_r: Texture2D,
    knight_small_l: Texture2D,
    knight_big_r: Texture2D,
    knight_big_l: Texture2D,
    sword_r: Texture2D,
    sword_l: Texture2D,
    slime: Texture2D,
    bat: Texture2D,
    boss: Texture2D,
    crystal: Texture2D,
    ground: Texture2D,
    cave: Texture2D,
    tower: Texture2D,
    breakable: Texture2D,
    spike: Texture2D,
    shard: Texture2D,
    fire_crystal: Texture2D,
    fireball: Texture2D,
}

impl Sprites {
    fn new() -> Self {
        Self {
            knight_small_r: create_sprite(&KNIGHT_SMALL_ART, &KNIGHT_SMALL_COLORS),
            knight_small_l: create_flipped_sprite(&KNIGHT_SMALL_ART, &KNIGHT_SMALL_COLORS),
            knight_big_r: create_sprite(&KNIGHT_BIG_ART, &KNIGHT_BIG_COLORS),
            knight_big_l: create_flipped_sprite(&KNIGHT_BIG_ART, &KNIGHT_BIG_COLORS),
            sword_r: create_sprite(&SWORD_ART, &SWORD_COLORS),
            sword_l: create_flipped_sprite(&SWORD_ART, &SWORD_COLORS),
            slime: create_sprite(&SLIME_ART, &SLIME_COLORS),
            bat: create_sprite(&BAT_ART_1, &BAT_COLORS),
            boss: create_sprite(&BOSS_ART, &BOSS_COLORS),
            crystal: create_sprite(&CRYSTAL_ART, &CRYSTAL_COLORS),
            ground: create_sprite(&GROUND_ART, &GROUND_COLORS),
            cave: create_sprite(&CAVE_ART, &CAVE_COLORS),
            tower: create_sprite(&TOWER_ART, &TOWER_COLORS),
            breakable: create_sprite(&BREAKABLE_ART, &BREAKABLE_COLORS),
            spike: create_sprite(&SPIKE_ART, &SPIKE_COLORS),
            shard: create_sprite(&SHARD_ART, &SHARD_COLORS),
            fire_crystal: create_sprite(&FIRE_CRYSTAL_ART, &FIRE_CRYSTAL_COLORS),
            fireball: create_sprite(&FIREBALL_ART, &FIREBALL_COLORS),
        }
    }
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
enum EnemyKind {
    Slime,
    Bat,
}

#[derive(Clone, Copy, PartialEq)]
enum PowerupKind {
    Shard,
    FireCrystal,
    Goal,
}

#[allow(dead_code)]
struct Enemy {
    kind: EnemyKind,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    alive: bool,
    hp: i32,
    start_x: f32,
    base_y: f32,
    anim_timer: f32,
}

#[allow(dead_code)]
struct Boss {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    hp: i32,
    max_hp: i32,
    alive: bool,
    attack_timer: i32,
    flash_timer: i32,
    charging: bool,
    charge_timer: i32,
    start_x: f32,
}

struct CrystalGem {
    x: f32,
    y: f32,
    alive: bool,
    sparkle: f32,
}

struct Powerup {
    x: f32,
    y: f32,
    kind: PowerupKind,
    alive: bool,
}

struct Fireball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    alive: bool,
    bounces: i32,
}

struct SwordSlash {
    x: f32,
    y: f32,
    facing_right: bool,
    timer: i32,
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    max_life: f32,
    color: Color,
    size: f32,
}

struct Popup {
    text: String,
    x: f32,
    y: f32,
    life: f32,
    vy: f32,
    color: Color,
}

struct DustMote {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    alpha: f32,
}

struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    w: f32,
    h: f32,
    on_ground: bool,
    facing_right: bool,
    big: bool,
    has_fire: bool,
    lives: i32,
    score: i32,
    coins: i32,
    dead: bool,
    coyote_frames: i32,
    jump_buffer: i32,
    jump_held: bool,
    attack_cooldown: i32,
    anim_timer: f32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 64.0, y: 320.0,
            vx: 0.0, vy: 0.0,
            w: SMALL_W, h: SMALL_H,
            on_ground: false,
            facing_right: true,
            big: false,
            has_fire: false,
            lives: 3,
            score: 0,
            coins: 0,
            dead: false,
            coyote_frames: 0,
            jump_buffer: 0,
            jump_held: false,
            attack_cooldown: 0,
            anim_timer: 0.0,
        }
    }
}

// ---------------------------------------------------------------------------
// AABB overlap
// ---------------------------------------------------------------------------
fn overlaps(ax: f32, ay: f32, aw: f32, ah: f32, bx: f32, by: f32, bw: f32, bh: f32) -> bool {
    ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
}

// ---------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------
struct Input {
    left: bool,
    right: bool,
    jump: bool,
    jump_pressed: bool,
    attack_pressed: bool,
    start: bool,
    prev_jump: bool,
    prev_attack: bool,
    prev_start: bool,
}

impl Input {
    fn new() -> Self {
        Self {
            left: false, right: false,
            jump: false, jump_pressed: false,
            attack_pressed: false, start: false,
            prev_jump: false, prev_attack: false, prev_start: false,
        }
    }

    fn poll(&mut self) {
        self.left = is_key_down(KeyCode::Left);
        self.right = is_key_down(KeyCode::Right);

        let jump_now = is_key_down(KeyCode::Space);
        self.jump_pressed = jump_now && !self.prev_jump;
        self.jump = jump_now;
        self.prev_jump = jump_now;

        let attack_now = is_key_down(KeyCode::X);
        self.attack_pressed = attack_now && !self.prev_attack;
        self.prev_attack = attack_now;

        let start_now = is_key_down(KeyCode::Enter);
        self.start = start_now && !self.prev_start;
        self.prev_start = start_now;
    }
}

// ---------------------------------------------------------------------------
// Story data
// ---------------------------------------------------------------------------
static STORY_INTRO: &[&str] = &[
    "Long ago, the Crystal Heart sustained",
    "all life in the kingdom with its light.",
    "",
    "But the sorcerer Malachar shattered it,",
    "scattering the fragments across the land.",
    "Eternal twilight fell upon the realm.",
    "",
    "You are the last Pixel Knight.",
    "Take up your sword and journey forth.",
    "Reclaim the crystal shards.",
    "Restore the light.",
];

static STORY_AFTER_1: &[&str] = &[
    "The first shard glows warm in your hand.",
    "You feel its power, a faint pulse of light",
    "pushing back the twilight around you.",
    "",
    "But the sorcerer's corruption runs deeper",
    "than you imagined. The caverns below",
    "writhe with dark magic.",
    "",
    "You descend into the Shadow Caverns.",
];

static STORY_AFTER_2: &[&str] = &[
    "Two shards recovered. The darkness recoils.",
    "But Malachar knows you are coming.",
    "",
    "His Dark Tower rises before you,",
    "a spire of obsidian and malice.",
    "His strongest lieutenant awaits within.",
    "",
    "This is the final push.",
    "The Crystal Heart must be made whole.",
];

static STORY_LEVEL_1: &[&str] = &[
    "LEVEL 1: CRYSTAL MEADOWS",
    "",
    "The meadows have grown dark and cold.",
    "Slimes and bats roam the corrupted fields.",
    "Find the crystal shard hidden at the end.",
    "",
    "D-Pad: Move   B(Space): Jump",
    "A(X): Sword / Fireball",
];

static STORY_LEVEL_2: &[&str] = &[
    "LEVEL 2: SHADOW CAVERNS",
    "",
    "The caves twist deeper into the earth.",
    "Strange echoes bounce off crystalline walls.",
    "The corruption is stronger here.",
    "",
    "Watch for spikes and falling bats!",
];

static STORY_LEVEL_3: &[&str] = &[
    "LEVEL 3: THE DARK TOWER",
    "",
    "Malachar's tower pierces the twilight sky.",
    "His dark knights patrol every corridor.",
    "At the summit, the Dark Knight awaits.",
    "",
    "Defeat him and restore the Crystal Heart!",
];

static STORY_VICTORY: &[&str] = &[
    "The Dark Knight falls. The final shard",
    "flies free from the broken armor.",
    "",
    "You hold all three fragments together.",
    "They merge, blazing with radiant light.",
    "",
    "The Crystal Heart is whole once more.",
    "",
    "The eternal twilight shatters.",
    "Dawn breaks across the kingdom",
    "for the first time in a hundred years.",
    "",
    "The Crystal Kingdom is saved.",
    "Your quest is complete, Pixel Knight.",
];

// ---------------------------------------------------------------------------
// Level maps
// '.' = empty, '#' = solid, 'K' = breakable, 'C' = crystal, 'S' = slime,
// 'B' = bat, 'P' = shard powerup, 'F' = fire crystal, '^' = spike,
// 'X' = boss, 'G' = goal, '@' = player spawn
// ---------------------------------------------------------------------------
static LEVEL_1_MAP: &[&str] = &[
    "............................................................................................",
    "............................................................................................",
    "............................................................................................",
    "............................................................................................",
    "...........C.C.C........................C.C.C......................................G........",
    "..........#######..........C............#######...C.C.C.........P................###........",
    "........................C.C.C.C................C..#######...C.C.C................###........",
    "..........................##.##..KKKKK.......####.......C..########.....C.C......###........",
    "............................................................................................",
    ".....................S............C...C.........S...........S.....C.C..####.......###........",
    ".........C........#####........########..S...#####......######........S..........###........",
    "..@.............S..............................................................................",
    "####.........####..............................................................................",
    "####..^^^.S..####...###..####..####..####..####..####..####..####..####.......##############",
    "############################################################################################",
];

static LEVEL_2_MAP: &[&str] = &[
    "############################################################################################",
    "#..........................................................................................#",
    "#...C.C.C.....B.......C.C.....B............C.C.C...............B........................G..#",
    "#..#######.........C........KKKK.......C..#######...............................####......#",
    "#......................####...................C.........C.C.C........B..........######......#",
    "#.............B..S..........C.C............#####.......########....C............######......#",
    "#..@......########.....S...........####..........S..............###.....####....######......#",
    "####...S..........^^^^.####.....###....F...####............S.............S.....######......#",
    "####.####............####..^^^.....########.....KKKK..####..S..........####...######......#",
    "####........^^^^...###...####............S...####..####...####.............P..######......#",
    "####....############.......####...####..####.........S....S.....^^^^........########......#",
    "########################.####.########.########.####.####.####.####.####.################.#",
    "############################################################################################",
];

static LEVEL_3_MAP: &[&str] = &[
    "############################################################################################",
    "#........................................................................................X..#",
    "#..........................................................................................#",
    "#.........................................................C.C.C.........................####",
    "#..........B..........B....C.C...B.................C..########..........C.C.C...........####",
    "#...C.C........C.C.C...########...........KKKK.......C............C..########..........####",
    "#..#####.....########.....S.......C.C............####.....C.C......S.......####........####",
    "#..@........S........F...........########........S........######...........S............####",
    "####....#####.^^^^.####.....S........^^^^...#####............^^^^.####..####............####",
    "####................####.####............S.####.......####......S.......^^^^.........#####.#",
    "####...^^^^.......####.####...####..####.........S.......S.....^^^^................####...#",
    "####.##############################.####.########.########.####.####.####.####.####.######.#",
    "############################################################################################",
];

fn get_level_map(idx: usize) -> &'static [&'static str] {
    match idx {
        0 => LEVEL_1_MAP,
        1 => LEVEL_2_MAP,
        _ => LEVEL_3_MAP,
    }
}

fn get_level_name(idx: usize) -> &'static str {
    match idx {
        0 => "Crystal Meadows",
        1 => "Shadow Caverns",
        _ => "The Dark Tower",
    }
}

fn get_level_story(idx: usize) -> &'static [&'static str] {
    match idx {
        0 => STORY_LEVEL_1,
        1 => STORY_LEVEL_2,
        _ => STORY_LEVEL_3,
    }
}

fn get_after_level_story(idx: usize) -> &'static [&'static str] {
    match idx {
        0 => STORY_AFTER_1,
        _ => STORY_AFTER_2,
    }
}

#[derive(Clone, Copy, PartialEq)]
enum TileType {
    Ground,
    Cave,
    Tower,
}

fn get_tile_type(idx: usize) -> TileType {
    match idx {
        0 => TileType::Ground,
        1 => TileType::Cave,
        _ => TileType::Tower,
    }
}

// ---------------------------------------------------------------------------
// Tile grid (web-style)
// ---------------------------------------------------------------------------
struct TileGrid {
    tiles: Vec<char>,
    w: usize,
    h: usize,
}

impl TileGrid {
    fn new() -> Self {
        Self { tiles: Vec::new(), w: 0, h: 0 }
    }

    fn get(&self, tx: i32, ty: i32) -> char {
        if tx < 0 || tx >= self.w as i32 || ty < 0 || ty >= self.h as i32 {
            return '#';
        }
        self.tiles[ty as usize * self.w + tx as usize]
    }

    fn set(&mut self, tx: i32, ty: i32, val: char) {
        if tx >= 0 && (tx as usize) < self.w && ty >= 0 && (ty as usize) < self.h {
            self.tiles[ty as usize * self.w + tx as usize] = val;
        }
    }

    fn is_solid(&self, tx: i32, ty: i32) -> bool {
        let t = self.get(tx, ty);
        t == '#' || t == 'K'
    }
}

// ---------------------------------------------------------------------------
// World
// ---------------------------------------------------------------------------
struct World {
    state: GameState,
    current_level: usize,
    tile_type: TileType,

    player: Player,
    grid: TileGrid,
    enemies: Vec<Enemy>,
    boss: Option<Boss>,
    crystals: Vec<CrystalGem>,
    powerups: Vec<Powerup>,
    fireballs: Vec<Fireball>,
    sword_slash: Option<SwordSlash>,
    particles: Vec<Particle>,
    popups: Vec<Popup>,
    dust_motes: Vec<DustMote>,

    camera_x: f32,

    shake_magnitude: f32,
    screen_shake_x: f32,
    screen_shake_y: f32,
    damage_flash_timer: f32,
    invincible_timer: i32,
    growth_anim: i32,
    death_timer: i32,
    level_complete_timer: i32,
    boss_defeated: bool,
    frame_count: u64,

    // Story/typewriter
    story_lines: &'static [&'static str],
    story_line_index: usize,
    story_char_index: usize,
    story_frame_counter: u64,
    story_full_text: String,
    story_is_victory: bool,
}

impl World {
    fn new() -> Self {
        Self {
            state: GameState::Start,
            current_level: 0,
            tile_type: TileType::Ground,

            player: Player::new(),
            grid: TileGrid::new(),
            enemies: Vec::new(),
            boss: None,
            crystals: Vec::new(),
            powerups: Vec::new(),
            fireballs: Vec::new(),
            sword_slash: None,
            particles: Vec::new(),
            popups: Vec::new(),
            dust_motes: Vec::new(),

            camera_x: 0.0,

            shake_magnitude: 0.0,
            screen_shake_x: 0.0,
            screen_shake_y: 0.0,
            damage_flash_timer: 0.0,
            invincible_timer: 0,
            growth_anim: 0,
            death_timer: 0,
            level_complete_timer: 0,
            boss_defeated: false,
            frame_count: 0,

            story_lines: &[],
            story_line_index: 0,
            story_char_index: 0,
            story_frame_counter: 0,
            story_full_text: String::new(),
            story_is_victory: false,
        }
    }

    fn start_story(&mut self, lines: &'static [&'static str], is_victory: bool) {
        self.story_lines = lines;
        self.story_line_index = 0;
        self.story_char_index = 0;
        self.story_frame_counter = 0;
        self.story_full_text = String::new();
        self.story_is_victory = is_victory;
        self.state = GameState::Story;
    }

    fn start_level_story(&mut self) {
        let lines = get_level_story(self.current_level);
        self.story_lines = lines;
        self.story_line_index = 0;
        self.story_char_index = 0;
        self.story_frame_counter = 0;
        self.story_full_text = String::new();
        self.story_is_victory = false;
        self.state = GameState::LevelStory;
    }

    fn load_level(&mut self) {
        self.enemies.clear();
        self.crystals.clear();
        self.powerups.clear();
        self.fireballs.clear();
        self.particles.clear();
        self.popups.clear();
        self.dust_motes.clear();
        self.boss = None;
        self.boss_defeated = false;
        self.level_complete_timer = 0;
        self.sword_slash = None;

        self.tile_type = get_tile_type(self.current_level);

        let map = get_level_map(self.current_level);
        let map_h = map.len();
        // Find max width
        let mut map_w: usize = 0;
        for row in map.iter() {
            if row.len() > map_w {
                map_w = row.len();
            }
        }

        self.grid.w = map_w;
        self.grid.h = map_h;
        self.grid.tiles = vec!['.'; map_w * map_h];

        for (row, line) in map.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let px = col as f32 * TILE;
                let py = row as f32 * TILE;

                match ch {
                    '@' => {
                        self.player.x = px + 12.0;
                        self.player.y = py + 8.0;
                        self.grid.tiles[row * map_w + col] = '.';
                    }
                    'C' => {
                        self.crystals.push(CrystalGem {
                            x: px + 8.0, y: py + 8.0,
                            alive: true,
                            sparkle: rand::gen_range(0.0_f32, 100.0),
                        });
                        self.grid.tiles[row * map_w + col] = '.';
                    }
                    'S' => {
                        self.enemies.push(Enemy {
                            kind: EnemyKind::Slime,
                            x: px, y: py,
                            vx: 1.0, vy: 0.0,
                            alive: true, hp: 1,
                            start_x: px, base_y: py,
                            anim_timer: 0.0,
                        });
                        self.grid.tiles[row * map_w + col] = '.';
                    }
                    'B' => {
                        self.enemies.push(Enemy {
                            kind: EnemyKind::Bat,
                            x: px, y: py,
                            vx: -1.5, vy: 0.0,
                            alive: true, hp: 1,
                            start_x: px, base_y: py,
                            anim_timer: rand::gen_range(0.0_f32, 100.0),
                        });
                        self.grid.tiles[row * map_w + col] = '.';
                    }
                    'X' => {
                        self.boss = Some(Boss {
                            x: px - 16.0, y: py - 16.0,
                            vx: 1.5, vy: 0.0,
                            hp: BOSS_HP, max_hp: BOSS_HP,
                            alive: true,
                            attack_timer: 0,
                            flash_timer: 0,
                            charging: false,
                            charge_timer: 0,
                            start_x: px - 16.0,
                        });
                        self.grid.tiles[row * map_w + col] = '.';
                    }
                    'P' => {
                        self.powerups.push(Powerup {
                            x: px + 8.0, y: py + 8.0,
                            kind: PowerupKind::Shard, alive: true,
                        });
                        self.grid.tiles[row * map_w + col] = '.';
                    }
                    'F' => {
                        self.powerups.push(Powerup {
                            x: px + 8.0, y: py + 8.0,
                            kind: PowerupKind::FireCrystal, alive: true,
                        });
                        self.grid.tiles[row * map_w + col] = '.';
                    }
                    'G' => {
                        self.powerups.push(Powerup {
                            x: px + 8.0, y: py + 8.0,
                            kind: PowerupKind::Goal, alive: true,
                        });
                        self.grid.tiles[row * map_w + col] = '.';
                    }
                    '^' => {
                        self.grid.tiles[row * map_w + col] = '^';
                    }
                    _ => {
                        self.grid.tiles[row * map_w + col] = ch;
                    }
                }
            }
        }

        self.camera_x = 0.0;
    }

    fn reset_player(&mut self) {
        self.player.vx = 0.0;
        self.player.vy = 0.0;
        self.player.w = SMALL_W;
        self.player.h = SMALL_H;
        self.player.on_ground = false;
        self.player.facing_right = true;
        self.player.big = false;
        self.player.has_fire = false;
        self.player.dead = false;
        self.player.coyote_frames = 0;
        self.player.jump_buffer = 0;
        self.player.jump_held = false;
        self.player.attack_cooldown = 0;
        self.player.anim_timer = 0.0;
        self.sword_slash = None;
        self.growth_anim = 0;
        self.invincible_timer = 0;
        self.damage_flash_timer = 0.0;
        self.shake_magnitude = 0.0;
        self.screen_shake_x = 0.0;
        self.screen_shake_y = 0.0;
    }

    fn spawn_particles(&mut self, x: f32, y: f32, count: usize, color: Color, speed: f32) {
        let budget = 200_usize.saturating_sub(self.particles.len());
        let actual = count.min(budget);
        for _ in 0..actual {
            let angle: f32 = rand::gen_range(0.0, 6.28);
            let spd: f32 = rand::gen_range(1.0, speed + 1.0);
            self.particles.push(Particle {
                x, y,
                vx: angle.cos() * spd,
                vy: angle.sin() * spd - 2.0,
                life: rand::gen_range(30.0, 50.0),
                max_life: 50.0,
                color,
                size: rand::gen_range(2.0, 4.0),
            });
        }
    }

    fn add_popup(&mut self, text: &str, x: f32, y: f32, color: Color) {
        self.popups.push(Popup {
            text: text.to_string(),
            x, y,
            life: 60.0,
            vy: -1.5,
            color,
        });
    }

    fn player_hit(&mut self) {
        if self.invincible_timer > 0 { return; }
        if self.player.big {
            self.player.big = false;
            self.player.has_fire = false;
            self.player.w = SMALL_W;
            self.player.h = SMALL_H;
            self.growth_anim = 10;
            self.invincible_timer = INVULN_MAX;
            self.damage_flash_timer = 30.0;
            self.shake_magnitude = 4.0;
        } else {
            self.player_die();
        }
    }

    fn player_die(&mut self) {
        if self.player.dead { return; }
        self.player.dead = true;
        self.player.vy = -8.0;
        self.player.vx = 0.0;
        self.death_timer = 90;
        self.shake_magnitude = 8.0;
        self.spawn_particles(
            self.player.x + self.player.w * 0.5,
            self.player.y + self.player.h * 0.5,
            15, Color::new(1.0, 0.27, 0.27, 1.0), 4.0,
        );
    }

    fn boss_die(&mut self) {
        if let Some(ref mut b) = self.boss {
            b.alive = false;
        }
        self.boss_defeated = true;
        self.player.score += 2000;
        self.shake_magnitude = 10.0;

        let (bx, by) = if let Some(ref b) = self.boss {
            (b.x + BOSS_W * 0.5, b.y + BOSS_H * 0.5)
        } else {
            (0.0, 0.0)
        };
        self.spawn_particles(bx, by, 30, Color::new(1.0, 0.27, 0.27, 1.0), 5.0);
        self.spawn_particles(bx, by, 20, Color::new(1.0, 0.67, 0.0, 1.0), 4.0);
        self.add_popup("+2000", bx - 20.0, by - 20.0, Color::new(1.0, 1.0, 0.0, 1.0));
        self.level_complete_timer = 120;
    }

    fn update(&mut self, input: &Input) {
        self.frame_count += 1;

        // Screen shake decay
        if self.shake_magnitude > 0.1 {
            self.screen_shake_x = rand::gen_range(-self.shake_magnitude, self.shake_magnitude);
            self.screen_shake_y = rand::gen_range(-self.shake_magnitude, self.shake_magnitude);
            self.shake_magnitude *= 0.85;
        } else {
            self.shake_magnitude = 0.0;
            self.screen_shake_x = 0.0;
            self.screen_shake_y = 0.0;
        }

        if self.damage_flash_timer > 0.0 {
            self.damage_flash_timer -= 1.0;
        }

        // Death state
        if self.player.dead {
            self.death_timer -= 1;
            self.player.vy += GRAVITY;
            self.player.y += self.player.vy;
            // Update particles during death
            for i in 0..self.particles.len() {
                self.particles[i].x += self.particles[i].vx;
                self.particles[i].y += self.particles[i].vy;
                self.particles[i].vy += 0.1;
                self.particles[i].life -= 1.0;
            }
            self.particles.retain(|p| p.life > 0.0);

            if self.death_timer <= 0 {
                self.player.lives -= 1;
                if self.player.lives <= 0 {
                    self.state = GameState::GameOver;
                } else {
                    self.load_level();
                    self.reset_player();
                }
            }
            return;
        }

        // Level complete transition
        if self.level_complete_timer > 0 {
            self.level_complete_timer -= 1;
            if self.level_complete_timer <= 0 {
                if self.current_level < 2 {
                    let story = get_after_level_story(self.current_level);
                    self.current_level += 1;
                    self.story_lines = story;
                    self.story_line_index = 0;
                    self.story_char_index = 0;
                    self.story_frame_counter = 0;
                    self.story_full_text = String::new();
                    self.story_is_victory = false;
                    self.state = GameState::LevelStory;
                } else {
                    self.start_story(STORY_VICTORY, true);
                }
            }
            // Still update particles/popups during level complete
            for i in 0..self.particles.len() {
                self.particles[i].x += self.particles[i].vx;
                self.particles[i].y += self.particles[i].vy;
                self.particles[i].vy += 0.1;
                self.particles[i].life -= 1.0;
            }
            self.particles.retain(|p| p.life > 0.0);
            for i in 0..self.popups.len() {
                self.popups[i].y += self.popups[i].vy;
                self.popups[i].life -= 1.0;
            }
            self.popups.retain(|p| p.life > 0.0);
            return;
        }

        // Player movement
        let mut move_dir: f32 = 0.0;
        if input.left { move_dir = -1.0; }
        if input.right { move_dir = 1.0; }

        if move_dir != 0.0 {
            self.player.vx += move_dir * MOVE_ACCEL;
            if self.player.vx > MOVE_SPEED { self.player.vx = MOVE_SPEED; }
            if self.player.vx < -MOVE_SPEED { self.player.vx = -MOVE_SPEED; }
            self.player.facing_right = move_dir > 0.0;
            self.player.anim_timer += 1.0;
        } else {
            if self.player.vx > 0.0 {
                self.player.vx = (self.player.vx - MOVE_DECEL).max(0.0);
            } else if self.player.vx < 0.0 {
                self.player.vx = (self.player.vx + MOVE_DECEL).min(0.0);
            }
        }

        // Coyote time
        if self.player.on_ground {
            self.player.coyote_frames = COYOTE_MAX;
        } else if self.player.coyote_frames > 0 {
            self.player.coyote_frames -= 1;
        }

        // Jump buffer
        if input.jump_pressed {
            self.player.jump_buffer = JUMP_BUFFER_MAX;
        } else if self.player.jump_buffer > 0 {
            self.player.jump_buffer -= 1;
        }

        // Jump
        if self.player.jump_buffer > 0 && self.player.coyote_frames > 0 {
            self.player.vy = JUMP_FORCE;
            self.player.on_ground = false;
            self.player.coyote_frames = 0;
            self.player.jump_buffer = 0;
            self.player.jump_held = true;
        }
        if !input.jump {
            self.player.jump_held = false;
        }

        // Attack / fireball / sword
        if self.player.attack_cooldown > 0 {
            self.player.attack_cooldown -= 1;
        }
        if input.attack_pressed && self.player.attack_cooldown <= 0 {
            if self.player.has_fire {
                let active: usize = self.fireballs.iter().filter(|f| f.alive).count();
                if active < 3 {
                    self.player.attack_cooldown = 15;
                    let fb_offset_x = if self.player.big {
                        if self.player.facing_right { 28.0 } else { -8.0 }
                    } else {
                        if self.player.facing_right { 10.0 } else { -8.0 }
                    };
                    let fb_offset_y = if self.player.big { 12.0 } else { 4.0 };
                    let dir: f32 = if self.player.facing_right { 1.0 } else { -1.0 };
                    self.fireballs.push(Fireball {
                        x: self.player.x + fb_offset_x,
                        y: self.player.y + fb_offset_y,
                        vx: dir * FIREBALL_SPEED,
                        vy: 0.0,
                        alive: true,
                        bounces: 0,
                    });
                }
            } else {
                self.player.attack_cooldown = 15;
                let slash_offset_x = if self.player.big {
                    if self.player.facing_right { 28.0 } else { -16.0 }
                } else {
                    if self.player.facing_right { 10.0 } else { -16.0 }
                };
                let slash_offset_y = if self.player.big { 8.0 } else { 0.0 };
                self.sword_slash = Some(SwordSlash {
                    x: self.player.x + slash_offset_x,
                    y: self.player.y + slash_offset_y,
                    facing_right: self.player.facing_right,
                    timer: 8,
                });
            }
        }

        // Update sword slash
        if let Some(ref mut slash) = self.sword_slash {
            slash.timer -= 1;
            let slash_offset_x = if self.player.big {
                if slash.facing_right { 28.0 } else { -16.0 }
            } else {
                if slash.facing_right { 10.0 } else { -16.0 }
            };
            let slash_offset_y = if self.player.big { 8.0 } else { 0.0 };
            slash.x = self.player.x + slash_offset_x;
            slash.y = self.player.y + slash_offset_y;
        }
        // Extract slash info to avoid borrow issues
        let slash_expired = self.sword_slash.as_ref().map_or(false, |s| s.timer <= 0);
        let slash_hit_frame = self.sword_slash.as_ref().map_or(false, |s| s.timer == 7);
        let (slash_x, slash_y) = self.sword_slash.as_ref().map_or((0.0, 0.0), |s| (s.x, s.y));

        // Sword vs enemies (only on first frame of slash)
        if slash_hit_frame {
            for i in 0..self.enemies.len() {
                if !self.enemies[i].alive { continue; }
                if overlaps(slash_x, slash_y, 16.0, 14.0,
                           self.enemies[i].x, self.enemies[i].y, ENEMY_W, ENEMY_H) {
                    self.enemies[i].alive = false;
                    self.player.score += 200;
                    self.shake_magnitude = 3.0;
                    let ex = self.enemies[i].x + ENEMY_W * 0.5;
                    let ey = self.enemies[i].y + ENEMY_H * 0.5;
                    self.spawn_particles(ex, ey, 8, WHITE, 3.0);
                    self.add_popup("+200", ex, ey - 10.0, WHITE);
                }
            }
        }
        // Sword vs boss (only on first frame) -- defer particles
        let mut boss_killed_by_sword = false;
        let mut sword_boss_hit_pos = (0.0_f32, 0.0_f32);
        let mut sword_boss_hit = false;
        if slash_hit_frame {
            if let Some(ref mut b) = self.boss {
                if b.alive && overlaps(slash_x, slash_y, 16.0, 14.0,
                                       b.x, b.y, BOSS_W, BOSS_H) {
                    b.hp -= 1;
                    b.flash_timer = 8;
                    self.shake_magnitude = 3.0;
                    sword_boss_hit_pos = (b.x + BOSS_W * 0.5, b.y + BOSS_H * 0.5);
                    sword_boss_hit = true;
                    if b.hp <= 0 {
                        boss_killed_by_sword = true;
                    }
                }
            }
        }
        if sword_boss_hit {
            self.spawn_particles(sword_boss_hit_pos.0, sword_boss_hit_pos.1, 6, WHITE, 2.0);
        }
        if boss_killed_by_sword {
            self.boss_die();
        }
        if slash_expired {
            self.sword_slash = None;
        }

        // Growth animation
        if self.growth_anim > 0 {
            self.growth_anim -= 1;
        }

        // Gravity (variable jump height)
        let grav = if self.player.jump_held && self.player.vy < 0.0 {
            JUMP_HOLD_GRAVITY
        } else {
            GRAVITY
        };
        self.player.vy += grav;
        if self.player.vy > MAX_FALL {
            self.player.vy = MAX_FALL;
        }

        // Move player with tile-grid collision
        let pw = self.player.w;
        let ph = self.player.h;

        // Horizontal
        self.player.x += self.player.vx;

        let left_tile = (self.player.x / TILE) as i32;
        let _right_tile = ((self.player.x + pw - 1.0) / TILE) as i32;
        let top_tile = (self.player.y / TILE) as i32;
        let bot_tile = ((self.player.y + ph - 1.0) / TILE) as i32;

        if self.player.vx < 0.0 {
            for ty in top_tile..=bot_tile {
                if self.grid.is_solid(left_tile, ty) {
                    self.player.x = (left_tile + 1) as f32 * TILE;
                    self.player.vx = 0.0;
                    break;
                }
            }
        } else if self.player.vx > 0.0 {
            let rt = ((self.player.x + pw - 1.0) / TILE) as i32;
            for ty in top_tile..=bot_tile {
                if self.grid.is_solid(rt, ty) {
                    self.player.x = rt as f32 * TILE - pw;
                    self.player.vx = 0.0;
                    break;
                }
            }
        }

        // Vertical
        self.player.y += self.player.vy;
        self.player.on_ground = false;

        let left_tile2 = (self.player.x / TILE) as i32;
        let right_tile2 = ((self.player.x + pw - 1.0) / TILE) as i32;

        // Floor
        if self.player.vy >= 0.0 {
            let bt = ((self.player.y + ph - 1.0) / TILE) as i32;
            for tx in left_tile2..=right_tile2 {
                if self.grid.is_solid(tx, bt) {
                    self.player.y = bt as f32 * TILE - ph;
                    self.player.vy = 0.0;
                    self.player.on_ground = true;
                    break;
                }
            }
        }

        // Ceiling
        if self.player.vy <= 0.0 {
            let tt = (self.player.y / TILE) as i32;
            for tx in left_tile2..=right_tile2 {
                if self.grid.is_solid(tx, tt) {
                    self.player.y = (tt + 1) as f32 * TILE;
                    self.player.vy = 0.0;
                    // Hit breakable from below
                    if self.grid.get(tx, tt) == 'K' {
                        self.grid.set(tx, tt, '.');
                        self.shake_magnitude = 3.0;
                        let bx = tx as f32 * TILE + TILE * 0.5;
                        let by = tt as f32 * TILE + TILE * 0.5;
                        self.spawn_particles(bx, by, 8, Color::new(0.67, 0.53, 0.27, 1.0), 3.0);
                        // Chance to spawn crystal
                        if rand::gen_range(0.0_f32, 1.0) < 0.4 {
                            self.crystals.push(CrystalGem {
                                x: tx as f32 * TILE + TILE * 0.5,
                                y: tt as f32 * TILE - 8.0,
                                alive: true,
                                sparkle: 0.0,
                            });
                        }
                    }
                    break;
                }
            }
        }

        // Spike collision
        {
            let sl = (self.player.x / TILE) as i32;
            let sr = ((self.player.x + pw - 1.0) / TILE) as i32;
            let st = (self.player.y / TILE) as i32;
            let sb = ((self.player.y + ph - 1.0) / TILE) as i32;
            let mut hit_spike = false;
            for tx in sl..=sr {
                for ty in st..=sb {
                    if self.grid.get(tx, ty) == '^' {
                        hit_spike = true;
                        break;
                    }
                }
                if hit_spike { break; }
            }
            if hit_spike {
                self.player_die();
                return;
            }
        }

        // Fell off bottom
        if self.player.y > self.grid.h as f32 * TILE + 32.0 {
            self.player_die();
            return;
        }

        // Invincibility
        if self.invincible_timer > 0 {
            self.invincible_timer -= 1;
        }

        // Camera
        let target_cam = self.player.x - SCREEN_W * 0.5 + pw * 0.5;
        self.camera_x += (target_cam - self.camera_x) * 0.1;
        if self.camera_x < 0.0 { self.camera_x = 0.0; }
        let max_cam = self.grid.w as f32 * TILE - SCREEN_W;
        if max_cam > 0.0 && self.camera_x > max_cam {
            self.camera_x = max_cam;
        }

        // Collect crystals
        for i in 0..self.crystals.len() {
            if !self.crystals[i].alive { continue; }
            self.crystals[i].sparkle += 1.0;
            if overlaps(self.player.x, self.player.y, pw, ph,
                       self.crystals[i].x - 6.0, self.crystals[i].y - 6.0, 12.0, 12.0) {
                self.crystals[i].alive = false;
                self.player.coins += 1;
                self.player.score += 100;
                let cx = self.crystals[i].x;
                let cy = self.crystals[i].y;
                self.spawn_particles(cx, cy, 6, Color::new(0.27, 0.87, 1.0, 1.0), 2.0);
                self.add_popup("+100", cx, cy - 10.0, Color::new(0.27, 0.87, 1.0, 1.0));
            }
        }

        // Collect powerups
        for i in 0..self.powerups.len() {
            if !self.powerups[i].alive { continue; }
            if overlaps(self.player.x, self.player.y, pw, ph,
                       self.powerups[i].x - 8.0, self.powerups[i].y - 8.0, 16.0, 16.0) {
                match self.powerups[i].kind {
                    PowerupKind::Shard => {
                        self.powerups[i].alive = false;
                        if !self.player.big {
                            self.player.big = true;
                            self.player.w = BIG_W;
                            self.player.h = BIG_H;
                            self.player.y -= 12.0;
                            self.growth_anim = 10;
                            self.invincible_timer = self.invincible_timer.max(30);
                            let px = self.powerups[i].x;
                            let py = self.powerups[i].y;
                            self.spawn_particles(px, py, 10, Color::new(1.0, 0.80, 0.0, 1.0), 3.0);
                            self.add_popup("POWER UP!", px, py - 10.0, Color::new(1.0, 0.80, 0.0, 1.0));
                        } else {
                            self.player.score += 500;
                            let px = self.powerups[i].x;
                            let py = self.powerups[i].y;
                            self.add_popup("+500", px, py - 10.0, Color::new(1.0, 0.80, 0.0, 1.0));
                        }
                    }
                    PowerupKind::FireCrystal => {
                        self.powerups[i].alive = false;
                        self.player.has_fire = true;
                        let px = self.powerups[i].x;
                        let py = self.powerups[i].y;
                        self.spawn_particles(px, py, 10, Color::new(1.0, 0.40, 0.0, 1.0), 3.0);
                        self.add_popup("FIRE!", px, py - 10.0, Color::new(1.0, 0.40, 0.0, 1.0));
                    }
                    PowerupKind::Goal => {
                        if self.current_level < 2 {
                            self.powerups[i].alive = false;
                            self.player.score += 1000;
                            let px = self.powerups[i].x;
                            let py = self.powerups[i].y;
                            self.add_popup("+1000", px, py - 10.0, Color::new(1.0, 1.0, 0.0, 1.0));
                            self.level_complete_timer = 120;
                        } else if self.boss_defeated {
                            self.powerups[i].alive = false;
                            self.player.score += 2000;
                            self.level_complete_timer = 120;
                        }
                    }
                }
            }
        }

        // Enemy update
        for i in 0..self.enemies.len() {
            if !self.enemies[i].alive { continue; }

            self.enemies[i].anim_timer += 1.0;

            match self.enemies[i].kind {
                EnemyKind::Slime => {
                    self.enemies[i].x += self.enemies[i].vx;
                    self.enemies[i].vy += GRAVITY;
                    if self.enemies[i].vy > MAX_FALL { self.enemies[i].vy = MAX_FALL; }
                    self.enemies[i].y += self.enemies[i].vy;

                    // Ground collision
                    let s_bot = ((self.enemies[i].y + ENEMY_H - 1.0) / TILE) as i32;
                    let s_left = (self.enemies[i].x / TILE) as i32;
                    let s_right = ((self.enemies[i].x + ENEMY_W - 1.0) / TILE) as i32;
                    for tx in s_left..=s_right {
                        if self.grid.is_solid(tx, s_bot) {
                            self.enemies[i].y = s_bot as f32 * TILE - ENEMY_H;
                            self.enemies[i].vy = 0.0;
                        }
                    }

                    // Wall/edge detection
                    let front_tx = ((self.enemies[i].x + if self.enemies[i].vx > 0.0 { ENEMY_W } else { 0.0 }) / TILE) as i32;
                    let front_ty = ((self.enemies[i].y + ENEMY_H * 0.5) / TILE) as i32;
                    let floor_ty = ((self.enemies[i].y + ENEMY_H + 1.0) / TILE) as i32;
                    let edge_tx = ((self.enemies[i].x + if self.enemies[i].vx > 0.0 { ENEMY_W + 2.0 } else { -2.0 }) / TILE) as i32;
                    if self.grid.is_solid(front_tx, front_ty) || !self.grid.is_solid(edge_tx, floor_ty) {
                        self.enemies[i].vx = -self.enemies[i].vx;
                    }
                }
                EnemyKind::Bat => {
                    self.enemies[i].x += self.enemies[i].vx;
                    self.enemies[i].y = self.enemies[i].base_y + (self.enemies[i].anim_timer * 0.05).sin() * 30.0;

                    let bat_tx = ((self.enemies[i].x + if self.enemies[i].vx > 0.0 { ENEMY_W } else { 0.0 }) / TILE) as i32;
                    let bat_ty = ((self.enemies[i].y + ENEMY_H * 0.5) / TILE) as i32;
                    if self.grid.is_solid(bat_tx, bat_ty) || self.enemies[i].x < 0.0 || self.enemies[i].x > self.grid.w as f32 * TILE {
                        self.enemies[i].vx = -self.enemies[i].vx;
                    }
                }
            }

            // Player collision with enemy
            if self.invincible_timer <= 0 && !self.player.dead {
                if overlaps(self.player.x, self.player.y, pw, ph,
                           self.enemies[i].x, self.enemies[i].y, ENEMY_W, ENEMY_H) {
                    if self.player.vy > 0.0 && self.player.y + ph - 4.0 < self.enemies[i].y + ENEMY_H * 0.5 {
                        // Stomp
                        self.enemies[i].alive = false;
                        self.player.vy = STOMP_BOUNCE;
                        self.player.score += 200;
                        self.shake_magnitude = 4.0;
                        let c = match self.enemies[i].kind {
                            EnemyKind::Slime => Color::new(0.27, 0.73, 0.27, 1.0),
                            EnemyKind::Bat => Color::new(0.53, 0.27, 0.53, 1.0),
                        };
                        let ex = self.enemies[i].x + ENEMY_W * 0.5;
                        let ey = self.enemies[i].y + ENEMY_H * 0.5;
                        self.spawn_particles(ex, ey, 12, c, 3.0);
                        self.add_popup("+200", ex, ey - 10.0, Color::new(1.0, 1.0, 0.0, 1.0));
                    } else {
                        self.player_hit();
                    }
                }
            }
        }

        // Boss update
        let mut boss_killed = false;
        let mut boss_collision: i32 = 0; // 0=none, 1=stomp, 2=player hit
        let mut boss_stomp_pos = (0.0_f32, 0.0_f32);
        if let Some(ref mut b) = self.boss {
            if b.alive {
                b.attack_timer += 1;
                if b.flash_timer > 0 { b.flash_timer -= 1; }

                if !b.charging {
                    b.x += b.vx;
                    b.vy += GRAVITY;
                    if b.vy > MAX_FALL { b.vy = MAX_FALL; }
                    b.y += b.vy;

                    // Ground
                    let b_bot = ((b.y + BOSS_H - 1.0) / TILE) as i32;
                    let b_left = (b.x / TILE) as i32;
                    let b_right = ((b.x + BOSS_W - 1.0) / TILE) as i32;
                    for tx in b_left..=b_right {
                        if self.grid.is_solid(tx, b_bot) {
                            b.y = b_bot as f32 * TILE - BOSS_H;
                            b.vy = 0.0;
                        }
                    }

                    // Wall
                    let b_front_tx = ((b.x + if b.vx > 0.0 { BOSS_W } else { 0.0 }) / TILE) as i32;
                    let b_front_ty = ((b.y + BOSS_H * 0.5) / TILE) as i32;
                    if self.grid.is_solid(b_front_tx, b_front_ty) {
                        b.vx = -b.vx;
                    }

                    // Charge trigger
                    if b.attack_timer > 120 && (self.player.x - b.x).abs() < 300.0 {
                        b.charging = true;
                        b.charge_timer = 40;
                        b.vx = if self.player.x > b.x { 4.0 } else { -4.0 };
                        b.attack_timer = 0;
                    }
                } else {
                    b.x += b.vx;
                    b.vy += GRAVITY;
                    if b.vy > MAX_FALL { b.vy = MAX_FALL; }
                    b.y += b.vy;

                    let b_bot = ((b.y + BOSS_H - 1.0) / TILE) as i32;
                    let b_left = (b.x / TILE) as i32;
                    let b_right = ((b.x + BOSS_W - 1.0) / TILE) as i32;
                    for tx in b_left..=b_right {
                        if self.grid.is_solid(tx, b_bot) {
                            b.y = b_bot as f32 * TILE - BOSS_H;
                            b.vy = 0.0;
                        }
                    }

                    let b_front_tx = ((b.x + if b.vx > 0.0 { BOSS_W } else { 0.0 }) / TILE) as i32;
                    let b_front_ty = ((b.y + BOSS_H * 0.5) / TILE) as i32;
                    if self.grid.is_solid(b_front_tx, b_front_ty) {
                        b.vx = -b.vx;
                        b.charging = false;
                    }

                    b.charge_timer -= 1;
                    if b.charge_timer <= 0 {
                        b.charging = false;
                        b.vx = if self.player.x > b.x { 1.5 } else { -1.5 };
                    }
                }

                // Boss-player collision -- collect result to handle after borrow ends
                if self.invincible_timer <= 0 && !self.player.dead {
                    if overlaps(self.player.x, self.player.y, pw, ph,
                               b.x, b.y, BOSS_W, BOSS_H) {
                        if self.player.vy > 0.0 && self.player.y + ph - 4.0 < b.y + BOSS_H * 0.5 {
                            b.hp -= 1;
                            b.flash_timer = 8;
                            self.player.vy = STOMP_BOUNCE;
                            self.shake_magnitude = 5.0;
                            boss_stomp_pos = (b.x + BOSS_W * 0.5, b.y);
                            boss_collision = 1;
                            if b.hp <= 0 {
                                boss_killed = true;
                            }
                        } else {
                            boss_collision = 2;
                        }
                    }
                }
            }
        }
        // Handle deferred boss collision results
        if boss_collision == 1 {
            self.spawn_particles(boss_stomp_pos.0, boss_stomp_pos.1, 8, Color::new(1.0, 0.27, 0.27, 1.0), 3.0);
        } else if boss_collision == 2 {
            self.player_hit();
        }
        if boss_killed {
            self.boss_die();
        }

        // Fireball update
        for i in 0..self.fireballs.len() {
            if !self.fireballs[i].alive { continue; }

            self.fireballs[i].vy += 0.15;
            self.fireballs[i].x += self.fireballs[i].vx;
            self.fireballs[i].y += self.fireballs[i].vy;

            let f_tx = ((self.fireballs[i].x + 4.0) / TILE) as i32;
            let f_ty_bot = ((self.fireballs[i].y + 8.0) / TILE) as i32;
            let f_ty_top = (self.fireballs[i].y / TILE) as i32;

            // Bounce on floor
            if self.grid.is_solid(f_tx, f_ty_bot) && self.fireballs[i].vy > 0.0 {
                self.fireballs[i].vy = -4.0;
                self.fireballs[i].bounces += 1;
            }

            // Hit ceiling/wall
            if self.grid.is_solid(f_tx, f_ty_top) {
                self.fireballs[i].alive = false;
                let fx = self.fireballs[i].x;
                let fy = self.fireballs[i].y;
                self.spawn_particles(fx, fy, 4, Color::new(1.0, 0.40, 0.0, 1.0), 2.0);
            }

            // Hit breakable
            if self.fireballs[i].alive {
                let bk_tx = ((self.fireballs[i].x + 4.0) / TILE) as i32;
                let bk_ty = (self.fireballs[i].y / TILE) as i32;
                if self.grid.get(bk_tx, bk_ty) == 'K' {
                    self.grid.set(bk_tx, bk_ty, '.');
                    self.fireballs[i].alive = false;
                    let bx = bk_tx as f32 * TILE + 16.0;
                    let by = bk_ty as f32 * TILE + 16.0;
                    self.spawn_particles(bx, by, 6, Color::new(0.67, 0.53, 0.27, 1.0), 3.0);
                }
            }

            // Out of bounds or too many bounces
            if self.fireballs[i].bounces > 3
                || self.fireballs[i].y > self.grid.h as f32 * TILE
                || self.fireballs[i].x < 0.0
                || self.fireballs[i].x > self.grid.w as f32 * TILE {
                self.fireballs[i].alive = false;
            }

            // Hit enemies
            if self.fireballs[i].alive {
                for j in 0..self.enemies.len() {
                    if !self.enemies[j].alive { continue; }
                    if overlaps(self.fireballs[i].x - 4.0, self.fireballs[i].y - 4.0, 12.0, 12.0,
                               self.enemies[j].x, self.enemies[j].y, ENEMY_W, ENEMY_H) {
                        self.enemies[j].alive = false;
                        self.fireballs[i].alive = false;
                        self.player.score += 200;
                        let ex = self.enemies[j].x + ENEMY_W * 0.5;
                        let ey = self.enemies[j].y + ENEMY_H * 0.5;
                        self.spawn_particles(ex, ey, 10, Color::new(1.0, 0.40, 0.0, 1.0), 3.0);
                        self.add_popup("+200", ex, ey - 10.0, Color::new(1.0, 0.40, 0.0, 1.0));
                        break;
                    }
                }
            }

            // Hit boss -- defer spawn_particles to avoid borrow conflict
            let mut fb_boss_killed = false;
            let mut fb_boss_hit_pos = (0.0_f32, 0.0_f32);
            let mut fb_boss_hit = false;
            if self.fireballs[i].alive {
                if let Some(ref mut b) = self.boss {
                    if b.alive && overlaps(self.fireballs[i].x - 4.0, self.fireballs[i].y - 4.0, 12.0, 12.0,
                                           b.x, b.y, BOSS_W, BOSS_H) {
                        b.hp -= 1;
                        b.flash_timer = 8;
                        self.fireballs[i].alive = false;
                        self.shake_magnitude = 3.0;
                        fb_boss_hit_pos = (self.fireballs[i].x, self.fireballs[i].y);
                        fb_boss_hit = true;
                        if b.hp <= 0 {
                            fb_boss_killed = true;
                        }
                    }
                }
            }
            if fb_boss_hit {
                self.spawn_particles(fb_boss_hit_pos.0, fb_boss_hit_pos.1, 6, Color::new(1.0, 0.27, 0.27, 1.0), 2.0);
            }
            if fb_boss_killed {
                self.boss_die();
            }
        }
        self.fireballs.retain(|f| f.alive);

        // Particle update
        for i in 0..self.particles.len() {
            self.particles[i].x += self.particles[i].vx;
            self.particles[i].y += self.particles[i].vy;
            self.particles[i].vy += 0.1;
            self.particles[i].life -= 1.0;
        }
        self.particles.retain(|p| p.life > 0.0);

        // Popup update
        for i in 0..self.popups.len() {
            self.popups[i].y += self.popups[i].vy;
            self.popups[i].life -= 1.0;
        }
        self.popups.retain(|p| p.life > 0.0);

        // Dust motes
        if rand::gen_range(0.0_f32, 1.0) < 0.1 && self.dust_motes.len() < 80 {
            self.dust_motes.push(DustMote {
                x: self.camera_x + rand::gen_range(0.0_f32, SCREEN_W),
                y: rand::gen_range(0.0_f32, SCREEN_H),
                vx: rand::gen_range(-0.15_f32, 0.15),
                vy: rand::gen_range(0.1_f32, 0.3),
                life: rand::gen_range(200.0_f32, 400.0),
                alpha: rand::gen_range(0.1_f32, 0.3),
            });
        }
        for i in 0..self.dust_motes.len() {
            self.dust_motes[i].x += self.dust_motes[i].vx;
            self.dust_motes[i].y += self.dust_motes[i].vy;
            self.dust_motes[i].life -= 1.0;
        }
        self.dust_motes.retain(|d| d.life > 0.0 && d.y < SCREEN_H + 10.0);

        // Running trail particles
        if self.player.vx.abs() > 2.5 && self.player.on_ground && self.particles.len() < 200 {
            if self.frame_count % 3 == 0 {
                let trail_x = self.player.x + if self.player.big { 7.0 } else { 4.0 };
                let trail_y = self.player.y + if self.player.big { 22.0 } else { 11.0 };
                self.particles.push(Particle {
                    x: trail_x, y: trail_y,
                    vx: -self.player.vx * 0.2 + rand::gen_range(-0.5_f32, 0.5),
                    vy: -rand::gen_range(0.0_f32, 0.5),
                    life: 15.0, max_life: 15.0,
                    color: Color::new(0.67, 0.67, 0.80, 0.6),
                    size: 2.0,
                });
            }
        }
    }

    fn update_story(&mut self, input: &Input) {
        self.story_frame_counter += 1;

        // Typewriter
        if self.story_frame_counter % 2 == 0 {
            if self.story_line_index < self.story_lines.len() {
                let line = self.story_lines[self.story_line_index];
                if self.story_char_index < line.len() {
                    self.story_full_text.push(line.as_bytes()[self.story_char_index] as char);
                    self.story_char_index += 1;
                } else {
                    self.story_full_text.push('\n');
                    self.story_line_index += 1;
                    self.story_char_index = 0;
                }
            }
        }

        if input.jump_pressed || input.attack_pressed || input.start {
            if self.story_line_index < self.story_lines.len() {
                self.story_full_text = self.story_lines.join("\n");
                self.story_line_index = self.story_lines.len();
            } else {
                if self.story_is_victory {
                    self.state = GameState::Win;
                } else if self.state == GameState::Story {
                    // After intro story, go to level story
                    self.start_level_story();
                } else {
                    // After level story, start playing
                    self.load_level();
                    self.reset_player();
                    self.state = GameState::Playing;
                }
            }
        }
    }

    fn reset_game(&mut self) {
        self.player = Player::new();
        self.current_level = 0;
        self.boss_defeated = false;
        self.level_complete_timer = 0;
    }
}

// ---------------------------------------------------------------------------
// Drawing
// ---------------------------------------------------------------------------
fn draw_background(world: &World) {
    let (c0, c1, c2) = match world.tile_type {
        TileType::Ground => (
            Color::new(0.04, 0.09, 0.16, 1.0),
            Color::new(0.09, 0.13, 0.25, 1.0),
            Color::new(0.10, 0.16, 0.28, 1.0),
        ),
        TileType::Cave => (
            Color::new(0.03, 0.03, 0.06, 1.0),
            Color::new(0.06, 0.06, 0.13, 1.0),
            Color::new(0.05, 0.05, 0.09, 1.0),
        ),
        TileType::Tower => (
            Color::new(0.02, 0.02, 0.07, 1.0),
            Color::new(0.05, 0.05, 0.12, 1.0),
            Color::new(0.03, 0.03, 0.10, 1.0),
        ),
    };

    // Gradient background
    let steps = 20;
    let step_h = SCREEN_H / steps as f32;
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let c = if t < 0.5 {
            let tt = t * 2.0;
            Color::new(
                c0.r + (c1.r - c0.r) * tt,
                c0.g + (c1.g - c0.g) * tt,
                c0.b + (c1.b - c0.b) * tt,
                1.0,
            )
        } else {
            let tt = (t - 0.5) * 2.0;
            Color::new(
                c1.r + (c2.r - c1.r) * tt,
                c1.g + (c2.g - c1.g) * tt,
                c1.b + (c2.b - c1.b) * tt,
                1.0,
            )
        };
        draw_rectangle(0.0, i as f32 * step_h, SCREEN_W, step_h + 1.0, c);
    }

    // Parallax mountain silhouettes
    let parallax1 = world.camera_x * 0.15;
    let bg_c1 = match world.tile_type {
        TileType::Ground => Color::new(0.05, 0.12, 0.18, 1.0),
        TileType::Cave => Color::new(0.05, 0.05, 0.10, 1.0),
        TileType::Tower => Color::new(0.04, 0.04, 0.08, 1.0),
    };
    for i in 0..15_i32 {
        let bx = i as f32 * 120.0 - (parallax1 % 120.0) - 60.0;
        let bh = 60.0 + (i as f32 * 1.3).sin() * 40.0 + 20.0;
        // Simple triangle
        for y in 0..(bh as i32) {
            let frac = y as f32 / bh;
            let half_w = (1.0 - frac) * 60.0;
            draw_rectangle(bx + 60.0 - half_w, SCREEN_H - bh + y as f32, half_w * 2.0, 1.0, bg_c1);
        }
    }
}

fn draw_world(world: &World, sprites: &Sprites) {
    let cam_x = world.camera_x + world.screen_shake_x;
    let cam_y = world.screen_shake_y;

    clear_background(BLACK);
    draw_background(world);

    // Dust motes
    for mote in &world.dust_motes {
        let dx = mote.x - cam_x;
        if dx < -5.0 || dx > SCREEN_W + 5.0 { continue; }
        let fade = if mote.life > 50.0 { 1.0 } else { mote.life / 50.0 };
        draw_rectangle(dx, mote.y, 1.0, 1.0, Color::new(0.67, 0.73, 0.80, mote.alpha * fade));
    }

    // Tiles
    let start_tx = ((cam_x / TILE) as i32 - 1).max(0);
    let end_tx = ((cam_x + SCREEN_W) / TILE) as i32 + 2;
    for ty in 0..world.grid.h as i32 {
        for tx in start_tx..end_tx.min(world.grid.w as i32) {
            let t = world.grid.get(tx, ty);
            let sx = tx as f32 * TILE - cam_x;
            let sy = ty as f32 * TILE + cam_y;

            let tex = match t {
                '#' => match world.tile_type {
                    TileType::Ground => Some(&sprites.ground),
                    TileType::Cave => Some(&sprites.cave),
                    TileType::Tower => Some(&sprites.tower),
                },
                'K' => Some(&sprites.breakable),
                '^' => Some(&sprites.spike),
                _ => None,
            };
            if let Some(tex) = tex {
                draw_texture_ex(
                    tex, sx, sy, WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(TILE, TILE)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    // Crystals
    for gem in &world.crystals {
        if !gem.alive { continue; }
        let cx = (gem.x - cam_x).round();
        let bob = (gem.sparkle * 0.05).sin() * 3.0;
        let cy = (gem.y + bob + cam_y).round();
        if cx < -16.0 || cx > SCREEN_W + 16.0 { continue; }
        draw_texture_ex(
            &sprites.crystal, cx - 8.0, cy - 8.0, WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(16.0, 16.0)),
                ..Default::default()
            },
        );
        // Sparkle
        if (gem.sparkle as i32) % 30 < 5 {
            draw_rectangle(cx - 1.0, cy - 1.0, 2.0, 2.0, Color::new(1.0, 1.0, 1.0, 0.5));
        }
    }

    // Powerups
    for pu in &world.powerups {
        if !pu.alive { continue; }
        let px = (pu.x - cam_x).round();
        let bob = (world.frame_count as f32 * 0.05).sin() * 4.0;
        let py = (pu.y + bob + cam_y).round();
        if px < -16.0 || px > SCREEN_W + 16.0 { continue; }
        let tex = match pu.kind {
            PowerupKind::Shard => &sprites.shard,
            PowerupKind::FireCrystal => &sprites.fire_crystal,
            PowerupKind::Goal => &sprites.shard,
        };
        draw_texture_ex(
            tex, px - 8.0, py - 8.0, WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(16.0, 16.0)),
                ..Default::default()
            },
        );
        if pu.kind == PowerupKind::Goal {
            let glow = (world.frame_count as f32 * 0.08).sin() * 0.15 + 0.3;
            draw_circle(px, py, 14.0, Color::new(1.0, 0.80, 0.0, glow));
        }
    }

    // Enemies
    for enemy in &world.enemies {
        if !enemy.alive { continue; }
        let ex = (enemy.x - cam_x).round();
        let ey = (enemy.y + cam_y).round();
        if ex < -32.0 || ex > SCREEN_W + 32.0 { continue; }

        match enemy.kind {
            EnemyKind::Slime => {
                let squish = 1.0 + (enemy.anim_timer * 0.1).sin() * 0.1;
                draw_texture_ex(
                    &sprites.slime, ex, ey + (1.0 - squish) * 8.0, WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(ENEMY_W, ENEMY_H * squish)),
                        ..Default::default()
                    },
                );
            }
            EnemyKind::Bat => {
                draw_texture_ex(
                    &sprites.bat, ex, ey, WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(ENEMY_W, ENEMY_H)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    // Boss
    if let Some(ref b) = world.boss {
        if b.alive {
            let bx = (b.x - cam_x).round();
            let by = (b.y + cam_y).round();
            let tint = if b.flash_timer > 0 && b.flash_timer % 2 == 0 {
                Color::new(1.0, 1.0, 1.0, 0.6)
            } else {
                WHITE
            };
            draw_texture_ex(
                &sprites.boss, bx, by, tint,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(BOSS_W, BOSS_H)),
                    ..Default::default()
                },
            );
            // Health bar
            let bar_w = 60.0_f32;
            let bar_h = 5.0_f32;
            let bar_x = bx + BOSS_W * 0.5 - bar_w * 0.5;
            let bar_y = by - 10.0;
            draw_rectangle(bar_x, bar_y, bar_w, bar_h, Color::new(0.2, 0.2, 0.2, 0.8));
            let hp_frac = b.hp as f32 / b.max_hp as f32;
            draw_rectangle(bar_x, bar_y, bar_w * hp_frac, bar_h, RED);
            draw_rectangle_lines(bar_x, bar_y, bar_w, bar_h, 1.0, WHITE);
        }
    }

    // Fireballs
    for fb in &world.fireballs {
        if !fb.alive { continue; }
        let fx = (fb.x - cam_x).round();
        let fy = (fb.y + cam_y).round();
        draw_texture_ex(
            &sprites.fireball, fx - 8.0, fy - 8.0, WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(16.0, 16.0)),
                ..Default::default()
            },
        );
    }

    // Sword slash
    if let Some(ref slash) = world.sword_slash {
        if slash.timer > 0 {
            let sx = (slash.x - cam_x).round();
            let sy = (slash.y + cam_y).round();
            let alpha = slash.timer as f32 / 8.0;
            let tex = if slash.facing_right { &sprites.sword_r } else { &sprites.sword_l };
            draw_texture_ex(
                tex, sx, sy, Color::new(1.0, 1.0, 1.0, alpha),
                DrawTextureParams {
                    dest_size: Some(Vec2::new(16.0, 16.0)),
                    ..Default::default()
                },
            );
        }
    }

    // Player
    if !world.player.dead {
        let visible = world.invincible_timer == 0 || (world.invincible_timer / 3) % 2 != 0;
        if visible {
            let px = (world.player.x - cam_x).round();
            let py = (world.player.y + cam_y).round();

            let mut draw_scale = 1.0_f32;
            if world.growth_anim > 0 {
                draw_scale = 1.0 + (world.growth_anim as f32 * 0.6).sin() * 0.3;
            }

            if world.player.big {
                let tex = if world.player.facing_right { &sprites.knight_big_r } else { &sprites.knight_big_l };
                let dw = 32.0 * draw_scale;
                let dh = 48.0 * draw_scale;
                let draw_x = px - 1.0 - (dw - 32.0) * 0.5;
                let draw_y = py - 8.0 - (dh - 48.0);
                draw_texture_ex(
                    tex, draw_x, draw_y, WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(dw, dh)),
                        ..Default::default()
                    },
                );
                if world.damage_flash_timer > 0.0 && (world.damage_flash_timer as i32) % 4 < 2 {
                    draw_rectangle(draw_x, draw_y, dw, dh, Color::new(1.0, 0.0, 0.0, 0.5));
                }
            } else {
                let tex = if world.player.facing_right { &sprites.knight_small_r } else { &sprites.knight_small_l };
                let bob = if world.player.on_ground && world.player.vx.abs() > 0.5 {
                    (world.player.anim_timer * 0.5).sin()
                } else { 0.0 };
                let dw = SMALL_W * draw_scale;
                let dh = SMALL_H * draw_scale;
                let draw_x = px - (dw - SMALL_W) * 0.5;
                let draw_y = py + bob - (dh - SMALL_H);
                draw_texture_ex(
                    tex, draw_x, draw_y, WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(dw, dh)),
                        ..Default::default()
                    },
                );
                if world.damage_flash_timer > 0.0 && (world.damage_flash_timer as i32) % 4 < 2 {
                    draw_rectangle(draw_x, draw_y, dw, dh, Color::new(1.0, 0.0, 0.0, 0.5));
                }
            }
        }
    } else {
        // Dead player falling
        let px = (world.player.x - cam_x).round();
        let py = (world.player.y + cam_y).round();
        let tex = if world.player.facing_right { &sprites.knight_small_r } else { &sprites.knight_small_l };
        draw_texture_ex(
            tex, px, py, Color::new(1.0, 1.0, 1.0, 0.5),
            DrawTextureParams {
                dest_size: Some(Vec2::new(SMALL_W, SMALL_H)),
                ..Default::default()
            },
        );
    }

    // Particles
    for p in &world.particles {
        let sx = p.x - cam_x;
        let alpha = (p.life / p.max_life).min(1.0);
        let c = Color::new(p.color.r, p.color.g, p.color.b, alpha);
        draw_rectangle(sx - p.size * 0.5, p.y + cam_y - p.size * 0.5, p.size, p.size, c);
    }

    // Popups
    for popup in &world.popups {
        let sx = popup.x - cam_x;
        let sy = popup.y + cam_y;
        let alpha = (popup.life / 20.0).min(1.0);
        let c = Color::new(popup.color.r, popup.color.g, popup.color.b, alpha);
        draw_text(&popup.text, sx, sy, 16.0, c);
    }

    // HUD
    draw_rectangle(0.0, 0.0, SCREEN_W, 28.0, Color::new(0.0, 0.0, 0.0, 0.5));

    // Lives
    draw_texture_ex(
        &sprites.knight_small_r, 12.0, 7.0, WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(12.0, 14.0)),
            ..Default::default()
        },
    );
    draw_text(&format!("x{}", world.player.lives), 30.0, 18.0, 20.0, Color::new(1.0, 0.27, 0.27, 1.0));

    // Weapon indicator
    if world.player.has_fire {
        draw_text("FIRE", 70.0, 18.0, 16.0, Color::new(1.0, 0.40, 0.0, 1.0));
    } else {
        draw_text("SWORD", 70.0, 18.0, 16.0, Color::new(0.80, 0.80, 0.80, 1.0));
    }
    if world.player.big {
        draw_text("BIG", 130.0, 18.0, 16.0, Color::new(1.0, 0.80, 0.0, 1.0));
    }

    // Level name
    draw_text(get_level_name(world.current_level), SCREEN_W * 0.5 - 50.0, 10.0, 16.0,
              Color::new(0.53, 0.53, 0.53, 0.6));

    // Score
    draw_text(&format!("SCORE: {}", world.player.score), SCREEN_W * 0.5 - 40.0, 18.0, 20.0, WHITE);

    // Crystals
    draw_texture_ex(
        &sprites.crystal, SCREEN_W - 60.0, 5.0, WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(14.0, 14.0)),
            ..Default::default()
        },
    );
    draw_text(&format!("{}", world.player.coins), SCREEN_W - 40.0, 18.0, 20.0, Color::new(0.27, 0.87, 1.0, 1.0));

    // Level complete banner
    if world.level_complete_timer > 0 {
        draw_text("LEVEL COMPLETE!", SCREEN_W * 0.5 - 90.0, SCREEN_H * 0.4, 32.0,
                  Color::new(1.0, 1.0, 0.5, 0.9));
    }

    // Damage flash overlay
    if world.damage_flash_timer > 15.0 {
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(1.0, 0.0, 0.0, 0.15));
    }

    draw_scanlines();
    draw_vignette();
}

fn draw_title(frame_count: u64, sprites: &Sprites) {
    // Gradient background
    let steps = 10;
    let step_h = SCREEN_H / steps as f32;
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let c = Color::new(0.02 + t * 0.02, 0.02 + t * 0.04, 0.09 + t * 0.07, 1.0);
        draw_rectangle(0.0, i as f32 * step_h, SCREEN_W, step_h + 1.0, c);
    }

    // Stars
    for i in 0..40_u32 {
        let seed = i as f32 * 73.0;
        let sx = (seed + frame_count as f32 * 0.3) % SCREEN_W;
        let sy = (i as f32 * 47.0 + (frame_count as f32 * 0.02 + i as f32).sin() * 10.0) % SCREEN_H;
        let a = (0.3 + (frame_count as f32 * 0.05 + i as f32).sin() * 0.2) as f32;
        draw_rectangle(sx, sy, 2.0, 2.0, Color::new(0.67, 0.73, 1.0, a));
    }

    // Floating crystals
    for i in 0..8_u32 {
        let fx = SCREEN_W * 0.5 + (frame_count as f32 * 0.015 + i as f32 * 0.8).cos() * (100.0 + i as f32 * 20.0);
        let fy = SCREEN_H * 0.5 + (frame_count as f32 * 0.02 + i as f32 * 1.2).sin() * 50.0 - 30.0;
        let a = (0.4 + (frame_count as f32 * 0.03 + i as f32).sin() * 0.2) as f32;
        draw_rectangle(fx - 2.0, fy - 2.0, 4.0, 4.0, Color::new(0.27, 0.67, 1.0, a));
    }

    // Title
    let title_glow = ((frame_count as f32 * 0.05).sin() * 0.3 + 0.7) as f32;
    // Shadow
    draw_text("PIXEL KNIGHT", SCREEN_W * 0.5 - 118.0, SCREEN_H * 0.5 - 78.0, 40.0,
              Color::new(0.0, 0.0, 0.17, 0.5));
    draw_text("PIXEL KNIGHT", SCREEN_W * 0.5 - 120.0, SCREEN_H * 0.5 - 80.0, 40.0,
              Color::new(0.27, 0.67, 1.0, title_glow));

    // Subtitle
    draw_text("The Crystal Kingdom", SCREEN_W * 0.5 - 90.0, SCREEN_H * 0.5 - 50.0, 20.0,
              Color::new(0.53, 0.60, 0.67, 0.8));

    // Knight sprite
    let knight_bob = (frame_count as f32 * 0.05).sin() * 4.0;
    draw_texture_ex(
        &sprites.knight_big_r, SCREEN_W * 0.5 - 16.0, SCREEN_H * 0.5 - 28.0 + knight_bob, WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(32.0, 48.0)),
            ..Default::default()
        },
    );

    // Orbiting crystals
    for i in 0..3_u32 {
        let angle: f32 = frame_count as f32 * 0.03 + i as f32 * (6.28 / 3.0);
        let rx = SCREEN_W * 0.5 + angle.cos() * 40.0;
        let ry = SCREEN_H * 0.5 - 4.0 + angle.sin() * 20.0 + knight_bob;
        draw_texture_ex(
            &sprites.crystal, rx - 6.0, ry - 6.0, WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(12.0, 12.0)),
                ..Default::default()
            },
        );
    }

    // Start prompt
    if (frame_count / 30) % 2 == 0 {
        draw_text("PRESS START", SCREEN_W * 0.5 - 62.0, SCREEN_H * 0.5 + 80.0, 24.0,
                  Color::new(1.0, 0.80, 0.27, 1.0));
    }

    // Controls
    draw_text("D-Pad: Move  B: Jump  A: Attack", SCREEN_W * 0.5 - 150.0, SCREEN_H - 40.0, 16.0,
              Color::new(0.33, 0.40, 0.47, 0.7));

    draw_scanlines();
    draw_vignette();
}

fn draw_story(world: &World) {
    clear_background(Color::new(0.0, 0.0, 0.0, 0.92));

    let lines: Vec<&str> = world.story_full_text.split('\n').collect();
    let start_y = SCREEN_H * 0.5 - (lines.len() as f32 * 18.0) * 0.5;

    for (i, line) in lines.iter().enumerate() {
        draw_text(line, 60.0, start_y + i as f32 * 18.0, 20.0,
                  Color::new(0.80, 0.87, 0.93, 1.0));
    }

    if world.story_line_index < world.story_lines.len() {
        if (world.story_frame_counter / 20) % 2 == 0 {
            let last_line = lines.last().unwrap_or(&"");
            let cursor_x = 60.0 + last_line.len() as f32 * 10.0;
            let cursor_y = start_y + (lines.len() as f32 - 1.0) * 18.0 - 10.0;
            draw_rectangle(cursor_x, cursor_y, 8.0, 12.0, Color::new(1.0, 0.80, 0.27, 0.8));
        }
    } else {
        if (world.frame_count / 30) % 2 == 0 {
            draw_text("PRESS START TO CONTINUE", SCREEN_W * 0.5 - 110.0, SCREEN_H - 40.0, 16.0,
                      Color::new(1.0, 0.80, 0.27, 0.9));
        }
    }

    draw_scanlines();
    draw_vignette();
}

fn draw_game_over(world: &World, frame_count: u64) {
    clear_background(Color::new(0.31, 0.0, 0.0, 0.85));

    draw_text("GAME OVER", SCREEN_W * 0.5 - 80.0, SCREEN_H * 0.5 - 40.0, 40.0, RED);
    draw_text(&format!("SCORE: {}", world.player.score), SCREEN_W * 0.5 - 60.0, SCREEN_H * 0.5 + 10.0, 24.0, WHITE);
    draw_text(&format!("COINS: {}", world.player.coins), SCREEN_W * 0.5 - 50.0, SCREEN_H * 0.5 + 35.0, 20.0,
              Color::new(0.80, 0.80, 0.80, 1.0));

    if (frame_count / 30) % 2 == 0 {
        draw_text("PRESS START TO RETRY", SCREEN_W * 0.5 - 110.0, SCREEN_H * 0.5 + 80.0, 20.0,
                  Color::new(1.0, 0.80, 0.27, 1.0));
    }

    draw_scanlines();
    draw_vignette();
}

fn draw_win_screen(world: &World, frame_count: u64) {
    // Gradient
    let steps = 10;
    let step_h = SCREEN_H / steps as f32;
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let c = Color::new(0.04 + t * 0.06, 0.09 + t * 0.06, 0.22 + t * 0.13, 1.0);
        draw_rectangle(0.0, i as f32 * step_h, SCREEN_W, step_h + 1.0, c);
    }

    // Confetti
    for i in 0..20_u32 {
        let px_val = (i as f32 * 97.0 + frame_count as f32 * 0.5) % SCREEN_W;
        let py_val = (i as f32 * 53.0 + frame_count as f32 * 0.3 + (frame_count as f32 * 0.02 + i as f32).sin() * 30.0) % SCREEN_H;
        let a = (0.4 + (frame_count as f32 * 0.05 + i as f32).sin() * 0.2) as f32;
        let colors = [RED, GREEN, BLUE, YELLOW, MAGENTA, Color::new(0.27, 1.0, 1.0, 1.0)];
        let c = colors[(i % 6) as usize];
        draw_rectangle(px_val, py_val, 3.0, 3.0, Color::new(c.r, c.g, c.b, a));
    }

    draw_text("VICTORY!", SCREEN_W * 0.5 - 70.0, SCREEN_H * 0.5 - 60.0, 40.0,
              Color::new(1.0, 0.80, 0.27, 1.0));
    draw_text("The Crystal Kingdom is saved!", SCREEN_W * 0.5 - 140.0, SCREEN_H * 0.5 - 25.0, 20.0,
              Color::new(0.67, 0.73, 0.80, 1.0));
    draw_text(&format!("SCORE: {}", world.player.score), SCREEN_W * 0.5 - 70.0, SCREEN_H * 0.5 + 20.0, 28.0, WHITE);
    draw_text(&format!("COINS: {}", world.player.coins), SCREEN_W * 0.5 - 50.0, SCREEN_H * 0.5 + 50.0, 20.0,
              Color::new(0.27, 0.87, 1.0, 1.0));

    if (frame_count / 30) % 2 == 0 {
        draw_text("PRESS START", SCREEN_W * 0.5 - 62.0, SCREEN_H * 0.5 + 100.0, 20.0,
                  Color::new(1.0, 0.80, 0.27, 1.0));
    }

    draw_scanlines();
    draw_vignette();
}

fn draw_scanlines() {
    let c = Color::new(0.0, 0.0, 0.0, 0.12);
    let mut y = 0.0_f32;
    while y < SCREEN_H {
        draw_rectangle(0.0, y, SCREEN_W, 2.0, c);
        y += 4.0;
    }
}

fn draw_vignette() {
    let depth = 60.0_f32;
    let layers = 4_u32;
    for i in 0..layers {
        let alpha = (i as f32 + 1.0) / layers as f32 * 0.4;
        let inset = depth * (1.0 - (i as f32 / layers as f32));
        let c = Color::new(0.0, 0.0, 0.0, alpha);
        draw_rectangle(0.0, 0.0, SCREEN_W, inset, c);
        draw_rectangle(0.0, SCREEN_H - inset, SCREEN_W, inset, c);
        draw_rectangle(0.0, 0.0, inset, SCREEN_H, c);
        draw_rectangle(SCREEN_W - inset, 0.0, inset, SCREEN_H, c);
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------
#[macroquad::main("Pixel Knight")]
async fn main() {
    request_new_screen_size(SCREEN_W, SCREEN_H);

    let sprites = Sprites::new();
    let mut world = World::new();
    let mut input = Input::new();
    let mut accumulator: f64 = 0.0;
    let mut last_time = get_time();
    let mut global_frame: u64 = 0;

    loop {
        let current_time = get_time();
        let mut frame_time = current_time - last_time;
        last_time = current_time;
        if frame_time > 0.25 { frame_time = 0.25; }
        accumulator += frame_time;

        input.poll();

        while accumulator >= TIME_STEP {
            match world.state {
                GameState::Start => {
                    global_frame += 1;
                    if input.start || input.jump_pressed || input.attack_pressed {
                        world.start_story(STORY_INTRO, false);
                    }
                }
                GameState::Story | GameState::LevelStory => {
                    global_frame += 1;
                    world.update_story(&input);
                }
                GameState::Playing => {
                    world.update(&input);
                }
                GameState::GameOver => {
                    global_frame += 1;
                    if input.start || input.jump_pressed {
                        world.reset_game();
                        world.state = GameState::Start;
                    }
                }
                GameState::Win => {
                    global_frame += 1;
                    if input.start || input.jump_pressed {
                        world.reset_game();
                        world.state = GameState::Start;
                    }
                }
            }

            // Clear one-shot inputs
            input.jump_pressed = false;
            input.attack_pressed = false;
            input.start = false;

            accumulator -= TIME_STEP;
        }

        // Draw
        match world.state {
            GameState::Start => draw_title(global_frame, &sprites),
            GameState::Story | GameState::LevelStory => draw_story(&world),
            GameState::Playing => draw_world(&world, &sprites),
            GameState::GameOver => draw_game_over(&world, global_frame),
            GameState::Win => draw_win_screen(&world, global_frame),
        }

        next_frame().await;
    }
}
