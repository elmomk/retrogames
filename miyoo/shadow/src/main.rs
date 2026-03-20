// Shadow Blade - Ninja Platformer for Miyoo Mini Plus
// Rust/Macroquad port — 640x480, 60fps fixed timestep

use macroquad::prelude::*;

// ── Constants ──────────────────────────────────────────────────────────────

const SCREEN_W: f32 = 640.0;
const SCREEN_H: f32 = 480.0;
const TILE: f32 = 16.0;
const MAP_ROWS: usize = 30; // 480 / 16
const MAP_COLS: usize = 260;
const GRAVITY: f32 = 0.5;
const MAX_FALL: f32 = 8.0;
const PLAYER_SPEED: f32 = 4.0;
const JUMP_VEL: f32 = -10.0;
const JUMP_HOLD_MAX: i32 = 12;
const DASH_SPEED: f32 = 16.0;
const DASH_FRAMES: i32 = 6;
const SLIDE_FRAMES: i32 = 10;
const SHURIKEN_SPEED: f32 = 8.0;
const SHURIKEN_COOLDOWN: i32 = 15;
const ARROW_SPEED: f32 = 5.0;
const MAX_PARTICLES: usize = 200;
const MAX_ENEMIES: usize = 20;
const MAX_PROJECTILES: usize = 30;
const COMBO_WINDOW: i32 = 20;
const ATTACK_DURATION: i32 = 12;
const INVULN_FRAMES: i32 = 40;
const CAMERA_MARGIN: f32 = 64.0;

// Tile types
const TILE_EMPTY: u8 = 0;
const TILE_GROUND: u8 = 1;
const TILE_WALL: u8 = 2;
const TILE_PLATFORM: u8 = 3;
const TILE_SPIKE: u8 = 4;

// ── Palette ────────────────────────────────────────────────────────────────

fn palette(ch: char) -> Option<Color> {
    match ch {
        'K' => Some(Color::new(0.067, 0.067, 0.067, 1.0)),
        'G' => Some(Color::new(0.333, 0.333, 0.333, 1.0)),
        'W' => Some(WHITE),
        'R' => Some(RED),
        'B' => Some(Color::new(0.0, 0.4, 1.0, 1.0)),
        'C' => Some(SKYBLUE),
        'Y' => Some(Color::new(1.0, 0.8, 0.0, 1.0)),
        'P' => Some(Color::new(0.6, 0.2, 1.0, 1.0)),
        'O' => Some(ORANGE),
        'N' => Some(Color::new(0.545, 0.271, 0.075, 1.0)),
        'D' => Some(Color::new(0.2, 0.2, 0.2, 1.0)),
        'S' => Some(Color::new(0.75, 0.75, 0.75, 1.0)),
        'M' => Some(MAGENTA),
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

// ── Sprite Data ────────────────────────────────────────────────────────────

fn ninja_idle_sprite() -> Vec<&'static str> {
    vec![
        "......KKKK......",
        ".....KDDDDK.....",
        ".....KDWWDK.....",
        ".....KKRRKK.....",
        "......KKKK......",
        ".....KDDDDK.....",
        "....KDDDDDDDK...",
        "...KDDCDDDDDK..",
        "...KDDDDDDDDDK.",
        "....KDDDDDDDK..",
        ".....KDDDDDK...",
        ".....KDDDDDK...",
        "......KDDDK....",
        ".....KK..KK....",
        "....KDK..KDK...",
        "....KKK..KKK...",
    ]
}

fn ninja_run_sprite() -> Vec<&'static str> {
    vec![
        "......KKKK......",
        ".....KDDDDK.....",
        ".....KDWWDK.....",
        ".....KKRRKK.....",
        "......KKKK......",
        "....KDDDDDK.....",
        "...KDDDDDDDDK..",
        "..KDDCDDDDDDDK.",
        "...KDDDDDDDDDK.",
        "....KDDDDDDDK..",
        "....KKDDDDKK...",
        "...KDK....KDK..",
        "..KDK......KDK.",
        "..KK........KK.",
        "................",
        "................",
    ]
}

fn ninja_jump_sprite() -> Vec<&'static str> {
    vec![
        "......KKKK......",
        ".....KDDDDK.....",
        ".....KDWWDK.....",
        ".....KKRRKK.....",
        "......KKKK......",
        "....KKDDDKKK....",
        "...KDDDDDDDDDK.",
        "..KDDDCDDDDDDDK",
        "...KDDDDDDDDDK.",
        "....KDDDDDDDK..",
        ".....KDDDDDK...",
        "....KDK..KDK...",
        "...KDK....KDK..",
        "...KK......KK..",
        "................",
        "................",
    ]
}

fn ninja_attack_sprite() -> Vec<&'static str> {
    vec![
        "......KKKK......",
        ".....KDDDDKCCCC.",
        ".....KDWWDKCCCC.",
        ".....KKRRKKCCCC.",
        "......KKKKCCCC..",
        "....KDDDDDDCCCC.",
        "...KDDDDDDDDCCCC",
        "..KDDCDDDDDDCCCC",
        "...KDDDDDDDDDK.",
        "....KDDDDDDDK..",
        ".....KDDDDDK...",
        ".....KK..KK....",
        "....KDK..KDK...",
        "....KKK..KKK...",
        "................",
        "................",
    ]
}

fn guard_sprite() -> Vec<&'static str> {
    vec![
        "....KKKK....",
        "...KGGGSK...",
        "...KGSSGK...",
        "...KKGGKK...",
        "....KKKK....",
        "...KGGSSGK..",
        "..KGGSSGGGK.",
        "..KGGGGGGKK.",
        "...KGGGGGK..",
        "....KGGGK...",
        "...KGK.KGK..",
        "..KGK...KGK.",
        "..KK.....KK.",
        "............",
    ]
}

fn archer_sprite() -> Vec<&'static str> {
    vec![
        "....KKKK....",
        "...KPDDPK...",
        "...KPWWPK...",
        "...KKPPKK...",
        "....KKKK....",
        "...KPDDPK...",
        "..KPDDDDPK..",
        "..KPDDDDPKN.",
        "..KPDDDDPKNK",
        "...KPDDPK.NK",
        "....KPPK..NK",
        "...KPK.KPK..",
        "..KPK...KPK.",
        "..KK.....KK.",
    ]
}

fn heart_sprite() -> Vec<&'static str> {
    vec![
        "........",
        ".RR.RR..",
        "RRRRRR..",
        "RRRRRR..",
        ".RRRR...",
        "..RR....",
        "........",
        "........",
    ]
}

fn scroll_sprite() -> Vec<&'static str> {
    vec![
        "..YYYY..",
        ".YNNNY..",
        ".YNNNY..",
        ".YNNNY..",
        ".YNNNY..",
        ".YNNNY..",
        "..YYYY..",
        "........",
    ]
}

fn ammo_sprite() -> Vec<&'static str> {
    vec![
        "...SS...",
        "..SSSS..",
        ".SSSSSS.",
        "..SSSS..",
        "...SS...",
        "........",
        "........",
        "........",
    ]
}

fn shuriken_proj_sprite() -> Vec<&'static str> {
    vec![
        ".SS.",
        "SSSS",
        "SSSS",
        ".SS.",
    ]
}

fn arrow_proj_sprite() -> Vec<&'static str> {
    vec![
        "NNNN",
        ".....",
    ]
}

// ── Game State ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum GamePhase {
    Title,
    Playing,
    Paused,
    Death,
    GameOver,
    Victory,
    Story,
}

#[derive(Clone, Copy, PartialEq)]
enum EnemyKind {
    Guard,
    Archer,
}

#[derive(Clone, Copy, PartialEq)]
enum PickupKind {
    Heart,
    Scroll,
    Ammo,
}

