// Pixel Knight - Mario-like Action Platformer for Miyoo Mini Plus
// Rust/Macroquad port -- 640x480, 60fps fixed timestep
// Story: "The Crystal Kingdom"

use macroquad::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------
const SCREEN_W: f32 = 640.0;
const SCREEN_H: f32 = 480.0;
const TIME_STEP: f64 = 1.0 / 60.0;
const TILE_SIZE: f32 = 32.0;

const GRAVITY: f32 = 0.4;
const JUMP_FORCE: f32 = -8.0;
const JUMP_HOLD_FORCE: f32 = -0.3;
const JUMP_HOLD_MAX: i32 = 12;
const MOVE_SPEED: f32 = 3.0;
const MAX_FALL: f32 = 8.0;
const STOMP_BOUNCE: f32 = -6.0;
const COYOTE_MAX: i32 = 6;
const JUMP_BUFFER_MAX: i32 = 6;
const FIREBALL_SPEED: f32 = 6.0;
const INVULN_MAX: i32 = 60;
const BOSS_HP: i32 = 10;

// ---------------------------------------------------------------------------
// Sprite art data (8x8, '.' = transparent, digits index into colour palette)
// ---------------------------------------------------------------------------
// Player knight - blue/silver
const KNIGHT_ART: [&str; 8] = [
    "..1221..",
    ".122221.",
    ".133331.",
    "11322311",
    ".122221.",
    ".124421.",
    ".12..21.",
    ".11..11.",
];
const KNIGHT_COLORS: [Color; 4] = [
    Color::new(0.2, 0.2, 0.6, 1.0),   // 1: dark blue
    Color::new(0.4, 0.5, 0.9, 1.0),   // 2: medium blue
    Color::new(0.9, 0.9, 0.95, 1.0),  // 3: silver/white (visor)
    Color::new(0.6, 0.5, 0.3, 1.0),   // 4: brown (belt)
];

// Slime enemy - green
const SLIME_ART: [&str; 8] = [
    "........",
    "..1111..",
    ".122221.",
    "12233221",
    "12233221",
    "12222221",
    ".122221.",
    "..1111..",
];
const SLIME_COLORS: [Color; 3] = [
    Color::new(0.1, 0.4, 0.1, 1.0),   // 1: dark green
    Color::new(0.2, 0.7, 0.2, 1.0),   // 2: green
    Color::new(0.8, 0.9, 0.8, 1.0),   // 3: highlight
];

// Bat enemy - purple/dark
const BAT_ART: [&str; 8] = [
    "1......1",
    "11....11",
    "121..121",
    ".122221.",
    "..1331..",
    "..1221..",
    "...11...",
    "........",
];
const BAT_COLORS: [Color; 3] = [
    Color::new(0.3, 0.1, 0.4, 1.0),   // 1: dark purple
    Color::new(0.5, 0.2, 0.7, 1.0),   // 2: purple
    Color::new(0.9, 0.2, 0.2, 1.0),   // 3: red eyes
];

// Boss dark knight - red/black
const BOSS_ART: [&str; 8] = [
    ".113311.",
    "11233211",
    "12344321",
    "12233221",
    "11233211",
    ".123321.",
    ".12..21.",
    ".11..11.",
];
const BOSS_COLORS: [Color; 4] = [
    Color::new(0.15, 0.05, 0.05, 1.0), // 1: near-black
    Color::new(0.6, 0.1, 0.1, 1.0),    // 2: dark red
    Color::new(0.9, 0.2, 0.1, 1.0),    // 3: bright red
    Color::new(1.0, 0.6, 0.1, 1.0),    // 4: orange glow
];

// Crystal/coin - yellow sparkle
const CRYSTAL_ART: [&str; 8] = [
    "...11...",
    "..1221..",
    ".123321.",
    "12333321",
    "12333321",
    ".123321.",
    "..1221..",
    "...11...",
];
const CRYSTAL_COLORS: [Color; 3] = [
    Color::new(0.8, 0.6, 0.0, 1.0),   // 1: dark gold
    Color::new(1.0, 0.85, 0.0, 1.0),  // 2: gold
    WHITE,                              // 3: sparkle
];

// Solid block - brown/gray brick
const BLOCK_ART: [&str; 8] = [
    "22222221",
    "33333331",
    "33333331",
    "11111111",
    "22212222",
    "33313333",
    "33313333",
    "11111111",
];
const BLOCK_COLORS: [Color; 3] = [
    Color::new(0.25, 0.15, 0.1, 1.0),  // 1: mortar
    Color::new(0.55, 0.35, 0.2, 1.0),  // 2: brick light
    Color::new(0.4, 0.25, 0.15, 1.0),  // 3: brick dark
];

// Breakable block - lighter with cracks
const BREAKABLE_ART: [&str; 8] = [
    "22222221",
    "23333331",
    "33133331",
    "11111111",
    "22212222",
    "33313133",
    "33313333",
    "11111111",
];
const BREAKABLE_COLORS: [Color; 3] = [
    Color::new(0.3, 0.2, 0.15, 1.0),
    Color::new(0.65, 0.5, 0.35, 1.0),
    Color::new(0.5, 0.38, 0.25, 1.0),
];

// Spike - metallic
const SPIKE_ART: [&str; 8] = [
    "........",
    "...11...",
    "..1221..",
    "..1221..",
    ".122221.",
    ".122221.",
    "12222221",
    "11111111",
];
const SPIKE_COLORS: [Color; 2] = [
    Color::new(0.3, 0.3, 0.35, 1.0),
    Color::new(0.5, 0.5, 0.55, 1.0),
];

// Powerup crystal shard - cyan glow
const SHARD_ART: [&str; 8] = [
    "...11...",
    "..1221..",
    ".123321.",
    ".123321.",
    "..1221..",
    "...11...",
    "........",
    "........",
];
const SHARD_COLORS: [Color; 3] = [
    Color::new(0.0, 0.5, 0.8, 1.0),
    Color::new(0.2, 0.8, 1.0, 1.0),
    WHITE,
];

// Fire crystal - orange/red
const FIRE_CRYSTAL_ART: [&str; 8] = [
    "...11...",
    "..1221..",
    ".123321.",
    ".123321.",
    "..1221..",
    "...11...",
    "........",
    "........",
];
const FIRE_CRYSTAL_COLORS: [Color; 3] = [
    Color::new(0.7, 0.1, 0.0, 1.0),
    Color::new(1.0, 0.5, 0.0, 1.0),
    Color::new(1.0, 1.0, 0.3, 1.0),
];

// Fireball projectile
const FIREBALL_ART: [&str; 8] = [
    "........",
    "...11...",
    "..1221..",
    ".123321.",
    "..1221..",
    "...11...",
    "........",
    "........",
];
const FIREBALL_COLORS: [Color; 3] = [
    Color::new(0.8, 0.2, 0.0, 1.0),
    Color::new(1.0, 0.6, 0.0, 1.0),
    Color::new(1.0, 1.0, 0.4, 1.0),
];

// Background tile - dark subtle pattern
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
const BG_COLORS: [Color; 2] = [
    Color::new(0.12, 0.1, 0.18, 1.0),
    Color::new(0.16, 0.14, 0.22, 1.0),
];

// ---------------------------------------------------------------------------
// Sprite builder (string art -> Texture2D)
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

struct Sprites {
    knight: Texture2D,
    slime: Texture2D,
    bat: Texture2D,
    boss: Texture2D,
    crystal: Texture2D,
    block: Texture2D,
    breakable: Texture2D,
    spike: Texture2D,
    shard: Texture2D,
    fire_crystal: Texture2D,
    fireball: Texture2D,
    bg: Texture2D,
}

