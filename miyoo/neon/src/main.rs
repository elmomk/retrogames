// Neon Runner - Cyberpunk Platformer for Miyoo Mini Plus
// Rust/Macroquad port -- 640x480, 60fps fixed timestep
// Story: "Ghost Protocol" -- Kira-7 runs across Neo-Kyoto rooftops

use macroquad::prelude::*;

// -- Constants ----------------------------------------------------------------

const SCREEN_W: f32 = 640.0;
const SCREEN_H: f32 = 480.0;
const TIME_STEP: f64 = 1.0 / 60.0;
const TILE_SIZE: f32 = 32.0;
const GRAVITY: f32 = 0.4;
const MAX_FALL: f32 = 8.0;
const JUMP_FORCE: f32 = -8.0;
const MOVE_SPEED: f32 = 3.5;
const DASH_SPEED: f32 = 12.0;
const DASH_FRAMES: i32 = 8;
const DASH_COOLDOWN: i32 = 30;
const WALL_SLIDE_SPEED: f32 = 1.5;
const WALL_JUMP_X: f32 = 6.0;
const WALL_JUMP_Y: f32 = -7.0;
const COYOTE_MAX: i32 = 6;
const JUMP_BUFFER_MAX: i32 = 6;
const JUMP_HOLD_MAX: i32 = 10;
const INVULN_FRAMES: i32 = 60;
const ATTACK_DURATION: i32 = 10;
const MAX_PARTICLES: usize = 300;
const MAX_PROJECTILES: usize = 30;
const MAP_ROWS: usize = 15;

// Tile types
const TILE_EMPTY: u8 = 0;
const TILE_SOLID: u8 = 1;
#[allow(dead_code)]
const TILE_LASER: u8 = 2;
const TILE_ELECTRIC: u8 = 3;
const TILE_FALLING: u8 = 4;
const TILE_TERMINAL: u8 = 5;

// -- Palette ------------------------------------------------------------------

fn palette(ch: char) -> Option<Color> {
    match ch {
        'K' => Some(Color::new(0.067, 0.067, 0.067, 1.0)),   // near-black
        'D' => Some(Color::new(0.15, 0.15, 0.2, 1.0)),       // dark gray-blue
        'G' => Some(Color::new(0.333, 0.333, 0.4, 1.0)),     // gray
        'W' => Some(WHITE),
        'M' => Some(Color::new(1.0, 0.0, 0.8, 1.0)),         // magenta/neon pink
        'C' => Some(Color::new(0.0, 1.0, 1.0, 1.0)),         // cyan
        'N' => Some(Color::new(0.0, 0.8, 0.2, 1.0)),         // neon green
        'A' => Some(Color::new(1.0, 0.7, 0.0, 1.0)),         // amber
        'R' => Some(Color::new(1.0, 0.15, 0.15, 1.0)),       // red
        'B' => Some(Color::new(0.2, 0.3, 0.8, 1.0)),         // blue
        'P' => Some(Color::new(0.5, 0.0, 1.0, 1.0)),         // purple
        'Y' => Some(Color::new(1.0, 1.0, 0.0, 1.0)),         // yellow
        'S' => Some(Color::new(0.5, 0.5, 0.6, 1.0)),         // steel
        _ => None,
    }
}

fn sprite_to_texture(rows: &[&str], w: usize, h: usize) -> Texture2D {
    let mut pixels = vec![0u8; w * h * 4];
    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if x < w && y < h {
                if let Some(c) = palette(ch) {
                    let i = (y * w + x) * 4;
                    pixels[i] = (c.r * 255.0) as u8;
                    pixels[i + 1] = (c.g * 255.0) as u8;
                    pixels[i + 2] = (c.b * 255.0) as u8;
                    pixels[i + 3] = 255;
                }
            }
        }
    }
    let tex = Texture2D::from_rgba8(w as u16, h as u16, &pixels);
    tex.set_filter(FilterMode::Nearest);
    tex
}

// -- Sprite Data --------------------------------------------------------------

fn player_idle_sprite() -> Vec<&'static str> {
    vec![
        "....CCCC....",
        "...CDDDDC...",
        "...CDWWDC...",
        "...CCMMCC...",
        "....KKKK....",
        "...KDDDDKM..",
        "..KDDDDDDK..",
        "..KMDDDMDK..",
        "..KDDDDDDKM.",
        "...KDDDDDK..",
        "....KDDDK...",
        "...KDK.KDK..",
        "..KDK...KDK.",
        "..KK.....KK.",
    ]
}

fn player_run_sprite() -> Vec<&'static str> {
    vec![
        "....CCCC....",
        "...CDDDDC...",
        "...CDWWDC...",
        "...CCMMCC...",
        "....KKKK....",
        "..KKDDDKKK..",
        "..KDDDDDDDK.",
        ".KMDDDDDMDK.",
        "..KDDDDDDDK.",
        "...KKDDKK...",
        "..KDK...KDK.",
        ".KDK.....KDK",
        ".KK.......KK",
        ".............",
    ]
}

fn player_jump_sprite() -> Vec<&'static str> {
    vec![
        "....CCCC....",
        "...CDDDDC...",
        "...CDWWDC...",
        "...CCMMCC...",
        "....KKKK....",
        "...KKDDDKK..",
        "..KDDDDDDDK.",
        ".KMDDDDDMDDK",
        "..KDDDDDDDK.",
        "...KDDDDDK..",
        "..KDK...KDK.",
        ".KDK.....KDK",
        ".KK.......KK",
        ".............",
    ]
}

fn player_attack_sprite() -> Vec<&'static str> {
    vec![
        "....CCCC....",
        "...CDDDDC...",
        "...CDWWDC...",
        "...CCMMCC...",
        "....KKKKCCCC",
        "..KKDDDKCCCC",
        "..KDDDDDDCCC",
        ".KMDDDDDMCCC",
        "..KDDDDDDDK.",
        "...KDDDDDK..",
        "....KDDDK...",
        "...KDK.KDK..",
        "..KDK...KDK.",
        "..KK.....KK.",
    ]
}

fn drone_sprite() -> Vec<&'static str> {
    vec![
        "..MMMM..",
        ".MDDDM..",
        "MDRRDM..",
        "MDDDDM..",
        ".MDDM...",
        "KMMMMK..",
        "K....K..",
        "........",
    ]
}

fn guard_sprite() -> Vec<&'static str> {
    vec![
        "...RRRR.....",
        "..RSSSSSR...",
        "..RSWWSR....",
        "..RRRRR.....",
        "...RSSSR....",
        "..RSSSSSSR..",
        ".RSSSSSSSR..",
        "..RSSSSSR...",
        "...RSSSSR...",
        "..RSR..RSR..",
        ".RSR....RSR.",
        ".RR......RR.",
    ]
}

fn turret_sprite() -> Vec<&'static str> {
    vec![
        "..SSSS..",
        ".SRRRS..",
        "SRRRRS..",
        "SRRRRRS.",
        "SRRRRS..",
        ".SRRRS..",
        "SSSSSS..",
        "SSSSSS..",
    ]
}

fn boss_sprite() -> Vec<&'static str> {
    vec![
        "....RRRRRR......",
        "...RSSSSSSSR....",
        "..RSSWWWWSSR....",
        "..RRRRRRRRRR....",
        "...RRSSSSRR.....",
        "..RSSSSSSSSRRR..",
        ".RSSSSSSSSSSSRR.",
        "RRSSSMSSSMSSSRR.",
        ".RSSSSSSSSSSRR..",
        "..RSSSSSSSSR....",
        "...RRSSSSRR.....",
        "..RRSR..RSRR....",
        ".RRSR....RSRR...",
        ".RRR......RRR...",
        "................",
        "................",
    ]
}

fn bullet_sprite() -> Vec<&'static str> {
    vec![
        ".CC.",
        "CCCC",
        "CCCC",
        ".CC.",
    ]
}

fn emp_sprite() -> Vec<&'static str> {
    vec![
        ".PP.",
        "PPPP",
        "PPPP",
        ".PP.",
    ]
}

fn blade_slash_sprite() -> Vec<&'static str> {
    vec![
        "..CC",
        ".CCC",
        "CCCC",
        ".CCC",
        "..CC",
    ]
}

fn data_chip_sprite() -> Vec<&'static str> {
    vec![
        ".AAAA.",
        "AANNAA",
        "ANNNNA",
        "ANNNNA",
        "AANNAA",
        ".AAAA.",
    ]
}

fn health_sprite() -> Vec<&'static str> {
    vec![
        ".N.N..",
        "NNNNN.",
        "NNNNN.",
        ".NNN..",
        "..N...",
        "......",
    ]
}

fn emp_ammo_sprite() -> Vec<&'static str> {
    vec![
        ".PP.",
        "PPPP",
        "PPPP",
        ".PP.",
    ]
}

fn terminal_sprite() -> Vec<&'static str> {
    vec![
        "NNNNNN..",
        "NKKKN...",
        "NKNKN...",
        "NKKKN...",
        "NNNNNN..",
        ".KKKK...",
        "KKKKKK..",
        "........",
    ]
}

// -- Data Structures ----------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Start,
    Story,
    #[allow(dead_code)]
    LevelStory,
    Playing,
    GameOver,
    Win,
}

#[derive(Clone, Copy, PartialEq)]
enum EnemyKind {
    Drone,
    Guard,
    Turret,
    Boss,
}

#[derive(Clone, Copy, PartialEq)]
enum PickupKind {
    DataChip,
    Health,
    EmpAmmo,
}

#[derive(Clone, Copy, PartialEq)]
enum ProjOwner {
    Player,
    Enemy,
}

#[derive(Clone, Copy, PartialEq)]
enum StoryCallback {
    None,
    StartLevelIntro,
    BeginLevel,
    AdvanceAfterPost,
}

struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    w: f32,
    h: f32,
    hp: i32,
    max_hp: i32,
    lives: i32,
    facing: f32,
    on_ground: bool,
    on_wall: i32,
    coyote: i32,
    jump_buffer: i32,
    jump_held: i32,
    can_dash: bool,
    dashing: i32,
    dash_cooldown: i32,
    attacking: i32,
    emp_count: i32,
    invuln: i32,
    dead: bool,
    anim_timer: i32,
    score: i32,
    chips_collected: i32,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x, y,
            vx: 0.0, vy: 0.0,
            w: 14.0, h: 26.0,
            hp: 3, max_hp: 3,
            lives: 3,
            facing: 1.0,
            on_ground: false,
            on_wall: 0,
            coyote: 0,
            jump_buffer: 0,
            jump_held: 0,
            can_dash: true,
            dashing: 0,
            dash_cooldown: 0,
            attacking: 0,
            emp_count: 3,
            invuln: 0,
            dead: false,
            anim_timer: 0,
            score: 0,
            chips_collected: 0,
        }
    }

    fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.w, self.h)
    }

    fn attack_rect(&self) -> Rect {
        let range = 20.0;
        let ax = if self.facing > 0.0 { self.x + self.w } else { self.x - range };
        Rect::new(ax, self.y, range, self.h)
    }
}