#[derive(Clone, Copy, PartialEq)]
enum ProjOwner {
    Player,
    Enemy,
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
    facing: f32, // 1.0 right, -1.0 left
    on_ground: bool,
    on_wall: i32, // 0=none, 1=right wall, -1=left wall
    jump_held: i32,
    can_dash: bool,
    dashing: i32,
    sliding: i32,
    attacking: i32,
    combo: i32,
    combo_timer: i32,
    shuriken: i32,
    shuriken_cd: i32,
    invuln: i32,
    dead: bool,
    anim_timer: i32,
    score: i32,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x, y,
            vx: 0.0, vy: 0.0,
            w: 16.0, h: 28.0,
            hp: 5, max_hp: 5,
            facing: 1.0,
            on_ground: false,
            on_wall: 0,
            jump_held: 0,
            can_dash: true,
            dashing: 0,
            sliding: 0,
            attacking: 0,
            combo: 0,
            combo_timer: 0,
            shuriken: 10,
            shuriken_cd: 0,
            invuln: 0,
            dead: false,
            anim_timer: 0,
            score: 0,
        }
    }

    fn rect(&self) -> Rect {
        let h = if self.sliding > 0 { self.h * 0.5 } else { self.h };
        let y = if self.sliding > 0 { self.y + self.h * 0.5 } else { self.y };
        Rect::new(self.x, y, self.w, h)
    }

    fn attack_rect(&self) -> Rect {
        let range = if self.combo == 3 { 28.0 } else { 22.0 };
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
    damage: i32,
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
    gravity: bool, // if true, apply heavier gravity (for blood/impact particles)
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

struct WallSpark {
    active: bool,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: i32,
    max_life: i32,
}

struct GrassTuft {
    x: f32,
    y: f32,
    h: f32,
    sway_offset: f32,
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

struct Camera {
    x: f32,
    y: f32,
}

struct EnvSign {
    tx: usize, // tile X position
    text: &'static str,
}

struct Star {
    x: f32,
    y: f32,
    brightness: f32,
    size: f32,
}

#[derive(Clone, Copy, PartialEq)]
enum StoryCallback {
    None,
    StartLevelIntro,   // After backstory, show level intro
    BeginLevel,        // After level intro, start playing
    ShowPostLevel,     // Not used directly; we call advance_level
    AdvanceAfterPost,  // After post-level story, advance to next level or victory
}

// ── Story Text Data ──────────────────────────────────────────────────────

const LEVEL_NAMES: [&str; 3] = [
    "TRIAL OF SPEED",
    "TRIAL OF COURAGE",
    "TRIAL OF TRUTH",
];

const LEVEL_SUBTITLES: [&str; 3] = [
    "Bamboo Forest",
    "Castle Rooftops",
    "Demon Shrine",
];

fn story_pre(index: usize) -> Vec<String> {
    match index {
        0 => vec![
            "You are Kaede, a ninja of the Shadow Lotus clan.".into(),
            "".into(),
            "Accused of assassinating the Shogun, you have been".into(),
            "stripped of your rank and marked for death.".into(),
            "".into(),
            "Your master, Sensei Takeshi, cast you out.".into(),
            "But you know the truth -- you were framed.".into(),
            "".into(),
            "The real assassin used Shadow Lotus techniques.".into(),
            "The traitor is within your own clan.".into(),
            "".into(),
            "To clear your name, you must complete the".into(),
            "Three Trials of the Crimson Oath --".into(),
            "ancient tests that reveal the truth".into(),
            "to anyone who survives them.".into(),
        ],
        1 => vec![
            "--- THE FIRST TRIAL: SPEED ---".into(),
            "".into(),
            "The first trial tests your swiftness.".into(),
            "Race through the Bamboo Forest before".into(),
            "dawn breaks.".into(),
            "".into(),
            "The forest itself will try to stop you --".into(),
            "its guardians see all intruders as threats.".into(),
        ],
        2 => vec![
            "--- THE SECOND TRIAL: COURAGE ---".into(),
            "".into(),
            "The second trial tests your courage.".into(),
            "Infiltrate Castle Kuroda, where the".into(),
            "Shogun was killed.".into(),
            "".into(),
            "The castle is now haunted by his guardsmen".into(),
            "who refuse to leave their posts,".into(),
            "even in death.".into(),
        ],
        3 => vec![
            "--- THE FINAL TRIAL: TRUTH ---".into(),
            "".into(),
            "The final trial reveals the truth.".into(),
            "Sensei Takeshi awaits at the Crimson Shrine.".into(),
            "".into(),
            "He knew you would come.".into(),
            "He has always known.".into(),
        ],
        _ => vec![],
    }
}

fn story_post(index: usize) -> Vec<String> {
    match index {
        0 => vec![
            "You emerge from the forest as the first".into(),
            "light breaks. In the clearing ahead,".into(),
            "you find a scroll. It reads:".into(),
            "".into(),
            "'The Shogun's killer moved like wind".into(),
            " through still air. Only two in the".into(),
            " Shadow Lotus possess such speed --".into(),
            " you, and your master.'".into(),
        ],
        1 => vec![
            "Deep in the castle archives, you find".into(),
            "the assassination report.".into(),
            "".into(),
            "The killing blow came from above --".into(),
            "from the rafters. But the guards found".into(),
            "no entry point.".into(),
            "".into(),
            "Whoever did this had a key.".into(),
            "".into(),
            "Only two people had keys to the Shogun's".into(),
            "private chambers: the head of security...".into(),
            "and Sensei Takeshi.".into(),
        ],
        2 => vec![
            "TAKESHI:".into(),
            "'You were always my finest student, Kaede.".into(),
            " Too fine. The Shogun ordered the Shadow".into(),
            " Lotus destroyed. I killed him to save our".into(),
            " clan. I framed you because... you would".into(),
            " have tried to stop me.'".into(),
            "".into(),
            "Takeshi kneels, offering his sword.".into(),
            "'The clan is safe. My crime is mine alone.".into(),
            " Take my blade and lead them, Kaede.'".into(),
            "".into(),
            "You take the sword.".into(),
            "The Crimson Oath is complete.".into(),
            "".into(),
            "The truth is revealed -- not the truth you".into(),
            "wanted, but the truth you needed.".into(),
            "Your master was a murderer, and a hero.".into(),
            "Now the burden passes to you.".into(),
        ],
        _ => vec![],
    }
}

fn env_signs_for_level(level: usize) -> Vec<EnvSign> {
    match level {
        0 => vec![
            EnvSign { tx: 20, text: "The forest remembers..." },
            EnvSign { tx: 45, text: "Speed is the shadow's first weapon" },
            EnvSign { tx: 65, text: "Run, Kaede. Do not look back." },
        ],
        1 => vec![
            EnvSign { tx: 15, text: "The Shogun's blood still stains these stones" },
            EnvSign { tx: 50, text: "Who benefits from the Shogun's death?" },
            EnvSign { tx: 75, text: "The guards see only enemies now" },
        ],
        2 => vec![
            EnvSign { tx: 20, text: "Your master's blade hangs here..." },
            EnvSign { tx: 50, text: "Truth cuts deeper than any sword" },
            EnvSign { tx: 70, text: "TAKESHI: 'You were always my finest student...'" },
        ],
        _ => vec![],
    }
}

struct Game {
    phase: GamePhase,
    player: Player,
    enemies: Vec<Enemy>,
    projectiles: Vec<Projectile>,
    particles: Vec<Particle>,
    pickups: Vec<Pickup>,
    camera: Camera,
    map: Vec<Vec<u8>>,
    shake_timer: i32,
    shake_intensity: f32,
    frame: i64,
    death_timer: i32,
    stars: Vec<Star>,
    level_name: String,
    blink_timer: i32,
    // Multi-level system
    current_level: usize,
    level_cols: usize, // MAP_COLS varies per level
    // Story system
    story_lines: Vec<String>,
    story_char_index: usize,
    story_displayed: String,
    story_type_timer: i32,
    story_skip_ready: bool,
    story_callback: StoryCallback,
    // Environmental signs
    env_signs: Vec<EnvSign>,
    // Visual polish
    dash_ghosts: Vec<DashGhost>,
    wall_sparks: Vec<WallSpark>,
    hit_stop: i32,
    grass_tufts: Vec<GrassTuft>,
    // Textures
    tex_ninja_idle: Texture2D,
    tex_ninja_run: Texture2D,
    tex_ninja_jump: Texture2D,
    tex_ninja_attack: Texture2D,
    tex_guard: Texture2D,
    tex_archer: Texture2D,
    tex_heart: Texture2D,
    tex_scroll: Texture2D,
    tex_ammo: Texture2D,
    tex_shuriken: Texture2D,
    tex_arrow: Texture2D,
}

// ── Level Generation ───────────────────────────────────────────────────────

fn generate_level(lvl: usize) -> (Vec<Vec<u8>>, Vec<Enemy>, Vec<Pickup>, usize) {
    let w = match lvl {
        0 => 90,
        1 => 100,
        _ => 95,
    };
    let h = MAP_ROWS;
    let mut map = vec![vec![TILE_EMPTY; w]; h];
    let mut enemies = Vec::new();
    let mut pickups = Vec::new();

    // Helper closures
    let add_ground = |map: &mut Vec<Vec<u8>>, gap_list: &[(usize, usize)]| {
        for x in 0..w {
            map[h - 1][x] = TILE_GROUND;
            map[h - 2][x] = TILE_GROUND;
            let mut is_gap = false;
            for &(gs, ge) in gap_list {
                if x > gs && x < ge {
                    is_gap = true;
                    break;
                }
            }
            if is_gap {
                map[h - 1][x] = TILE_SPIKE;
                map[h - 2][x] = TILE_EMPTY;
            }
        }
    };

    let add_wall = |map: &mut Vec<Vec<u8>>, x: usize, y_start: usize, y_end: usize| {
        for y in y_start..=y_end {
            if y < h && x < w {
                map[y][x] = TILE_WALL;
            }
        }
    };

    let add_platform = |map: &mut Vec<Vec<u8>>, x: usize, y: usize, len: usize, tile_type: u8| {
        for i in 0..len {
            if x + i < w && y < h {
                map[y][x + i] = tile_type;
            }
        }
    };

    fn spawn_guard(enemies: &mut Vec<Enemy>, map: &[Vec<u8>], gx: f32, h: usize) {
        let tile_x = (gx / TILE) as usize;
        let mut spawn_y = 25.0 * TILE;
        for ty in 20..h {
            let t = if tile_x < map[0].len() && ty < map.len() { map[ty][tile_x] } else { 0 };
            if t == TILE_GROUND || t == TILE_WALL {
                spawn_y = ty as f32 * TILE - 28.0;
                break;
            }
        }
        let patrol_range = 80.0;
        enemies.push(Enemy {
            active: true,
            kind: EnemyKind::Guard,
            x: gx,
            y: spawn_y,
            vx: 1.5,
            vy: 0.0,
            w: 16.0,
            h: 28.0,
            hp: 2,
            facing: 1.0,
            patrol_left: (gx - patrol_range).max(0.0),
            patrol_right: gx + patrol_range,
            shoot_timer: 0,
            hurt_timer: 0,
            score_val: 100,
        });
    }

    fn spawn_archer(enemies: &mut Vec<Enemy>, map: &[Vec<u8>], ax: f32, ay: f32, h: usize) {
        let tx = (ax / TILE) as usize;
        let mut spawn_y = ay;
        let start_ty = (ay / TILE) as usize;
        for ty in start_ty..h {
            let t = if tx < map[0].len() && ty < map.len() { map[ty][tx] } else { 0 };
            if t == TILE_GROUND || t == TILE_WALL || t == TILE_PLATFORM {
                spawn_y = ty as f32 * TILE - 28.0;
                break;
            }
        }
        enemies.push(Enemy {
            active: true,
            kind: EnemyKind::Archer,
            x: ax,
            y: spawn_y,
            vx: 0.0,
            vy: 0.0,
            w: 16.0,
            h: 28.0,
            hp: 1,
            facing: -1.0,
            patrol_left: ax,
            patrol_right: ax,
            shoot_timer: 60,
            hurt_timer: 0,
            score_val: 150,
        });
    }

    match lvl {
        0 => {
            // LEVEL 1: TRIAL OF SPEED - Bamboo Forest
            add_ground(&mut map, &[(30, 34), (70, 74)]);

            // Section 1: Tutorial area
            add_platform(&mut map, 8, 24, 5, TILE_PLATFORM);
            add_platform(&mut map, 15, 21, 4, TILE_PLATFORM);
            add_platform(&mut map, 21, 18, 5, TILE_PLATFORM);
            add_platform(&mut map, 10, 17, 3, TILE_PLATFORM);

            // Wall jump section
            add_wall(&mut map, 28, 15, 27);
            add_wall(&mut map, 32, 12, 27);
            add_platform(&mut map, 29, 14, 3, TILE_PLATFORM);
            add_platform(&mut map, 33, 11, 4, TILE_PLATFORM);

            // Section 2: Forest platforms
            add_platform(&mut map, 42, 24, 6, TILE_PLATFORM);
            add_platform(&mut map, 50, 22, 4, TILE_PLATFORM);
            add_platform(&mut map, 46, 19, 5, TILE_PLATFORM);
            add_platform(&mut map, 55, 20, 3, TILE_PLATFORM);
            add_platform(&mut map, 60, 17, 6, TILE_PLATFORM);
            add_platform(&mut map, 52, 15, 3, TILE_PLATFORM);
            add_platform(&mut map, 58, 13, 4, TILE_PLATFORM);

            add_wall(&mut map, 68, 10, 27);
            add_wall(&mut map, 72, 8, 27);
            add_platform(&mut map, 69, 14, 3, TILE_PLATFORM);
            add_platform(&mut map, 64, 22, 4, TILE_PLATFORM);
            add_platform(&mut map, 73, 10, 5, TILE_PLATFORM);

            // End platform
            add_platform(&mut map, 80, 24, 10, TILE_GROUND);
            add_platform(&mut map, 80, 23, 10, TILE_GROUND);
            for y in 0..h - 2 {
                if w >= 2 {
                    map[y][w - 1] = TILE_WALL;
                    map[y][w - 2] = TILE_WALL;
                }
            }

            // Enemies
            spawn_guard(&mut enemies, &map, 200.0, h);
            spawn_guard(&mut enemies, &map, 400.0, h);
            spawn_guard(&mut enemies, &map, 600.0, h);
            spawn_guard(&mut enemies, &map, 900.0, h);
            spawn_archer(&mut enemies, &map, 15.0 * TILE, 20.0 * TILE, h);
            spawn_archer(&mut enemies, &map, 60.0 * TILE, 16.0 * TILE, h);

            // Pickups
            pickups.push(Pickup { active: true, kind: PickupKind::Scroll, x: 12.0 * TILE, y: 22.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Scroll, x: 22.0 * TILE, y: 16.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Scroll, x: 50.0 * TILE, y: 20.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Scroll, x: 73.0 * TILE, y: 8.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Heart, x: 35.0 * TILE, y: 24.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Ammo, x: 55.0 * TILE, y: 18.0 * TILE, w: 8.0, h: 8.0 });
        }
        1 => {
            // LEVEL 2: TRIAL OF COURAGE - Castle Rooftops
            add_ground(&mut map, &[(30, 34), (55, 59), (80, 84)]);

            add_platform(&mut map, 8, 24, 8, TILE_PLATFORM);
            add_platform(&mut map, 18, 22, 6, TILE_PLATFORM);
            add_platform(&mut map, 12, 19, 5, TILE_PLATFORM);
            add_platform(&mut map, 26, 20, 4, TILE_PLATFORM);
            add_platform(&mut map, 21, 16, 6, TILE_PLATFORM);
            add_platform(&mut map, 32, 18, 5, TILE_PLATFORM);
            add_platform(&mut map, 28, 14, 4, TILE_PLATFORM);
            add_platform(&mut map, 38, 22, 5, TILE_PLATFORM);
            add_platform(&mut map, 36, 16, 3, TILE_PLATFORM);

            add_wall(&mut map, 44, 10, 27);
            add_wall(&mut map, 48, 8, 27);
            add_platform(&mut map, 45, 12, 3, TILE_PLATFORM);
            add_platform(&mut map, 49, 9, 4, TILE_PLATFORM);

            // Vertical challenge section
            add_platform(&mut map, 54, 25, 4, TILE_PLATFORM);
            add_platform(&mut map, 60, 23, 3, TILE_PLATFORM);
            add_platform(&mut map, 56, 20, 4, TILE_PLATFORM);
            add_platform(&mut map, 64, 18, 3, TILE_PLATFORM);
            add_platform(&mut map, 68, 21, 4, TILE_PLATFORM);
            add_platform(&mut map, 62, 15, 4, TILE_PLATFORM);
            add_platform(&mut map, 72, 16, 5, TILE_PLATFORM);
            add_platform(&mut map, 76, 13, 4, TILE_PLATFORM);
            add_platform(&mut map, 70, 11, 3, TILE_PLATFORM);

            add_wall(&mut map, 82, 8, 27);
            add_wall(&mut map, 86, 6, 27);
            add_platform(&mut map, 83, 10, 3, TILE_PLATFORM);
            add_platform(&mut map, 87, 7, 4, TILE_PLATFORM);

            // End platform
            add_platform(&mut map, 90, 24, 10, TILE_GROUND);
            add_platform(&mut map, 90, 23, 10, TILE_GROUND);
            for y in 0..h - 2 {
                if w >= 2 {
                    map[y][w - 1] = TILE_WALL;
                    map[y][w - 2] = TILE_WALL;
                }
            }

            // Enemies - higher density
            spawn_guard(&mut enemies, &map, 150.0, h);
            spawn_guard(&mut enemies, &map, 300.0, h);
            spawn_guard(&mut enemies, &map, 450.0, h);
            spawn_guard(&mut enemies, &map, 600.0, h);
            spawn_guard(&mut enemies, &map, 800.0, h);
            spawn_guard(&mut enemies, &map, 950.0, h);
            spawn_guard(&mut enemies, &map, 1100.0, h);
            spawn_guard(&mut enemies, &map, 1300.0, h);
            spawn_archer(&mut enemies, &map, 21.0 * TILE, 15.0 * TILE, h);
            spawn_archer(&mut enemies, &map, 36.0 * TILE, 15.0 * TILE, h);
            spawn_archer(&mut enemies, &map, 62.0 * TILE, 14.0 * TILE, h);
            spawn_archer(&mut enemies, &map, 76.0 * TILE, 12.0 * TILE, h);

            // Pickups
            pickups.push(Pickup { active: true, kind: PickupKind::Scroll, x: 10.0 * TILE, y: 22.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Scroll, x: 49.0 * TILE, y: 8.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Scroll, x: 70.0 * TILE, y: 10.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Heart, x: 30.0 * TILE, y: 18.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Heart, x: 65.0 * TILE, y: 20.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Ammo, x: 45.0 * TILE, y: 10.0 * TILE, w: 8.0, h: 8.0 });
        }
        _ => {
            // LEVEL 3: TRIAL OF TRUTH - Demon Shrine
            add_ground(&mut map, &[(25, 29), (50, 54), (75, 79)]);

            add_platform(&mut map, 8, 24, 5, TILE_PLATFORM);
            add_platform(&mut map, 15, 22, 4, TILE_PLATFORM);
            add_platform(&mut map, 11, 18, 5, TILE_PLATFORM);
            add_platform(&mut map, 21, 20, 6, TILE_PLATFORM);
            add_platform(&mut map, 28, 17, 4, TILE_PLATFORM);
            add_platform(&mut map, 24, 14, 3, TILE_PLATFORM);

            add_wall(&mut map, 34, 8, 27);
            add_wall(&mut map, 38, 6, 27);
            add_platform(&mut map, 35, 10, 3, TILE_PLATFORM);
            add_platform(&mut map, 39, 8, 4, TILE_PLATFORM);

            // Inner shrine
            add_platform(&mut map, 42, 24, 6, TILE_PLATFORM);
            add_platform(&mut map, 44, 18, 4, TILE_PLATFORM);
            add_platform(&mut map, 50, 20, 3, TILE_PLATFORM);
            add_platform(&mut map, 55, 22, 5, TILE_PLATFORM);
            add_platform(&mut map, 53, 16, 5, TILE_PLATFORM);
            add_platform(&mut map, 60, 18, 4, TILE_PLATFORM);
            add_platform(&mut map, 58, 13, 3, TILE_PLATFORM);

            add_wall(&mut map, 66, 8, 27);
            add_wall(&mut map, 70, 6, 27);
            add_platform(&mut map, 67, 10, 3, TILE_PLATFORM);
            add_platform(&mut map, 71, 7, 4, TILE_PLATFORM);

            add_platform(&mut map, 74, 22, 4, TILE_PLATFORM);
            add_platform(&mut map, 76, 16, 4, TILE_PLATFORM);
            add_platform(&mut map, 80, 20, 3, TILE_PLATFORM);

            // End platform - Takeshi's arena
            add_platform(&mut map, 84, 24, 11, TILE_GROUND);
            add_platform(&mut map, 84, 23, 11, TILE_GROUND);
            for y in 0..h - 2 {
                if w >= 2 {
                    map[y][w - 1] = TILE_WALL;
                    map[y][w - 2] = TILE_WALL;
                }
            }

            // Enemies - tough
            spawn_guard(&mut enemies, &map, 150.0, h);
            spawn_guard(&mut enemies, &map, 350.0, h);
            spawn_guard(&mut enemies, &map, 500.0, h);
            spawn_guard(&mut enemies, &map, 700.0, h);
            spawn_guard(&mut enemies, &map, 900.0, h);
            spawn_guard(&mut enemies, &map, 1050.0, h);
            spawn_guard(&mut enemies, &map, 1200.0, h);
            spawn_archer(&mut enemies, &map, 15.0 * TILE, 16.0 * TILE, h);
            spawn_archer(&mut enemies, &map, 28.0 * TILE, 16.0 * TILE, h);
            spawn_archer(&mut enemies, &map, 53.0 * TILE, 15.0 * TILE, h);
            spawn_archer(&mut enemies, &map, 76.0 * TILE, 15.0 * TILE, h);

            // Pickups
            pickups.push(Pickup { active: true, kind: PickupKind::Scroll, x: 20.0 * TILE, y: 18.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Scroll, x: 39.0 * TILE, y: 6.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Scroll, x: 67.0 * TILE, y: 8.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Heart, x: 48.0 * TILE, y: 20.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Heart, x: 78.0 * TILE, y: 20.0 * TILE, w: 8.0, h: 8.0 });
            pickups.push(Pickup { active: true, kind: PickupKind::Ammo, x: 58.0 * TILE, y: 12.0 * TILE, w: 8.0, h: 8.0 });
        }
    }

    (map, enemies, pickups, w)
}

// ── Collision Helpers ──────────────────────────────────────────────────────

fn tile_at_sized(map: &[Vec<u8>], x: f32, y: f32, cols: usize) -> u8 {
    let col = (x / TILE) as isize;
    let row = (y / TILE) as isize;
    if col < 0 || row < 0 || col >= cols as isize || row >= MAP_ROWS as isize {
        return TILE_EMPTY;
    }
    map[row as usize][col as usize]
}

fn tile_at(map: &[Vec<u8>], x: f32, y: f32) -> u8 {
    let cols = if !map.is_empty() { map[0].len() } else { MAP_COLS };
    tile_at_sized(map, x, y, cols)
}

fn is_solid(map: &[Vec<u8>], x: f32, y: f32) -> bool {
    let t = tile_at(map, x, y);
    t == TILE_GROUND || t == TILE_WALL
}

fn is_solid_or_platform_top(map: &[Vec<u8>], x: f32, y: f32, vy: f32, foot_y: f32) -> bool {
    let t = tile_at(map, x, y);
    if t == TILE_GROUND || t == TILE_WALL {
        return true;
    }
    // Platform: only solid when falling onto top
    if t == TILE_PLATFORM && vy >= 0.0 {
        let tile_top = (y / TILE).floor() * TILE;
        if foot_y <= tile_top + 2.0 {
            return true;
        }
    }
    false
}

fn rects_overlap(a: &Rect, b: &Rect) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

// ── Particle Spawners ──────────────────────────────────────────────────────

fn spawn_particles(particles: &mut Vec<Particle>, x: f32, y: f32, count: usize, color: Color, speed: f32, life: i32) {
    for _ in 0..count {
        if particles.len() >= MAX_PARTICLES {
            // Reuse oldest
            if let Some(p) = particles.iter_mut().find(|p| !p.active) {
                p.active = true;
                p.x = x;
                p.y = y;
                p.vx = rand::gen_range(-speed, speed);
                p.vy = rand::gen_range(-speed, speed);
                p.life = life;
                p.max_life = life;
                p.color = color;
                p.size = rand::gen_range(1.0, 3.0);
                p.gravity = false;
            }
        } else {
            particles.push(Particle {
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
            });
        }
    }
}

fn spawn_blood_particles(particles: &mut Vec<Particle>, x: f32, y: f32) {
    let count = rand::gen_range(5, 9) as usize;
    for _ in 0..count {
        let p = Particle {
            active: true,
            x,
            y,
            vx: rand::gen_range(-3.0, 3.0),
            vy: rand::gen_range(-4.0, -1.0),
            life: 25,
            max_life: 25,
            color: Color::new(
                rand::gen_range(0.7, 1.0),
                rand::gen_range(0.0, 0.15),
                rand::gen_range(0.0, 0.1),
                1.0,
            ),
            size: rand::gen_range(1.5, 3.5),
            gravity: true,
        };
        if particles.len() < MAX_PARTICLES {
            particles.push(p);
        } else if let Some(slot) = particles.iter_mut().find(|pp| !pp.active) {
            *slot = p;
        }
    }
}

fn spawn_slash_particles(particles: &mut Vec<Particle>, x: f32, y: f32, facing: f32) {
    for i in 0..8 {
        let angle = (i as f32 / 8.0) * std::f32::consts::PI - std::f32::consts::FRAC_PI_2;
        let spd = rand::gen_range(2.0, 5.0);
        let color = if rand::gen_range(0.0, 1.0) > 0.5 { WHITE } else { SKYBLUE };
        let p = Particle {
            active: true,
            x: x + facing * 10.0,
            y,
            vx: angle.cos() * spd * facing,
            vy: angle.sin() * spd,
            life: 10,
            max_life: 10,
            color,
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

fn spawn_dash_trail(particles: &mut Vec<Particle>, x: f32, y: f32, h: f32) {
    for _ in 0..3 {
        let p = Particle {
            active: true,
            x,
            y: y + rand::gen_range(0.0, h),
            vx: rand::gen_range(-0.5, 0.5),
            vy: rand::gen_range(-0.5, 0.5),
            life: 12,
            max_life: 12,
            color: Color::new(0.0, 0.8, 1.0, 0.7),
            size: rand::gen_range(3.0, 6.0),
            gravity: false,
        };
        if particles.len() < MAX_PARTICLES {
            particles.push(p);
        } else if let Some(slot) = particles.iter_mut().find(|pp| !pp.active) {
            *slot = p;
        }
    }
}

// ── Game Implementation ────────────────────────────────────────────────────

impl Game {
    fn new() -> Self {
        let current_level = 0;
        let (map, enemies, pickups, level_cols) = generate_level(current_level);
        let mut stars = Vec::new();
        for _ in 0..80 {
            stars.push(Star {
                x: rand::gen_range(0.0, level_cols as f32 * TILE),
                y: rand::gen_range(0.0, SCREEN_H * 0.6),
                brightness: rand::gen_range(0.3, 1.0),
                size: rand::gen_range(1.0, 2.5),
            });
        }

        let grass_tufts = Self::generate_grass(&map, level_cols);

        Self {
            phase: GamePhase::Title,
            player: Player::new(3.0 * TILE, 24.0 * TILE),
            enemies,
            projectiles: Vec::with_capacity(MAX_PROJECTILES),
            particles: Vec::with_capacity(MAX_PARTICLES),
            pickups,
            camera: Camera { x: 0.0, y: 0.0 },
            map,
            shake_timer: 0,
            shake_intensity: 0.0,
            frame: 0,
            death_timer: 0,
            stars,
            level_name: LEVEL_SUBTITLES[0].to_string(),
            blink_timer: 0,
            current_level,
            level_cols,
            story_lines: Vec::new(),
            story_char_index: 0,
            story_displayed: String::new(),
            story_type_timer: 0,
            story_skip_ready: false,
            story_callback: StoryCallback::None,
            env_signs: env_signs_for_level(0),
            dash_ghosts: Vec::with_capacity(20),
            wall_sparks: Vec::with_capacity(30),
            hit_stop: 0,
            grass_tufts,
            tex_ninja_idle: sprite_to_texture(&ninja_idle_sprite(), 16, 16),
            tex_ninja_run: sprite_to_texture(&ninja_run_sprite(), 16, 16),
            tex_ninja_jump: sprite_to_texture(&ninja_jump_sprite(), 16, 16),
            tex_ninja_attack: sprite_to_texture(&ninja_attack_sprite(), 16, 16),
            tex_guard: sprite_to_texture(&guard_sprite(), 12, 14),
            tex_archer: sprite_to_texture(&archer_sprite(), 14, 14),
            tex_heart: sprite_to_texture(&heart_sprite(), 8, 8),
            tex_scroll: sprite_to_texture(&scroll_sprite(), 8, 8),
            tex_ammo: sprite_to_texture(&ammo_sprite(), 8, 8),
            tex_shuriken: sprite_to_texture(&shuriken_proj_sprite(), 4, 4),
            tex_arrow: sprite_to_texture(&arrow_proj_sprite(), 5, 2),
        }
    }

    fn generate_grass(map: &[Vec<u8>], level_cols: usize) -> Vec<GrassTuft> {
        let mut grass_tufts = Vec::new();
        for col in 0..level_cols {
            if col < map[0].len() && map[27][col] == TILE_GROUND
                && (col == 0 || map[26][col] == TILE_EMPTY || map[26][col] == TILE_SPIKE)
            {
                for _ in 0..rand::gen_range(0, 3) {
                    grass_tufts.push(GrassTuft {
                        x: col as f32 * TILE + rand::gen_range(0.0, TILE),
                        y: 27.0 * TILE,
                        h: rand::gen_range(3.0, 7.0),
                        sway_offset: rand::gen_range(0.0, std::f32::consts::TAU),
                    });
                }
            }
            for row in 20..27 {
                if row < MAP_ROWS && col < map[0].len() && map[row][col] == TILE_GROUND
                    && (row == 0 || map[row - 1][col] == TILE_EMPTY)
                {
                    if rand::gen_range(0.0, 1.0) < 0.4 {
                        grass_tufts.push(GrassTuft {
                            x: col as f32 * TILE + rand::gen_range(0.0, TILE),
                            y: row as f32 * TILE,
                            h: rand::gen_range(3.0, 6.0),
                            sway_offset: rand::gen_range(0.0, std::f32::consts::TAU),
                        });
                    }
                }
            }
        }
        grass_tufts
    }

    fn reset_game(&mut self) {
        self.current_level = 0;
        self.build_level(0);
        self.player.score = 0;
    }

    fn build_level(&mut self, lvl: usize) {
        let (map, enemies, pickups, level_cols) = generate_level(lvl);
        self.map = map;
        self.enemies = enemies;
        self.pickups = pickups;
        self.level_cols = level_cols;
        self.projectiles.clear();
        self.particles.clear();
        self.dash_ghosts.clear();
        self.wall_sparks.clear();
        self.hit_stop = 0;
        self.player.x = 3.0 * TILE;
        self.player.y = 24.0 * TILE;
        self.player.vx = 0.0;
        self.player.vy = 0.0;
        self.player.dead = false;
        self.player.hp = self.player.max_hp;
        self.player.invuln = 0;
        self.player.shuriken = 10;
        self.camera = Camera { x: 0.0, y: 0.0 };
        self.shake_timer = 0;
        self.death_timer = 0;
        self.frame = 0;
        self.level_name = LEVEL_SUBTITLES[lvl.min(2)].to_string();
        self.env_signs = env_signs_for_level(lvl);
        self.grass_tufts = Self::generate_grass(&self.map, self.level_cols);
    }

    fn show_story(&mut self, lines: Vec<String>, callback: StoryCallback) {
        self.phase = GamePhase::Story;
        self.story_lines = lines;
        self.story_char_index = 0;
        let full_text = self.story_lines.join("\n");
        self.story_displayed = String::new();
        self.story_type_timer = 0;
        self.story_skip_ready = false;
        self.story_callback = callback;
        // store full text in story_displayed capacity
        let _ = full_text; // we'll reconstruct from story_lines each frame
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
        self.phase = GamePhase::Playing;
    }

    fn start_shake(&mut self, intensity: f32, frames: i32) {
        self.shake_intensity = intensity;
        self.shake_timer = frames;
    }

    // ── Update ─────────────────────────────────────────────────────────

    fn update(&mut self) {
        self.frame += 1;
        self.blink_timer = (self.blink_timer + 1) % 60;

        match self.phase {
            GamePhase::Title => self.update_title(),
            GamePhase::Playing => self.update_playing(),
            GamePhase::Paused => self.update_paused(),
            GamePhase::Death => self.update_death(),
            GamePhase::GameOver => self.update_game_over(),
            GamePhase::Victory => self.update_victory(),
            GamePhase::Story => self.update_story(),
        }
    }

    fn update_title(&mut self) {
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::X) {
            self.reset_game();
            // Show backstory first, then level intro, then begin
            let lines = story_pre(0);
            self.show_story(lines, StoryCallback::StartLevelIntro);
        }
    }

    fn update_victory(&mut self) {
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::X) {
            self.phase = GamePhase::Title;
        }
    }

    fn update_story(&mut self) {
        let full_text = self.story_full_text();
        self.story_type_timer += 1;
        if self.story_type_timer >= 2 && self.story_char_index < full_text.len() {
            self.story_char_index += 1;
            // Safely slice at char boundary
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

        // Z/Enter: skip or advance
        if is_key_pressed(KeyCode::Z) || is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::X) {
            if !self.story_skip_ready {
                // Skip to end
                self.story_displayed = full_text;
                self.story_char_index = self.story_displayed.len();
                self.story_skip_ready = true;
            } else {
                // Advance
                let cb = self.story_callback;
                self.story_callback = StoryCallback::None;
                match cb {
                    StoryCallback::StartLevelIntro => {
                        // Show the level-specific intro (index = current_level + 1)
                        let lines = story_pre(self.current_level + 1);
                        self.show_story(lines, StoryCallback::BeginLevel);
                    }
                    StoryCallback::BeginLevel => {
                        self.begin_level();
                    }
                    StoryCallback::AdvanceAfterPost => {
                        self.current_level += 1;
                        if self.current_level > 2 {
                            self.phase = GamePhase::Victory;
                        } else {
                            let lines = story_pre(self.current_level + 1);
                            self.show_story(lines, StoryCallback::BeginLevel);
                        }
                    }
                    _ => {
                        self.phase = GamePhase::Playing;
                    }
                }
            }
        }
    }

    fn update_paused(&mut self) {
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Escape) {
            self.phase = GamePhase::Playing;
        }
    }

    fn update_death(&mut self) {
        self.death_timer += 1;
        // Update particles during death
        for p in self.particles.iter_mut() {
            if p.active {
                p.x += p.vx;
                p.y += p.vy;
                p.vy += 0.1;
                p.life -= 1;
                if p.life <= 0 {
                    p.active = false;
                }
            }
        }
        if self.death_timer > 120 {
            if self.player.hp <= 0 {
                self.phase = GamePhase::GameOver;
            } else {
                // Respawn
                self.player.dead = false;
                self.player.invuln = INVULN_FRAMES * 2;
                self.player.vy = 0.0;
                self.player.vx = 0.0;
                self.phase = GamePhase::Playing;
            }
        }
    }

    fn update_game_over(&mut self) {
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::X) {
            self.reset_game();
            self.phase = GamePhase::Title;
        }
    }

    fn update_playing(&mut self) {
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Escape) {
            self.phase = GamePhase::Paused;
            return;
        }

        // Hit stop: freeze gameplay but keep drawing
        if self.hit_stop > 0 {
            self.hit_stop -= 1;
            return;
        }

        // Update dash ghosts (fade out)
        for ghost in self.dash_ghosts.iter_mut() {
            if ghost.active {
                ghost.life -= 1;
                if ghost.life <= 0 {
                    ghost.active = false;
                }
            }
        }
        self.dash_ghosts.retain(|g| g.active);

        // Update wall sparks
        for spark in self.wall_sparks.iter_mut() {
            if spark.active {
                spark.x += spark.vx;
                spark.y += spark.vy;
                spark.vy += 0.15;
                spark.life -= 1;
                if spark.life <= 0 {
                    spark.active = false;
                }
            }
        }
        self.wall_sparks.retain(|s| s.active);

        self.update_player();
        self.update_enemies();
        self.update_projectiles();
        self.update_particles();
        self.update_pickups();
        self.update_camera();

        if self.shake_timer > 0 {
            self.shake_timer -= 1;
        }

        // Level end check: reach end platform area
        let end_zone = (self.level_cols as f32 - 12.0) * TILE;
        if self.player.x > end_zone {
            self.advance_level();
        }
    }

    fn update_player(&mut self) {
        let p = &mut self.player;
        if p.dead {
            return;
        }

        p.anim_timer += 1;
        if p.invuln > 0 {
            p.invuln -= 1;
        }
        if p.shuriken_cd > 0 {
            p.shuriken_cd -= 1;
        }
        if p.combo_timer > 0 {
            p.combo_timer -= 1;
            if p.combo_timer == 0 {
                p.combo = 0;
            }
        }

        // Movement input
        let mut move_x = 0.0f32;
        if p.dashing <= 0 && p.sliding <= 0 {
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                move_x = -1.0;
                p.facing = -1.0;
            }
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                move_x = 1.0;
                p.facing = 1.0;
            }
        }

        // Dashing
        if p.dashing > 0 {
            p.vx = DASH_SPEED * p.facing;
            p.vy = 0.0;
            p.dashing -= 1;
            // Spawn trail
            spawn_dash_trail(&mut self.particles, p.x, p.y, p.h);
            // Spawn dash afterimage ghost every 2 frames
            if self.frame % 2 == 0 {
                self.dash_ghosts.push(DashGhost {
                    active: true,
                    x: p.x,
                    y: p.y,
                    w: p.w,
                    h: p.h,
                    life: 10,
                    max_life: 10,
                });
            }
            if p.dashing == 0 {
                p.vx = PLAYER_SPEED * p.facing;
            }
        } else if p.sliding > 0 {
            p.vx = PLAYER_SPEED * 1.5 * p.facing;
            p.sliding -= 1;
        } else {
            p.vx = move_x * PLAYER_SPEED;
        }

        // Jump
        if is_key_pressed(KeyCode::X) {
            if p.on_ground {
                p.vy = JUMP_VEL;
                p.on_ground = false;
                p.jump_held = 1;
            } else if p.on_wall != 0 {
                // Wall jump
                p.vy = JUMP_VEL;
                p.vx = -p.on_wall as f32 * PLAYER_SPEED * 1.5;
                p.facing = -p.on_wall as f32;
                p.on_wall = 0;
                p.jump_held = 1;
                spawn_particles(&mut self.particles, p.x + if p.facing < 0.0 { p.w } else { 0.0 }, p.y + p.h * 0.5, 5, WHITE, 2.0, 8);
            }
        }

        // Variable jump height
        if is_key_down(KeyCode::X) && p.jump_held > 0 && p.jump_held < JUMP_HOLD_MAX {
            p.jump_held += 1;
        } else {
            p.jump_held = 0;
        }
        if !is_key_down(KeyCode::X) && p.vy < 0.0 && p.jump_held == 0 {
            p.vy *= 0.6; // cut jump short
        }

        // Dash / Slide
        if is_key_pressed(KeyCode::Z) && p.attacking <= 0 {
            if !p.on_ground && p.can_dash && p.dashing <= 0 {
                p.dashing = DASH_FRAMES;
                p.can_dash = false;
                p.invuln = DASH_FRAMES; // brief invincibility
            } else if p.on_ground && p.vx.abs() > 0.1 && p.sliding <= 0 {
                p.sliding = SLIDE_FRAMES;
            }
        }

        // Attack
        if is_key_pressed(KeyCode::Space) && p.attacking <= 0 && p.dashing <= 0 {
            if p.combo_timer > 0 && p.combo < 3 {
                p.combo += 1;
            } else {
                p.combo = 1;
            }
            p.attacking = ATTACK_DURATION;
            p.combo_timer = COMBO_WINDOW + ATTACK_DURATION;
            spawn_slash_particles(&mut self.particles, p.x + p.w * 0.5, p.y + p.h * 0.5, p.facing);
            self.shake_intensity = 2.0;
            self.shake_timer = 4;
        }

        if p.attacking > 0 {
            p.attacking -= 1;
            // Hit detection on enemies handled in update_enemies
        }

        // Shuriken
        if is_key_pressed(KeyCode::C) && p.shuriken > 0 && p.shuriken_cd <= 0 {
            p.shuriken -= 1;
            p.shuriken_cd = SHURIKEN_COOLDOWN;
            let sx = if p.facing > 0.0 { p.x + p.w } else { p.x - 4.0 };
            let sy = p.y + p.h * 0.3;
            if self.projectiles.len() < MAX_PROJECTILES {
                self.projectiles.push(Projectile {
                    active: true,
                    x: sx,
                    y: sy,
                    vx: SHURIKEN_SPEED * p.facing,
                    vy: 0.0,
                    w: 4.0,
                    h: 4.0,
                    owner: ProjOwner::Player,
                    damage: 1,
                    life: 120,
                });
            }
        }

        // Apply gravity
        if p.dashing <= 0 {
            if p.on_wall != 0 && p.vy > 0.0 {
                p.vy += GRAVITY * 0.3; // wall slide
            } else {
                p.vy += GRAVITY;
            }
            if p.vy > MAX_FALL {
                p.vy = MAX_FALL;
            }
        }

        // Move and collide X
        let new_x = p.x + p.vx;
        let pr = if p.sliding > 0 {
            Rect::new(new_x, p.y + p.h * 0.5, p.w, p.h * 0.5)
        } else {
            Rect::new(new_x, p.y, p.w, p.h)
        };

        let mut blocked_x = false;
        // Check left/right edges of player rect
        if p.vx < 0.0 {
            if is_solid(&self.map, pr.x, pr.y + 2.0) || is_solid(&self.map, pr.x, pr.y + pr.h - 2.0) {
                blocked_x = true;
            }
        } else if p.vx > 0.0 {
            if is_solid(&self.map, pr.x + pr.w, pr.y + 2.0) || is_solid(&self.map, pr.x + pr.w, pr.y + pr.h - 2.0) {
                blocked_x = true;
            }
        }
        if blocked_x {
            // Snap to tile edge
            if p.vx > 0.0 {
                p.x = ((p.x + p.w + p.vx) / TILE).floor() * TILE - p.w;
            } else {
                p.x = ((p.x + p.vx) / TILE).ceil() * TILE;
            }
            p.vx = 0.0;
        } else {
            p.x = new_x;
        }

        // Wall detection
        p.on_wall = 0;
        if !p.on_ground {
            if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A))
                && (is_solid(&self.map, p.x - 1.0, p.y + 4.0) || is_solid(&self.map, p.x - 1.0, p.y + p.h - 4.0))
            {
                p.on_wall = -1;
            }
            if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D))
                && (is_solid(&self.map, p.x + p.w + 1.0, p.y + 4.0) || is_solid(&self.map, p.x + p.w + 1.0, p.y + p.h - 4.0))
            {
                p.on_wall = 1;
            }
        }

        // Wall-slide sparks
        if p.on_wall != 0 && p.vy > 0.0 {
            let wx = if p.on_wall < 0 { p.x } else { p.x + p.w };
            let scatter_dir = -p.on_wall as f32;
            for _ in 0..rand::gen_range(2, 4) {
                self.wall_sparks.push(WallSpark {
                    active: true,
                    x: wx + rand::gen_range(-1.0, 1.0),
                    y: p.y + p.h * rand::gen_range(0.3, 0.9),
                    vx: scatter_dir * rand::gen_range(0.5, 2.5),
                    vy: rand::gen_range(-2.0, 0.5),
                    life: rand::gen_range(6, 12),
                    max_life: 10,
                });
            }
        }

        // Move and collide Y
        let new_y = p.y + p.vy;
        let foot_y = p.y + p.h;
        p.on_ground = false;

        if p.vy >= 0.0 {
            // Falling / standing
            let check_y = new_y + p.h;
            let left_solid = is_solid_or_platform_top(&self.map, p.x + 2.0, check_y, p.vy, foot_y);
            let right_solid = is_solid_or_platform_top(&self.map, p.x + p.w - 2.0, check_y, p.vy, foot_y);
            if left_solid || right_solid {
                p.y = (check_y / TILE).floor() * TILE - p.h;
                p.vy = 0.0;
                p.on_ground = true;
                p.can_dash = true;
                p.jump_held = 0;
            } else {
                p.y = new_y;
            }
        } else {
            // Rising
            let check_y = new_y;
            if is_solid(&self.map, p.x + 2.0, check_y) || is_solid(&self.map, p.x + p.w - 2.0, check_y) {
                p.y = (check_y / TILE).ceil() * TILE;
                p.vy = 0.0;
            } else {
                p.y = new_y;
            }
        }

        // Spike damage — drop `p` borrow before calling self methods
        let prect = p.rect();
        let foot_tile = tile_at(&self.map, prect.x + prect.w * 0.5, prect.y + prect.h + 1.0);
        let spike_hit = foot_tile == TILE_SPIKE && p.invuln <= 0;

        // Clamp position
        if p.x < 0.0 { p.x = 0.0; }
        let max_x = self.level_cols as f32 * TILE - p.w;
        if p.x > max_x { p.x = max_x; }

        // Fall death
        let fell = p.y > MAP_ROWS as f32 * TILE;
        if fell {
            p.hp = 0;
        }

        // End the mutable borrow on self.player before calling self methods
        let _ = p;

        if spike_hit {
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
        spawn_particles(&mut self.particles, self.player.x + self.player.w * 0.5, self.player.y + self.player.h * 0.5, 8, RED, 3.0, 15);
        if self.player.hp <= 0 {
            self.player_die();
        }
    }

    fn player_die(&mut self) {
        self.player.dead = true;
        self.death_timer = 0;
        self.phase = GamePhase::Death;
        spawn_particles(
            &mut self.particles,
            self.player.x + self.player.w * 0.5,
            self.player.y + self.player.h * 0.5,
            20,
            RED,
            4.0,
            30,
        );
        self.start_shake(6.0, 15);
    }

    fn update_enemies(&mut self) {
        let px = self.player.x;
        let py = self.player.y;
        let p_rect = self.player.rect();
        let p_attacking = self.player.attacking > 0 && self.player.attacking > ATTACK_DURATION - 6;
        let attack_rect = self.player.attack_rect();
        let attack_dmg = if self.player.combo == 3 { 2 } else { 1 };

        for i in 0..self.enemies.len() {
            if !self.enemies[i].active {
                continue;
            }

            let e = &mut self.enemies[i];
            if e.hurt_timer > 0 {
                e.hurt_timer -= 1;
            }

            // Cull off-screen
            let dist_to_cam = (e.x - self.camera.x).abs();
            if dist_to_cam > SCREEN_W + CAMERA_MARGIN * 2.0 {
                continue;
            }

            match e.kind {
                EnemyKind::Guard => {
                    // Patrol
                    let detect_range = 120.0;
                    let dx = px - e.x;
                    let speed = if dx.abs() < detect_range && (py - e.y).abs() < 60.0 {
                        e.facing = if dx > 0.0 { 1.0 } else { -1.0 };
                        3.75 // 1.5 * 2.5
                    } else {
                        1.5
                    };

                    e.x += speed * e.facing;

                    // Turn at patrol bounds
                    if e.x <= e.patrol_left {
                        e.x = e.patrol_left;
                        e.facing = 1.0;
                    }
                    if e.x >= e.patrol_right {
                        e.x = e.patrol_right;
                        e.facing = -1.0;
                    }

                    // Simple gravity for guards
                    e.vy += GRAVITY;
                    if e.vy > MAX_FALL { e.vy = MAX_FALL; }
                    let new_y = e.y + e.vy;
                    let foot_check = new_y + e.h;
                    if is_solid(&self.map, e.x + 2.0, foot_check) || is_solid(&self.map, e.x + e.w - 2.0, foot_check) {
                        e.y = (foot_check / TILE).floor() * TILE - e.h;
                        e.vy = 0.0;
                    } else {
                        e.y = new_y;
                    }

                    // Turn at edges
                    let ahead_x = e.x + e.facing * (e.w + 2.0);
                    let below = e.y + e.h + 4.0;
                    if !is_solid(&self.map, ahead_x, below) {
                        e.facing = -e.facing;
                    }
                }
                EnemyKind::Archer => {
                    // Stationary, just shoot
                    let dx = px - e.x;
                    if dx.abs() < 200.0 && (py - e.y).abs() < 40.0 {
                        e.facing = if dx > 0.0 { 1.0 } else { -1.0 };
                        e.shoot_timer -= 1;
                        if e.shoot_timer <= 0 {
                            e.shoot_timer = 90;
                            // Fire arrow
                            if self.projectiles.len() < MAX_PROJECTILES {
                                self.projectiles.push(Projectile {
                                    active: true,
                                    x: e.x + if e.facing > 0.0 { e.w } else { -5.0 },
                                    y: e.y + e.h * 0.3,
                                    vx: ARROW_SPEED * e.facing,
                                    vy: 0.0,
                                    w: 5.0,
                                    h: 2.0,
                                    owner: ProjOwner::Enemy,
                                    damage: 1,
                                    life: 150,
                                });
                            }
                        }
                    }

                    // Gravity for archer
                    e.vy += GRAVITY;
                    if e.vy > MAX_FALL { e.vy = MAX_FALL; }
                    let new_y = e.y + e.vy;
                    let foot_check = new_y + e.h;
                    if is_solid(&self.map, e.x + 2.0, foot_check) || is_solid(&self.map, e.x + e.w - 2.0, foot_check)
                        || tile_at(&self.map, e.x + 2.0, foot_check) == TILE_PLATFORM
                        || tile_at(&self.map, e.x + e.w - 2.0, foot_check) == TILE_PLATFORM
                    {
                        e.y = (foot_check / TILE).floor() * TILE - e.h;
                        e.vy = 0.0;
                    } else {
                        e.y = new_y;
                    }
                }
            }

            // Player attack hits enemy
            let e_rect = self.enemies[i].rect();
            if p_attacking && rects_overlap(&attack_rect, &e_rect) && self.enemies[i].hurt_timer <= 0 {
                self.enemies[i].hp -= attack_dmg;
                self.enemies[i].hurt_timer = 10;
                spawn_particles(
                    &mut self.particles,
                    self.enemies[i].x + self.enemies[i].w * 0.5,
                    self.enemies[i].y + self.enemies[i].h * 0.5,
                    6,
                    WHITE,
                    3.0,
                    10,
                );
                // Blood/impact particles on enemy hit
                spawn_blood_particles(
                    &mut self.particles,
                    self.enemies[i].x + self.enemies[i].w * 0.5,
                    self.enemies[i].y + self.enemies[i].h * 0.4,
                );
                // Hit stop on combo finisher (3rd hit)
                if self.player.combo == 3 {
                    self.hit_stop = 4;
                }
                if self.enemies[i].hp <= 0 {
                    self.enemies[i].active = false;
                    self.player.score += self.enemies[i].score_val;
                    spawn_particles(
                        &mut self.particles,
                        self.enemies[i].x + self.enemies[i].w * 0.5,
                        self.enemies[i].y + self.enemies[i].h * 0.5,
                        15,
                        ORANGE,
                        4.0,
                        20,
                    );
                    // Chance to drop pickup
                    if rand::gen_range(0.0, 1.0) < 0.3 {
                        self.pickups.push(Pickup {
                            active: true,
                            kind: if rand::gen_range(0.0, 1.0) < 0.5 { PickupKind::Heart } else { PickupKind::Scroll },
                            x: self.enemies[i].x,
                            y: self.enemies[i].y,
                            w: 8.0,
                            h: 8.0,
                        });
                    }
                }
            }

            // Enemy contact damages player
            if self.enemies[i].active && rects_overlap(&p_rect, &e_rect) {
                let dmg = match self.enemies[i].kind {
                    EnemyKind::Guard => 1,
                    EnemyKind::Archer => 1,
                };
                self.damage_player(dmg);
            }
        }
    }

    fn update_projectiles(&mut self) {
        let p_rect = self.player.rect();
        let mut player_damage = 0;

        for proj in self.projectiles.iter_mut() {
            if !proj.active {
                continue;
            }

            proj.x += proj.vx;
            proj.y += proj.vy;
            proj.life -= 1;

            if proj.life <= 0 {
                proj.active = false;
                continue;
            }

            // Off-screen cull
            if proj.x < self.camera.x - CAMERA_MARGIN
                || proj.x > self.camera.x + SCREEN_W + CAMERA_MARGIN
            {
                proj.active = false;
                continue;
            }

            // Tile collision
            if is_solid(&self.map, proj.x + proj.w * 0.5, proj.y + proj.h * 0.5) {
                proj.active = false;
                spawn_particles(&mut self.particles, proj.x, proj.y, 3, GRAY, 2.0, 8);
                continue;
            }

            match proj.owner {
                ProjOwner::Player => {
                    // Hit enemies
                    let pr = proj.rect();
                    for e in self.enemies.iter_mut() {
                        if e.active && rects_overlap(&pr, &e.rect()) {
                            e.hp -= proj.damage;
                            e.hurt_timer = 8;
                            proj.active = false;
                            spawn_particles(&mut self.particles, proj.x, proj.y, 4, SKYBLUE, 2.0, 8);
                            if e.hp <= 0 {
                                e.active = false;
                                self.player.score += e.score_val;
                                spawn_particles(
                                    &mut self.particles,
                                    e.x + e.w * 0.5,
                                    e.y + e.h * 0.5,
                                    15,
                                    ORANGE,
                                    4.0,
                                    20,
                                );
                            }
                            break;
                        }
                    }
                }
                ProjOwner::Enemy => {
                    // Hit player — defer damage to avoid borrow conflict
                    if rects_overlap(&proj.rect(), &p_rect) {
                        proj.active = false;
                        player_damage += proj.damage;
                    }
                }
            }
        }

        self.projectiles.retain(|p| p.active);

        // Apply deferred player damage
        if player_damage > 0 {
            self.damage_player(player_damage);
        }
    }

    fn update_particles(&mut self) {
        for p in self.particles.iter_mut() {
            if !p.active {
                continue;
            }
            p.x += p.vx;
            p.y += p.vy;
            if p.gravity {
                p.vy += 0.25; // heavier gravity for blood/impact
            } else {
                p.vy += 0.05;
            }
            p.life -= 1;
            if p.life <= 0 {
                p.active = false;
            }
        }
    }

    fn update_pickups(&mut self) {
        let p_rect = self.player.rect();
        for pickup in self.pickups.iter_mut() {
            if !pickup.active {
                continue;
            }
            if rects_overlap(&p_rect, &pickup.rect()) {
                pickup.active = false;
                match pickup.kind {
                    PickupKind::Heart => {
                        if self.player.hp < self.player.max_hp {
                            self.player.hp += 1;
                        }
                        spawn_particles(&mut self.particles, pickup.x, pickup.y, 5, RED, 2.0, 15);
                    }
                    PickupKind::Scroll => {
                        self.player.score += 200;
                        spawn_particles(&mut self.particles, pickup.x, pickup.y, 5, YELLOW, 2.0, 15);
                    }
                    PickupKind::Ammo => {
                        self.player.shuriken = (self.player.shuriken + 5).min(30);
                        spawn_particles(&mut self.particles, pickup.x, pickup.y, 5, Color::new(0.75, 0.75, 0.75, 1.0), 2.0, 15);
                    }
                }
            }
        }
    }

    fn update_camera(&mut self) {
        let target_x = self.player.x - SCREEN_W * 0.4;
        self.camera.x += (target_x - self.camera.x) * 0.1;
        if self.camera.x < 0.0 {
            self.camera.x = 0.0;
        }
        let max_cam = self.level_cols as f32 * TILE - SCREEN_W;
        if self.camera.x > max_cam {
            self.camera.x = max_cam;
        }

        let target_y = self.player.y - SCREEN_H * 0.5;
        self.camera.y += (target_y - self.camera.y) * 0.05;
        self.camera.y = self.camera.y.clamp(-(SCREEN_H * 0.3), 0.0);
    }

    // ── Draw ───────────────────────────────────────────────────────────

    fn draw(&self) {
        clear_background(BLACK);

        match self.phase {
            GamePhase::Title => self.draw_title(),
            GamePhase::Playing | GamePhase::Paused => {
                self.draw_game();
                if self.phase == GamePhase::Paused {
                    self.draw_pause_overlay();
                }
            }
            GamePhase::Death => self.draw_game(),
            GamePhase::GameOver => self.draw_game_over(),
            GamePhase::Victory => self.draw_victory(),
            GamePhase::Story => self.draw_story(),
        }
    }

    fn draw_title(&self) {
        // Background gradient
        for y in 0..SCREEN_H as i32 {
            let t = y as f32 / SCREEN_H;
            let r = 0.05 + t * 0.1;
            let g = 0.0;
            let b = 0.1 + t * 0.15;
            draw_rectangle(0.0, y as f32, SCREEN_W, 1.0, Color::new(r, g, b, 1.0));
        }

        // Stars
        for star in &self.stars {
            let sx = star.x % SCREEN_W;
            let flicker = 0.7 + 0.3 * ((self.frame as f32 * 0.02 + star.x).sin());
            let a = star.brightness * flicker;
            draw_circle(sx, star.y, star.size, Color::new(1.0, 1.0, 1.0, a));
        }

        // Floating embers
        for i in 0..15 {
            let t = self.frame as f32 * 0.01 + i as f32 * 17.0;
            let ex = (SCREEN_W * 0.5 + t.sin() * 200.0 + (t * 0.7).cos() * 100.0) % SCREEN_W;
            let ey = (SCREEN_H - (t * 30.0 + i as f32 * 50.0) % (SCREEN_H + 50.0)).max(0.0);
            let a = 0.3 + 0.7 * ((t * 2.0).sin() * 0.5 + 0.5);
            draw_circle(ex, ey, 2.0, Color::new(1.0, 0.4, 0.0, a));
        }

        // Title
        let title = "SHADOW BLADE";
        let title_size = 48.0;
        let tw = title.len() as f32 * title_size * 0.45;
        // Shadow
        draw_text(title, SCREEN_W * 0.5 - tw * 0.5 + 3.0, 160.0 + 3.0, title_size, Color::new(0.0, 0.0, 0.0, 0.5));
        // Main
        draw_text(title, SCREEN_W * 0.5 - tw * 0.5, 160.0, title_size, Color::new(0.9, 0.1, 0.1, 1.0));

        // Crimson Oath subtitle
        let sub = "THE CRIMSON OATH";
        let sub_size = 20.0;
        let sw = sub.len() as f32 * sub_size * 0.45;
        draw_text(sub, SCREEN_W * 0.5 - sw * 0.5, 195.0, sub_size, Color::new(0.9, 0.3, 0.3, 1.0));

        // Tagline
        let tag = "A tale of honor and betrayal";
        let tag_size = 14.0;
        let tw2 = tag.len() as f32 * tag_size * 0.38;
        draw_text(tag, SCREEN_W * 0.5 - tw2 * 0.5, 215.0, tag_size, Color::new(0.7, 0.7, 0.7, 1.0));

        // Ninja silhouette (simple)
        draw_rectangle(SCREEN_W * 0.5 - 12.0, 230.0, 24.0, 32.0, Color::new(0.15, 0.15, 0.15, 1.0));
        draw_rectangle(SCREEN_W * 0.5 - 6.0, 220.0, 12.0, 12.0, Color::new(0.15, 0.15, 0.15, 1.0));
        // Eyes
        draw_rectangle(SCREEN_W * 0.5 - 4.0, 224.0, 3.0, 2.0, RED);
        draw_rectangle(SCREEN_W * 0.5 + 1.0, 224.0, 3.0, 2.0, RED);

        // Blink text
        if self.blink_timer < 40 {
            let start_text = "PRESS START";
            let st_size = 24.0;
            let stw = start_text.len() as f32 * st_size * 0.45;
            draw_text(start_text, SCREEN_W * 0.5 - stw * 0.5, 340.0, st_size, WHITE);
        }

        // Controls hint
        let hint = "ARROWS:Move  X:Jump  SPACE:Attack  Z:Dash  C:Shuriken  ENTER:Start";
        let hint_size = 14.0;
        let hw = hint.len() as f32 * hint_size * 0.38;
        draw_text(hint, SCREEN_W * 0.5 - hw * 0.5, 440.0, hint_size, Color::new(0.5, 0.5, 0.5, 1.0));
    }

    fn draw_game(&self) {
        let mut cam_x = self.camera.x;
        let mut cam_y = self.camera.y;

        // Screen shake
        if self.shake_timer > 0 {
            cam_x += rand::gen_range(-self.shake_intensity, self.shake_intensity);
            cam_y += rand::gen_range(-self.shake_intensity, self.shake_intensity);
        }

        self.draw_background(cam_x, cam_y);
        self.draw_tiles(cam_x, cam_y);
        self.draw_pickups(cam_x, cam_y);
        self.draw_enemies(cam_x, cam_y);
        self.draw_projectiles(cam_x, cam_y);

        // Dash afterimage ghosts (drawn before player for layering)
        for ghost in &self.dash_ghosts {
            if ghost.active {
                let gx = ghost.x - cam_x;
                let gy = ghost.y - cam_y;
                let alpha = 0.5 * (ghost.life as f32 / ghost.max_life as f32);
                // Purple-tinted afterimage (#a040ff)
                draw_rectangle(gx, gy, ghost.w, ghost.h, Color::new(0.627, 0.251, 1.0, alpha));
            }
        }

        // Wall-slide sparks
        for spark in &self.wall_sparks {
            if spark.active {
                let sx = spark.x - cam_x;
                let sy = spark.y - cam_y;
                let alpha = spark.life as f32 / spark.max_life as f32;
                // Yellow/white spark
                let c = if rand::gen_range(0.0, 1.0) > 0.5 {
                    Color::new(1.0, 1.0, 0.8, alpha)
                } else {
                    Color::new(1.0, 0.9, 0.3, alpha)
                };
                draw_circle(sx, sy, 1.5 * alpha, c);
            }
        }

        self.draw_player(cam_x, cam_y);

        // Sword slash arc
        self.draw_slash_arc(cam_x, cam_y);

        self.draw_particles(cam_x, cam_y);

        // Parallax foreground grass tufts (slightly faster than camera)
        self.draw_foreground_grass(cam_x, cam_y);

        self.draw_hud();
        self.draw_env_signs(cam_x);

        // Death flash
        if self.phase == GamePhase::Death && self.death_timer < 10 {
            let a = 1.0 - self.death_timer as f32 / 10.0;
            draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(1.0, 0.0, 0.0, a * 0.4));
        }

        // CRT scanline overlay
        {
            let mut y = 0;
            while y < SCREEN_H as i32 {
                draw_rectangle(0.0, y as f32, SCREEN_W, 2.0, Color::new(0.0, 0.0, 0.0, 0.12));
                y += 4;
            }
        }
    }

    fn draw_background(&self, cam_x: f32, _cam_y: f32) {
        // Sky gradient
        for y in 0..SCREEN_H as i32 {
            let t = y as f32 / SCREEN_H;
            let r = 0.08 + t * 0.12;
            let g = 0.02 + t * 0.04;
            let b = 0.15 + t * 0.2;
            draw_rectangle(0.0, y as f32, SCREEN_W, 1.0, Color::new(r, g, b, 1.0));
        }

        // Stars
        for star in &self.stars {
            let sx = star.x - cam_x * 0.05;
            if sx >= -5.0 && sx <= SCREEN_W + 5.0 {
                let flicker = 0.7 + 0.3 * ((self.frame as f32 * 0.015 + star.x).sin());
                let a = star.brightness * flicker;
                draw_circle(sx, star.y, star.size, Color::new(1.0, 1.0, 1.0, a));
            }
        }

        // Moon
        let moon_x = SCREEN_W * 0.75 - cam_x * 0.02;
        draw_circle(moon_x, 60.0, 30.0, Color::new(0.95, 0.92, 0.8, 0.9));
        draw_circle(moon_x + 8.0, 55.0, 26.0, Color::new(0.08, 0.02, 0.15, 1.0)); // shadow for crescent

        // Parallax layer 1 - far mountains
        let mountain_col = Color::new(0.12, 0.06, 0.2, 1.0);
        for i in 0..20 {
            let mx = i as f32 * 120.0 - cam_x * 0.1;
            if mx > -120.0 && mx < SCREEN_W + 120.0 {
                let mh = 60.0 + (i as f32 * 37.0).sin().abs() * 80.0;
                let my = SCREEN_H - 180.0;
                // Triangle mountain
                draw_triangle(
                    Vec2::new(mx, my),
                    Vec2::new(mx + 60.0, my - mh),
                    Vec2::new(mx + 120.0, my),
                    mountain_col,
                );
            }
        }

        // Parallax layer 2 - bamboo/trees mid
        let tree_col = Color::new(0.08, 0.15, 0.08, 0.7);
        for i in 0..40 {
            let tx = i as f32 * 80.0 - cam_x * 0.3;
            if tx > -20.0 && tx < SCREEN_W + 20.0 {
                let th = 40.0 + (i as f32 * 23.0).sin().abs() * 100.0;
                let ty = SCREEN_H - 120.0;
                // Bamboo stalk
                draw_rectangle(tx, ty - th, 4.0, th, tree_col);
                draw_rectangle(tx + 12.0, ty - th * 0.7, 3.0, th * 0.7, tree_col);
                // Leaves
                draw_circle(tx + 2.0, ty - th - 5.0, 8.0, Color::new(0.1, 0.25, 0.1, 0.5));
            }
        }

        // Parallax layer 3 - near trees
        let near_col = Color::new(0.05, 0.1, 0.05, 0.5);
        for i in 0..60 {
            let tx = i as f32 * 55.0 - cam_x * 0.5;
            if tx > -15.0 && tx < SCREEN_W + 15.0 {
                let th = 30.0 + (i as f32 * 13.0).sin().abs() * 60.0;
                let ty = SCREEN_H - 60.0;
                draw_rectangle(tx, ty - th, 3.0, th, near_col);
            }
        }
    }

    fn draw_tiles(&self, cam_x: f32, cam_y: f32) {
        let start_col = (cam_x / TILE).floor() as isize - 1;
        let end_col = ((cam_x + SCREEN_W) / TILE).ceil() as isize + 1;
        let start_row = 0isize;
        let end_row = MAP_ROWS as isize;

        for row in start_row..end_row {
            for col in start_col..end_col {
                if col < 0 || col >= self.level_cols as isize || row < 0 || row >= MAP_ROWS as isize {
                    continue;
                }
                let tile = self.map[row as usize][col as usize];
                if tile == TILE_EMPTY {
                    continue;
                }
                let tx = col as f32 * TILE - cam_x;
                let ty = row as f32 * TILE - cam_y;

                match tile {
                    TILE_GROUND => {
                        let shade = if row as usize == 27 { 0.35 } else { 0.25 };
                        draw_rectangle(tx, ty, TILE, TILE, Color::new(shade, 0.2, 0.1, 1.0));
                        // Top edge highlight
                        if row == 0 || self.map[(row - 1) as usize][col as usize] == TILE_EMPTY {
                            draw_rectangle(tx, ty, TILE, 2.0, Color::new(0.4, 0.35, 0.2, 1.0));
                            // Grass tufts
                            if (col * 7 + row * 3) % 5 == 0 {
                                draw_rectangle(tx + 3.0, ty - 2.0, 2.0, 3.0, Color::new(0.15, 0.4, 0.1, 1.0));
                                draw_rectangle(tx + 9.0, ty - 1.0, 2.0, 2.0, Color::new(0.15, 0.4, 0.1, 1.0));
                            }
                        }
                    }
                    TILE_WALL => {
                        draw_rectangle(tx, ty, TILE, TILE, Color::new(0.3, 0.28, 0.25, 1.0));
                        // Brick pattern
                        draw_line(tx, ty + TILE * 0.5, tx + TILE, ty + TILE * 0.5, 1.0, Color::new(0.2, 0.18, 0.15, 1.0));
                        let offset = if row % 2 == 0 { TILE * 0.5 } else { 0.0 };
                        draw_line(tx + offset, ty, tx + offset, ty + TILE * 0.5, 1.0, Color::new(0.2, 0.18, 0.15, 1.0));
                    }
                    TILE_PLATFORM => {
                        draw_rectangle(tx, ty, TILE, 4.0, Color::new(0.45, 0.3, 0.15, 1.0));
                        draw_rectangle(tx, ty + 4.0, TILE, 2.0, Color::new(0.35, 0.22, 0.1, 1.0));
                    }
                    TILE_SPIKE => {
                        // Spikes as triangles
                        for s in 0..4 {
                            let sx = tx + s as f32 * 4.0;
                            draw_triangle(
                                Vec2::new(sx, ty + TILE),
                                Vec2::new(sx + 2.0, ty + 4.0),
                                Vec2::new(sx + 4.0, ty + TILE),
                                Color::new(0.6, 0.6, 0.6, 1.0),
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn draw_player(&self, cam_x: f32, cam_y: f32) {
        let p = &self.player;
        if p.dead {
            return;
        }

        // Invulnerability flicker
        if p.invuln > 0 && (p.invuln / 3) % 2 == 0 {
            return;
        }

        let sx = p.x - cam_x;
        let sy = p.y - cam_y;
        let flip = p.facing < 0.0;

        let tex = if p.attacking > 0 {
            &self.tex_ninja_attack
        } else if p.dashing > 0 {
            &self.tex_ninja_jump
        } else if !p.on_ground {
            &self.tex_ninja_jump
        } else if p.vx.abs() > 0.5 {
            &self.tex_ninja_run
        } else {
            &self.tex_ninja_idle
        };

        let scale = 2.0;
        let dw = tex.width() * scale;
        let dh = tex.height() * scale;

        let draw_params = DrawTextureParams {
            dest_size: Some(Vec2::new(if flip { -dw } else { dw }, dh)),
            ..Default::default()
        };

        let dx = if flip { sx + dw * 0.5 + 2.0 } else { sx - dw * 0.25 };
        let dy = sy + p.h - dh;

        draw_texture_ex(tex, dx, dy, WHITE, draw_params);

        // Slash visual during attack
        if p.attacking > 0 && p.attacking > ATTACK_DURATION - 8 {
            let slash_x = if p.facing > 0.0 { sx + p.w + 2.0 } else { sx - 20.0 };
            let slash_y = sy + p.h * 0.2;
            let alpha = p.attacking as f32 / ATTACK_DURATION as f32;
            let slash_color = match p.combo {
                1 => Color::new(1.0, 1.0, 1.0, alpha),
                2 => Color::new(0.5, 0.8, 1.0, alpha),
                3 => Color::new(0.0, 1.0, 1.0, alpha),
                _ => Color::new(1.0, 1.0, 1.0, alpha),
            };
            // Arc-like slash effect
            let arc_w = if p.combo == 3 { 28.0 } else { 20.0 };
            let arc_h = if p.combo == 3 { 24.0 } else { 16.0 };
            draw_rectangle(slash_x, slash_y, arc_w, 2.0, slash_color);
            draw_rectangle(slash_x + arc_w * 0.3, slash_y - arc_h * 0.3, 2.0, arc_h, slash_color);
            if p.combo == 3 {
                // Shockwave for combo 3
                let sw_y = sy + p.h;
                let sw_w = 40.0 * alpha;
                draw_rectangle(sx - sw_w * 0.5 + p.w * 0.5, sw_y - 4.0, sw_w, 4.0, Color::new(0.0, 1.0, 1.0, alpha * 0.5));
            }
        }

        // Wall slide indicator
        if p.on_wall != 0 {
            let wx = if p.on_wall < 0 { sx - 2.0 } else { sx + p.w };
            for i in 0..3 {
                let spark_y = sy + p.h * 0.3 + i as f32 * 6.0 + (self.frame as f32 * 0.3).sin() * 2.0;
                draw_circle(wx, spark_y, 1.5, Color::new(1.0, 0.8, 0.3, 0.7));
            }
        }
    }

    fn draw_enemies(&self, cam_x: f32, cam_y: f32) {
        for e in &self.enemies {
            if !e.active {
                continue;
            }
            let sx = e.x - cam_x;
            let sy = e.y - cam_y;
            if sx < -CAMERA_MARGIN || sx > SCREEN_W + CAMERA_MARGIN {
                continue;
            }

            // Hurt flash
            let tint = if e.hurt_timer > 0 { RED } else { WHITE };

            let tex = match e.kind {
                EnemyKind::Guard => &self.tex_guard,
                EnemyKind::Archer => &self.tex_archer,
            };

            let scale = 2.0;
            let dw = tex.width() * scale;
            let dh = tex.height() * scale;
            let flip = e.facing < 0.0;

            let draw_params = DrawTextureParams {
                dest_size: Some(Vec2::new(if flip { -dw } else { dw }, dh)),
                ..Default::default()
            };
            let dx = if flip { sx + dw * 0.5 } else { sx - dw * 0.25 };
            let dy = sy + e.h - dh;

            draw_texture_ex(tex, dx, dy, tint, draw_params);

            // HP bar for enemies with more than 1 hp
            if e.kind == EnemyKind::Guard {
                let bar_w = e.w;
                let bar_h = 3.0;
                let bar_x = sx;
                let bar_y = sy - 6.0;
                draw_rectangle(bar_x, bar_y, bar_w, bar_h, Color::new(0.3, 0.0, 0.0, 0.8));
                let hp_frac = e.hp as f32 / 2.0;
                draw_rectangle(bar_x, bar_y, bar_w * hp_frac, bar_h, RED);
            }
        }
    }

    fn draw_projectiles(&self, cam_x: f32, cam_y: f32) {
        for proj in &self.projectiles {
            if !proj.active {
                continue;
            }
            let sx = proj.x - cam_x;
            let sy = proj.y - cam_y;

            match proj.owner {
                ProjOwner::Player => {
                    // Shuriken: spinning star
                    let angle = self.frame as f32 * 0.3;
                    let cx = sx + proj.w * 0.5;
                    let cy = sy + proj.h * 0.5;
                    let r = 3.0;
                    for i in 0..4 {
                        let a = angle + i as f32 * std::f32::consts::FRAC_PI_2;
                        let px = cx + a.cos() * r;
                        let py = cy + a.sin() * r;
                        draw_line(cx, cy, px, py, 1.5, Color::new(0.75, 0.75, 0.75, 1.0));
                    }
                    draw_circle(cx, cy, 1.5, WHITE);
                }
                ProjOwner::Enemy => {
                    // Arrow
                    draw_rectangle(sx, sy, proj.w, proj.h, Color::new(0.55, 0.27, 0.07, 1.0));
                    // Arrowhead
                    let tip_x = if proj.vx > 0.0 { sx + proj.w } else { sx - 3.0 };
                    draw_triangle(
                        Vec2::new(tip_x, sy - 1.0),
                        Vec2::new(tip_x + 3.0 * proj.vx.signum(), sy + proj.h * 0.5),
                        Vec2::new(tip_x, sy + proj.h + 1.0),
                        Color::new(0.6, 0.6, 0.6, 1.0),
                    );
                }
            }
        }
    }

    fn draw_particles(&self, cam_x: f32, cam_y: f32) {
        for p in &self.particles {
            if !p.active {
                continue;
            }
            let sx = p.x - cam_x;
            let sy = p.y - cam_y;
            let alpha = p.life as f32 / p.max_life as f32;
            let c = Color::new(p.color.r, p.color.g, p.color.b, alpha);
            draw_circle(sx, sy, p.size * alpha, c);
        }
    }

    fn draw_pickups(&self, cam_x: f32, cam_y: f32) {
        for pickup in &self.pickups {
            if !pickup.active {
                continue;
            }
            let sx = pickup.x - cam_x;
            let sy = pickup.y - cam_y;
            if sx < -CAMERA_MARGIN || sx > SCREEN_W + CAMERA_MARGIN {
                continue;
            }

            // Bobbing animation
            let bob = (self.frame as f32 * 0.05 + pickup.x * 0.1).sin() * 3.0;
            let dy = sy + bob;

            let tex = match pickup.kind {
                PickupKind::Heart => &self.tex_heart,
                PickupKind::Scroll => &self.tex_scroll,
                PickupKind::Ammo => &self.tex_ammo,
            };

            let scale = 2.0;
            let params = DrawTextureParams {
                dest_size: Some(Vec2::new(tex.width() * scale, tex.height() * scale)),
                ..Default::default()
            };
            draw_texture_ex(tex, sx, dy, WHITE, params);

            // Glow
            let glow_color = match pickup.kind {
                PickupKind::Heart => Color::new(1.0, 0.2, 0.2, 0.15),
                PickupKind::Scroll => Color::new(1.0, 0.9, 0.0, 0.15),
                PickupKind::Ammo => Color::new(0.7, 0.7, 0.7, 0.15),
            };
            draw_circle(sx + pickup.w, dy + pickup.h, 10.0, glow_color);
        }
    }

    fn draw_hud(&self) {
        // Semi-transparent HUD background bar
        draw_rectangle(0.0, 0.0, SCREEN_W, 32.0, Color::new(0.0, 0.0, 0.0, 0.5));

        // Hearts
        for i in 0..self.player.max_hp {
            let hx = 10.0 + i as f32 * 20.0;
            let hy = 8.0;
            if i < self.player.hp {
                let scale = 2.0;
                let params = DrawTextureParams {
                    dest_size: Some(Vec2::new(self.tex_heart.width() * scale, self.tex_heart.height() * scale)),
                    ..Default::default()
                };
                draw_texture_ex(&self.tex_heart, hx, hy, WHITE, params);
            } else {
                draw_rectangle(hx, hy, 14.0, 14.0, Color::new(0.3, 0.0, 0.0, 0.5));
            }
        }

        // Level trial name centered
        let trial_name = LEVEL_NAMES[self.current_level.min(2)];
        let name_size = 16.0;
        let nw = trial_name.len() as f32 * name_size * 0.42;
        draw_text(trial_name, SCREEN_W * 0.5 - nw * 0.5, 18.0, name_size, Color::new(0.8, 0.8, 0.9, 1.0));

        // Level subtitle
        let sub_size = 12.0;
        let sw = self.level_name.len() as f32 * sub_size * 0.42;
        draw_text(&self.level_name, SCREEN_W * 0.5 - sw * 0.5, 30.0, sub_size, Color::new(0.6, 0.6, 0.7, 1.0));

        // Score top-right
        let score_text = format!("{:08}", self.player.score);
        draw_text(&score_text, SCREEN_W - 130.0, 22.0, 20.0, Color::new(1.0, 0.85, 0.0, 1.0));

        // Shuriken count bottom-left
        draw_rectangle(0.0, SCREEN_H - 28.0, 120.0, 28.0, Color::new(0.0, 0.0, 0.0, 0.5));
        // Shuriken icon
        let sx = 10.0;
        let sy = SCREEN_H - 22.0;
        for i in 0..4 {
            let a = i as f32 * std::f32::consts::FRAC_PI_2;
            draw_line(sx + 6.0, sy + 6.0, sx + 6.0 + a.cos() * 5.0, sy + 6.0 + a.sin() * 5.0, 1.5, Color::new(0.75, 0.75, 0.75, 1.0));
        }
        draw_text(&format!("x{}", self.player.shuriken), 30.0, SCREEN_H - 10.0, 18.0, WHITE);
    }

    fn draw_slash_arc(&self, cam_x: f32, cam_y: f32) {
        let p = &self.player;
        if p.dead || p.attacking <= 0 || p.attacking <= ATTACK_DURATION - 8 {
            return;
        }
        let ar = p.attack_rect();
        let cx = ar.x + ar.w * 0.5 - cam_x;
        let cy = ar.y + ar.h * 0.5 - cam_y;
        let radius = if p.combo == 3 { 22.0 } else { 16.0 };
        let progress = 1.0 - (p.attacking as f32 / ATTACK_DURATION as f32);
        let segments = 8;
        // Quarter-circle arc sweeping from top to bottom (or bottom to top based on facing)
        let start_angle = -std::f32::consts::FRAC_PI_2;
        let end_angle = start_angle + std::f32::consts::FRAC_PI_2 * (0.3 + progress * 0.7);
        let alpha = p.attacking as f32 / ATTACK_DURATION as f32;
        let arc_color = if p.combo == 3 {
            // Yellow glow for combo 3
            Color::new(1.0, 0.95, 0.3, alpha * 0.9)
        } else {
            // White/cyan
            Color::new(0.8, 1.0, 1.0, alpha * 0.7)
        };
        for i in 0..segments {
            let t0 = i as f32 / segments as f32;
            let t1 = (i + 1) as f32 / segments as f32;
            let a0 = start_angle + (end_angle - start_angle) * t0;
            let a1 = start_angle + (end_angle - start_angle) * t1;
            let x0 = cx + a0.cos() * radius * p.facing;
            let y0 = cy + a0.sin() * radius;
            let x1 = cx + a1.cos() * radius * p.facing;
            let y1 = cy + a1.sin() * radius;
            let thickness = if p.combo == 3 { 2.5 } else { 1.5 };
            draw_line(x0, y0, x1, y1, thickness, arc_color);
        }
        // Extra glow ring for combo 3
        if p.combo == 3 {
            let glow_r = radius + 4.0;
            for i in 0..segments {
                let t0 = i as f32 / segments as f32;
                let t1 = (i + 1) as f32 / segments as f32;
                let a0 = start_angle + (end_angle - start_angle) * t0;
                let a1 = start_angle + (end_angle - start_angle) * t1;
                let x0 = cx + a0.cos() * glow_r * p.facing;
                let y0 = cy + a0.sin() * glow_r;
                let x1 = cx + a1.cos() * glow_r * p.facing;
                let y1 = cy + a1.sin() * glow_r;
                draw_line(x0, y0, x1, y1, 1.0, Color::new(1.0, 0.85, 0.0, alpha * 0.4));
            }
        }
    }

    fn draw_foreground_grass(&self, cam_x: f32, cam_y: f32) {
        // Foreground grass parallaxes slightly faster (1.05x) for depth
        let fg_cam_x = cam_x * 1.05;
        let fg_cam_y = cam_y;
        for tuft in &self.grass_tufts {
            let sx = tuft.x - fg_cam_x;
            if sx < -10.0 || sx > SCREEN_W + 10.0 {
                continue;
            }
            let sy = tuft.y - fg_cam_y;
            let sway = (self.frame as f32 * 0.03 + tuft.sway_offset).sin() * 1.5;
            let green = Color::new(0.12, 0.35, 0.08, 0.45);
            // Thin grass blade
            draw_line(sx, sy, sx + sway, sy - tuft.h, 1.0, green);
            // Second blade slightly offset
            draw_line(sx + 2.0, sy, sx + 2.0 + sway * 0.7, sy - tuft.h * 0.7, 1.0,
                Color::new(0.15, 0.4, 0.1, 0.35));
        }
    }

    fn draw_pause_overlay(&self) {
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.0, 0.0, 0.0, 0.6));
        let txt = "PAUSED";
        let size = 40.0;
        let w = txt.len() as f32 * size * 0.45;
        draw_text(txt, SCREEN_W * 0.5 - w * 0.5, SCREEN_H * 0.45, size, WHITE);

        let hint = "Press ENTER to resume";
        let hs = 18.0;
        let hw = hint.len() as f32 * hs * 0.42;
        draw_text(hint, SCREEN_W * 0.5 - hw * 0.5, SCREEN_H * 0.55, hs, GRAY);
    }

    fn draw_game_over(&self) {
        // Dark background
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.05, 0.0, 0.0, 1.0));

        let txt = "GAME OVER";
        let size = 48.0;
        let w = txt.len() as f32 * size * 0.45;
        draw_text(txt, SCREEN_W * 0.5 - w * 0.5, SCREEN_H * 0.35, size, RED);

        let score = format!("Final Score: {:08}", self.player.score);
        let ss = 22.0;
        let sw = score.len() as f32 * ss * 0.42;
        draw_text(&score, SCREEN_W * 0.5 - sw * 0.5, SCREEN_H * 0.5, ss, YELLOW);

        if self.blink_timer < 40 {
            let hint = "PRESS START TO CONTINUE";
            let hs = 20.0;
            let hw = hint.len() as f32 * hs * 0.42;
            draw_text(hint, SCREEN_W * 0.5 - hw * 0.5, SCREEN_H * 0.65, hs, WHITE);
        }
    }

    fn draw_victory(&self) {
        // Dark crimson gradient background
        for y in 0..SCREEN_H as i32 {
            let t = y as f32 / SCREEN_H;
            draw_rectangle(0.0, y as f32, SCREEN_W, 1.0, Color::new(0.02 + t * 0.1, 0.0, 0.0, 1.0));
        }

        // Title
        let txt = "THE CRIMSON OATH";
        let size = 36.0;
        let w = txt.len() as f32 * size * 0.45;
        draw_text(txt, SCREEN_W * 0.5 - w * 0.5, 60.0, size, Color::new(0.9, 0.27, 0.27, 1.0));

        let sub = "COMPLETE";
        let sub_size = 22.0;
        let sw = sub.len() as f32 * sub_size * 0.45;
        draw_text(sub, SCREEN_W * 0.5 - sw * 0.5, 90.0, sub_size, Color::new(1.0, 0.67, 0.0, 1.0));

        // Ending text
        let end_lines = [
            "The three trials are finished.",
            "The truth stands revealed.",
            "",
            "Sensei Takeshi saved the Shadow Lotus",
            "by slaying the Shogun who ordered",
            "the clan's destruction.",
            "",
            "He framed his finest student to shield",
            "the clan from retribution.",
            "",
            "Now Kaede carries the Crimson Blade",
            "and leads the Shadow Lotus forward.",
            "",
            "The oath is fulfilled.",
        ];
        let line_size = 14.0;
        for (i, line) in end_lines.iter().enumerate() {
            let lw = line.len() as f32 * line_size * 0.38;
            draw_text(line, SCREEN_W * 0.5 - lw * 0.5, 130.0 + i as f32 * 16.0, line_size, Color::new(0.8, 0.8, 0.8, 1.0));
        }

        // Score
        let score = format!("FINAL SCORE: {:08}", self.player.score);
        let ss = 18.0;
        let score_w = score.len() as f32 * ss * 0.42;
        draw_text(&score, SCREEN_W * 0.5 - score_w * 0.5, SCREEN_H - 80.0, ss, YELLOW);

        if self.blink_timer < 40 {
            let hint = "PRESS START";
            let hs = 22.0;
            let hw = hint.len() as f32 * hs * 0.42;
            draw_text(hint, SCREEN_W * 0.5 - hw * 0.5, SCREEN_H - 40.0, hs, WHITE);
        }
    }

    fn draw_story(&self) {
        // Dark background
        for y in 0..SCREEN_H as i32 {
            let t = y as f32 / SCREEN_H;
            draw_rectangle(0.0, y as f32, SCREEN_W, 1.0, Color::new(0.02, 0.0, 0.03 + t * 0.05, 1.0));
        }

        // Decorative border
        draw_rectangle_lines(30.0, 30.0, SCREEN_W - 60.0, SCREEN_H - 60.0, 2.0, Color::new(0.5, 0.0, 0.0, 1.0));
        draw_rectangle_lines(34.0, 34.0, SCREEN_W - 68.0, SCREEN_H - 68.0, 2.0, Color::new(0.25, 0.0, 0.0, 1.0));

        // Draw typewriter text
        let lines: Vec<&str> = self.story_displayed.split('\n').collect();
        let start_y = 80.0;
        let line_h = 18.0;
        let text_size = 14.0;
        for (i, line) in lines.iter().enumerate() {
            let color = if line.starts_with("---") {
                Color::new(0.9, 0.27, 0.27, 1.0)
            } else if line.starts_with("TAKESHI:") || line.starts_with("'") {
                Color::new(1.0, 0.67, 0.0, 1.0)
            } else {
                Color::new(0.8, 0.8, 0.8, 1.0)
            };
            draw_text(line, 60.0, start_y + i as f32 * line_h, text_size, color);
        }

        // Prompt
        if self.story_skip_ready && (self.frame / 30) % 2 == 0 {
            let prompt = "PRESS Z / ENTER TO CONTINUE";
            let ps = 14.0;
            let pw = prompt.len() as f32 * ps * 0.38;
            draw_text(prompt, SCREEN_W * 0.5 - pw * 0.5, SCREEN_H - 50.0, ps, YELLOW);
        } else if !self.story_skip_ready {
            let prompt = "PRESS Z / ENTER TO SKIP";
            let ps = 12.0;
            let pw = prompt.len() as f32 * ps * 0.38;
            draw_text(prompt, SCREEN_W * 0.5 - pw * 0.5, SCREEN_H - 50.0, ps, Color::new(0.4, 0.4, 0.4, 1.0));
        }
    }

    fn draw_env_signs(&self, cam_x: f32) {
        if self.phase != GamePhase::Playing {
            return;
        }
        for sign in &self.env_signs {
            let sign_world_x = sign.tx as f32 * TILE;
            let dist = (self.player.x - sign_world_x).abs();
            if dist < 120.0 {
                let alpha = (1.0 - dist / 120.0).max(0.0) * 0.7;
                let pulse = 0.7 + (self.frame as f32 * 0.04).sin() * 0.3;
                let a = alpha * pulse;
                let color = match self.current_level {
                    0 => Color::new(0.27, 0.67, 0.5, a),
                    1 => Color::new(0.67, 0.4, 0.4, a),
                    _ => Color::new(0.8, 0.27, 0.27, a),
                };
                let sx = sign_world_x - cam_x;
                let sy = 70.0 + (self.frame as f32 * 0.03).sin() * 4.0;
                let text_size = 12.0;
                let tw = sign.text.len() as f32 * text_size * 0.38;
                draw_text(sign.text, sx - tw * 0.5, sy, text_size, color);
            }
        }
    }
}

// ── Main ───────────────────────────────────────────────────────────────────

fn window_conf() -> Conf {
    Conf {
        window_title: "Shadow Blade".to_owned(),
        window_width: SCREEN_W as i32,
        window_height: SCREEN_H as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