impl Sprites {
    fn new() -> Self {
        Self {
            knight: create_sprite(&KNIGHT_ART, &KNIGHT_COLORS),
            slime: create_sprite(&SLIME_ART, &SLIME_COLORS),
            bat: create_sprite(&BAT_ART, &BAT_COLORS),
            boss: create_sprite(&BOSS_ART, &BOSS_COLORS),
            crystal: create_sprite(&CRYSTAL_ART, &CRYSTAL_COLORS),
            block: create_sprite(&BLOCK_ART, &BLOCK_COLORS),
            breakable: create_sprite(&BREAKABLE_ART, &BREAKABLE_COLORS),
            spike: create_sprite(&SPIKE_ART, &SPIKE_COLORS),
            shard: create_sprite(&SHARD_ART, &SHARD_COLORS),
            fire_crystal: create_sprite(&FIRE_CRYSTAL_ART, &FIRE_CRYSTAL_COLORS),
            fireball: create_sprite(&FIREBALL_ART, &FIREBALL_COLORS),
            bg: create_sprite(&BG_ART, &BG_COLORS),
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
enum TileKind {
    Solid,
    Breakable,
    Spike,
}

#[derive(Clone, Copy, PartialEq)]
enum EnemyKind {
    Slime,
    Bat,
    Boss,
}

#[derive(Clone, Copy, PartialEq)]
enum PowerupKind {
    Shard,       // extra hit point
    FireCrystal, // shoot fireballs
}

struct Tile {
    x: f32,
    y: f32,
    kind: TileKind,
    alive: bool,
}

#[allow(dead_code)]
struct Enemy {
    kind: EnemyKind,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    start_x: f32,
    start_y: f32,
    alive: bool,
    hp: i32,
    facing_right: bool,
    anim_timer: f32,
    hurt_timer: i32,
    attack_timer: i32,
}

struct CrystalGem {
    x: f32,
    y: f32,
    alive: bool,
    anim_offset: f32,
}

struct Powerup {
    x: f32,
    y: f32,
    kind: PowerupKind,
    alive: bool,
    anim_offset: f32,
}

struct Fireball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    alive: bool,
    life: i32,
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

struct Popup {
    text: String,
    x: f32,
    y: f32,
    life: f32,
}

struct DustMote {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    size: f32,
    alpha: f32,
}

struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    on_ground: bool,
    facing_right: bool,
    hp: i32,
    max_hp: i32,
    has_fire: bool,
    invuln: i32,
    jump_held: i32,
    lives: i32,
    crystals_collected: i32,
    score: i32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 64.0, y: 320.0,
            vx: 0.0, vy: 0.0,
            on_ground: false,
            facing_right: true,
            hp: 1, max_hp: 2,
            has_fire: false,
            invuln: 0,
            jump_held: 0,
            lives: 3,
            crystals_collected: 0,
            score: 0,
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
// Input state
// ---------------------------------------------------------------------------
struct Input {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    jump: bool,
    jump_pressed: bool,
    jump_released: bool,
    attack: bool,
    attack_pressed: bool,
    start: bool,
    jump_buffer: i32,
    prev_jump: bool,
}

impl Input {
    fn new() -> Self {
        Self {
            left: false, right: false, up: false, down: false,
            jump: false, jump_pressed: false, jump_released: false,
            attack: false, attack_pressed: false,
            start: false,
            jump_buffer: 0,
            prev_jump: false,
        }
    }

    fn poll(&mut self) {
        self.left = is_key_down(KeyCode::Left);
        self.right = is_key_down(KeyCode::Right);
        self.up = is_key_down(KeyCode::Up);
        self.down = is_key_down(KeyCode::Down);

        let jump_now = is_key_down(KeyCode::Space);
        self.jump_pressed = jump_now && !self.prev_jump;
        self.jump_released = !jump_now && self.prev_jump;
        self.jump = jump_now;
        self.prev_jump = jump_now;

        if self.jump_pressed {
            self.jump_buffer = JUMP_BUFFER_MAX;
        }

        self.attack_pressed = is_key_pressed(KeyCode::X);
        self.attack = is_key_down(KeyCode::X);
        self.start = is_key_pressed(KeyCode::Enter);
    }
}

// ---------------------------------------------------------------------------
// Story data
// ---------------------------------------------------------------------------
static STORY_INTRO: &[&str] = &[
    "The Crystal Kingdom once shone with eternal light.",
    "Its radiance kept the darkness at bay",
    "and all creatures lived in peace.",
    "",
    "But the sorcerer Malachar grew jealous of the light.",
    "He shattered the Crystal Heart into three shards,",
    "plunging the realm into endless twilight.",
    "",
    "You are the last Pixel Knight.",
    "Take up your sword and reclaim the crystal shards.",
    "Restore the light... or all is lost.",
];

static STORY_LEVEL_1: &[&str] = &[
    "LEVEL 1: CRYSTAL MEADOWS",
    "",
    "The meadows have grown dark and cold.",
    "Slimes and bats roam the corrupted fields.",
    "Find the crystal shard hidden at the end.",
    "",
    "Move: D-Pad   Jump: B (Space)",
    "Attack: A (X)   Stomp enemies from above!",
];

static STORY_LEVEL_2: &[&str] = &[
    "LEVEL 2: SHADOW CAVERNS",
    "",
    "The caves twist deeper into the earth.",
    "Strange echoes bounce off crystalline walls.",
    "The corruption is stronger here --",
    "more creatures guard the second shard.",
    "",
    "Watch for spikes and falling bats!",
];

static STORY_LEVEL_3: &[&str] = &[
    "LEVEL 3: THE DARK TOWER",
    "",
    "Malachar's tower pierces the twilight sky.",
    "His dark knights patrol every corridor.",
    "At the summit, the sorcerer himself awaits.",
    "",
    "Defeat Malachar and restore the Crystal Heart!",
];

static STORY_VICTORY: &[&str] = &[
    "With a final blow, Malachar falls.",
    "The three crystal shards rise into the air,",
    "drawn together by an ancient force.",
    "",
    "The Crystal Heart reforms, and light",
    "erupts across the kingdom.",
    "",
    "The twilight shatters like glass.",
    "Flowers bloom. Rivers sparkle.",
    "The darkness retreats to the forgotten places.",
    "",
    "You kneel, exhausted but triumphant.",
    "The Crystal Kingdom is reborn.",
    "",
    "Thank you, Pixel Knight.",
    "",
    "THE END",
];

// ---------------------------------------------------------------------------
// Level maps
// '.' = empty, '#' = solid, '?' = breakable, 'C' = crystal, 'S' = slime,
// 'B' = bat, 'P' = shard powerup, 'F' = fire crystal, 'K' = spike,
// 'X' = boss, 'G' = goal (level end trigger)
// ---------------------------------------------------------------------------
static LEVEL_1_MAP: &[&str] = &[
    //        1111111111222222222233333333334444444444555555555566666666667777777777888888888899999999990000
    //234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123
    "......................................................................................................",
    "......................................................................................................",
    "......................................................................................................",
    "......................................................................................................",
    "......................................................................................................",
    "......................................................................................................",
    "..........C.C.C..........................................................C.C.C........................",
    "..........#####.......................C...............................##########.........G..............",
    "....................C.............#####.......C.C.....................................####..............",
    ".................######.......................###..........C..C..C.........####.........................",
    "..............................................................########................................",
    "...S.................S..........S..........S.........S............................................S....",
    "####..........####.......####.......####.......####.....####.......####..####.....####.....KK..####.##",
    "####..........####.......####.......####.......####.....####.......####..####.....####.....KK..####.##",
    "####..........####.......####.......####.......####.....####.......####..####.....####.........####.##",
];

static LEVEL_2_MAP: &[&str] = &[
    "######..........................................................................######################",
    "#.............C........C.......C...............................................................C......#",
    "#........................................................................##########..................#",
    "#.C.C......####..........####.......??.........................................B.........C............#",
    "#.###.....................B.................C.C.C.............####.....................................#",
    "#..........................####.....########.........####.........................................S...#",
    "#........S.......S.........................S.............................####.....####.....####..####.#",
    "#..####.....####.....####.....####.....####.....####.....C.C.C..................................####.#",
    "#...........................................P...............###......S.........B...........C.C.C......#",
    "#...............B..............B..........................................####.............######......#",
    "#.......####.......????............####.......####..........####.....................................G#",
    "#....S........S........S.....KKKK.......S..........S...........S........S.......S.....KKKK.....S..###",
    "####..####..####..####..####.####..####..####..####..####..####..####..####..####..####.####..####.##",
    "####..####..####..####..####.####..####..####..####..####..####..####..####..####..####.####..####.##",
    "####..####..####..####..####.####..####..####..####..####..####..####..####..####..####.####..####.##",
];

static LEVEL_3_MAP: &[&str] = &[
    "####################################################################################################",
    "#..................................................................................................G.#",
    "#...........................................................................#####.............####..#",
    "#.............C..........C..........C.......................................#.....................X..#",
    "#...........####.......####.......####.........................####.........#.....####..............#",
    "#....................................................................####..#..........................#",
    "#.......B...........B............B..............F......................#..........................S..#",
    "#......####.......####.......####..........####.....####...............#.....####.....####..####.####",
    "#...............................S.....S..........S.............####...#...............................#",
    "#..####.....####.....####..####...####...####...####..........#..........S.........B..........S.....#",
    "#..........................................................................####.....####.....####...#",
    "#...S..........S.........S......KKKK.......S..........S..........S..........KKKKK.......S..........#",
    "####..####..####..####..####.####..####..####..####..####..####..####..####..####..####..####..#####",
    "####..####..####..####..####.####..####..####..####..####..####..####..####..####..####..####..#####",
    "####..####..####..####..####.####..####..####..####..####..####..####..####..####..####..####..#####",
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
        0 => "CRYSTAL MEADOWS",
        1 => "SHADOW CAVERNS",
        _ => "THE DARK TOWER",
    }
}

fn get_level_story(idx: usize) -> &'static [&'static str] {
    match idx {
        0 => STORY_LEVEL_1,
        1 => STORY_LEVEL_2,
        _ => STORY_LEVEL_3,
    }
}

// ---------------------------------------------------------------------------
// World (all mutable game state)
// ---------------------------------------------------------------------------
struct World {
    state: GameState,
    current_level: usize,