struct Enemy {
    active: bool,
    kind: EnemyKind,
    x: f32,
    y: f32,
    vx: f32,
    #[allow(dead_code)]
    vy: f32,
    w: f32,
    h: f32,
    hp: i32,
    facing: f32,
    patrol_left: f32,
    patrol_right: f32,
    shoot_timer: i32,
    hurt_timer: i32,
    score_val: i32,
    stunned: i32,
}

impl Enemy {
    fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.w, self.h)
    }
}

struct Projectile {
    active: bool,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    w: f32,
    h: f32,
    owner: ProjOwner,
    is_emp: bool,
    life: i32,
}

impl Projectile {
    fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.w, self.h)
    }
}

struct Particle {
    active: bool,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: i32,
    max_life: i32,
    color: Color,
    size: f32,
    gravity: bool,
}

struct DashGhost {
    active: bool,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    life: i32,
    max_life: i32,
}

struct Pickup {
    active: bool,
    kind: PickupKind,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Pickup {
    fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.w, self.h)
    }
}

struct FallingPlatform {
    x: f32,
    y: f32,
    w: f32,
    timer: i32,
    falling: bool,
    vy: f32,
    gone: bool,
}

struct LaserGrid {
    x: f32,
    y: f32,
    h: f32,
    disabled: bool,
}

struct RainDrop {
    x: f32,
    y: f32,
    speed: f32,
    len: f32,
}

struct Camera {
    x: f32,
}

struct Popup {
    text: String,
    x: f32,
    y: f32,
    life: i32,
}

// -- Story Data ---------------------------------------------------------------

fn story_pre(index: usize) -> Vec<String> {
    match index {
        0 => vec![
            "Neo-Kyoto, 2087.".into(),
            "".into(),
            "The megacorps control everything --".into(),
            "information, resources, lives.".into(),
            "".into(),
            "You are Kira-7, a rogue hacker.".into(),
            "Once an engineer for Nexus Corp,".into(),
            "you discovered their darkest secret:".into(),
            "Project Ghost Protocol.".into(),
            "".into(),
            "They are rewriting memories.".into(),
            "Erasing dissidents from existence.".into(),
            "".into(),
            "You stole the proof. They erased your identity.".into(),
            "Now you must reach the broadcast tower".into(),
            "and expose the truth to the world.".into(),
        ],
        1 => vec![
            "--- SECTOR 1: NEON DISTRICT ---".into(),
            "".into(),
            "The rooftops of the Neon District.".into(),
            "Security drones patrol the skyline.".into(),
            "".into(),
            "Keep moving. They are tracking your signal.".into(),
        ],
        2 => vec![
            "--- SECTOR 2: INDUSTRIAL ZONE ---".into(),
            "".into(),
            "The old factories. Automated turrets".into(),
            "guard every corridor.".into(),
            "".into(),
            "Your contact left EMP charges hidden".into(),
            "along the route. Use them wisely.".into(),
        ],
        3 => vec![
            "--- SECTOR 3: NEXUS TOWER ---".into(),
            "".into(),
            "The broadcast tower looms above.".into(),
            "Nexus Corp has deployed everything --".into(),
            "drones, guards, and their prototype:".into(),
            "the Cerberus combat unit.".into(),
            "".into(),
            "This is it. No turning back.".into(),
        ],
        _ => vec![],
    }
}

fn story_post(index: usize) -> Vec<String> {
    match index {
        0 => vec![
            "You clear the Neon District rooftops.".into(),
            "A message flickers on a broken screen:".into(),
            "".into(),
            "'Kira -- they know you are coming.".into(),
            " The Industrial Zone is locked down.".into(),
            " I left supplies. Stay ghost.'".into(),
            "".into(),
            "-- ZERO (Unknown Ally)".into(),
        ],
        1 => vec![
            "The factory sector falls behind you.".into(),
            "Your decoder crackles with static.".into(),
            "".into(),
            "A familiar voice: your old partner, Ren.".into(),
            "'Kira, I am inside Nexus Tower.".into(),
            " I have been feeding them false data.".into(),
            " The broadcast array is on the roof.".into(),
            " But they deployed Cerberus.'".into(),
            "".into(),
            "'I will keep the doors open. Hurry.'".into(),
        ],
        2 => vec![
            "The Cerberus unit crashes to the ground.".into(),
            "You plug into the broadcast array.".into(),
            "".into(),
            "Every screen in Neo-Kyoto flickers.".into(),
            "The Ghost Protocol files stream out".into(),
            "to millions of eyes.".into(),
            "".into(),
            "Names. Dates. Erased lives restored.".into(),
            "".into(),
            "Nexus Corp's towers go dark, one by one.".into(),
            "".into(),
            "Ren's voice, barely a whisper:".into(),
            "'You did it, Kira. The ghosts are free.'".into(),
            "".into(),
            "The rain washes over Neo-Kyoto.".into(),
            "For the first time in years,".into(),
            "the city breathes.".into(),
        ],
        _ => vec![],
    }
}

const LEVEL_NAMES: [&str; 3] = [
    "NEON DISTRICT",
    "INDUSTRIAL ZONE",
    "NEXUS TOWER",
];

#[allow(dead_code)]
const LEVEL_SUBTITLES: [&str; 3] = [
    "Rooftop Run",
    "Factory Gauntlet",
    "The Broadcast",
];

// -- Level Maps ---------------------------------------------------------------
// 15 rows tall, variable width
// # solid, . empty, D drone, G guard, T turret, B boss
// L laser, E electric, C data chip, H health, M emp ammo
// K terminal, F falling platform

static LEVEL_1: &[&str] = &[
    ".......................................................................................................",
    ".......................................................................................................",
    "..............................................C.........................................................",
    ".............####...................#####..####...........................C.............................",
    ".......................................................................................................",
    "......C...............................................................................................#",
    ".....####...........##...............................................####.......####.............######",
    ".....................................................................................##................",
    "#####........##..........###.....D....####.......##...D.......###.............##..............##......#",
    ".......................................................##..........................................####",
    "..........D.......##.........####.............##.........####........D........####....####..........H.#",
    "##..............................................................##.....................................#",
    "......####...........###...........####..H.........####...........####.........####.......####......##",
    "##################################################################################################.#",
    "####################################################################################################",
];

static LEVEL_2: &[&str] = &[
    ".......................................................................................................",
    "......C...........................................................C....................................",
    "....####..........................................................................####.................",
    "...........##...............T.............####...........T.............................................#",
    ".......................####.......................................................####.................",
    "#####.........###...............##..............###...........##..............###.............##......#",
    "..........G..........................................................G.................................#",
    "....####...........####..K..........####............####..K..........####............####.......####.#",
    "..................................................................................................M..#",
    ".............###..........####..........H...####..........####..........####..........####..........##",
    "#####.............##.................##.............##.................##.............##.............G.#",
    "..........####..........####..........####..........####..........####..........####..........####...#",
    ".....G............................G............................G............................G........#",
    "####################################################################################################",
    "####################################################################################################",
];

static LEVEL_3: &[&str] = &[
    ".......................................................................................................",
    "...........................................................................C...........................",
    "......C...........T.......................................T...........####.............................",
    "....####...............####.......####............####..............................................####",
    ".................##.......................................................................####........",
    "#####........##..........###.........####.......##.........###..........####......##................G.#",
    "..........G..............................G.....................................................####..#",
    "....####...........####..K..........####............####..........####..K.......####.............H...#",
    "..................................................................................................####",
    ".............###..........####..........####..........####.......M..####..........####..........####.#",
    "#####.............##.................##.............##.......####..........##.............##.........B#",
    "..........####..........####..........####..........####..........####..........####..........####...#",
    ".....G............................G............................G......................D.....D........#",
    "####################################################################################################",
    "####################################################################################################",
];

// -- Collision ----------------------------------------------------------------

fn tile_at(map: &[Vec<u8>], map_cols: usize, x: f32, y: f32) -> u8 {
    let col = (x / TILE_SIZE) as isize;
    let row = (y / TILE_SIZE) as isize;
    if col < 0 || row < 0 || col >= map_cols as isize || row >= MAP_ROWS as isize {
        return TILE_EMPTY;
    }
    map[row as usize][col as usize]
}

fn is_solid(map: &[Vec<u8>], map_cols: usize, x: f32, y: f32) -> bool {
    tile_at(map, map_cols, x, y) == TILE_SOLID
}

fn rects_overlap(a: &Rect, b: &Rect) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

// -- Particle Spawners --------------------------------------------------------

fn spawn_particles(particles: &mut Vec<Particle>, x: f32, y: f32, count: usize, color: Color, speed: f32, life: i32) {
    for _ in 0..count {
        let p = Particle {
            active: true,
            x,
            y,
            vx: rand::gen_range(-speed, speed),
            vy: rand::gen_range(-speed, speed),
            life,
            max_life: life,
            color,
            size: rand::gen_range(1.0, 3.0),
            gravity: false,
        };
        if particles.len() < MAX_PARTICLES {
            particles.push(p);
        } else if let Some(slot) = particles.iter_mut().find(|pp| !pp.active) {
            *slot = p;
        }
    }
}

fn spawn_neon_particles(particles: &mut Vec<Particle>, x: f32, y: f32, count: usize) {
    let colors = [
        Color::new(1.0, 0.0, 0.8, 1.0),
        Color::new(0.0, 1.0, 1.0, 1.0),
        Color::new(0.0, 0.8, 0.2, 1.0),
    ];
    for _ in 0..count {
        let c = colors[rand::gen_range(0, 3) as usize];
        let p = Particle {
            active: true,
            x: x + rand::gen_range(-4.0, 4.0),
            y: y + rand::gen_range(-4.0, 4.0),
            vx: rand::gen_range(-2.0, 2.0),
            vy: rand::gen_range(-3.0, -0.5),
            life: rand::gen_range(10, 20),
            max_life: 15,
            color: c,
            size: rand::gen_range(1.0, 3.0),
            gravity: false,
        };
        if particles.len() < MAX_PARTICLES {
            particles.push(p);
        } else if let Some(slot) = particles.iter_mut().find(|pp| !pp.active) {
            *slot = p;
        }
    }
}