    player: Player,
    tiles: Vec<Tile>,
    enemies: Vec<Enemy>,
    crystals: Vec<CrystalGem>,
    powerups: Vec<Powerup>,
    fireballs: Vec<Fireball>,
    particles: Vec<Particle>,
    popups: Vec<Popup>,
    dust_motes: Vec<DustMote>,

    camera_x: f32,
    map_width: f32,
    map_height: f32,

    coyote_frames: i32,
    shake_magnitude: f32,
    screen_shake_x: f32,
    screen_shake_y: f32,
    damage_flash_timer: f32,
    hit_stop_frames: i32,
    frame_count: u64,

    // Story/typewriter
    story_lines: &'static [&'static str],
    story_line_index: usize,
    story_char_index: usize,
    story_frame_counter: u64,
    story_full_text: String,
    story_is_victory: bool,

    // Boss defeated flag per level
    boss_defeated: bool,

    // Level-end goal reached
    goal_reached: bool,
    goal_timer: i32,

    // Total crystals in level (for display)
    total_crystals: i32,
}

impl World {
    fn new() -> Self {
        let mut w = Self {
            state: GameState::Start,
            current_level: 0,

            player: Player::new(),
            tiles: Vec::new(),
            enemies: Vec::new(),
            crystals: Vec::new(),
            powerups: Vec::new(),
            fireballs: Vec::new(),
            particles: Vec::new(),
            popups: Vec::new(),
            dust_motes: Vec::new(),

            camera_x: 0.0,
            map_width: 0.0,
            map_height: 0.0,

            coyote_frames: 0,
            shake_magnitude: 0.0,
            screen_shake_x: 0.0,
            screen_shake_y: 0.0,
            damage_flash_timer: 0.0,
            hit_stop_frames: 0,
            frame_count: 0,

            story_lines: &[],
            story_line_index: 0,
            story_char_index: 0,
            story_frame_counter: 0,
            story_full_text: String::new(),
            story_is_victory: false,

            boss_defeated: false,
            goal_reached: false,
            goal_timer: 0,
            total_crystals: 0,
        };

        // Initialize dust motes
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

        w
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
        self.tiles.clear();
        self.enemies.clear();
        self.crystals.clear();
        self.powerups.clear();
        self.fireballs.clear();
        self.particles.clear();
        self.popups.clear();
        self.boss_defeated = false;
        self.goal_reached = false;
        self.goal_timer = 0;
        self.hit_stop_frames = 0;
        self.damage_flash_timer = 0.0;
        self.shake_magnitude = 0.0;
        self.total_crystals = 0;

        let map = get_level_map(self.current_level);
        self.map_height = map.len() as f32 * TILE_SIZE;

        let mut max_col: usize = 0;
        for row in map.iter() {
            if row.len() > max_col {
                max_col = row.len();
            }
        }
        self.map_width = max_col as f32 * TILE_SIZE;

        for (row, line) in map.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let px = col as f32 * TILE_SIZE;
                let py = row as f32 * TILE_SIZE;

                match ch {
                    '#' => self.tiles.push(Tile { x: px, y: py, kind: TileKind::Solid, alive: true }),
                    '?' => self.tiles.push(Tile { x: px, y: py, kind: TileKind::Breakable, alive: true }),
                    'K' => self.tiles.push(Tile { x: px, y: py, kind: TileKind::Spike, alive: true }),
                    'C' => {
                        self.total_crystals += 1;
                        self.crystals.push(CrystalGem {
                            x: px, y: py, alive: true,
                            anim_offset: rand::gen_range(0.0_f32, 6.28),
                        });
                    }
                    'S' => {
                        self.enemies.push(Enemy {
                            kind: EnemyKind::Slime, x: px, y: py,
                            vx: 1.0, vy: 0.0, start_x: px, start_y: py,
                            alive: true, hp: 1, facing_right: true,
                            anim_timer: 0.0, hurt_timer: 0, attack_timer: 0,
                        });
                    }
                    'B' => {
                        self.enemies.push(Enemy {
                            kind: EnemyKind::Bat, x: px, y: py,
                            vx: 1.5, vy: 0.0, start_x: px, start_y: py,
                            alive: true, hp: 1, facing_right: true,
                            anim_timer: rand::gen_range(0.0_f32, 6.28),
                            hurt_timer: 0, attack_timer: 0,
                        });
                    }
                    'X' => {
                        self.enemies.push(Enemy {
                            kind: EnemyKind::Boss, x: px, y: py - TILE_SIZE,
                            vx: 1.5, vy: 0.0, start_x: px, start_y: py - TILE_SIZE,
                            alive: true, hp: BOSS_HP, facing_right: false,
                            anim_timer: 0.0, hurt_timer: 0, attack_timer: 0,
                        });
                    }
                    'P' => {
                        self.powerups.push(Powerup {
                            x: px, y: py, kind: PowerupKind::Shard, alive: true,
                            anim_offset: rand::gen_range(0.0_f32, 6.28),
                        });
                    }
                    'F' => {
                        self.powerups.push(Powerup {
                            x: px, y: py, kind: PowerupKind::FireCrystal, alive: true,
                            anim_offset: rand::gen_range(0.0_f32, 6.28),
                        });
                    }
                    'G' => {
                        // Goal marker -- no tile, just store position in a crystal
                        // We will handle goal as a special invisible trigger
                        // Store as a powerup with Shard kind but mark it specially
                        // Actually, let's just use the position directly
                        // We'll check player position against this
                        self.crystals.push(CrystalGem {
                            x: px, y: py, alive: true,
                            anim_offset: -999.0, // sentinel for goal
                        });
                    }
                    _ => {}
                }
            }
        }

        // Place player at left side on ground
        self.player.x = 64.0;
        self.player.y = (map.len() as f32 - 5.0) * TILE_SIZE;
        self.player.vx = 0.0;
        self.player.vy = 0.0;
        self.player.on_ground = false;
        self.player.invuln = 60; // brief spawn invuln
        self.player.jump_held = 0;
        // Keep hp/lives/fire from previous level

        self.camera_x = 0.0;
        self.coyote_frames = 0;
    }

    fn spawn_particles(&mut self, x: f32, y: f32, count: usize, color: Color) {
        for _ in 0..count {
            let angle: f32 = rand::gen_range(0.0, 6.28);
            let speed: f32 = rand::gen_range(1.0, 4.0);
            self.particles.push(Particle {
                x, y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                life: rand::gen_range(15.0, 30.0),
                color,
                size: rand::gen_range(2.0, 5.0),
            });
        }
    }

    fn add_popup(&mut self, text: &str, x: f32, y: f32) {
        self.popups.push(Popup {
            text: text.to_string(),
            x, y,
            life: 60.0,
        });
    }

    fn player_die(&mut self) {
        self.player.lives -= 1;
        self.shake_magnitude = 8.0;
        self.damage_flash_timer = 10.0;
        self.spawn_particles(
            self.player.x + TILE_SIZE * 0.5,
            self.player.y + TILE_SIZE * 0.5,
            15,
            Color::new(0.4, 0.5, 0.9, 1.0),
        );

        if self.player.lives <= 0 {
            self.state = GameState::GameOver;
        } else {
            // Respawn
            self.player.x = 64.0;
            let map = get_level_map(self.current_level);
            self.player.y = (map.len() as f32 - 5.0) * TILE_SIZE;
            self.player.vx = 0.0;
            self.player.vy = 0.0;
            self.player.on_ground = false;
            self.player.invuln = INVULN_MAX;
            self.player.hp = 1;
            self.player.has_fire = false;
            self.player.jump_held = 0;
        }
    }

    fn damage_player(&mut self) {
        if self.player.invuln > 0 { return; }

        self.player.hp -= 1;
        self.shake_magnitude = 5.0;
        self.damage_flash_timer = 8.0;
        self.hit_stop_frames = 3;

        if self.player.hp <= 0 {
            self.player_die();
        } else {
            self.player.invuln = INVULN_MAX;
        }
    }

    fn update(&mut self, input: &Input) {
        if self.hit_stop_frames > 0 {
            self.hit_stop_frames -= 1;
            return;
        }

        self.frame_count += 1;

        // --- Screen shake decay ---
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

        // --- Goal check ---
        if self.goal_reached {
            self.goal_timer += 1;
            if self.goal_timer > 60 {
                self.current_level += 1;
                if self.current_level >= 3 {
                    self.start_story(STORY_VICTORY, true);
                } else {
                    self.start_level_story();
                }
            }
            return;
        }

        // --- Player invulnerability ---
        if self.player.invuln > 0 {
            self.player.invuln -= 1;
        }

        // --- Player horizontal movement ---
        if input.left {
            self.player.vx = -MOVE_SPEED;
            self.player.facing_right = false;
        } else if input.right {
            self.player.vx = MOVE_SPEED;
            self.player.facing_right = true;
        } else {
            self.player.vx *= 0.7; // friction
            if self.player.vx.abs() < 0.1 {
                self.player.vx = 0.0;
            }
        }

        // --- Coyote time ---
        if self.player.on_ground {
            self.coyote_frames = COYOTE_MAX;
        } else if self.coyote_frames > 0 {
            self.coyote_frames -= 1;
        }

        // --- Jump ---
        let can_jump = self.coyote_frames > 0;
        if input.jump_buffer > 0 && can_jump {
            self.player.vy = JUMP_FORCE;
            self.player.on_ground = false;
            self.coyote_frames = 0;
            self.player.jump_held = 1;

            // Jump particles
            self.spawn_particles(
                self.player.x + TILE_SIZE * 0.5,
                self.player.y + TILE_SIZE,
                4,
                Color::new(0.7, 0.7, 0.8, 0.6),
            );
        }

        // Variable jump height (hold to go higher)
        if input.jump && self.player.jump_held > 0 && self.player.jump_held < JUMP_HOLD_MAX {
            self.player.vy += JUMP_HOLD_FORCE;
            self.player.jump_held += 1;
        }
        if input.jump_released {
            self.player.jump_held = 0;
        }

        // --- Gravity ---
        self.player.vy += GRAVITY;
        if self.player.vy > MAX_FALL {
            self.player.vy = MAX_FALL;
        }

        // --- Fireball ---
        if input.attack_pressed && self.player.has_fire {
            let dir: f32 = if self.player.facing_right { 1.0 } else { -1.0 };
            self.fireballs.push(Fireball {
                x: self.player.x + if self.player.facing_right { TILE_SIZE } else { -8.0 },
                y: self.player.y + TILE_SIZE * 0.3,
                vx: FIREBALL_SPEED * dir,
                vy: 0.0,
                alive: true,
                life: 120,
            });
        }

        // --- Move player with collision ---
        // Horizontal
        let new_x = self.player.x + self.player.vx;
        let mut collided_x = false;
        for i in 0..self.tiles.len() {
            if !self.tiles[i].alive { continue; }
            if self.tiles[i].kind == TileKind::Spike { continue; }
            if overlaps(new_x, self.player.y, TILE_SIZE, TILE_SIZE,
                       self.tiles[i].x, self.tiles[i].y, TILE_SIZE, TILE_SIZE) {
                collided_x = true;
                // If breakable and player is attacking
                if self.tiles[i].kind == TileKind::Breakable && input.attack_pressed {
                    self.tiles[i].alive = false;
                    self.spawn_particles(
                        self.tiles[i].x + TILE_SIZE * 0.5,
                        self.tiles[i].y + TILE_SIZE * 0.5,
                        8,
                        Color::new(0.65, 0.5, 0.35, 1.0),
                    );
                    self.player.score += 10;
                    self.add_popup("+10", self.tiles[i].x, self.tiles[i].y);
                    collided_x = false; // don't block since we broke it
                }
                if collided_x { break; }
            }
        }
        if !collided_x {
            self.player.x = new_x;
        } else {
            self.player.vx = 0.0;
        }

        // Vertical
        let new_y = self.player.y + self.player.vy;
        let mut collided_y = false;
        let mut landed = false;
        for i in 0..self.tiles.len() {
            if !self.tiles[i].alive { continue; }
            if self.tiles[i].kind == TileKind::Spike { continue; }
            if overlaps(self.player.x, new_y, TILE_SIZE, TILE_SIZE,
                       self.tiles[i].x, self.tiles[i].y, TILE_SIZE, TILE_SIZE) {
                collided_y = true;
                if self.player.vy > 0.0 {
                    landed = true;
                    self.player.y = self.tiles[i].y - TILE_SIZE;
                } else {
                    self.player.y = self.tiles[i].y + TILE_SIZE;
                    // Hit breakable from below
                    if self.tiles[i].kind == TileKind::Breakable {
                        self.tiles[i].alive = false;
                        self.spawn_particles(
                            self.tiles[i].x + TILE_SIZE * 0.5,
                            self.tiles[i].y + TILE_SIZE * 0.5,
                            8,
                            Color::new(0.65, 0.5, 0.35, 1.0),
                        );
                        self.player.score += 10;
                        self.add_popup("+10", self.tiles[i].x, self.tiles[i].y);
                    }
                }
                self.player.vy = 0.0;
                break;
            }
        }
        if !collided_y {
            self.player.y = new_y;
            self.player.on_ground = false;
        }
        if landed {
            self.player.on_ground = true;
            self.player.jump_held = 0;
        }

        // --- Spike collision ---
        for i in 0..self.tiles.len() {
            if !self.tiles[i].alive { continue; }
            if self.tiles[i].kind != TileKind::Spike { continue; }
            if overlaps(self.player.x + 4.0, self.player.y + 4.0, TILE_SIZE - 8.0, TILE_SIZE - 8.0,
                       self.tiles[i].x + 4.0, self.tiles[i].y + 4.0, TILE_SIZE - 8.0, TILE_SIZE - 8.0) {
                self.damage_player();
                break;
            }
        }

        // --- Fall off map ---
        if self.player.y > self.map_height + 100.0 {
            self.player_die();
        }

        // --- Clamp player to map bounds ---
        if self.player.x < 0.0 { self.player.x = 0.0; }
        if self.player.x > self.map_width - TILE_SIZE { self.player.x = self.map_width - TILE_SIZE; }

        // --- Crystal collection ---
        let mut i = self.crystals.len();
        while i > 0 {
            i -= 1;
            if !self.crystals[i].alive { continue; }

            // Check if goal sentinel
            if self.crystals[i].anim_offset < -900.0 {
                // Goal trigger
                if overlaps(self.player.x, self.player.y, TILE_SIZE, TILE_SIZE,
                           self.crystals[i].x, self.crystals[i].y, TILE_SIZE, TILE_SIZE) {
                    // Check if boss level and boss not defeated
                    if self.current_level == 2 && !self.boss_defeated {
                        // Don't trigger goal yet
                    } else {
                        self.goal_reached = true;
                        self.goal_timer = 0;
                        self.shake_magnitude = 6.0;
                        self.spawn_particles(
                            self.player.x + TILE_SIZE * 0.5,
                            self.player.y + TILE_SIZE * 0.5,
                            20,
                            Color::new(1.0, 1.0, 0.5, 1.0),
                        );
                        self.add_popup("LEVEL CLEAR!", self.player.x - 30.0, self.player.y - 30.0);
                    }
                }
                continue;
            }

            if overlaps(self.player.x, self.player.y, TILE_SIZE, TILE_SIZE,
                       self.crystals[i].x, self.crystals[i].y, TILE_SIZE, TILE_SIZE) {
                self.crystals[i].alive = false;
                self.player.crystals_collected += 1;
                self.player.score += 50;
                self.add_popup("+50", self.crystals[i].x, self.crystals[i].y - 10.0);
                self.spawn_particles(
                    self.crystals[i].x + TILE_SIZE * 0.5,
                    self.crystals[i].y + TILE_SIZE * 0.5,
                    6,
                    Color::new(1.0, 0.85, 0.0, 1.0),
                );
            }
        }

        // --- Powerup collection ---
        for i in 0..self.powerups.len() {
            if !self.powerups[i].alive { continue; }
            if overlaps(self.player.x, self.player.y, TILE_SIZE, TILE_SIZE,
                       self.powerups[i].x, self.powerups[i].y, TILE_SIZE, TILE_SIZE) {
                self.powerups[i].alive = false;
                match self.powerups[i].kind {
                    PowerupKind::Shard => {
                        self.player.hp = self.player.max_hp;
                        self.add_popup("SHIELD!", self.powerups[i].x, self.powerups[i].y - 10.0);
                        self.spawn_particles(
                            self.powerups[i].x + TILE_SIZE * 0.5,
                            self.powerups[i].y + TILE_SIZE * 0.5,
                            8,
                            Color::new(0.2, 0.8, 1.0, 1.0),
                        );
                    }
                    PowerupKind::FireCrystal => {
                        self.player.has_fire = true;
                        self.add_popup("FIRE!", self.powerups[i].x, self.powerups[i].y - 10.0);
                        self.spawn_particles(
                            self.powerups[i].x + TILE_SIZE * 0.5,
                            self.powerups[i].y + TILE_SIZE * 0.5,
                            8,
                            Color::new(1.0, 0.5, 0.0, 1.0),
                        );
                    }
                }
                self.player.score += 100;
            }
        }

        // --- Enemy update ---
        for i in 0..self.enemies.len() {
            if !self.enemies[i].alive { continue; }

            self.enemies[i].anim_timer += 0.05;
            if self.enemies[i].hurt_timer > 0 {
                self.enemies[i].hurt_timer -= 1;
            }

            match self.enemies[i].kind {
                EnemyKind::Slime => {
                    // Patrol back and forth
                    self.enemies[i].x += self.enemies[i].vx;
                    if (self.enemies[i].x - self.enemies[i].start_x).abs() > 80.0 {
                        self.enemies[i].vx = -self.enemies[i].vx;
                        self.enemies[i].facing_right = self.enemies[i].vx > 0.0;
                    }
                    // Check tile collision for turning at edges
                    let ahead_x = if self.enemies[i].vx > 0.0 {
                        self.enemies[i].x + TILE_SIZE
                    } else {
                        self.enemies[i].x - 1.0
                    };
                    let below_y = self.enemies[i].y + TILE_SIZE + 2.0;
                    let mut has_floor = false;
                    for j in 0..self.tiles.len() {
                        if !self.tiles[j].alive { continue; }
                        if self.tiles[j].kind == TileKind::Spike { continue; }
                        if overlaps(ahead_x, below_y, 2.0, 2.0,
                                   self.tiles[j].x, self.tiles[j].y, TILE_SIZE, TILE_SIZE) {
                            has_floor = true;
                            break;
                        }
                    }
                    if !has_floor {
                        self.enemies[i].vx = -self.enemies[i].vx;
                        self.enemies[i].facing_right = self.enemies[i].vx > 0.0;
                    }
                }
                EnemyKind::Bat => {
                    // Sine wave flight
                    self.enemies[i].x += self.enemies[i].vx;
                    self.enemies[i].y = self.enemies[i].start_y + (self.enemies[i].anim_timer * 2.0).sin() * 30.0;
                    if (self.enemies[i].x - self.enemies[i].start_x).abs() > 100.0 {
                        self.enemies[i].vx = -self.enemies[i].vx;
                        self.enemies[i].facing_right = self.enemies[i].vx > 0.0;
                    }
                }
                EnemyKind::Boss => {
                    // Boss AI: move toward player, periodically charge
                    self.enemies[i].attack_timer += 1;

                    let dx = self.player.x - self.enemies[i].x;
                    self.enemies[i].facing_right = dx > 0.0;

                    if self.enemies[i].attack_timer < 120 {
                        // Move toward player slowly
                        let dir: f32 = if dx > 0.0 { 1.0 } else { -1.0 };
                        self.enemies[i].x += dir * 1.0;
                    } else if self.enemies[i].attack_timer < 150 {
                        // Charge!
                        let dir: f32 = if self.enemies[i].facing_right { 1.0 } else { -1.0 };
                        self.enemies[i].x += dir * 4.0;
                    } else {
                        self.enemies[i].attack_timer = 0;
                    }

                    // Keep boss in bounds
                    if self.enemies[i].x < 0.0 { self.enemies[i].x = 0.0; }
                    if self.enemies[i].x > self.map_width - TILE_SIZE * 2.0 {
                        self.enemies[i].x = self.map_width - TILE_SIZE * 2.0;
                    }
                }
            }

            // Enemy-player collision
            let ew = if self.enemies[i].kind == EnemyKind::Boss { TILE_SIZE * 2.0 } else { TILE_SIZE };
            let eh = if self.enemies[i].kind == EnemyKind::Boss { TILE_SIZE * 2.0 } else { TILE_SIZE };

            if overlaps(self.player.x + 2.0, self.player.y + 2.0, TILE_SIZE - 4.0, TILE_SIZE - 4.0,
                       self.enemies[i].x, self.enemies[i].y, ew, eh) {
                // Check if stomping (player falling and above enemy)
                let player_bottom = self.player.y + TILE_SIZE;
                let enemy_top = self.enemies[i].y + eh * 0.3;

                if self.player.vy > 0.0 && player_bottom < enemy_top + 8.0 {
                    // Stomp!
                    self.player.vy = STOMP_BOUNCE;
                    self.player.on_ground = false;
                    self.player.jump_held = 0;
                    self.enemies[i].hp -= 1;
                    self.enemies[i].hurt_timer = 10;
                    self.hit_stop_frames = 3;
                    self.shake_magnitude = 3.0;
                    self.player.score += 100;

                    if self.enemies[i].hp <= 0 {
                        self.enemies[i].alive = false;
                        let ex = self.enemies[i].x + ew * 0.5;
                        let ey = self.enemies[i].y + eh * 0.5;
                        let c = match self.enemies[i].kind {
                            EnemyKind::Slime => Color::new(0.2, 0.7, 0.2, 1.0),
                            EnemyKind::Bat => Color::new(0.5, 0.2, 0.7, 1.0),
                            EnemyKind::Boss => Color::new(0.9, 0.2, 0.1, 1.0),
                        };
                        self.spawn_particles(ex, ey, 12, c);

                        if self.enemies[i].kind == EnemyKind::Boss {
                            self.boss_defeated = true;
                            self.shake_magnitude = 12.0;
                            self.player.score += 1000;
                            self.add_popup("+1000", ex - 20.0, ey - 20.0);
                        } else {
                            self.add_popup("+100", ex - 10.0, ey - 10.0);
                        }
                    }
                } else {
                    // Player takes damage
                    self.damage_player();
                }
            }
        }

        // --- Fireball update ---
        for i in 0..self.fireballs.len() {
            if !self.fireballs[i].alive { continue; }

            self.fireballs[i].x += self.fireballs[i].vx;
            self.fireballs[i].vy += 0.05; // slight gravity on fireballs
            self.fireballs[i].y += self.fireballs[i].vy;
            self.fireballs[i].life -= 1;

            if self.fireballs[i].life <= 0 {
                self.fireballs[i].alive = false;
                continue;
            }

            // Fireball vs tiles
            let mut hit_tile = false;
            for j in 0..self.tiles.len() {
                if !self.tiles[j].alive { continue; }
                if self.tiles[j].kind == TileKind::Spike { continue; }
                if overlaps(self.fireballs[i].x, self.fireballs[i].y, 8.0, 8.0,
                           self.tiles[j].x, self.tiles[j].y, TILE_SIZE, TILE_SIZE) {
                    hit_tile = true;
                    if self.tiles[j].kind == TileKind::Breakable {
                        self.tiles[j].alive = false;
                        self.player.score += 10;
                    }
                    break;
                }
            }
            if hit_tile {
                self.fireballs[i].alive = false;
                self.spawn_particles(
                    self.fireballs[i].x, self.fireballs[i].y,
                    4, Color::new(1.0, 0.5, 0.0, 1.0),
                );
                continue;
            }

            // Fireball vs enemies
            for j in 0..self.enemies.len() {
                if !self.enemies[j].alive { continue; }
                let ew = if self.enemies[j].kind == EnemyKind::Boss { TILE_SIZE * 2.0 } else { TILE_SIZE };
                let eh = if self.enemies[j].kind == EnemyKind::Boss { TILE_SIZE * 2.0 } else { TILE_SIZE };
                if overlaps(self.fireballs[i].x, self.fireballs[i].y, 8.0, 8.0,
                           self.enemies[j].x, self.enemies[j].y, ew, eh) {
                    self.fireballs[i].alive = false;
                    self.enemies[j].hp -= 1;
                    self.enemies[j].hurt_timer = 10;
                    self.hit_stop_frames = 2;
                    self.shake_magnitude = 2.0;

                    if self.enemies[j].hp <= 0 {
                        self.enemies[j].alive = false;
                        let ex = self.enemies[j].x + ew * 0.5;
                        let ey = self.enemies[j].y + eh * 0.5;
                        let c = match self.enemies[j].kind {
                            EnemyKind::Slime => Color::new(0.2, 0.7, 0.2, 1.0),
                            EnemyKind::Bat => Color::new(0.5, 0.2, 0.7, 1.0),
                            EnemyKind::Boss => Color::new(0.9, 0.2, 0.1, 1.0),
                        };
                        self.spawn_particles(ex, ey, 12, c);
                        self.player.score += 100;
                        self.add_popup("+100", ex - 10.0, ey - 10.0);

                        if self.enemies[j].kind == EnemyKind::Boss {
                            self.boss_defeated = true;
                            self.shake_magnitude = 12.0;
                            self.player.score += 1000;
                        }
                    }
                    break;
                }
            }
        }

        // --- Clean up dead fireballs ---
        self.fireballs.retain(|f| f.alive);

        // --- Particle update ---
        for i in 0..self.particles.len() {
            self.particles[i].x += self.particles[i].vx;
            self.particles[i].y += self.particles[i].vy;
            self.particles[i].vy += 0.1;
            self.particles[i].life -= 1.0;
            self.particles[i].size *= 0.97;
        }
        self.particles.retain(|p| p.life > 0.0);

        // --- Popup update ---
        for i in 0..self.popups.len() {
            self.popups[i].y -= 0.8;
            self.popups[i].life -= 1.0;
        }
        self.popups.retain(|p| p.life > 0.0);

        // --- Dust mote update ---
        for mote in &mut self.dust_motes {
            mote.x += mote.vx;
            mote.y += mote.vy;
            if mote.y < -10.0 {
                mote.y = SCREEN_H + 10.0;
                mote.x = rand::gen_range(0.0_f32, SCREEN_W);
            }
            if mote.x < -10.0 { mote.x = SCREEN_W + 10.0; }
            if mote.x > SCREEN_W + 10.0 { mote.x = -10.0; }
        }

        // --- Player trail when moving fast ---
        if self.player.vx.abs() > 2.0 && self.frame_count % 3 == 0 {
            self.particles.push(Particle {
                x: self.player.x + TILE_SIZE * 0.5,
                y: self.player.y + TILE_SIZE * 0.8,
                vx: -self.player.vx * 0.1,
                vy: rand::gen_range(-0.5_f32, 0.0),
                life: 10.0,
                color: Color::new(0.4, 0.5, 0.9, 0.3),
                size: rand::gen_range(2.0_f32, 4.0),
            });
        }

        // --- Camera tracking ---
        let target_cam = self.player.x - SCREEN_W * 0.4;
        self.camera_x += (target_cam - self.camera_x) * 0.1;
        if self.camera_x < 0.0 { self.camera_x = 0.0; }
        if self.camera_x > self.map_width - SCREEN_W {
            self.camera_x = (self.map_width - SCREEN_W).max(0.0);
        }
    }

    fn update_story(&mut self, input: &Input) {
        self.story_frame_counter += 1;

        // Typewriter: 1 char every 2 frames
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

        // Skip with jump/attack/start
        if input.jump_pressed || input.attack_pressed || input.start {
            if self.story_line_index < self.story_lines.len() {
                // Reveal all remaining text
                self.story_full_text.clear();
                for (idx, line) in self.story_lines.iter().enumerate() {
                    self.story_full_text.push_str(line);
                    if idx < self.story_lines.len() - 1 {
                        self.story_full_text.push('\n');
                    }
                }
                self.story_line_index = self.story_lines.len();
            } else {
                // Text fully shown, proceed
                if self.story_is_victory {
                    self.state = GameState::Win;
                } else if self.state == GameState::Story {
                    // After intro story, go to level story
                    self.start_level_story();
                } else {
                    // After level story, start playing
                    self.load_level();
                    self.state = GameState::Playing;
                }
            }
        }
    }

    fn reset_game(&mut self) {
        self.player = Player::new();
        self.current_level = 0;
        self.boss_defeated = false;
        self.goal_reached = false;
        self.goal_timer = 0;
    }
}

// ---------------------------------------------------------------------------
// Drawing
// ---------------------------------------------------------------------------
fn draw_world(world: &World, sprites: &Sprites) {
    let cam_x = world.camera_x + world.screen_shake_x;
    let cam_y = world.screen_shake_y;

    // Background color based on level
    let bg_color = match world.current_level {
        0 => Color::new(0.05, 0.08, 0.15, 1.0),  // dark blue-purple
        1 => Color::new(0.04, 0.04, 0.08, 1.0),   // near black (caverns)
        _ => Color::new(0.08, 0.03, 0.05, 1.0),   // dark red (tower)
    };
    clear_background(bg_color);

    // Background tile pattern
    let bg_start_col = (cam_x / TILE_SIZE) as i32 - 1;
    let bg_end_col = bg_start_col + (SCREEN_W / TILE_SIZE) as i32 + 3;
    let bg_rows = (SCREEN_H / TILE_SIZE) as i32 + 2;
    for row in 0..bg_rows {
        for col in bg_start_col..bg_end_col {
            let bx = col as f32 * TILE_SIZE - (cam_x % TILE_SIZE);
            let by = row as f32 * TILE_SIZE;
            draw_texture_ex(
                &sprites.bg, bx, by,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
            );
        }
    }

    // Tiles
    for tile in &world.tiles {
        if !tile.alive { continue; }
        let sx = tile.x - cam_x + cam_y * 0.0; // cam_y used in shake
        let sy = tile.y + cam_y;
        if sx < -TILE_SIZE || sx > SCREEN_W + TILE_SIZE { continue; }
        if sy < -TILE_SIZE || sy > SCREEN_H + TILE_SIZE { continue; }

        let tex = match tile.kind {
            TileKind::Solid => &sprites.block,
            TileKind::Breakable => &sprites.breakable,
            TileKind::Spike => &sprites.spike,
        };
        draw_texture_ex(
            tex, sx, sy,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
        );
    }

    // Crystals
    for gem in &world.crystals {
        if !gem.alive { continue; }
        if gem.anim_offset < -900.0 {
            // Goal marker - draw as pulsing golden rectangle
            let sx = gem.x - cam_x;
            let sy = gem.y + cam_y;
            let pulse = ((world.frame_count as f32 * 0.05).sin() * 0.3 + 0.7) as f32;
            draw_rectangle(sx, sy, TILE_SIZE, TILE_SIZE, Color::new(1.0, 0.85, 0.0, pulse * 0.5));
            draw_rectangle_lines(sx, sy, TILE_SIZE, TILE_SIZE, 2.0, Color::new(1.0, 1.0, 0.5, pulse));
            continue;
        }
        let bob = (world.frame_count as f32 * 0.06 + gem.anim_offset).sin() * 3.0;
        let sx = gem.x - cam_x;
        let sy = gem.y + cam_y + bob;
        if sx < -TILE_SIZE || sx > SCREEN_W + TILE_SIZE { continue; }
        draw_texture_ex(
            &sprites.crystal, sx, sy,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
        );
        // Sparkle glow
        let glow_alpha = ((world.frame_count as f32 * 0.1 + gem.anim_offset).sin() * 0.3 + 0.2) as f32;
        draw_circle(sx + TILE_SIZE * 0.5, sy + TILE_SIZE * 0.5, TILE_SIZE * 0.6,
                    Color::new(1.0, 0.85, 0.0, glow_alpha));
    }

    // Powerups
    for pu in &world.powerups {
        if !pu.alive { continue; }
        let bob = (world.frame_count as f32 * 0.08 + pu.anim_offset).sin() * 4.0;
        let sx = pu.x - cam_x;
        let sy = pu.y + cam_y + bob;
        if sx < -TILE_SIZE || sx > SCREEN_W + TILE_SIZE { continue; }
        let tex = match pu.kind {
            PowerupKind::Shard => &sprites.shard,
            PowerupKind::FireCrystal => &sprites.fire_crystal,
        };
        draw_texture_ex(
            tex, sx, sy,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
        );
        // Glow
        let gc = match pu.kind {
            PowerupKind::Shard => Color::new(0.2, 0.8, 1.0, 0.2),
            PowerupKind::FireCrystal => Color::new(1.0, 0.5, 0.0, 0.2),
        };
        draw_circle(sx + TILE_SIZE * 0.5, sy + TILE_SIZE * 0.5, TILE_SIZE * 0.7, gc);
    }

    // Enemies
    for enemy in &world.enemies {
        if !enemy.alive { continue; }
        let ew = if enemy.kind == EnemyKind::Boss { TILE_SIZE * 2.0 } else { TILE_SIZE };
        let eh = if enemy.kind == EnemyKind::Boss { TILE_SIZE * 2.0 } else { TILE_SIZE };
        let sx = enemy.x - cam_x;
        let sy = enemy.y + cam_y;
        if sx < -ew || sx > SCREEN_W + ew { continue; }

        let tint = if enemy.hurt_timer > 0 { RED } else { WHITE };
        let tex = match enemy.kind {
            EnemyKind::Slime => &sprites.slime,
            EnemyKind::Bat => &sprites.bat,
            EnemyKind::Boss => &sprites.boss,
        };
        let flip = !enemy.facing_right;
        draw_texture_ex(
            tex, sx, sy,
            tint,
            DrawTextureParams {
                dest_size: Some(Vec2::new(ew, eh)),
                flip_x: flip,
                ..Default::default()
            },
        );

        // Boss health bar
        if enemy.kind == EnemyKind::Boss {
            let bar_w = ew;
            let bar_h = 4.0;
            let bar_x = sx;
            let bar_y = sy - 8.0;
            draw_rectangle(bar_x, bar_y, bar_w, bar_h, Color::new(0.3, 0.0, 0.0, 0.8));
            let hp_frac = enemy.hp as f32 / BOSS_HP as f32;
            draw_rectangle(bar_x, bar_y, bar_w * hp_frac, bar_h, RED);
        }
    }

    // Fireballs
    for fb in &world.fireballs {
        if !fb.alive { continue; }
        let sx = fb.x - cam_x;
        let sy = fb.y + cam_y;
        draw_texture_ex(
            &sprites.fireball, sx, sy,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(16.0, 16.0)),
                ..Default::default()
            },
        );
        // Trail glow
        draw_circle(sx + 8.0, sy + 8.0, 10.0, Color::new(1.0, 0.5, 0.0, 0.15));
    }

    // Player
    {
        // Blink when invulnerable
        let visible = world.player.invuln == 0 || (world.frame_count % 4 < 2);
        if visible {
            let sx = world.player.x - cam_x;
            let sy = world.player.y + cam_y;
            let flip = !world.player.facing_right;

            let tint = if world.player.has_fire {
                Color::new(1.0, 0.9, 0.8, 1.0) // slight warm tint
            } else {
                WHITE
            };

            draw_texture_ex(
                &sprites.knight, sx, sy,
                tint,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    flip_x: flip,
                    ..Default::default()
                },
            );

            // Fire aura
            if world.player.has_fire && world.frame_count % 5 == 0 {
                let glow_r = rand::gen_range(8.0_f32, 14.0);
                draw_circle(
                    sx + TILE_SIZE * 0.5 + rand::gen_range(-4.0_f32, 4.0),
                    sy + TILE_SIZE * 0.5 + rand::gen_range(-4.0_f32, 4.0),
                    glow_r,
                    Color::new(1.0, 0.4, 0.0, 0.1),
                );
            }
        }
    }

    // Particles
    for p in &world.particles {
        let sx = p.x - cam_x;
        let sy = p.y + cam_y;
        let alpha = (p.life / 30.0).min(1.0);
        let c = Color::new(p.color.r, p.color.g, p.color.b, alpha);
        draw_rectangle(sx - p.size * 0.5, sy - p.size * 0.5, p.size, p.size, c);
    }

    // Popups (floating text)
    for popup in &world.popups {
        let sx = popup.x - cam_x;
        let sy = popup.y + cam_y;
        let alpha = (popup.life / 60.0).min(1.0);
        draw_text(&popup.text, sx, sy, 16.0, Color::new(1.0, 1.0, 0.5, alpha));
    }

    // Dust motes (screen-space, no camera offset)
    for mote in &world.dust_motes {
        draw_circle(mote.x, mote.y, mote.size, Color::new(1.0, 1.0, 1.0, mote.alpha));
    }

    // --- HUD ---
    // Lives
    draw_text(&format!("LIVES: {}", world.player.lives), 10.0, 24.0, 20.0, WHITE);

    // HP
    let hp_text = if world.player.hp >= 2 { "HP: ##" } else { "HP: #" };
    draw_text(hp_text, 10.0, 46.0, 20.0,
              if world.player.hp >= 2 { Color::new(0.2, 0.8, 1.0, 1.0) } else { WHITE });

    // Score
    draw_text(&format!("SCORE: {}", world.player.score), SCREEN_W - 180.0, 24.0, 20.0, WHITE);

    // Crystals
    draw_text(&format!("CRYSTALS: {}", world.player.crystals_collected),
              SCREEN_W - 180.0, 46.0, 20.0, Color::new(1.0, 0.85, 0.0, 1.0));

    // Fire indicator
    if world.player.has_fire {
        draw_text("FIRE", 10.0, 68.0, 20.0, Color::new(1.0, 0.5, 0.0, 1.0));
    }

    // Level name
    draw_text(get_level_name(world.current_level), SCREEN_W * 0.5 - 60.0, 24.0, 16.0,
              Color::new(0.6, 0.6, 0.7, 0.6));

    // Goal reached banner
    if world.goal_reached {
        let alpha = ((world.goal_timer as f32 * 0.05).sin() * 0.3 + 0.7) as f32;
        draw_text("LEVEL COMPLETE!", SCREEN_W * 0.5 - 90.0, SCREEN_H * 0.4, 32.0,
                  Color::new(1.0, 1.0, 0.5, alpha));
    }

    // Damage flash overlay
    if world.damage_flash_timer > 0.0 {
        let alpha = (world.damage_flash_timer / 10.0).min(0.3);
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(1.0, 0.0, 0.0, alpha));
    }

    // Scanlines
    draw_scanlines();
    draw_vignette();
}