fn spawn_emp_burst(particles: &mut Vec<Particle>, x: f32, y: f32) {
    for i in 0..16 {
        let angle: f32 = (i as f32 / 16.0) * std::f32::consts::TAU;
        let spd: f32 = rand::gen_range(2.0, 5.0);
        let p = Particle {
            active: true,
            x,
            y,
            vx: angle.cos() * spd,
            vy: angle.sin() * spd,
            life: 15,
            max_life: 15,
            color: Color::new(0.5, 0.0, 1.0, 1.0),
            size: rand::gen_range(2.0, 4.0),
            gravity: false,
        };
        if particles.len() < MAX_PARTICLES {
            particles.push(p);
        } else if let Some(slot) = particles.iter_mut().find(|pp| !pp.active) {
            *slot = p;
        }
    }
}

// -- Game Struct --------------------------------------------------------------

struct Game {
    state: GameState,
    player: Player,
    enemies: Vec<Enemy>,
    projectiles: Vec<Projectile>,
    particles: Vec<Particle>,
    pickups: Vec<Pickup>,
    falling_platforms: Vec<FallingPlatform>,
    laser_grids: Vec<LaserGrid>,
    rain: Vec<RainDrop>,
    dash_ghosts: Vec<DashGhost>,
    popups: Vec<Popup>,
    camera: Camera,
    map: Vec<Vec<u8>>,
    map_cols: usize,
    shake_timer: i32,
    shake_intensity: f32,
    hit_stop: i32,
    frame: i64,
    blink_timer: i32,
    current_level: usize,
    // Story system
    story_lines: Vec<String>,
    story_char_index: usize,
    story_displayed: String,
    story_type_timer: i32,
    story_skip_ready: bool,
    story_callback: StoryCallback,
    // Textures
    tex_player_idle: Texture2D,
    tex_player_run: Texture2D,
    tex_player_jump: Texture2D,
    tex_player_attack: Texture2D,
    tex_drone: Texture2D,
    tex_guard: Texture2D,
    tex_turret: Texture2D,
    tex_boss: Texture2D,
    #[allow(dead_code)]
    tex_bullet: Texture2D,
    tex_emp: Texture2D,
    tex_blade: Texture2D,
    tex_data_chip: Texture2D,
    tex_health: Texture2D,
    tex_emp_ammo: Texture2D,
    tex_terminal: Texture2D,
}

// -- Level Building -----------------------------------------------------------

fn parse_level(map_data: &[&str]) -> (
    Vec<Vec<u8>>,
    Vec<Enemy>,
    Vec<Pickup>,
    Vec<FallingPlatform>,
    Vec<LaserGrid>,
    usize,
) {
    let rows = map_data.len();
    let cols = map_data[0].len();
    let mut map = vec![vec![TILE_EMPTY; cols]; rows];
    let mut enemies = Vec::new();
    let mut pickups = Vec::new();
    let mut falling = Vec::new();
    let mut lasers = Vec::new();

    for (r, row_str) in map_data.iter().enumerate() {
        for (c, ch) in row_str.chars().enumerate() {
            let px = c as f32 * TILE_SIZE;
            let py = r as f32 * TILE_SIZE;
            match ch {
                '#' => {
                    map[r][c] = TILE_SOLID;
                }
                'D' => {
                    enemies.push(Enemy {
                        active: true,
                        kind: EnemyKind::Drone,
                        x: px, y: py,
                        vx: 1.5, vy: 0.0,
                        w: 16.0, h: 16.0,
                        hp: 1,
                        facing: 1.0,
                        patrol_left: (px - 80.0).max(0.0),
                        patrol_right: px + 80.0,
                        shoot_timer: 0,
                        hurt_timer: 0,
                        score_val: 100,
                        stunned: 0,
                    });
                }
                'G' => {
                    enemies.push(Enemy {
                        active: true,
                        kind: EnemyKind::Guard,
                        x: px, y: py - 6.0,
                        vx: 1.0, vy: 0.0,
                        w: 14.0, h: 24.0,
                        hp: 2,
                        facing: -1.0,
                        patrol_left: (px - 60.0).max(0.0),
                        patrol_right: px + 60.0,
                        shoot_timer: 60,
                        hurt_timer: 0,
                        score_val: 150,
                        stunned: 0,
                    });
                }
                'T' => {
                    enemies.push(Enemy {
                        active: true,
                        kind: EnemyKind::Turret,
                        x: px, y: py,
                        vx: 0.0, vy: 0.0,
                        w: 16.0, h: 16.0,
                        hp: 3,
                        facing: -1.0,
                        patrol_left: px,
                        patrol_right: px,
                        shoot_timer: 40,
                        hurt_timer: 0,
                        score_val: 200,
                        stunned: 0,
                    });
                }
                'B' => {
                    enemies.push(Enemy {
                        active: true,
                        kind: EnemyKind::Boss,
                        x: px, y: py - 16.0,
                        vx: 1.5, vy: 0.0,
                        w: 28.0, h: 32.0,
                        hp: 12,
                        facing: -1.0,
                        patrol_left: (px - 80.0).max(0.0),
                        patrol_right: px + 80.0,
                        shoot_timer: 30,
                        hurt_timer: 0,
                        score_val: 1000,
                        stunned: 0,
                    });
                }
                'L' => {
                    lasers.push(LaserGrid {
                        x: px,
                        y: py,
                        h: TILE_SIZE * 2.0,
                        disabled: false,
                    });
                }
                'E' => {
                    map[r][c] = TILE_ELECTRIC;
                }
                'C' => {
                    pickups.push(Pickup {
                        active: true,
                        kind: PickupKind::DataChip,
                        x: px + 8.0, y: py + 8.0,
                        w: 12.0, h: 12.0,
                    });
                }
                'H' => {
                    pickups.push(Pickup {
                        active: true,
                        kind: PickupKind::Health,
                        x: px + 8.0, y: py + 8.0,
                        w: 12.0, h: 12.0,
                    });
                }
                'M' => {
                    pickups.push(Pickup {
                        active: true,
                        kind: PickupKind::EmpAmmo,
                        x: px + 8.0, y: py + 8.0,
                        w: 8.0, h: 8.0,
                    });
                }
                'K' => {
                    map[r][c] = TILE_TERMINAL;
                }
                'F' => {
                    falling.push(FallingPlatform {
                        x: px, y: py,
                        w: TILE_SIZE,
                        timer: 0,
                        falling: false,
                        vy: 0.0,
                        gone: false,
                    });
                    map[r][c] = TILE_FALLING;
                }
                _ => {}
            }
        }
    }

    (map, enemies, pickups, falling, lasers, cols)
}

fn level_data(lvl: usize) -> &'static [&'static str] {
    match lvl {
        0 => LEVEL_1,
        1 => LEVEL_2,
        _ => LEVEL_3,
    }
}

// -- Game Implementation ------------------------------------------------------

impl Game {
    fn new() -> Self {
        let (map, enemies, pickups, falling, lasers, cols) = parse_level(level_data(0));

        let mut rain = Vec::with_capacity(80);
        for _ in 0..80 {
            rain.push(RainDrop {
                x: rand::gen_range(0.0, SCREEN_W + 200.0),
                y: rand::gen_range(-SCREEN_H, SCREEN_H),
                speed: rand::gen_range(4.0, 8.0),
                len: rand::gen_range(4.0, 12.0),
            });
        }

        Self {
            state: GameState::Start,
            player: Player::new(2.0 * TILE_SIZE, 11.0 * TILE_SIZE),
            enemies,
            projectiles: Vec::with_capacity(MAX_PROJECTILES),
            particles: Vec::with_capacity(MAX_PARTICLES),
            pickups,
            falling_platforms: falling,
            laser_grids: lasers,
            rain,
            dash_ghosts: Vec::with_capacity(20),
            popups: Vec::with_capacity(10),
            camera: Camera { x: 0.0 },
            map,
            map_cols: cols,
            shake_timer: 0,
            shake_intensity: 0.0,
            hit_stop: 0,
            frame: 0,
            blink_timer: 0,
            current_level: 0,
            story_lines: Vec::new(),
            story_char_index: 0,
            story_displayed: String::new(),
            story_type_timer: 0,
            story_skip_ready: false,
            story_callback: StoryCallback::None,
            tex_player_idle: sprite_to_texture(&player_idle_sprite(), 13, 14),
            tex_player_run: sprite_to_texture(&player_run_sprite(), 13, 14),
            tex_player_jump: sprite_to_texture(&player_jump_sprite(), 13, 14),
            tex_player_attack: sprite_to_texture(&player_attack_sprite(), 13, 14),
            tex_drone: sprite_to_texture(&drone_sprite(), 8, 8),
            tex_guard: sprite_to_texture(&guard_sprite(), 12, 12),
            tex_turret: sprite_to_texture(&turret_sprite(), 8, 8),
            tex_boss: sprite_to_texture(&boss_sprite(), 16, 16),
            tex_bullet: sprite_to_texture(&bullet_sprite(), 4, 4),
            tex_emp: sprite_to_texture(&emp_sprite(), 4, 4),
            tex_blade: sprite_to_texture(&blade_slash_sprite(), 4, 5),
            tex_data_chip: sprite_to_texture(&data_chip_sprite(), 6, 6),
            tex_health: sprite_to_texture(&health_sprite(), 6, 6),
            tex_emp_ammo: sprite_to_texture(&emp_ammo_sprite(), 4, 4),
            tex_terminal: sprite_to_texture(&terminal_sprite(), 8, 8),
        }
    }

    fn reset_game(&mut self) {
        self.current_level = 0;
        self.player = Player::new(2.0 * TILE_SIZE, 11.0 * TILE_SIZE);
        self.build_level(0);
    }

    fn build_level(&mut self, lvl: usize) {
        let (map, enemies, pickups, falling, lasers, cols) = parse_level(level_data(lvl));
        self.map = map;
        self.enemies = enemies;
        self.pickups = pickups;
        self.falling_platforms = falling;
        self.laser_grids = lasers;
        self.map_cols = cols;
        self.projectiles.clear();
        self.particles.clear();
        self.dash_ghosts.clear();
        self.popups.clear();
        self.hit_stop = 0;
        self.player.x = 2.0 * TILE_SIZE;
        self.player.y = 11.0 * TILE_SIZE;
        self.player.vx = 0.0;
        self.player.vy = 0.0;
        self.player.dead = false;
        self.player.hp = self.player.max_hp;
        self.player.invuln = 0;
        self.player.dashing = 0;
        self.player.dash_cooldown = 0;
        self.player.attacking = 0;
        self.player.on_ground = false;
        self.player.on_wall = 0;
        self.player.coyote = 0;
        self.player.jump_buffer = 0;
        self.camera = Camera { x: 0.0 };
        self.shake_timer = 0;
        self.frame = 0;
    }

    fn show_story(&mut self, lines: Vec<String>, callback: StoryCallback) {
        self.state = GameState::Story;
        self.story_lines = lines;
        self.story_char_index = 0;
        self.story_displayed = String::new();
        self.story_type_timer = 0;
        self.story_skip_ready = false;
        self.story_callback = callback;
    }

    fn story_full_text(&self) -> String {
        self.story_lines.join("\n")
    }