fn draw_title(frame_count: u64) {
    clear_background(Color::new(0.02, 0.02, 0.06, 1.0));

    // Animated background stars
    for i in 0..30_u32 {
        let seed = i as f32 * 137.5;
        let sx = (seed * 3.7 + (frame_count as f32 * 0.01 + seed).sin() * 20.0) % SCREEN_W;
        let sy = (seed * 2.3 + (frame_count as f32 * 0.008 + seed * 0.5).cos() * 15.0) % SCREEN_H;
        let brightness = ((frame_count as f32 * 0.03 + seed).sin() * 0.4 + 0.6) as f32;
        draw_circle(sx, sy, 1.5, Color::new(brightness, brightness, brightness * 1.2, brightness));
    }

    // Title
    let title_y = 140.0 + (frame_count as f32 * 0.03).sin() * 5.0;
    // Shadow
    draw_text("PIXEL KNIGHT", SCREEN_W * 0.5 - 118.0, title_y + 3.0, 40.0,
              Color::new(0.0, 0.0, 0.3, 0.5));
    // Main
    draw_text("PIXEL KNIGHT", SCREEN_W * 0.5 - 120.0, title_y, 40.0,
              Color::new(0.4, 0.5, 0.9, 1.0));

    // Subtitle
    draw_text("The Crystal Kingdom", SCREEN_W * 0.5 - 95.0, title_y + 40.0, 24.0,
              Color::new(1.0, 0.85, 0.0, 0.8));

    // Blinking start prompt
    if (frame_count / 30) % 2 == 0 {
        draw_text("PRESS START", SCREEN_W * 0.5 - 62.0, 340.0, 24.0, WHITE);
    }

    // Controls info
    draw_text("D-Pad: Move   B: Jump   A: Attack", SCREEN_W * 0.5 - 160.0, 400.0, 18.0,
              Color::new(0.5, 0.5, 0.6, 0.7));
    draw_text("Stomp enemies from above!", SCREEN_W * 0.5 - 115.0, 425.0, 18.0,
              Color::new(0.5, 0.5, 0.6, 0.7));

    draw_scanlines();
    draw_vignette();
}

fn draw_story(world: &World) {
    clear_background(Color::new(0.02, 0.02, 0.06, 1.0));

    // Draw revealed text
    let mut y = 100.0;
    for line in world.story_full_text.split('\n') {
        draw_text(line, 60.0, y, 20.0, Color::new(0.8, 0.85, 0.9, 1.0));
        y += 26.0;
    }

    // Blinking cursor
    if world.story_line_index < world.story_lines.len() {
        if (world.story_frame_counter / 15) % 2 == 0 {
            let cursor_y = y;
            let last_line = world.story_full_text.lines().last().unwrap_or("");
            let cursor_x = 60.0 + last_line.len() as f32 * 10.0;
            draw_rectangle(cursor_x, cursor_y - 14.0, 8.0, 16.0, Color::new(0.8, 0.85, 0.9, 0.8));
        }
    } else {
        // All text shown
        if (world.frame_count / 30) % 2 == 0 {
            draw_text("PRESS START", SCREEN_W * 0.5 - 62.0, SCREEN_H - 60.0, 24.0,
                      Color::new(1.0, 1.0, 0.5, 0.9));
        }
    }

    draw_scanlines();
    draw_vignette();
}

fn draw_game_over(world: &World, frame_count: u64) {
    clear_background(Color::new(0.05, 0.0, 0.0, 1.0));

    let shake = if frame_count < 30 { (frame_count as f32 * 0.5).sin() * 3.0 } else { 0.0 };

    draw_text("GAME OVER", SCREEN_W * 0.5 - 80.0 + shake, 180.0, 40.0, RED);
    draw_text(&format!("SCORE: {}", world.player.score), SCREEN_W * 0.5 - 60.0, 240.0, 24.0, WHITE);
    draw_text(&format!("CRYSTALS: {}", world.player.crystals_collected),
              SCREEN_W * 0.5 - 70.0, 280.0, 24.0, Color::new(1.0, 0.85, 0.0, 1.0));

    if (frame_count / 30) % 2 == 0 {
        draw_text("PRESS START TO RETRY", SCREEN_W * 0.5 - 110.0, 360.0, 24.0, WHITE);
    }

    draw_scanlines();
    draw_vignette();
}