    fn advance_level(&mut self) {
        let post_index = self.current_level;
        let lines = story_post(post_index);
        self.show_story(lines, StoryCallback::AdvanceAfterPost);
    }

    fn begin_level(&mut self) {
        self.build_level(self.current_level);
        self.state = GameState::Playing;
    }

    fn start_shake(&mut self, intensity: f32, frames: i32) {
        self.shake_intensity = intensity;
        self.shake_timer = frames;
    }

    // -- Update ---------------------------------------------------------------

    fn update(&mut self) {
        self.frame += 1;
        self.blink_timer = (self.blink_timer + 1) % 60;

        match self.state {
            GameState::Start => self.update_start(),
            GameState::Story => self.update_story(),
            GameState::LevelStory => self.update_story(),
            GameState::Playing => self.update_playing(),
            GameState::GameOver => self.update_game_over(),
            GameState::Win => self.update_win(),
        }
    }

    fn update_start(&mut self) {
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Space) {
            self.reset_game();
            let lines = story_pre(0);
            self.show_story(lines, StoryCallback::StartLevelIntro);
        }
    }

    fn update_win(&mut self) {
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::X) {
            self.state = GameState::Start;
        }
    }

    fn update_game_over(&mut self) {
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::X) {
            self.reset_game();
            self.state = GameState::Start;
        }
    }

    fn update_story(&mut self) {
        let full_text = self.story_full_text();
        self.story_type_timer += 1;
        if self.story_type_timer >= 2 && self.story_char_index < full_text.len() {
            self.story_char_index += 1;
            let mut end = self.story_char_index;
            while end < full_text.len() && !full_text.is_char_boundary(end) {
                end += 1;
            }
            self.story_displayed = full_text[..end].to_string();
            self.story_char_index = end;
            self.story_type_timer = 0;
        }
        if self.story_char_index >= full_text.len() {
            self.story_skip_ready = true;
        }

        if is_key_pressed(KeyCode::Z) || is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Space)
        {
            if !self.story_skip_ready {
                self.story_displayed = full_text;
                self.story_char_index = self.story_displayed.len();
                self.story_skip_ready = true;
            } else {
                let cb = self.story_callback;
                self.story_callback = StoryCallback::None;
                match cb {
                    StoryCallback::StartLevelIntro => {
                        let lines = story_pre(self.current_level + 1);
                        self.show_story(lines, StoryCallback::BeginLevel);
                    }
                    StoryCallback::BeginLevel => {
                        self.begin_level();
                    }
                    StoryCallback::AdvanceAfterPost => {
                        self.current_level += 1;
                        if self.current_level > 2 {
                            self.state = GameState::Win;
                        } else {
                            let lines = story_pre(self.current_level + 1);
                            self.show_story(lines, StoryCallback::BeginLevel);
                        }
                    }
                    StoryCallback::None => {
                        self.state = GameState::Playing;
                    }
                }
            }
        }
    }

    fn update_playing(&mut self) {
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Escape) {
            // Pause not fully implemented, just skip
        }

        if self.hit_stop > 0 {
            self.hit_stop -= 1;
            return;
        }

        // Update dash ghosts
        for ghost in self.dash_ghosts.iter_mut() {
            if ghost.active {
                ghost.life -= 1;
                if ghost.life <= 0 {
                    ghost.active = false;
                }
            }
        }
        self.dash_ghosts.retain(|g| g.active);

        // Update popups
        for pop in self.popups.iter_mut() {
            pop.y -= 0.5;
            pop.life -= 1;
        }
        self.popups.retain(|p| p.life > 0);

        self.update_player();
        self.update_enemies();
        self.update_projectiles();
        self.update_particles();
        self.update_pickups();
        self.update_falling_platforms();
        self.update_camera();
        self.update_rain();

        if self.shake_timer > 0 {
            self.shake_timer -= 1;
        }

        // Level end check
        let end_zone = (self.map_cols as f32 - 8.0) * TILE_SIZE;
        if self.player.x > end_zone && !self.player.dead {
            // Check if boss level and boss alive
            let boss_alive = self.enemies.iter().any(|e| e.active && e.kind == EnemyKind::Boss);
            if !boss_alive {
                self.advance_level();
            }
        }
    }

    fn update_player(&mut self) {
        if self.player.dead {
            return;
        }

        self.player.anim_timer += 1;
        if self.player.invuln > 0 {
            self.player.invuln -= 1;
        }
        if self.player.dash_cooldown > 0 {
            self.player.dash_cooldown -= 1;
        }

        // Coyote time
        if self.player.on_ground {
            self.player.coyote = COYOTE_MAX;
        } else if self.player.coyote > 0 {
            self.player.coyote -= 1;
        }

        // Jump buffer
        if self.player.jump_buffer > 0 {
            self.player.jump_buffer -= 1;
        }

        // Movement input
        let mut move_x = 0.0f32;
        if self.player.dashing <= 0 {
            if is_key_down(KeyCode::Left) {
                move_x = -1.0;
                self.player.facing = -1.0;
            }
            if is_key_down(KeyCode::Right) {
                move_x = 1.0;
                self.player.facing = 1.0;
            }
        }

        // Dashing
        if self.player.dashing > 0 {
            self.player.vx = DASH_SPEED * self.player.facing;
            self.player.vy = 0.0;
            self.player.dashing -= 1;
            // Spawn afterimage
            if self.frame % 2 == 0 {
                self.dash_ghosts.push(DashGhost {
                    active: true,
                    x: self.player.x, y: self.player.y,
                    w: self.player.w, h: self.player.h,
                    life: 8, max_life: 8,
                });
            }
            if self.player.dashing == 0 {
                self.player.vx = MOVE_SPEED * self.player.facing;
                self.player.dash_cooldown = DASH_COOLDOWN;
            }
        } else {
            self.player.vx = move_x * MOVE_SPEED;
        }

        // Jump (B = Space)
        if is_key_pressed(KeyCode::Space) {
            self.player.jump_buffer = JUMP_BUFFER_MAX;
        }

        if self.player.jump_buffer > 0 {
            if self.player.coyote > 0 {
                self.player.vy = JUMP_FORCE;
                self.player.on_ground = false;
                self.player.coyote = 0;
                self.player.jump_buffer = 0;
                self.player.jump_held = 1;
                let cx = self.player.x + self.player.w * 0.5;
                let cy = self.player.y + self.player.h;
                spawn_particles(&mut self.particles, cx, cy, 4, WHITE, 2.0, 8);
            } else if self.player.on_wall != 0 {
                self.player.vy = WALL_JUMP_Y;
                self.player.vx = -self.player.on_wall as f32 * WALL_JUMP_X;
                self.player.facing = -self.player.on_wall as f32;
                self.player.on_wall = 0;
                self.player.jump_buffer = 0;
                self.player.jump_held = 1;
                let sx = self.player.x + if self.player.facing < 0.0 { self.player.w } else { 0.0 };
                let sy = self.player.y + self.player.h * 0.5;
                spawn_particles(&mut self.particles, sx, sy, 5, SKYBLUE, 2.0, 8);
            }
        }

        // Variable jump height
        if is_key_down(KeyCode::Space) && self.player.jump_held > 0 && self.player.jump_held < JUMP_HOLD_MAX {
            self.player.jump_held += 1;
        } else {
            self.player.jump_held = 0;
        }
        if !is_key_down(KeyCode::Space) && self.player.vy < 0.0 && self.player.jump_held == 0 {
            self.player.vy *= 0.6;
        }

        // Dash (LeftShift or Z)
        if (is_key_pressed(KeyCode::LeftShift) || is_key_pressed(KeyCode::Z))
            && self.player.can_dash && self.player.dashing <= 0 && self.player.dash_cooldown <= 0
        {
            self.player.dashing = DASH_FRAMES;
            self.player.can_dash = false;
            self.player.invuln = DASH_FRAMES;
        }

        // Attack (A = X key)
        if is_key_pressed(KeyCode::X) && self.player.attacking <= 0 && self.player.dashing <= 0 {
            self.player.attacking = ATTACK_DURATION;
            let sx = if self.player.facing > 0.0 { self.player.x + self.player.w } else { self.player.x - 10.0 };
            let sy = self.player.y + self.player.h * 0.5;
            let facing = self.player.facing;
            for _ in 0..6 {
                let angle: f32 = rand::gen_range(-0.5, 0.5);
                let spd: f32 = rand::gen_range(2.0, 4.0);
                let pp = Particle {
                    active: true,
                    x: sx, y: sy,
                    vx: angle.cos() * spd * facing,
                    vy: angle.sin() * spd,
                    life: 8, max_life: 8,
                    color: Color::new(0.0, 1.0, 1.0, 1.0),
                    size: rand::gen_range(2.0, 4.0),
                    gravity: false,
                };
                if self.particles.len() < MAX_PARTICLES {
                    self.particles.push(pp);
                }
            }
            self.shake_intensity = 2.0;
            self.shake_timer = 3;
        }

        if self.player.attacking > 0 {
            self.player.attacking -= 1;
        }

        // EMP grenade (Up)
        if is_key_pressed(KeyCode::Up) && self.player.emp_count > 0 {
            self.player.emp_count -= 1;
            let sx = self.player.x + self.player.w * 0.5;
            let sy = self.player.y;
            let facing = self.player.facing;
            if self.projectiles.len() < MAX_PROJECTILES {
                self.projectiles.push(Projectile {
                    active: true,
                    x: sx, y: sy,
                    vx: facing * 4.0,
                    vy: -5.0,
                    w: 8.0, h: 8.0,
                    owner: ProjOwner::Player,
                    is_emp: true,
                    life: 90,
                });
            }
        }

        // Gravity
        if self.player.dashing <= 0 {
            if self.player.on_wall != 0 && self.player.vy > 0.0 {
                self.player.vy += GRAVITY * 0.3;
                if self.player.vy > WALL_SLIDE_SPEED {
                    self.player.vy = WALL_SLIDE_SPEED;
                }
            } else {
                self.player.vy += GRAVITY;
            }
            if self.player.vy > MAX_FALL {
                self.player.vy = MAX_FALL;
            }
        }

        // Move X with collision
        let new_x = self.player.x + self.player.vx;
        let mut blocked_x = false;
        let py = self.player.y;
        let pw = self.player.w;
        let ph = self.player.h;
        if self.player.vx < 0.0 {
            if is_solid(&self.map, self.map_cols, new_x, py + 2.0)
                || is_solid(&self.map, self.map_cols, new_x, py + ph - 2.0)
            {
                blocked_x = true;
            }
        } else if self.player.vx > 0.0 {
            if is_solid(&self.map, self.map_cols, new_x + pw, py + 2.0)
                || is_solid(&self.map, self.map_cols, new_x + pw, py + ph - 2.0)
            {
                blocked_x = true;
            }
        }
        if blocked_x {
            if self.player.vx > 0.0 {
                self.player.x = ((self.player.x + pw + self.player.vx) / TILE_SIZE).floor() * TILE_SIZE - pw;
            } else {
                self.player.x = ((self.player.x + self.player.vx) / TILE_SIZE).ceil() * TILE_SIZE;
            }
            self.player.vx = 0.0;
        } else {
            self.player.x = new_x;
        }

        // Wall detection
        self.player.on_wall = 0;
        if !self.player.on_ground {
            let px2 = self.player.x;
            let py2 = self.player.y;
            let pw2 = self.player.w;
            let ph2 = self.player.h;
            if is_key_down(KeyCode::Left)
                && (is_solid(&self.map, self.map_cols, px2 - 1.0, py2 + 4.0)
                    || is_solid(&self.map, self.map_cols, px2 - 1.0, py2 + ph2 - 4.0))
            {
                self.player.on_wall = -1;
            }
            if is_key_down(KeyCode::Right)
                && (is_solid(&self.map, self.map_cols, px2 + pw2 + 1.0, py2 + 4.0)
                    || is_solid(&self.map, self.map_cols, px2 + pw2 + 1.0, py2 + ph2 - 4.0))
            {
                self.player.on_wall = 1;
            }
        }

        // Wall slide sparks
        if self.player.on_wall != 0 && self.player.vy > 0.0 && self.frame % 3 == 0 {
            let wx = if self.player.on_wall < 0 { self.player.x } else { self.player.x + self.player.w };
            let scatter = -self.player.on_wall as f32;
            let spark_y = self.player.y + self.player.h * rand::gen_range(0.3, 0.9);
            let pp = Particle {
                active: true,
                x: wx, y: spark_y,
                vx: scatter * rand::gen_range(1.0, 3.0),
                vy: rand::gen_range(-1.0, 1.0),
                life: 8, max_life: 8,
                color: Color::new(1.0, 0.7, 0.0, 1.0),
                size: rand::gen_range(1.0, 2.5),
                gravity: false,
            };
            if self.particles.len() < MAX_PARTICLES {
                self.particles.push(pp);
            }
        }

        // Move Y with collision
        let new_y = self.player.y + self.player.vy;
        self.player.on_ground = false;

        if self.player.vy >= 0.0 {
            let check_y = new_y + self.player.h;
            let px3 = self.player.x;
            let pw3 = self.player.w;
            let ph3 = self.player.h;
            let left_solid = is_solid(&self.map, self.map_cols, px3 + 2.0, check_y);
            let right_solid = is_solid(&self.map, self.map_cols, px3 + pw3 - 2.0, check_y);
            if left_solid || right_solid {
                self.player.y = (check_y / TILE_SIZE).floor() * TILE_SIZE - ph3;
                self.player.vy = 0.0;
                self.player.on_ground = true;
                self.player.can_dash = true;
                self.player.jump_held = 0;
            } else {
                // Check falling platforms
                let mut on_falling = false;
                let pvy = self.player.vy;
                for fp in self.falling_platforms.iter_mut() {
                    if !fp.gone && !fp.falling {
                        let fp_rect = Rect::new(fp.x, fp.y, fp.w, 8.0);
                        let foot_rect = Rect::new(px3, new_y + ph3 - 2.0, pw3, 4.0);
                        if rects_overlap(&foot_rect, &fp_rect) && pvy >= 0.0 {
                            self.player.y = fp.y - ph3;
                            self.player.vy = 0.0;
                            self.player.on_ground = true;
                            self.player.can_dash = true;
                            on_falling = true;
                            fp.timer += 1;
                            if fp.timer > 30 {
                                fp.falling = true;
                            }
                            break;
                        }
                    }
                }
                if !on_falling {
                    self.player.y = new_y;
                }
            }
        } else {
            // Rising
            let check_y = new_y;
            let px4 = self.player.x;
            let pw4 = self.player.w;
            if is_solid(&self.map, self.map_cols, px4 + 2.0, check_y)
                || is_solid(&self.map, self.map_cols, px4 + pw4 - 2.0, check_y)
            {
                self.player.y = (check_y / TILE_SIZE).ceil() * TILE_SIZE;
                self.player.vy = 0.0;
            } else {
                self.player.y = new_y;
            }
        }

        // Electric floor damage
        let foot_tile = tile_at(&self.map, self.map_cols, self.player.x + self.player.w * 0.5, self.player.y + self.player.h + 1.0);
        let electric_hit = foot_tile == TILE_ELECTRIC && self.player.invuln <= 0;

        // Laser grid collision
        let p_rect = self.player.rect();
        let p_invuln = self.player.invuln;
        let mut laser_hit = false;
        for laser in &self.laser_grids {
            if !laser.disabled {
                let lr = Rect::new(laser.x, laser.y, 4.0, laser.h);
                if rects_overlap(&p_rect, &lr) && p_invuln <= 0 {
                    laser_hit = true;
                    break;
                }
            }
        }

        // Clamp
        if self.player.x < 0.0 { self.player.x = 0.0; }
        let max_x = self.map_cols as f32 * TILE_SIZE - self.player.w;
        if self.player.x > max_x { self.player.x = max_x; }

        // Fall death
        let fell = self.player.y > MAP_ROWS as f32 * TILE_SIZE;
        if fell {
            self.player.lives -= 1;
            self.player.hp = 0;
        }

        if electric_hit || laser_hit {
            self.damage_player(1);
        }
        if fell {
            self.player_die();
        }
    }

    fn damage_player(&mut self, dmg: i32) {
        if self.player.invuln > 0 || self.player.dead {
            return;
        }
        self.player.hp -= dmg;
        self.player.invuln = INVULN_FRAMES;
        self.start_shake(4.0, 8);
        self.hit_stop = 4;
        spawn_particles(
            &mut self.particles,
            self.player.x + self.player.w * 0.5,
            self.player.y + self.player.h * 0.5,
            8, RED, 3.0, 15,
        );
        if self.player.hp <= 0 {
            self.player.lives -= 1;
            if self.player.lives <= 0 {
                self.player_die();
            } else {
                // Respawn
                self.player.hp = self.player.max_hp;
                self.player.x = 2.0 * TILE_SIZE;
                self.player.y = 11.0 * TILE_SIZE;
                self.player.vx = 0.0;
                self.player.vy = 0.0;
                self.player.invuln = INVULN_FRAMES * 2;
            }
        }
    }

    fn player_die(&mut self) {
        self.player.dead = true;
        self.state = GameState::GameOver;
        spawn_particles(
            &mut self.particles,
            self.player.x + self.player.w * 0.5,
            self.player.y + self.player.h * 0.5,
            20, RED, 4.0, 30,
        );
        self.start_shake(6.0, 15);
    }

    fn update_enemies(&mut self) {
        let px = self.player.x;
        let py = self.player.y;
        let p_rect = self.player.rect();
        let p_attacking = self.player.attacking > 0 && self.player.attacking > ATTACK_DURATION - 6;
        let attack_rect = self.player.attack_rect();
        let p_invuln = self.player.invuln;
        let p_dashing = self.player.dashing;

        for i in 0..self.enemies.len() {
            if !self.enemies[i].active {
                continue;
            }

            // Stunned check
            if self.enemies[i].stunned > 0 {
                self.enemies[i].stunned -= 1;
                continue;
            }

            if self.enemies[i].hurt_timer > 0 {
                self.enemies[i].hurt_timer -= 1;
            }

            let cam_x = self.camera.x;
            let dist_to_cam = (self.enemies[i].x - cam_x).abs();
            if dist_to_cam > SCREEN_W + 200.0 {
                continue;
            }

            match self.enemies[i].kind {
                EnemyKind::Drone => {
                    let e = &mut self.enemies[i];
                    // Patrol horizontally, sine wave vertically
                    e.x += e.vx;
                    if e.x <= e.patrol_left || e.x >= e.patrol_right {
                        e.vx = -e.vx;
                        e.facing = if e.vx > 0.0 { 1.0 } else { -1.0 };
                    }
                    e.y += (self.frame as f32 * 0.05).sin() * 0.3;
                }
                EnemyKind::Guard => {
                    let e = &mut self.enemies[i];
                    let dx = px - e.x;
                    let detect_range = 150.0;
                    if dx.abs() < detect_range && (py - e.y).abs() < 60.0 {
                        e.facing = if dx > 0.0 { 1.0 } else { -1.0 };
                        e.x += e.facing * 2.0;
                    } else {
                        e.x += e.vx;
                        if e.x <= e.patrol_left || e.x >= e.patrol_right {
                            e.vx = -e.vx;
                            e.facing = if e.vx > 0.0 { 1.0 } else { -1.0 };
                        }
                    }
                    // Shoot
                    e.shoot_timer -= 1;
                    if e.shoot_timer <= 0 && dx.abs() < 200.0 && (py - e.y).abs() < 40.0 {
                        e.shoot_timer = 90;
                        let bvx = if dx > 0.0 { 4.0 } else { -4.0 };
                        if self.projectiles.len() < MAX_PROJECTILES {
                            self.projectiles.push(Projectile {
                                active: true,
                                x: e.x + e.w * 0.5,
                                y: e.y + e.h * 0.3,
                                vx: bvx, vy: 0.0,
                                w: 6.0, h: 4.0,
                                owner: ProjOwner::Enemy,
                                is_emp: false,
                                life: 120,
                            });
                        }
                    }
                }
                EnemyKind::Turret => {
                    let e = &mut self.enemies[i];
                    let dx = px - e.x;
                    e.facing = if dx > 0.0 { 1.0 } else { -1.0 };
                    e.shoot_timer -= 1;
                    if e.shoot_timer <= 0 && dx.abs() < 300.0 {
                        e.shoot_timer = 50;
                        let bvx = if dx > 0.0 { 5.0 } else { -5.0 };
                        if self.projectiles.len() < MAX_PROJECTILES {
                            self.projectiles.push(Projectile {
                                active: true,
                                x: e.x + e.w * 0.5,
                                y: e.y + e.h * 0.5,
                                vx: bvx, vy: 0.0,
                                w: 6.0, h: 4.0,
                                owner: ProjOwner::Enemy,
                                is_emp: false,
                                life: 150,
                            });
                        }
                    }
                }
                EnemyKind::Boss => {
                    let e = &mut self.enemies[i];
                    // Patrol and shoot
                    e.x += e.vx;
                    if e.x <= e.patrol_left || e.x >= e.patrol_right {
                        e.vx = -e.vx;
                        e.facing = if e.vx > 0.0 { 1.0 } else { -1.0 };
                    }
                    e.shoot_timer -= 1;
                    if e.shoot_timer <= 0 {
                        e.shoot_timer = 25;
                        let dx = px - e.x;
                        let dy = py - e.y;
                        let dist = (dx * dx + dy * dy).sqrt().max(1.0);
                        let spd = 4.0f32;
                        if self.projectiles.len() < MAX_PROJECTILES {
                            self.projectiles.push(Projectile {
                                active: true,
                                x: e.x + e.w * 0.5,
                                y: e.y + e.h * 0.5,
                                vx: dx / dist * spd,
                                vy: dy / dist * spd,
                                w: 6.0, h: 6.0,
                                owner: ProjOwner::Enemy,
                                is_emp: false,
                                life: 120,
                            });
                        }
                    }
                }
            }

            // Contact damage to player
            let e_rect = self.enemies[i].rect();
            if rects_overlap(&p_rect, &e_rect) && p_invuln <= 0 && p_dashing <= 0 {
                self.damage_player(1);
            }

            // Player attack hit
            if p_attacking {
                if rects_overlap(&attack_rect, &e_rect) && self.enemies[i].hurt_timer <= 0 {
                    self.enemies[i].hp -= 1;
                    self.enemies[i].hurt_timer = 10;
                    self.hit_stop = 3;
                    spawn_neon_particles(&mut self.particles, self.enemies[i].x + self.enemies[i].w * 0.5, self.enemies[i].y + self.enemies[i].h * 0.5, 6);
                    if self.enemies[i].hp <= 0 {
                        self.enemies[i].active = false;
                        self.player.score += self.enemies[i].score_val;
                        spawn_particles(&mut self.particles, self.enemies[i].x, self.enemies[i].y, 12,
                            Color::new(1.0, 0.0, 0.8, 1.0), 4.0, 20);
                        self.popups.push(Popup {
                            text: format!("+{}", self.enemies[i].score_val),
                            x: self.enemies[i].x,
                            y: self.enemies[i].y - 10.0,
                            life: 40,
                        });
                        self.start_shake(3.0, 5);
                    }
                }
            }
        }
    }

    fn update_projectiles(&mut self) {
        let p_rect = self.player.rect();
        let p_invuln = self.player.invuln;

        let mut i = self.projectiles.len();
        while i > 0 {
            i -= 1;
            if !self.projectiles[i].active {
                continue;
            }

            self.projectiles[i].x += self.projectiles[i].vx;
            self.projectiles[i].y += self.projectiles[i].vy;
            self.projectiles[i].life -= 1;

            // EMP gravity arc
            if self.projectiles[i].is_emp && self.projectiles[i].owner == ProjOwner::Player {
                self.projectiles[i].vy += 0.15;
            }

            // Off-screen or expired
            let expired = self.projectiles[i].life <= 0
                || self.projectiles[i].x < self.camera.x - 50.0
                || self.projectiles[i].x > self.camera.x + SCREEN_W + 50.0
                || self.projectiles[i].y < -50.0
                || self.projectiles[i].y > SCREEN_H + 50.0;

            // Tile collision
            let tcx = self.projectiles[i].x + self.projectiles[i].w * 0.5;
            let tcy = self.projectiles[i].y + self.projectiles[i].h * 0.5;
            let hit_tile = tile_at(&self.map, self.map_cols, tcx, tcy) == TILE_SOLID;

            if expired || hit_tile {
                // EMP explodes
                if self.projectiles[i].is_emp && self.projectiles[i].owner == ProjOwner::Player {
                    let ex = self.projectiles[i].x;
                    let ey = self.projectiles[i].y;
                    spawn_emp_burst(&mut self.particles, ex, ey);
                    for e in self.enemies.iter_mut() {
                        if e.active {
                            let dist = ((e.x - ex).powi(2) + (e.y - ey).powi(2)).sqrt();
                            if dist < 80.0 {
                                e.stunned = 120;
                            }
                        }
                    }
                    for laser in self.laser_grids.iter_mut() {
                        let dist = ((laser.x - ex).powi(2) + (laser.y - ey).powi(2)).sqrt();
                        if dist < 80.0 {
                            laser.disabled = true;
                        }
                    }
                    self.start_shake(3.0, 6);
                }
                self.projectiles[i].active = false;
                continue;
            }

            let pr = self.projectiles[i].rect();

            if self.projectiles[i].owner == ProjOwner::Enemy {
                // Hit player
                if rects_overlap(&pr, &p_rect) && p_invuln <= 0 {
                    self.projectiles[i].active = false;
                    self.damage_player(1);
                }
            } else if !self.projectiles[i].is_emp {
                // Player bullet hit enemies
                for e in self.enemies.iter_mut() {
                    if e.active && rects_overlap(&pr, &e.rect()) {
                        e.hp -= 1;
                        e.hurt_timer = 8;
                        self.projectiles[i].active = false;
                        if e.hp <= 0 {
                            e.active = false;
                            self.player.score += e.score_val;
                            spawn_particles(&mut self.particles, e.x, e.y, 10,
                                Color::new(1.0, 0.0, 0.8, 1.0), 3.0, 15);
                        }
                        break;
                    }
                }
            }
        }

        self.projectiles.retain(|p| p.active);
    }

    fn update_particles(&mut self) {
        for p in self.particles.iter_mut() {
            if p.active {
                p.x += p.vx;
                p.y += p.vy;
                if p.gravity {
                    p.vy += 0.15;
                }
                p.life -= 1;
                if p.life <= 0 {
                    p.active = false;
                }
            }
        }
    }

    fn update_pickups(&mut self) {
        let p_rect = self.player.rect();

        for i in 0..self.pickups.len() {
            if !self.pickups[i].active {
                continue;
            }
            let pk_rect = self.pickups[i].rect();
            if rects_overlap(&p_rect, &pk_rect) {
                self.pickups[i].active = false;
                match self.pickups[i].kind {
                    PickupKind::DataChip => {
                        self.player.chips_collected += 1;
                        self.player.score += 250;
                        self.popups.push(Popup {
                            text: "+250 DATA".to_string(),
                            x: self.pickups[i].x,
                            y: self.pickups[i].y - 10.0,
                            life: 40,
                        });
                        spawn_neon_particles(&mut self.particles, self.pickups[i].x, self.pickups[i].y, 8);
                    }
                    PickupKind::Health => {
                        if self.player.hp < self.player.max_hp {
                            self.player.hp += 1;
                        }
                        spawn_particles(&mut self.particles, self.pickups[i].x, self.pickups[i].y, 5,
                            Color::new(0.0, 1.0, 0.3, 1.0), 2.0, 12);
                    }
                    PickupKind::EmpAmmo => {
                        self.player.emp_count += 2;
                        spawn_particles(&mut self.particles, self.pickups[i].x, self.pickups[i].y, 5,
                            Color::new(0.5, 0.0, 1.0, 1.0), 2.0, 12);
                    }
                }
            }
        }

        // Terminal hack (stand near and press X)
        if is_key_pressed(KeyCode::X) {
            let px = self.player.x + self.player.w * 0.5;
            let py = self.player.y + self.player.h * 0.5;
            for r in 0..MAP_ROWS {
                for c in 0..self.map_cols {
                    if self.map[r][c] == TILE_TERMINAL {
                        let tx = c as f32 * TILE_SIZE + TILE_SIZE * 0.5;
                        let ty = r as f32 * TILE_SIZE + TILE_SIZE * 0.5;
                        let dist = ((px - tx).powi(2) + (py - ty).powi(2)).sqrt();
                        if dist < TILE_SIZE * 1.5 {
                            self.map[r][c] = TILE_EMPTY;
                            // Disable nearby lasers
                            for laser in self.laser_grids.iter_mut() {
                                let ld = ((laser.x - tx).powi(2) + (laser.y - ty).powi(2)).sqrt();
                                if ld < TILE_SIZE * 8.0 {
                                    laser.disabled = true;
                                }
                            }
                            spawn_neon_particles(&mut self.particles, tx, ty, 12);
                            self.player.score += 100;
                            self.popups.push(Popup {
                                text: "HACKED!".to_string(),
                                x: tx, y: ty - 20.0, life: 50,
                            });
                        }
                    }
                }
            }
        }
    }

    fn update_falling_platforms(&mut self) {
        for fp in self.falling_platforms.iter_mut() {
            if fp.gone {
                continue;
            }
            if fp.falling {
                fp.vy += 0.2;
                fp.y += fp.vy;
                if fp.y > MAP_ROWS as f32 * TILE_SIZE + 100.0 {
                    fp.gone = true;
                }
            }
        }
    }

    fn update_camera(&mut self) {
        let target = self.player.x - SCREEN_W * 0.35;
        self.camera.x += (target - self.camera.x) * 0.08;
        if self.camera.x < 0.0 {
            self.camera.x = 0.0;
        }
        let max_cam = self.map_cols as f32 * TILE_SIZE - SCREEN_W;
        if max_cam > 0.0 && self.camera.x > max_cam {
            self.camera.x = max_cam;
        }
    }

    fn update_rain(&mut self) {
        for drop in self.rain.iter_mut() {
            drop.y += drop.speed;
            drop.x -= 1.0;
            if drop.y > SCREEN_H {
                drop.y = rand::gen_range(-20.0, -5.0);
                drop.x = rand::gen_range(0.0, SCREEN_W + 200.0);
                drop.speed = rand::gen_range(4.0, 8.0);
                drop.len = rand::gen_range(4.0, 12.0);
            }
        }
    }

    // -- Draw -----------------------------------------------------------------

    fn draw(&self) {
        clear_background(Color::new(0.02, 0.02, 0.06, 1.0));

        match self.state {
            GameState::Start => self.draw_start(),
            GameState::Story | GameState::LevelStory => self.draw_story(),
            GameState::Playing => self.draw_playing(),
            GameState::GameOver => self.draw_game_over(),
            GameState::Win => self.draw_win(),
        }

        // CRT scanlines
        for y in (0..SCREEN_H as i32).step_by(4) {
            draw_rectangle(0.0, y as f32, SCREEN_W, 1.0, Color::new(0.0, 0.0, 0.0, 0.15));
        }

        // Vignette
        let vr = 8;
        for i in 0..vr {
            let alpha = 0.12 * (1.0 - i as f32 / vr as f32);
            let c = Color::new(0.0, 0.0, 0.0, alpha);
            let fi = i as f32 * 8.0;
            draw_rectangle(0.0, fi, SCREEN_W, 2.0, c);
            draw_rectangle(0.0, SCREEN_H - fi - 2.0, SCREEN_W, 2.0, c);
            draw_rectangle(fi, 0.0, 2.0, SCREEN_H, c);
            draw_rectangle(SCREEN_W - fi - 2.0, 0.0, 2.0, SCREEN_H, c);
        }
    }

    fn draw_start(&self) {
        // Parallax city background
        self.draw_city_bg(0.0);

        // Rain
        for drop in &self.rain {
            draw_line(
                drop.x, drop.y,
                drop.x - 1.0, drop.y + drop.len,
                1.0,
                Color::new(0.3, 0.4, 0.8, 0.3),
            );
        }

        // Title
        let title = "NEON RUNNER";
        let tw = measure_text(title, None, 40, 1.0).width;
        // Neon glow
        draw_text(title, SCREEN_W * 0.5 - tw * 0.5 + 2.0, 142.0, 40.0, Color::new(1.0, 0.0, 0.8, 0.3));
        draw_text(title, SCREEN_W * 0.5 - tw * 0.5, 140.0, 40.0, Color::new(1.0, 0.0, 0.8, 1.0));

        let sub = "Ghost Protocol";
        let sw = measure_text(sub, None, 20, 1.0).width;
        draw_text(sub, SCREEN_W * 0.5 - sw * 0.5, 175.0, 20.0, Color::new(0.0, 1.0, 1.0, 1.0));

        if self.blink_timer < 40 {
            let press = "PRESS START";
            let pw = measure_text(press, None, 20, 1.0).width;
            draw_text(press, SCREEN_W * 0.5 - pw * 0.5, 350.0, 20.0, Color::new(0.0, 0.8, 0.2, 1.0));
        }

        // Controls
        let controls = [
            "D-Pad: Move    B: Jump    A: Attack",
            "Shift/Z: Dash    Up: EMP Grenade",
        ];
        for (ci, line) in controls.iter().enumerate() {
            let lw = measure_text(line, None, 14, 1.0).width;
            draw_text(line, SCREEN_W * 0.5 - lw * 0.5, 410.0 + ci as f32 * 18.0, 14.0,
                Color::new(0.4, 0.4, 0.5, 1.0));
        }
    }

    fn draw_story(&self) {
        clear_background(Color::new(0.01, 0.01, 0.03, 1.0));

        // Terminal-style border
        draw_rectangle_lines(30.0, 30.0, SCREEN_W - 60.0, SCREEN_H - 60.0, 1.0,
            Color::new(0.0, 0.8, 0.2, 0.5));

        let header = "> GHOST PROTOCOL // CLASSIFIED";
        draw_text(header, 50.0, 60.0, 16.0, Color::new(0.0, 0.8, 0.2, 0.6));

        // Story text
        let mut y = 100.0;
        for line in self.story_displayed.split('\n') {
            draw_text(line, 50.0, y, 16.0, Color::new(0.0, 0.9, 0.3, 1.0));
            y += 20.0;
        }

        // Cursor blink
        if !self.story_skip_ready && self.blink_timer < 30 {
            let last_line = self.story_displayed.lines().last().unwrap_or("");
            let lw = measure_text(last_line, None, 16, 1.0).width;
            draw_rectangle(50.0 + lw + 2.0, y - 32.0, 8.0, 14.0, Color::new(0.0, 1.0, 0.3, 1.0));
        }

        if self.story_skip_ready && self.blink_timer < 40 {
            let skip = "[PRESS ANY KEY TO CONTINUE]";
            let skw = measure_text(skip, None, 14, 1.0).width;
            draw_text(skip, SCREEN_W * 0.5 - skw * 0.5, SCREEN_H - 50.0, 14.0,
                Color::new(0.0, 0.8, 0.2, 0.8));
        }
    }

    fn draw_playing(&self) {
        let cam_x = self.camera.x;
        let shake_x = if self.shake_timer > 0 {
            rand::gen_range(-self.shake_intensity, self.shake_intensity)
        } else { 0.0 };
        let shake_y = if self.shake_timer > 0 {
            rand::gen_range(-self.shake_intensity, self.shake_intensity)
        } else { 0.0 };

        // Parallax city
        self.draw_city_bg(cam_x);

        // Rain behind everything
        for drop in &self.rain {
            let rx = drop.x - cam_x * 0.2 + shake_x;
            draw_line(
                rx, drop.y + shake_y,
                rx - 1.0, drop.y + drop.len + shake_y,
                1.0,
                Color::new(0.3, 0.4, 0.8, 0.2),
            );
        }

        // Tiles
        let start_col = (cam_x / TILE_SIZE) as usize;
        let end_col = ((cam_x + SCREEN_W) / TILE_SIZE) as usize + 2;
        for r in 0..MAP_ROWS {
            for c in start_col..end_col.min(self.map_cols) {
                let tx = c as f32 * TILE_SIZE - cam_x + shake_x;
                let ty = r as f32 * TILE_SIZE + shake_y;
                match self.map[r][c] {
                    TILE_SOLID => {
                        // Neon-edged platform
                        draw_rectangle(tx, ty, TILE_SIZE, TILE_SIZE, Color::new(0.08, 0.08, 0.15, 1.0));
                        // Top edge glow
                        if r == 0 || self.map[r - 1][c] != TILE_SOLID {
                            draw_rectangle(tx, ty, TILE_SIZE, 2.0, Color::new(0.0, 1.0, 1.0, 0.5));
                        }
                        // Side edges
                        if c == 0 || self.map[r][c - 1] != TILE_SOLID {
                            draw_rectangle(tx, ty, 1.0, TILE_SIZE, Color::new(1.0, 0.0, 0.8, 0.3));
                        }
                        if c + 1 >= self.map_cols || self.map[r][c + 1] != TILE_SOLID {
                            draw_rectangle(tx + TILE_SIZE - 1.0, ty, 1.0, TILE_SIZE, Color::new(1.0, 0.0, 0.8, 0.3));
                        }
                    }
                    TILE_ELECTRIC => {
                        draw_rectangle(tx, ty, TILE_SIZE, TILE_SIZE, Color::new(0.08, 0.08, 0.15, 1.0));
                        // Electric sparks on top
                        let flicker = ((self.frame as f32 * 0.2 + c as f32).sin() * 0.5 + 0.5) as f32;
                        draw_rectangle(tx, ty, TILE_SIZE, 3.0,
                            Color::new(1.0, 1.0, 0.0, 0.4 + flicker * 0.4));
                    }
                    TILE_TERMINAL => {
                        draw_texture_ex(
                            &self.tex_terminal,
                            tx + 4.0, ty + 4.0,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(24.0, 24.0)),
                                ..Default::default()
                            },
                        );
                        // Glow pulse
                        let pulse = ((self.frame as f32 * 0.1).sin() * 0.3 + 0.5) as f32;
                        draw_rectangle(tx + 2.0, ty + 2.0, 28.0, 28.0,
                            Color::new(0.0, 1.0, 0.3, pulse * 0.15));
                    }
                    _ => {}
                }
            }
        }

        // Falling platforms
        for fp in &self.falling_platforms {
            if fp.gone { continue; }
            let fx = fp.x - cam_x + shake_x;
            let fy = fp.y + shake_y;
            let alpha = if fp.falling { 0.5 } else { 1.0 };
            draw_rectangle(fx, fy, fp.w, 8.0, Color::new(0.6, 0.3, 0.0, alpha));
            draw_rectangle(fx, fy, fp.w, 2.0, Color::new(1.0, 0.7, 0.0, alpha * 0.6));
            if fp.timer > 0 && !fp.falling {
                // Shake warning
                let sw2 = rand::gen_range(-1.0, 1.0) * (fp.timer as f32 / 30.0);
                draw_rectangle(fx + sw2, fy, fp.w, 8.0, Color::new(1.0, 0.0, 0.0, 0.2));
            }
        }

        // Laser grids
        for laser in &self.laser_grids {
            if laser.disabled { continue; }
            let lx = laser.x - cam_x + shake_x;
            let ly = laser.y + shake_y;
            let flicker = ((self.frame as f32 * 0.3).sin() * 0.3 + 0.7) as f32;
            draw_rectangle(lx, ly, 4.0, laser.h, Color::new(1.0, 0.0, 0.0, flicker));
            draw_rectangle(lx - 2.0, ly, 8.0, laser.h, Color::new(1.0, 0.0, 0.0, flicker * 0.15));
        }

        // Dash ghosts
        for ghost in &self.dash_ghosts {
            if !ghost.active { continue; }
            let alpha = ghost.life as f32 / ghost.max_life as f32 * 0.4;
            draw_rectangle(
                ghost.x - cam_x + shake_x, ghost.y + shake_y,
                ghost.w, ghost.h,
                Color::new(0.0, 1.0, 1.0, alpha),
            );
        }

        // Pickups
        for pk in &self.pickups {
            if !pk.active { continue; }
            let px = pk.x - cam_x + shake_x;
            let py = pk.y + shake_y + (self.frame as f32 * 0.08).sin() * 3.0;
            let tex = match pk.kind {
                PickupKind::DataChip => &self.tex_data_chip,
                PickupKind::Health => &self.tex_health,
                PickupKind::EmpAmmo => &self.tex_emp_ammo,
            };
            draw_texture_ex(
                tex, px, py, WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(pk.w, pk.h)),
                    ..Default::default()
                },
            );
            // Glow
            let glow_color = match pk.kind {
                PickupKind::DataChip => Color::new(1.0, 0.7, 0.0, 0.15),
                PickupKind::Health => Color::new(0.0, 1.0, 0.3, 0.15),
                PickupKind::EmpAmmo => Color::new(0.5, 0.0, 1.0, 0.15),
            };
            draw_rectangle(px - 2.0, py - 2.0, pk.w + 4.0, pk.h + 4.0, glow_color);
        }

        // Enemies
        for e in &self.enemies {
            if !e.active { continue; }
            let ex = e.x - cam_x + shake_x;
            let ey = e.y + shake_y;

            if e.stunned > 0 {
                // Stunned flash
                let flash = if self.frame % 4 < 2 { 0.3 } else { 0.7 };
                draw_rectangle(ex, ey, e.w, e.h, Color::new(0.5, 0.0, 1.0, flash));
                continue;
            }

            let hurt_flash = e.hurt_timer > 0 && self.frame % 4 < 2;

            let tex = match e.kind {
                EnemyKind::Drone => &self.tex_drone,
                EnemyKind::Guard => &self.tex_guard,
                EnemyKind::Turret => &self.tex_turret,
                EnemyKind::Boss => &self.tex_boss,
            };

            let flip = e.facing < 0.0;
            let color = if hurt_flash { RED } else { WHITE };

            draw_texture_ex(
                tex, ex, ey, color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(e.w, e.h)),
                    flip_x: flip,
                    ..Default::default()
                },
            );

            // Boss HP bar
            if e.kind == EnemyKind::Boss {
                let bar_w = 40.0;
                let bar_h = 4.0;
                let hp_frac = e.hp as f32 / 12.0;
                draw_rectangle(ex, ey - 8.0, bar_w, bar_h, Color::new(0.2, 0.2, 0.2, 0.8));
                draw_rectangle(ex, ey - 8.0, bar_w * hp_frac, bar_h, RED);
            }
        }

        // Projectiles
        for proj in &self.projectiles {
            if !proj.active { continue; }
            let px = proj.x - cam_x + shake_x;
            let py = proj.y + shake_y;
            if proj.is_emp {
                draw_texture_ex(
                    &self.tex_emp, px, py, WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(proj.w, proj.h)),
                        ..Default::default()
                    },
                );
            } else {
                let color = if proj.owner == ProjOwner::Enemy {
                    Color::new(1.0, 0.15, 0.15, 1.0)
                } else {
                    Color::new(0.0, 1.0, 1.0, 1.0)
                };
                draw_rectangle(px, py, proj.w, proj.h, color);
                draw_rectangle(px - 1.0, py - 1.0, proj.w + 2.0, proj.h + 2.0,
                    Color::new(color.r, color.g, color.b, 0.2));
            }
        }

        // Particles
        for p in &self.particles {
            if !p.active { continue; }
            let px = p.x - cam_x + shake_x;
            let py = p.y + shake_y;
            let alpha = p.life as f32 / p.max_life as f32;
            let c = Color::new(p.color.r, p.color.g, p.color.b, alpha);
            draw_rectangle(px, py, p.size, p.size, c);
        }

        // Player
        if !self.player.dead {
            let p = &self.player;
            let px = p.x - cam_x + shake_x;
            let py = p.y + shake_y;

            // Invuln blink
            if p.invuln > 0 && self.frame % 4 < 2 {
                // Skip drawing
            } else {
                let tex = if p.attacking > 0 {
                    &self.tex_player_attack
                } else if !p.on_ground {
                    &self.tex_player_jump
                } else if p.vx.abs() > 0.5 {
                    &self.tex_player_run
                } else {
                    &self.tex_player_idle
                };

                let flip = p.facing < 0.0;
                draw_texture_ex(
                    tex, px, py, WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(p.w + 4.0, p.h + 4.0)),
                        flip_x: flip,
                        ..Default::default()
                    },
                );

                // Blade slash effect
                if p.attacking > 0 {
                    let sx = if p.facing > 0.0 { px + p.w } else { px - 16.0 };
                    draw_texture_ex(
                        &self.tex_blade, sx, py + 4.0, WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(16.0, 20.0)),
                            flip_x: p.facing < 0.0,
                            ..Default::default()
                        },
                    );
                }
            }
        }

        // Popups
        for pop in &self.popups {
            let px = pop.x - cam_x + shake_x;
            let alpha = pop.life as f32 / 40.0;
            draw_text(&pop.text, px, pop.y + shake_y, 14.0,
                Color::new(1.0, 1.0, 0.0, alpha));
        }

        // HUD
        self.draw_hud();
    }

    fn draw_city_bg(&self, cam_x: f32) {
        // Parallax city skyline
        let bg_color = Color::new(0.02, 0.02, 0.06, 1.0);
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, bg_color);

        // Stars
        for i in 0..30 {
            let sx = ((i * 73 + 17) % 640) as f32;
            let sy = ((i * 47 + 31) % 200) as f32;
            let bright = ((self.frame as f32 * 0.02 + i as f32).sin() * 0.5 + 0.5) as f32;
            draw_rectangle(sx, sy, 1.0, 1.0, Color::new(1.0, 1.0, 1.0, bright * 0.6));
        }

        // Far buildings (slow parallax)
        let offset1 = cam_x * 0.05;
        for i in 0..20 {
            let bx = (i as f32 * 80.0 - offset1 % 1600.0 + 1600.0) % 1600.0 - 200.0;
            let bh = 60.0 + ((i * 37) % 80) as f32;
            let by = SCREEN_H - bh - 40.0;
            draw_rectangle(bx, by, 50.0, bh + 40.0, Color::new(0.03, 0.03, 0.08, 1.0));
            // Windows
            for wy in (0..bh as i32).step_by(12) {
                for wx in (5..45).step_by(10) {
                    let on = ((i + wy as usize + wx as usize) % 3) != 0;
                    if on {
                        let wc = if (i + wx as usize) % 5 == 0 {
                            Color::new(1.0, 0.0, 0.8, 0.15)
                        } else {
                            Color::new(1.0, 0.8, 0.3, 0.1)
                        };
                        draw_rectangle(bx + wx as f32, by + wy as f32, 4.0, 6.0, wc);
                    }
                }
            }
        }

        // Near buildings (faster parallax)
        let offset2 = cam_x * 0.15;
        for i in 0..12 {
            let bx = (i as f32 * 120.0 - offset2 % 1440.0 + 1440.0) % 1440.0 - 200.0;
            let bh = 100.0 + ((i * 53) % 120) as f32;
            let by = SCREEN_H - bh;
            draw_rectangle(bx, by, 80.0, bh, Color::new(0.04, 0.04, 0.1, 1.0));
            // Neon signs
            if i % 3 == 0 {
                let neon_c = if i % 2 == 0 {
                    Color::new(1.0, 0.0, 0.8, 0.3)
                } else {
                    Color::new(0.0, 1.0, 1.0, 0.3)
                };
                draw_rectangle(bx + 10.0, by + 10.0, 60.0, 8.0, neon_c);
            }
        }
    }

    fn draw_hud(&self) {
        // HP bar
        let bar_x = 10.0;
        let bar_y = 10.0;
        let bar_w = 80.0;
        let bar_h = 12.0;
        draw_rectangle(bar_x - 1.0, bar_y - 1.0, bar_w + 2.0, bar_h + 2.0,
            Color::new(0.0, 1.0, 1.0, 0.3));
        draw_rectangle(bar_x, bar_y, bar_w, bar_h, Color::new(0.1, 0.1, 0.15, 0.8));
        let hp_frac = self.player.hp as f32 / self.player.max_hp as f32;
        let hp_color = if hp_frac > 0.5 {
            Color::new(0.0, 1.0, 0.3, 1.0)
        } else if hp_frac > 0.25 {
            Color::new(1.0, 0.7, 0.0, 1.0)
        } else {
            Color::new(1.0, 0.0, 0.2, 1.0)
        };
        draw_rectangle(bar_x, bar_y, bar_w * hp_frac, bar_h, hp_color);

        // Lives
        let lives_text = format!("x{}", self.player.lives);
        draw_text(&lives_text, bar_x + bar_w + 8.0, bar_y + 11.0, 14.0,
            Color::new(0.0, 1.0, 1.0, 1.0));

        // EMP count
        let emp_text = format!("EMP:{}", self.player.emp_count);
        draw_text(&emp_text, 10.0, 36.0, 14.0, Color::new(0.5, 0.0, 1.0, 1.0));

        // Score
        let score_text = format!("SCORE:{}", self.player.score);
        let stw = measure_text(&score_text, None, 16, 1.0).width;
        draw_text(&score_text, SCREEN_W - stw - 10.0, 22.0, 16.0,
            Color::new(1.0, 0.7, 0.0, 1.0));

        // Data chips
        let chip_text = format!("DATA:{}", self.player.chips_collected);
        let ctw = measure_text(&chip_text, None, 14, 1.0).width;
        draw_text(&chip_text, SCREEN_W - ctw - 10.0, 40.0, 14.0,
            Color::new(0.0, 1.0, 1.0, 1.0));

        // Level name
        let lname = if self.current_level < 3 { LEVEL_NAMES[self.current_level] } else { "???" };
        draw_text(lname, SCREEN_W * 0.5 - measure_text(lname, None, 14, 1.0).width * 0.5, 22.0, 14.0,
            Color::new(0.0, 0.8, 0.2, 0.6));
    }

    fn draw_game_over(&self) {
        self.draw_city_bg(0.0);

        let title = "SIGNAL LOST";
        let tw = measure_text(title, None, 36, 1.0).width;
        draw_text(title, SCREEN_W * 0.5 - tw * 0.5, 180.0, 36.0, RED);

        let score = format!("FINAL SCORE: {}", self.player.score);
        let sw = measure_text(&score, None, 20, 1.0).width;
        draw_text(&score, SCREEN_W * 0.5 - sw * 0.5, 230.0, 20.0,
            Color::new(1.0, 0.7, 0.0, 1.0));

        if self.blink_timer < 40 {
            let press = "PRESS START TO RETRY";
            let pw = measure_text(press, None, 18, 1.0).width;
            draw_text(press, SCREEN_W * 0.5 - pw * 0.5, 320.0, 18.0,
                Color::new(0.0, 0.8, 0.2, 1.0));
        }
    }

    fn draw_win(&self) {
        self.draw_city_bg(0.0);

        let title = "GHOST PROTOCOL";
        let tw = measure_text(title, None, 36, 1.0).width;
        draw_text(title, SCREEN_W * 0.5 - tw * 0.5 + 2.0, 122.0, 36.0,
            Color::new(0.0, 1.0, 1.0, 0.3));
        draw_text(title, SCREEN_W * 0.5 - tw * 0.5, 120.0, 36.0,
            Color::new(0.0, 1.0, 1.0, 1.0));

        let sub = "TRANSMISSION COMPLETE";
        let sw = measure_text(sub, None, 20, 1.0).width;
        draw_text(sub, SCREEN_W * 0.5 - sw * 0.5, 160.0, 20.0,
            Color::new(0.0, 0.8, 0.2, 1.0));

        let score = format!("FINAL SCORE: {}", self.player.score);
        let scw = measure_text(&score, None, 20, 1.0).width;
        draw_text(&score, SCREEN_W * 0.5 - scw * 0.5, 220.0, 20.0,
            Color::new(1.0, 0.7, 0.0, 1.0));

        let chips = format!("DATA RECOVERED: {}", self.player.chips_collected);
        let cw = measure_text(&chips, None, 16, 1.0).width;
        draw_text(&chips, SCREEN_W * 0.5 - cw * 0.5, 250.0, 16.0,
            Color::new(0.0, 1.0, 1.0, 1.0));

        if self.blink_timer < 40 {
            let press = "PRESS START";
            let pw = measure_text(press, None, 18, 1.0).width;
            draw_text(press, SCREEN_W * 0.5 - pw * 0.5, 350.0, 18.0,
                Color::new(0.0, 0.8, 0.2, 1.0));
        }
    }
}

// -- Main Loop ----------------------------------------------------------------

fn window_conf() -> Conf {
    Conf {
        window_title: "Neon Runner - Ghost Protocol".to_string(),
        window_width: SCREEN_W as i32,
        window_height: SCREEN_H as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    let mut accumulator: f64 = 0.0;
    let mut last_time = get_time();

    loop {
        let current_time = get_time();
        let mut dt = current_time - last_time;
        last_time = current_time;

        // Death spiral prevention
        if dt > 0.1 {
            dt = 0.1;
        }

        accumulator += dt;

        while accumulator >= TIME_STEP {
            game.update();
            accumulator -= TIME_STEP;
        }

        game.draw();
        next_frame().await;
    }
}