fn draw_win_screen(world: &World, frame_count: u64) {
    clear_background(Color::new(0.02, 0.04, 0.1, 1.0));

    // Sparkle particles
    for i in 0..20_u32 {
        let seed = i as f32 * 97.3;
        let sx = (seed * 4.1 + frame_count as f32 * 0.3) % SCREEN_W;
        let sy = (seed * 2.7 + frame_count as f32 * 0.2) % SCREEN_H;
        let brightness = ((frame_count as f32 * 0.05 + seed).sin() * 0.5 + 0.5) as f32;
        draw_circle(sx, sy, 2.0, Color::new(1.0, 0.85, 0.0, brightness * 0.6));
    }

    draw_text("VICTORY!", SCREEN_W * 0.5 - 70.0, 160.0, 40.0, Color::new(1.0, 0.85, 0.0, 1.0));
    draw_text("The Crystal Kingdom is saved!", SCREEN_W * 0.5 - 140.0, 210.0, 24.0, WHITE);
    draw_text(&format!("FINAL SCORE: {}", world.player.score), SCREEN_W * 0.5 - 90.0, 260.0, 24.0, WHITE);
    draw_text(&format!("CRYSTALS: {}", world.player.crystals_collected),
              SCREEN_W * 0.5 - 70.0, 300.0, 24.0, Color::new(1.0, 0.85, 0.0, 1.0));

    if (frame_count / 30) % 2 == 0 {
        draw_text("PRESS START", SCREEN_W * 0.5 - 62.0, 380.0, 24.0, WHITE);
    }

    draw_scanlines();
    draw_vignette();
}

fn draw_scanlines() {
    let scanline_color = Color::new(0.0, 0.0, 0.0, 0.15);
    let mut y = 0.0_f32;
    while y < SCREEN_H {
        draw_line(0.0, y, SCREEN_W, y, 1.0, scanline_color);
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
        // Top
        draw_rectangle(0.0, 0.0, SCREEN_W, inset, c);
        // Bottom
        draw_rectangle(0.0, SCREEN_H - inset, SCREEN_W, inset, c);
        // Left
        draw_rectangle(0.0, 0.0, inset, SCREEN_H, c);
        // Right
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
        if frame_time > 0.25 { frame_time = 0.25; } // death spiral prevention
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
                    // Decrement jump buffer
                    if input.jump_buffer > 0 {
                        // pass to world for use, then decrement
                    }
                    world.update(&input);
                }
                GameState::GameOver => {
                    global_frame += 1;
                    if input.start || input.jump_pressed {
                        world.reset_game();
                        world.start_story(STORY_INTRO, false);
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

            // Decrement jump buffer after processing
            if input.jump_buffer > 0 {
                input.jump_buffer -= 1;
            }
            // Clear one-shot inputs after each tick
            input.jump_pressed = false;
            input.jump_released = false;
            input.attack_pressed = false;
            input.start = false;

            accumulator -= TIME_STEP;
        }

        // Draw
        match world.state {
            GameState::Start => draw_title(global_frame),
            GameState::Story | GameState::LevelStory => draw_story(&world),
            GameState::Playing => draw_world(&world, &sprites),
            GameState::GameOver => draw_game_over(&world, global_frame),
            GameState::Win => draw_win_screen(&world, global_frame),
        }

        next_frame().await;
    }
}
