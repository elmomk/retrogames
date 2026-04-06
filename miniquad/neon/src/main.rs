// Neon Runner - Cyberpunk Platformer for Miyoo Mini Plus
// Rust/Macroquad port -- 800x600, 60fps fixed timestep
// Story: "Ghost Protocol" -- Kira-7 runs across Neo-Kyoto rooftops
// Synced with web version

use macroquad::prelude::*;

// -- Constants ----------------------------------------------------------------

const SCREEN_W: f32 = 800.0;
const SCREEN_H: f32 = 600.0;
const TIME_STEP: f64 = 1.0 / 60.0;
const TILE: f32 = 20.0;
const GRAVITY: f32 = 0.45;
const MAX_FALL: f32 = 8.0;
const JUMP_FORCE: f32 = -8.0;
const MOVE_SPEED: f32 = 3.5;
const WALL_SLIDE_SPEED: f32 = 1.5;
const WALL_JUMP_Y: f32 = -7.0;
const WALL_JUMP_X: f32 = 5.0;
const DASH_SPEED: f32 = 8.0;
const DASH_FRAMES: i32 = 8;
const DASH_COOLDOWN: i32 = 30;
const COYOTE_MAX: i32 = 6;
const JUMP_BUFFER_MAX: i32 = 6;
const INVULN_FRAMES: i32 = 60;
const SLASH_RANGE: f32 = 24.0;
const SLASH_CD: i32 = 12;
const EMP_RADIUS: f32 = 60.0;
const EMP_STUN: i32 = 120;
const HACK_RANGE: f32 = 30.0;
const HACK_DUR: i32 = 300;
const MAX_PARTICLES: usize = 400;
#[allow(dead_code)]
const MAX_PROJECTILES: usize = 30;

// -- Palette ------------------------------------------------------------------

fn palette(ch: char) -> Option<Color> {
    match ch {
        'K' => Some(Color::new(0.067, 0.067, 0.067, 1.0)),   // near-black
        'D' => Some(Color::new(0.15, 0.15, 0.2, 1.0)),       // dark gray-blue
        'G' => Some(Color::new(0.333, 0.333, 0.4, 1.0)),     // gray
        'W' => Some(WHITE),
        'M' => Some(Color::new(1.0, 0.0, 0.5, 1.0)),         // magenta/neon pink
        'C' => Some(Color::new(0.0, 0.83, 1.0, 1.0)),        // cyan
        'N' => Some(Color::new(0.0, 1.0, 0.255, 1.0)),       // neon green
        'A' => Some(Color::new(1.0, 0.72, 0.0, 1.0)),        // amber
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

// -- Sprite Data (matching web 16x16 player, 12x12 drone, etc.) ---------------

fn player_idle_sprite() -> Vec<&'static str> {
    vec![
        "....CCCCCCCC....",
        "...CDDDDDDDC...",
        "..CDDMMMMDDCC..",
        "..CDDMMMMDDCC..",
        "..CDDDDDDDDCC..",
        "...CWWC..WWCM..",
        "....CWWWWWWCM...",
        "...KKKKKKKKKK...",
        "..KKKKMMMMKKKK..",
        "..KKMMMMMMMKK...",
        ".KKKMMMMMMMMKK..",
        "..KKKKMMMMMKKK..",
        "...KKKKKKKKKKK..",
        "....CC...CC.....",
        "...CCC...CCC....",
        "...NNN...NNN....",
    ]
}

fn player_run_sprite() -> Vec<&'static str> {
    vec![
        "....CCCCCCCC....",
        "...CDDDDDDDC...",
        "..CDDMMMMDDCC..",
        "..CDDMMMMDDCC..",
        "..CDDDDDDDDCC..",
        "...CWWC..WWCM..",
        "....CWWWWWWCM...",
        "...KKKKKKKKKK...",
        "..KKKKMMMMKKKK..",
        "..KKMMMMMMMKK...",
        ".KKKMMMMMMMMKK..",
        "..KKKKMMMMMKKK..",
        "...KKKKKKKKKKK..",
        "...CC....CC.....",
        "..CCC..CCC......",
        "..NNN..NNN......",
    ]
}

fn player_jump_sprite() -> Vec<&'static str> {
    vec![
        "....CCCCCCCC....",
        "...CDDDDDDDC...",
        "..CDDMMMMDDCC..",
        "..CDDMMMMDDCC..",
        "..CDDDDDDDDCC..",
        "...CWWC..WWCM..",
        "....CWWWWWWCM...",
        "...KKKKKKKKKK...",
        "..KKKKMMMMKKKK..",
        "..KKMMMMMMMKK...",
        ".KKKMMMMMMMMKK..",
        "..KKKKMMMMMKKK..",
        "...KKKKKKKKKKK..",
        "..CCC.....CCC...",
        ".CCC.......CCC..",
        ".NNN.......NNN..",
    ]
}

fn player_attack_sprite() -> Vec<&'static str> {
    vec![
        "....CCCCCCCC....",
        "...CDDDDDDDC...",
        "..CDDMMMMDDCC..",
        "..CDDMMMMDDCC..",
        "..CDDDDDDDDCCCC",
        "...CWWC..WWCCCC",
        "....CWWWWWWCCC..",
        "...KKKKKKKKKK...",
        "..KKKKMMMMKKKK..",
        "..KKMMMMMMMKK...",
        ".KKKMMMMMMMMKK..",
        "..KKKKMMMMMKKK..",
        "...KKKKKKKKKKK..",
        "....CC...CC.....",
        "...CCC...CCC....",
        "...NNN...NNN....",
    ]
}

fn drone_sprite() -> Vec<&'static str> {
    vec![
        "....CCCC....",
        "..CCDDDDCC..",
        ".CDDMMMMDC..",
        "CDDMWWWMDD.C",
        "CDDMWWWMDC.C",
        "CDDMMMMMDDC.",
        ".CDDMMMMDC..",
        "..CCDDDDCC..",
        "....CCCC....",
        "..C......C..",
        ".C........C.",
        "C..........C",
    ]
}

fn guard_sprite() -> Vec<&'static str> {
    vec![
        "...KKKKKK...",
        "..KSSSSSSSK.",
        "..KSSMMSSK..",
        "..KSSMMSSK..",
        "..KSSSSSSSK.",
        "...KKKKKK...",
        ".KGGGGGGGGK.",
        "KGGGGGGGGGGK",
        "KGGGRRRRGGKK",
        "KGGGRRRRGGKK",
        ".KGGGGGGGGK.",
        "..KGGGGGGK..",
        "...KK..KK...",
        "..KKK..KKK..",
        "..KKK..KKK..",
        "..SSS..SSS..",
    ]
}

fn turret_sprite() -> Vec<&'static str> {
    vec![
        "....SSSS....",
        "...SSSSSS...",
        "..SSRRRRRSS.",
        ".SSRRRAAASS.",
        "SSRRRAAARRSS",
        "SSRRRAAARRSS",
        ".SSRRRAAASS.",
        "..SSRRRRRSS.",
        "...SSSSSS...",
        "....SSSS....",
        "..SSSSSSSS..",
        ".SSSSSSSSSS.",
    ]
}

fn boss_sprite() -> Vec<&'static str> {
    vec![
        ".......KKKKK............",
        "......KPPPPPPK..........",
        ".....KPPMMMPPPK.........",
        "....KPPMWWMPPKKKKKKK....",
        "...KPPMWWWMPKPPPPPPK....",
        "..KPPMWWWWMKPPPPPPPPK...",
        ".KPPMWWWWMPKPPPPPPPPPK..",
        "KPPMWWWWMPKPPPPPPPPPK...",
        ".KPPMWWWMPKPPPPPPPPK....",
        "..KPPMWWMPKPPPPPPPK.....",
        "...KPPPPPK.KPPPPPK......",
        "....KKKK...KK..KK.......",
        "..........KKK..KKK......",
        "..........SSS..SSS......",
    ]
}

fn slash_sprite() -> Vec<&'static str> {
    vec![
        "............",
        ".........CC.",
        "........CWC.",
        ".......CWWC.",
        "......CWWWC.",
        ".....CWWWWC.",
        "....CWWWWWC.",
        "...CWWWWWWC.",
        "..CWWWWWWWC.",
        ".CWWWWWWWWC.",
        "CWWWWWWWWWC.",
        "............",
    ]
}

fn chip_sprite() -> Vec<&'static str> {
    vec![
        "..AAAA..",
        ".AYYAA..",
        "AYYYYYA.",
        "AYWWYYA.",
        "AYWWYYA.",
        "AYYYYYA.",
        ".AYYAA..",
        "..AAAA..",
    ]
}

fn health_sprite() -> Vec<&'static str> {
    vec![
        "...RR...",
        "...RR...",
        ".RRRRRR.",
        ".RMMMMR.",
        ".RMMMMR.",
        ".RRRRRR.",
        "...RR...",
        "...RR...",
    ]
}

fn emp_ammo_sprite() -> Vec<&'static str> {
    vec![
        "..BBBB..",
        ".BCCCB..",
        "BCCCCB..",
        "BCWWCB..",
        "BCWWCB..",
        "BCCCCB..",
        ".BCCCB..",
        "..BBBB..",
    ]
}

fn terminal_sprite() -> Vec<&'static str> {
    vec![
        "KKKKKKKK",
        "KNNNNNNK",
        "KNMMMMNK",
        "KNMWMNK.",
        "KNMMMMNK",
        "KNNNNNNK",
        "KKKKKKKK",
        ".KKKKKK.",
    ]
}

fn exit_sprite() -> Vec<&'static str> {
    vec![
        "..NNNN..",
        ".NCCCN..",
        "NCCWWCN.",
        "NCWWWCN.",
        "NCWWWCN.",
        "NCCWWCN.",
        ".NCCCN..",
        "..NNNN..",
    ]
}

// -- Data Structures ----------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Start,
    IntroStory,
    LevelStory,
    WinStory,
    Playing,
    GameOver,
    Win,
}

#[derive(Clone, Copy, PartialEq)]
enum EnemyType {
    Drone,
    Guard,
    Turret,
}

#[derive(Clone, Copy, PartialEq)]
enum PickupKind {
    Chip,
    Health,
    Emp,
}

struct Player {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    facing: f32,
    hp: i32,
    emps: i32,
    on_ground: bool,
    wall_slide: i32,
    coyote: i32,
    jump_buf: i32,
    dashing: i32,
    dash_cd: i32,
    iframes: i32,
    slashing: i32,
    slash_cd: i32,
    anim_frame: i32,
    anim_timer: i32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 0.0, y: 0.0,
            vx: 0.0, vy: 0.0,
            facing: 1.0,
            hp: 3,
            emps: 3,
            on_ground: false,
            wall_slide: 0,
            coyote: 0,
            jump_buf: 0,
            dashing: 0,
            dash_cd: 0,
            iframes: 0,
            slashing: 0,
            slash_cd: 0,
            anim_frame: 0,
            anim_timer: 0,
        }
    }
}

const PW: f32 = 14.0;
const PH: f32 = 16.0;

struct Enemy {
    alive: bool,
    etype: EnemyType,
    x: f32,
    y: f32,
    start_x: f32,
    start_y: f32,
    vx: f32,
    hp: i32,
    timer: i32,
    stun: i32,
    dir: f32,
    hacked: bool,
    hack_timer: i32,
    angle: f32,
}

struct Boss {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    hp: i32,
    max_hp: i32,
    state: BossState,
    timer: i32,
    dir: f32,
    enraged: bool,
    alive: bool,
    stun: i32,
    flash_timer: i32,
}

#[derive(Clone, Copy, PartialEq)]
enum BossState {
    Patrol,
    Charge,
    Leap,
    Stunned,
}

struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: i32,
}

struct Pickup {
    x: f32,
    y: f32,
    kind: PickupKind,
    alive: bool,
}

struct Terminal {
    x: f32,
    y: f32,
    hacked: bool,
    hack_timer: i32,
}

struct Laser {
    x: f32,
    y: f32,
    active: bool,
    timer: i32,
}

struct FallingPlat {
    x: f32,
    y: f32,
    orig_y: f32,
    state: FPState,
    timer: i32,
}

#[derive(Clone, Copy, PartialEq)]
enum FPState {
    Solid,
    Shaking,
    Falling,
    Respawning,
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: i32,
    max_life: i32,
    color: Color,
    size: f32,
}

struct Afterimage {
    x: f32,
    y: f32,
    alpha: f32,
    facing: f32,
}

struct RainDrop {
    x: f32,
    y: f32,
    speed: f32,
    len: f32,
    alpha: f32,
}

// -- Colors -------------------------------------------------------------------

const COL_MAGENTA: Color = Color::new(1.0, 0.0, 0.502, 1.0);
const COL_BLUE: Color = Color::new(0.0, 0.831, 1.0, 1.0);
const COL_GREEN: Color = Color::new(0.0, 1.0, 0.255, 1.0);
const COL_AMBER: Color = Color::new(1.0, 0.722, 0.0, 1.0);
const COL_DARK: Color = Color::new(0.051, 0.051, 0.102, 1.0);

// -- Story Data (matching web version) ----------------------------------------

fn story_intro() -> Vec<String> {
    vec![
        "> SYSTEM BREACH DETECTED".into(),
        "> OPERATOR: KIRA-7".into(),
        "".into(),
        "You did it. You cracked NEXUS-Corp's".into(),
        "mainframe and stole everything.".into(),
        "".into(),
        "Project MINDGATE -- a neural control".into(),
        "network embedded in every citizen's".into(),
        "cybernetic implant. Total obedience.".into(),
        "".into(),
        "But they traced you. Kill order issued.".into(),
        "Every corpo enforcer in Neo-Kyoto".into(),
        "is hunting you now.".into(),
        "".into(),
        "There's one chance: reach the underground".into(),
        "broadcast tower and send the data out.".into(),
        "The truth must be heard.".into(),
        "".into(),
        "> OBJECTIVE: REACH THE BROADCAST TOWER".into(),
        "> STATUS: RUNNING...".into(),
    ]
}

fn story_after_level(idx: usize) -> Vec<String> {
    match idx {
        0 => vec![
            "> ROOFTOPS CLEARED".into(),
            "> ENTERING CORPO TOWER".into(),
            "".into(),
            "The corpo tower looms above you,".into(),
            "its laser grid defenses humming.".into(),
            "".into(),
            "Inside is the encryption key that".into(),
            "will decode the MINDGATE files.".into(),
            "Without it, the broadcast is gibberish.".into(),
            "".into(),
            "Security is tight. Turrets, drones,".into(),
            "and enough lasers to slice you in half.".into(),
            "".into(),
            "You flex your cybernetic legs.".into(),
            "Time to hack their own systems".into(),
            "against them.".into(),
            "".into(),
            "> WARNING: HEAVY SECURITY AHEAD".into(),
        ],
        1 => vec![
            "> ENCRYPTION KEY ACQUIRED".into(),
            "> DESCENDING TO UNDERGROUND".into(),
            "".into(),
            "The key burns in your neural cache.".into(),
            "You can feel NEXUS-Corp's AI hunting".into(),
            "you through the network.".into(),
            "".into(),
            "Below the city, through the old sewers,".into(),
            "lies the pirate broadcast tower.".into(),
            "Built by the resistance. Forgotten.".into(),
            "".into(),
            "But they've sent their enforcer --".into(),
            "a CY-HOUND unit. Military grade.".into(),
            "It's between you and the tower.".into(),
            "".into(),
            "> FINAL OBJECTIVE: BROADCAST THE TRUTH".into(),
            "> SURVIVE.".into(),
        ],
        _ => vec![],
    }
}

fn story_victory() -> Vec<String> {
    vec![
        "> BROADCAST INITIATED...".into(),
        "> SIGNAL STRENGTH: MAXIMUM".into(),
        "> RECIPIENT: EVERYONE".into(),
        "".into(),
        "The tower hums to life.".into(),
        "Data streams across every screen,".into(),
        "every implant, every network node".into(),
        "in Neo-Kyoto.".into(),
        "".into(),
        "They see it all. MINDGATE exposed.".into(),
        "The neural shackles dissolving".into(),
        "as citizens wake up for the first time.".into(),
        "".into(),
        "NEXUS-Corp's stock crashes.".into(),
        "Their board members flee.".into(),
        "The corpo-police stand down.".into(),
        "".into(),
        "You lean against the tower,".into(),
        "rain washing the blood from your coat.".into(),
        "".into(),
        "The fight isn't over.".into(),
        "But tonight, the truth runs free.".into(),
        "".into(),
        "> CONNECTION: LIBERATED".into(),
    ]
}

const LEVEL_NAMES: [&str; 3] = ["ROOFTOP CHASE", "CORPO TOWER", "THE UNDERGROUND"];

// -- Level Maps (matching web version) ----------------------------------------
// P=player start, X=exit, #=solid, D=drone, G=guard, T=turret
// L=laser, E=electric, C=chip, H=health, M=emp, K=terminal, F=falling, A=acid

static LEVEL_1: &[&str] = &[
    "........................................................................................................................",
    "........................................................................................................................",
    "........................................................................................................................",
    "........................................................................................................................",
    "...........................................................C............................................................",
    "..........................................................###...........C................................................",
    "..C...............................C..............C.....C........####....###...............C.......C....................X...",
    ".###.........C.......C..........###.....C.......###...###..D..........#....#............###.....###.........C.........###.",
    "......D....###......###..............D.###...........#....#..........#......#..........#...#...#...#......###..D..........",
    "...............................D.................D..#......#........#........#........#.....#.#.....#....................#.",
    "..........................................................#........#........#........#.....#.#.....#...............G...#.",
    "##..........F..F..F............G.......F..F..F........G...#........#........#...G....#.....#.#.....#....F..F..F.....####.",
    "..##.......................####.......................####..#...G....#........#..####..#.....###.....#.................#...",
    "....#..G.................#............................................................................................#..",
    ".....####.........G....#...............G..............................................................................#..",
    "..........###########.#.............####.................C...C...C...............................G....................#...",
    "....................................................................................#########.......................####..",
    ".P.................................................................................#.........#..G....................#....",
    "####...............................................................................................################.....",
    "........................................................................................................................",
    "........................................................................................................................",
    "........................................................................................................................",
    "........................................................................................................................",
    "########################################################################################################..########..###",
];

static LEVEL_2: &[&str] = &[
    "........................................................................................................................",
    "........................................................................................................................",
    "..........................................................C.....C.......................................................",
    "..C......................................................##...##........................................................",
    "..##.....................................................#.....#.................C.......................................",
    ".......C.....................................K............#..D..#................###....C...........C.....................",
    "......###.............C.....C...........#######...........#.....#..........C..........###.........###.........X..........",
    "...........C.......L###L...###.....D..........#......D...#.....#.........###..................D.........C...####........",
    "..........###......L...L..........##..........#....##....#.....#.....................G.................###..#....#.......",
    "..P...............LL...LL..........#..........#.........#..G...#..........F..F..F.............F..F.........#....#.......",
    "######...........L.L...L.L.........#..G.......#........#......#.............................##....###.....#.G..#.......",
    "......###.......LL.L...L.LL........########...#.......#.......#........G...............G..#......T..#....######........",
    "..........##...L...L.K.L...L..............T...#......#........#..........####......####..#..........#...................",
    "..............LL...LLLLL...LL.................#.....#..........#....................#....#............#...................",
    "...............L...........L.............C...#....#............#..........C...C...#......#..........#....................",
    "...............L...........L............###.#...#..............####.....###.###..#........########.#.....................",
    "...............LLLLLLLLLLLLL...............##..#.........K..........D.............#.............T.#......................",
    "...............................................#########################..........################.......................",
    "........................................................................................................................",
    "........................................................................................................................",
    "........................................................................................................................",
    "........................................................................................................................",
    "........................................................................................................................",
    "###################################################################################################..#################",
];

static LEVEL_3: &[&str] = &[
    "........................................................................................................................",
    "........................................................................................................................",
    "........................................................................................................................",
    "........................................................................................................................",
    "........................................................................................................................",
    "...C.....C.........................................................................C...C................................",
    "..###...###........M..............................C...C..........................D..###.###...............................",
    "...........#.....####.........C.....C...........D.##.##.......C.....C..............................................X....",
    ".P..........#...........D...###...###............#...#.......###...###.......G..........K.......C....C..............####.",
    "####.........#.......G........#...#.............#.....#........#...#.......####.....#######....###..###.....G...........",
    "....##........##...####..E.E..#.K.#......G....#.......#...E.E.#...#...........#...........#.........H.......####.......",
    "......#.........#.#....#.EEE..#...#....####..#.........#..EEE.#...#.M..........#.........#..........##..............#..",
    ".......#..H.....#.#....#.EEE.##...##.......#...........#..EEE.##..##............#.......#............#.............#...",
    "........########..#....#.EEE.#.....#......#.............#.EEE.#....#.............#.....#.............#............#....",
    "...................#....#.....#.....#.....#...............#.....#....#..............#...#..............#...........#.....",
    "...................######.....#.....#....#.................#....#....#...............###...............#..........#......",
    "............................##.......##.#...................#...#.....##...............................#.........#.......",
    "...........................#...........#....................#...#.......#..............................#........#........",
    "..........................#...............................................................G..........#.......#.........",
    "..........................#.............................................................................#......#..........",
    "..........................#.............................................................................#.....#..........",
    "..........................#############################################################################....############.",
    "........................................................................................................................",
    "########################################################################################################################",
];

// -- Collision ----------------------------------------------------------------

fn tile_solid(map: &[Vec<u8>], map_w: usize, map_h: usize, px: f32, py: f32) -> bool {
    let tx = (px / TILE) as i32;
    let ty = (py / TILE) as i32;
    if tx < 0 || ty < 0 || tx >= map_w as i32 || ty >= map_h as i32 {
        return true; // OOB = solid (matching web)
    }
    map[ty as usize][tx as usize] == 1
}

#[allow(dead_code)]
fn tile_at(map: &[Vec<u8>], map_w: usize, map_h: usize, px: f32, py: f32) -> u8 {
    let tx = (px / TILE) as i32;
    let ty = (py / TILE) as i32;
    if tx < 0 || ty < 0 || tx >= map_w as i32 || ty >= map_h as i32 {
        return 1;
    }
    map[ty as usize][tx as usize]
}

fn solid_at_with_fp(map: &[Vec<u8>], map_w: usize, map_h: usize, fps: &[FallingPlat], px: f32, py: f32) -> bool {
    if tile_solid(map, map_w, map_h, px, py) { return true; }
    for fp in fps {
        if fp.state == FPState::Solid || fp.state == FPState::Shaking {
            if px >= fp.x && px < fp.x + TILE && py >= fp.y && py < fp.y + TILE {
                return true;
            }
        }
    }
    false
}

fn rect_overlap(ax: f32, ay: f32, aw: f32, ah: f32, bx: f32, by: f32, bw: f32, bh: f32) -> bool {
    ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
}

// -- Particle Spawners --------------------------------------------------------

fn spawn_particles(particles: &mut Vec<Particle>, x: f32, y: f32, color: Color, count: usize, speed: f32, life: i32) {
    for _ in 0..count {
        let a: f32 = rand::gen_range(0.0, std::f32::consts::TAU);
        let s: f32 = rand::gen_range(0.0, speed);
        let p = Particle {
            x, y,
            vx: a.cos() * s,
            vy: a.sin() * s - 1.0,
            life,
            max_life: life,
            color,
            size: 1.0 + rand::gen_range(0.0_f32, 2.0),
        };
        if particles.len() < MAX_PARTICLES {
            particles.push(p);
        }
    }
}

fn spawn_sparks(particles: &mut Vec<Particle>, x: f32, y: f32) {
    for _ in 0..3 {
        let p = Particle {
            x, y,
            vx: (rand::gen_range(0.0_f32, 1.0) - 0.5) * 3.0,
            vy: -rand::gen_range(0.0_f32, 3.0),
            life: 10,
            max_life: 10,
            color: COL_AMBER,
            size: 1.0,
        };
        if particles.len() < MAX_PARTICLES {
            particles.push(p);
        }
    }
}

// -- Game Struct ---------------------------------------------------------------

struct Game {
    state: GameState,
    player: Player,
    score: i32,
    lives: i32,
    current_level: usize,
    global_frame: i64,

    map: Vec<Vec<u8>>,
    map_w: usize,
    map_h: usize,

    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    particles: Vec<Particle>,
    pickups: Vec<Pickup>,
    terminals: Vec<Terminal>,
    lasers: Vec<Laser>,
    falling_plats: Vec<FallingPlat>,
    afterimages: Vec<Afterimage>,
    raindrops: Vec<RainDrop>,

    boss: Option<Boss>,

    exit_x: f32,
    exit_y: f32,
    cam_x: f32,
    cam_y: f32,
    shake_mag: f32,
    shake_x: f32,
    shake_y: f32,
    glitch_timer: i32,
    damage_flash: i32,

    // Story
    story_queue: Vec<String>,
    story_line_idx: usize,
    story_char_idx: usize,
    story_frame: i32,
    story_revealed: String,

    // Input edge detection
    prev_jump: bool,
    prev_attack: bool,
    prev_dash: bool,
    prev_emp: bool,
    prev_any: bool,
    jp: bool,
    ap: bool,
    dp: bool,
    ep: bool,
    anyp: bool,

    // Textures
    tex_player_idle: Texture2D,
    tex_player_run: Texture2D,
    #[allow(dead_code)]
    tex_player_jump: Texture2D,
    #[allow(dead_code)]
    tex_player_attack: Texture2D,
    tex_drone: Texture2D,
    tex_guard: Texture2D,
    tex_turret: Texture2D,
    tex_boss: Texture2D,
    tex_slash: Texture2D,
    tex_chip: Texture2D,
    tex_health: Texture2D,
    tex_emp_ammo: Texture2D,
    tex_terminal: Texture2D,
    tex_exit: Texture2D,
}

impl Game {
    fn new() -> Self {
        let rain: Vec<RainDrop> = (0..150).map(|_| RainDrop {
            x: rand::gen_range(0.0_f32, SCREEN_W * 3.0),
            y: rand::gen_range(0.0_f32, SCREEN_H),
            speed: 3.0 + rand::gen_range(0.0_f32, 5.0),
            len: 4.0 + rand::gen_range(0.0_f32, 8.0),
            alpha: 0.1 + rand::gen_range(0.0_f32, 0.2),
        }).collect();

        let mut g = Self {
            state: GameState::Start,
            player: Player::new(),
            score: 0,
            lives: 3,
            current_level: 0,
            global_frame: 0,
            map: Vec::new(),
            map_w: 0,
            map_h: 0,
            enemies: Vec::new(),
            bullets: Vec::new(),
            particles: Vec::with_capacity(MAX_PARTICLES),
            pickups: Vec::new(),
            terminals: Vec::new(),
            lasers: Vec::new(),
            falling_plats: Vec::new(),
            afterimages: Vec::new(),
            raindrops: rain,
            boss: None,
            exit_x: 0.0,
            exit_y: 0.0,
            cam_x: 0.0,
            cam_y: 0.0,
            shake_mag: 0.0,
            shake_x: 0.0,
            shake_y: 0.0,
            glitch_timer: 0,
            damage_flash: 0,
            story_queue: Vec::new(),
            story_line_idx: 0,
            story_char_idx: 0,
            story_frame: 0,
            story_revealed: String::new(),
            prev_jump: false,
            prev_attack: false,
            prev_dash: false,
            prev_emp: false,
            prev_any: false,
            jp: false,
            ap: false,
            dp: false,
            ep: false,
            anyp: false,
            tex_player_idle: sprite_to_texture(&player_idle_sprite(), 16, 16),
            tex_player_run: sprite_to_texture(&player_run_sprite(), 16, 16),
            tex_player_jump: sprite_to_texture(&player_jump_sprite(), 16, 16),
            tex_player_attack: sprite_to_texture(&player_attack_sprite(), 16, 16),
            tex_drone: sprite_to_texture(&drone_sprite(), 12, 12),
            tex_guard: sprite_to_texture(&guard_sprite(), 12, 16),
            tex_turret: sprite_to_texture(&turret_sprite(), 12, 12),
            tex_boss: sprite_to_texture(&boss_sprite(), 24, 14),
            tex_slash: sprite_to_texture(&slash_sprite(), 12, 12),
            tex_chip: sprite_to_texture(&chip_sprite(), 8, 8),
            tex_health: sprite_to_texture(&health_sprite(), 8, 8),
            tex_emp_ammo: sprite_to_texture(&emp_ammo_sprite(), 8, 8),
            tex_terminal: sprite_to_texture(&terminal_sprite(), 8, 8),
            tex_exit: sprite_to_texture(&exit_sprite(), 8, 8),
        };
        g.load_level(0);
        g
    }

    fn input_jump(&self) -> bool {
        is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) || is_key_down(KeyCode::Space)
    }
    fn input_attack(&self) -> bool {
        is_key_down(KeyCode::X)
    }
    fn input_dash(&self) -> bool {
        is_key_down(KeyCode::Z) || is_key_down(KeyCode::LeftShift)
    }
    fn input_emp(&self) -> bool {
        is_key_down(KeyCode::C)
    }
    fn input_left(&self) -> bool {
        is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)
    }
    fn input_right(&self) -> bool {
        is_key_down(KeyCode::Right) || is_key_down(KeyCode::D)
    }
    fn input_any(&self) -> bool {
        is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::Z) || is_key_pressed(KeyCode::Up)
    }

    fn update_input_edges(&mut self) {
        let j = self.input_jump();
        let a = self.input_attack();
        let d = self.input_dash();
        let e = self.input_emp();
        let any = self.input_any();
        self.jp = j && !self.prev_jump;
        self.ap = a && !self.prev_attack;
        self.dp = d && !self.prev_dash;
        self.ep = e && !self.prev_emp;
        self.anyp = any && !self.prev_any;
        self.prev_jump = j;
        self.prev_attack = a;
        self.prev_dash = d;
        self.prev_emp = e;
        self.prev_any = any;
    }

    fn trigger_shake(&mut self, mag: f32) {
        if mag > self.shake_mag {
            self.shake_mag = mag;
        }
    }

    fn level_data(idx: usize) -> &'static [&'static str] {
        match idx {
            0 => LEVEL_1,
            1 => LEVEL_2,
            _ => LEVEL_3,
        }
    }

    fn load_level(&mut self, idx: usize) {
        let map_data = Self::level_data(idx);
        self.enemies.clear();
        self.bullets.clear();
        self.particles.clear();
        self.pickups.clear();
        self.terminals.clear();
        self.lasers.clear();
        self.falling_plats.clear();
        self.afterimages.clear();
        self.boss = None;

        self.map_h = map_data.len();
        self.map_w = map_data[0].len();
        self.map = vec![vec![0u8; self.map_w]; self.map_h];

        for r in 0..self.map_h {
            let row_bytes = map_data[r].as_bytes();
            for c in 0..self.map_w.min(row_bytes.len()) {
                let ch = row_bytes[c] as char;
                match ch {
                    '#' => { self.map[r][c] = 1; }
                    'P' => {
                        self.player.x = c as f32 * TILE;
                        self.player.y = r as f32 * TILE;
                    }
                    'X' => {
                        self.exit_x = c as f32 * TILE;
                        self.exit_y = r as f32 * TILE;
                    }
                    'D' => {
                        self.enemies.push(Enemy {
                            alive: true, etype: EnemyType::Drone,
                            x: c as f32 * TILE, y: r as f32 * TILE,
                            start_x: c as f32 * TILE, start_y: r as f32 * TILE,
                            vx: 1.0, hp: 1, timer: 0, stun: 0, dir: 1.0,
                            hacked: false, hack_timer: 0, angle: 0.0,
                        });
                    }
                    'G' => {
                        self.enemies.push(Enemy {
                            alive: true, etype: EnemyType::Guard,
                            x: c as f32 * TILE, y: r as f32 * TILE,
                            start_x: c as f32 * TILE, start_y: r as f32 * TILE,
                            vx: 1.0, hp: 2, timer: 0, stun: 0, dir: 1.0,
                            hacked: false, hack_timer: 0, angle: 0.0,
                        });
                    }
                    'T' => {
                        self.enemies.push(Enemy {
                            alive: true, etype: EnemyType::Turret,
                            x: c as f32 * TILE, y: r as f32 * TILE,
                            start_x: c as f32 * TILE, start_y: r as f32 * TILE,
                            vx: 0.0, hp: 3, timer: 0, stun: 0, dir: 1.0,
                            hacked: false, hack_timer: 0, angle: 0.0,
                        });
                    }
                    'C' => {
                        self.pickups.push(Pickup {
                            x: c as f32 * TILE + 6.0,
                            y: r as f32 * TILE + 6.0,
                            kind: PickupKind::Chip,
                            alive: true,
                        });
                    }
                    'H' => {
                        self.pickups.push(Pickup {
                            x: c as f32 * TILE,
                            y: r as f32 * TILE,
                            kind: PickupKind::Health,
                            alive: true,
                        });
                    }
                    'M' => {
                        self.pickups.push(Pickup {
                            x: c as f32 * TILE,
                            y: r as f32 * TILE,
                            kind: PickupKind::Emp,
                            alive: true,
                        });
                    }
                    'K' => {
                        self.terminals.push(Terminal {
                            x: c as f32 * TILE,
                            y: r as f32 * TILE,
                            hacked: false,
                            hack_timer: 0,
                        });
                    }
                    'L' => {
                        self.lasers.push(Laser {
                            x: c as f32 * TILE,
                            y: r as f32 * TILE,
                            active: true,
                            timer: 0,
                        });
                    }
                    'E' => {
                        self.map[r][c] = 2; // electric floor
                    }
                    'A' => {
                        self.map[r][c] = 3; // acid pool
                    }
                    'F' => {
                        self.falling_plats.push(FallingPlat {
                            x: c as f32 * TILE,
                            y: r as f32 * TILE,
                            orig_y: r as f32 * TILE,
                            state: FPState::Solid,
                            timer: 0,
                        });
                    }
                    _ => {}
                }
            }
        }

        // Boss for level 3
        if idx == 2 {
            self.boss = Some(Boss {
                x: self.map_w as f32 * TILE * 0.7,
                y: (self.map_h as f32 - 5.0) * TILE,
                vx: 0.0, vy: 0.0,
                hp: 15, max_hp: 15,
                state: BossState::Patrol,
                timer: 0,
                dir: -1.0,
                enraged: false,
                alive: true,
                stun: 0,
                flash_timer: 0,
            });
        }

        // Reset player physics
        self.player.vx = 0.0;
        self.player.vy = 0.0;
        self.player.on_ground = false;
        self.player.wall_slide = 0;
        self.player.facing = 1.0;
        self.player.dashing = 0;
        self.player.dash_cd = 0;
        self.player.iframes = 0;
        self.player.slash_cd = 0;
        self.player.slashing = 0;
        self.player.coyote = 0;
        self.player.jump_buf = 0;
        self.player.anim_frame = 0;
        self.player.anim_timer = 0;

        self.cam_x = self.player.x - SCREEN_W / 2.0;
        self.cam_y = self.player.y - SCREEN_H / 2.0;
    }

    fn reset_player(&mut self) {
        self.player.hp = 3;
        self.player.emps = 3;
        self.player.vx = 0.0;
        self.player.vy = 0.0;
        self.player.dashing = 0;
        self.player.dash_cd = 0;
        self.player.iframes = 60;
        self.player.slash_cd = 0;
        self.player.slashing = 0;
    }

    fn init_game(&mut self) {
        self.score = 0;
        self.lives = 3;
        self.current_level = 0;
        self.player = Player::new();
        self.player.hp = 3;
        self.player.emps = 3;
        self.load_level(0);
        self.reset_player();
    }

    #[allow(dead_code)]
    fn hurt_player(&mut self) {
        if self.player.iframes > 0 || self.player.dashing > 0 { return; }
        self.player.hp -= 1;
        self.player.iframes = INVULN_FRAMES;
        self.trigger_shake(6.0);
        self.glitch_timer = 10;
        self.damage_flash = 8;
        spawn_particles(&mut self.particles, self.player.x + PW / 2.0, self.player.y + PH / 2.0, RED, 8, 3.0, 15);
        if self.player.hp <= 0 {
            self.lives -= 1;
            if self.lives <= 0 {
                self.state = GameState::GameOver;
            } else {
                self.load_level(self.current_level);
                self.reset_player();
            }
        }
    }

    fn complete_level(&mut self) {
        self.current_level += 1;
        if self.current_level >= 3 {
            self.story_queue = story_victory();
            self.story_char_idx = 0;
            self.story_line_idx = 0;
            self.story_frame = 0;
            self.story_revealed.clear();
            self.state = GameState::WinStory;
        } else {
            self.story_queue = story_after_level(self.current_level - 1);
            self.story_char_idx = 0;
            self.story_line_idx = 0;
            self.story_frame = 0;
            self.story_revealed.clear();
            self.state = GameState::LevelStory;
        }
    }

    // -- Update ---------------------------------------------------------------

    fn update(&mut self) {
        self.global_frame += 1;
        self.update_input_edges();

        match self.state {
            GameState::Start => {
                if self.anyp || self.jp {
                    self.story_queue = story_intro();
                    self.story_char_idx = 0;
                    self.story_line_idx = 0;
                    self.story_frame = 0;
                    self.story_revealed.clear();
                    self.state = GameState::IntroStory;
                    self.init_game();
                }
            }
            GameState::IntroStory | GameState::LevelStory | GameState::WinStory => {
                self.update_story();
            }
            GameState::Playing => {
                self.update_playing();
            }
            GameState::GameOver => {
                if self.anyp {
                    self.state = GameState::Start;
                }
            }
            GameState::Win => {
                if self.anyp {
                    self.state = GameState::Start;
                }
            }
        }
    }

    fn update_story(&mut self) {
        if self.anyp {
            if self.story_line_idx < self.story_queue.len() {
                // Reveal all
                self.story_revealed = self.story_queue.join("\n");
                self.story_line_idx = self.story_queue.len();
            } else {
                // Advance
                match self.state {
                    GameState::IntroStory => {
                        self.state = GameState::Playing;
                    }
                    GameState::LevelStory => {
                        self.load_level(self.current_level);
                        self.reset_player();
                        self.state = GameState::Playing;
                    }
                    GameState::WinStory => {
                        self.state = GameState::Win;
                    }
                    _ => {}
                }
                return;
            }
        }

        // Typewriter
        self.story_frame += 1;
        if self.story_frame % 2 == 0 && self.story_line_idx < self.story_queue.len() {
            let line = &self.story_queue[self.story_line_idx];
            if self.story_char_idx < line.len() {
                // Safely get next char
                let ch = line.as_bytes()[self.story_char_idx] as char;
                self.story_revealed.push(ch);
                self.story_char_idx += 1;
            } else {
                self.story_revealed.push('\n');
                self.story_line_idx += 1;
                self.story_char_idx = 0;
            }
        }
    }

    fn update_playing(&mut self) {
        // Screen shake decay
        if self.shake_mag > 0.0 {
            self.shake_x = (rand::gen_range(0.0_f32, 1.0) - 0.5) * self.shake_mag;
            self.shake_y = (rand::gen_range(0.0_f32, 1.0) - 0.5) * self.shake_mag;
            self.shake_mag *= 0.85;
            if self.shake_mag < 0.5 { self.shake_mag = 0.0; self.shake_x = 0.0; self.shake_y = 0.0; }
        }
        if self.glitch_timer > 0 { self.glitch_timer -= 1; }
        if self.damage_flash > 0 { self.damage_flash -= 1; }

        self.update_player_physics();
        self.update_enemies_logic();
        self.update_boss_logic();
        self.update_bullets_logic();
        self.update_lasers_logic();
        self.update_falling_plats_logic();
        self.update_terminals_logic();
        self.update_pickups_logic();
        self.update_hazards_logic();
        self.update_exit_check();
        self.update_particles_logic();
        self.update_afterimages();
        self.update_rain_logic();
        self.update_camera_logic();
    }

    fn update_player_physics(&mut self) {
        let pw = PW;
        let ph = PH;

        // Horizontal input
        let mut move_input: f32 = 0.0;
        if self.input_left() { move_input = -1.0; }
        if self.input_right() { move_input = 1.0; }

        if self.player.dashing <= 0 {
            self.player.vx = move_input * MOVE_SPEED;
            if move_input != 0.0 { self.player.facing = move_input; }
        }

        // Gravity
        if self.player.dashing <= 0 {
            self.player.vy += GRAVITY;
            if self.player.vy > MAX_FALL { self.player.vy = MAX_FALL; }
        }

        // Wall slide
        self.player.wall_slide = 0;
        if !self.player.on_ground && self.player.dashing <= 0 {
            let wall_right = solid_at_with_fp(&self.map, self.map_w, self.map_h, &self.falling_plats, self.player.x + pw + 1.0, self.player.y + 2.0)
                || solid_at_with_fp(&self.map, self.map_w, self.map_h, &self.falling_plats, self.player.x + pw + 1.0, self.player.y + ph - 2.0);
            let wall_left = solid_at_with_fp(&self.map, self.map_w, self.map_h, &self.falling_plats, self.player.x - 1.0, self.player.y + 2.0)
                || solid_at_with_fp(&self.map, self.map_w, self.map_h, &self.falling_plats, self.player.x - 1.0, self.player.y + ph - 2.0);
            if wall_right && move_input > 0.0 && self.player.vy > 0.0 {
                self.player.vy = self.player.vy.min(WALL_SLIDE_SPEED);
                self.player.wall_slide = 1;
                if self.global_frame % 4 == 0 {
                    spawn_sparks(&mut self.particles, self.player.x + pw, self.player.y + ph / 2.0);
                }
            }
            if wall_left && move_input < 0.0 && self.player.vy > 0.0 {
                self.player.vy = self.player.vy.min(WALL_SLIDE_SPEED);
                self.player.wall_slide = -1;
                if self.global_frame % 4 == 0 {
                    spawn_sparks(&mut self.particles, self.player.x, self.player.y + ph / 2.0);
                }
            }
        }

        // Coyote time
        if self.player.on_ground { self.player.coyote = COYOTE_MAX; }
        else if self.player.coyote > 0 { self.player.coyote -= 1; }

        // Jump buffer
        if self.jp { self.player.jump_buf = JUMP_BUFFER_MAX; }
        else if self.player.jump_buf > 0 { self.player.jump_buf -= 1; }

        // Jump
        if self.player.jump_buf > 0 && self.player.dashing <= 0 {
            if self.player.coyote > 0 {
                self.player.vy = JUMP_FORCE;
                self.player.coyote = 0;
                self.player.jump_buf = 0;
                self.player.on_ground = false;
            } else if self.player.wall_slide != 0 {
                self.player.vx = -self.player.wall_slide as f32 * WALL_JUMP_X;
                self.player.vy = WALL_JUMP_Y;
                self.player.facing = -self.player.wall_slide as f32;
                self.player.wall_slide = 0;
                self.player.jump_buf = 0;
            }
        }

        // Variable jump height
        if !self.input_jump() && self.player.vy < -2.0 && self.player.dashing <= 0 {
            self.player.vy *= 0.5;
        }

        // Dash
        if self.dp && self.player.dash_cd <= 0 && self.player.dashing <= 0 {
            self.player.dashing = DASH_FRAMES;
            self.player.dash_cd = DASH_COOLDOWN;
            self.player.vx = self.player.facing * DASH_SPEED;
            self.player.vy = 0.0;
        }
        if self.player.dashing > 0 {
            self.player.dashing -= 1;
            if self.player.dashing % 2 == 0 {
                self.afterimages.push(Afterimage {
                    x: self.player.x,
                    y: self.player.y,
                    alpha: 0.6,
                    facing: self.player.facing,
                });
            }
            if self.player.dashing <= 0 {
                self.player.vx = move_input * MOVE_SPEED;
            }
        }
        if self.player.dash_cd > 0 { self.player.dash_cd -= 1; }

        // Move X
        let mut new_x = self.player.x + self.player.vx;
        if self.player.vx > 0.0 {
            if solid_at_with_fp(&self.map, self.map_w, self.map_h, &self.falling_plats, new_x + pw, self.player.y + 2.0)
                || solid_at_with_fp(&self.map, self.map_w, self.map_h, &self.falling_plats, new_x + pw, self.player.y + ph - 2.0)
            {
                new_x = ((new_x + pw) / TILE).floor() * TILE - pw;
                self.player.vx = 0.0;
            }
        } else if self.player.vx < 0.0 {
            if solid_at_with_fp(&self.map, self.map_w, self.map_h, &self.falling_plats, new_x, self.player.y + 2.0)
                || solid_at_with_fp(&self.map, self.map_w, self.map_h, &self.falling_plats, new_x, self.player.y + ph - 2.0)
            {
                new_x = (new_x / TILE).floor() * TILE + TILE;
                self.player.vx = 0.0;
            }
        }
        self.player.x = new_x;

        // Move Y
        let mut new_y = self.player.y + self.player.vy;
        self.player.on_ground = false;
        if self.player.vy > 0.0 {
            if solid_at_with_fp(&self.map, self.map_w, self.map_h, &self.falling_plats, self.player.x + 2.0, new_y + ph)
                || solid_at_with_fp(&self.map, self.map_w, self.map_h, &self.falling_plats, self.player.x + pw - 2.0, new_y + ph)
            {
                new_y = ((new_y + ph) / TILE).floor() * TILE - ph;
                self.player.vy = 0.0;
                self.player.on_ground = true;
            }
        } else if self.player.vy < 0.0 {
            if solid_at_with_fp(&self.map, self.map_w, self.map_h, &self.falling_plats, self.player.x + 2.0, new_y)
                || solid_at_with_fp(&self.map, self.map_w, self.map_h, &self.falling_plats, self.player.x + pw - 2.0, new_y)
            {
                new_y = (new_y / TILE).floor() * TILE + TILE;
                self.player.vy = 0.0;
            }
        }
        self.player.y = new_y;

        // Trigger falling platforms
        if self.player.on_ground {
            for fp in &mut self.falling_plats {
                if fp.state == FPState::Solid {
                    if self.player.x + pw > fp.x && self.player.x < fp.x + TILE
                        && (self.player.y + ph - fp.y).abs() < 3.0
                    {
                        fp.state = FPState::Shaking;
                        fp.timer = 30;
                    }
                }
            }
        }

        // iframes
        if self.player.iframes > 0 { self.player.iframes -= 1; }

        // Slash / Hack
        if self.ap && self.player.slash_cd <= 0 {
            let mut hacked = false;
            // Check terminals first
            for term in &mut self.terminals {
                if !term.hacked {
                    let dx = (self.player.x + pw / 2.0) - (term.x + TILE / 2.0);
                    let dy = (self.player.y + ph / 2.0) - (term.y + TILE / 2.0);
                    if dx.abs() < HACK_RANGE && dy.abs() < HACK_RANGE {
                        term.hacked = true;
                        term.hack_timer = HACK_DUR;
                        hacked = true;
                        break;
                    }
                }
            }
            if hacked {
                // Disable connected lasers and turrets
                // (Need to find which terminal was just hacked)
                let mut hack_x = 0.0_f32;
                let mut hack_y = 0.0_f32;
                for term in &self.terminals {
                    if term.hacked && term.hack_timer == HACK_DUR {
                        hack_x = term.x;
                        hack_y = term.y;
                        break;
                    }
                }
                for l in &mut self.lasers {
                    let dist = (l.x - hack_x).abs() + (l.y - hack_y).abs();
                    if dist < TILE * 15.0 { l.active = false; l.timer = HACK_DUR; }
                }
                for e in &mut self.enemies {
                    if e.etype == EnemyType::Turret && e.alive {
                        let dist = (e.x - hack_x).abs() + (e.y - hack_y).abs();
                        if dist < TILE * 15.0 { e.hacked = true; e.hack_timer = HACK_DUR; e.stun = HACK_DUR; }
                    }
                }
                spawn_particles(&mut self.particles, hack_x + TILE / 2.0, hack_y + TILE / 2.0, COL_GREEN, 10, 3.0, 20);
            } else {
                // Slash attack
                self.player.slashing = 8;
                self.player.slash_cd = SLASH_CD;
                let sx = self.player.x + if self.player.facing > 0.0 { pw } else { -SLASH_RANGE };
                let sy = self.player.y - 4.0;
                // Hit enemies
                for e in &mut self.enemies {
                    if !e.alive { continue; }
                    let ew: f32 = 12.0;
                    let eh: f32 = if e.etype == EnemyType::Guard { 16.0 } else { 12.0 };
                    if rect_overlap(sx, sy, SLASH_RANGE, ph + 8.0, e.x, e.y, ew, eh) {
                        e.hp -= 1;
                        if e.hp <= 0 {
                            e.alive = false;
                            self.score += if e.etype == EnemyType::Guard { 200 } else { 150 };
                            let color = if e.etype == EnemyType::Drone { COL_BLUE } else { COL_MAGENTA };
                            spawn_particles(&mut self.particles, e.x + ew / 2.0, e.y + eh / 2.0, color, 12, 3.0, 20);
                        } else {
                            spawn_particles(&mut self.particles, e.x + ew / 2.0, e.y + eh / 2.0, WHITE, 5, 2.0, 10);
                        }
                    }
                }
                // Hit boss
                if let Some(ref mut boss) = self.boss {
                    if boss.alive {
                        if rect_overlap(sx, sy, SLASH_RANGE, ph + 8.0, boss.x, boss.y, 20.0, 14.0) {
                            boss.hp -= 1;
                            boss.flash_timer = 6;
                            if boss.hp <= boss.max_hp / 2 && !boss.enraged {
                                boss.enraged = true;
                            }
                            if boss.hp <= 0 {
                                boss.alive = false;
                                self.score += 1000;
                                spawn_particles(&mut self.particles, boss.x + 10.0, boss.y + 7.0, COL_MAGENTA, 30, 5.0, 40);
                                self.trigger_shake(10.0);
                            } else {
                                spawn_particles(&mut self.particles, boss.x + 10.0, boss.y + 7.0, WHITE, 8, 3.0, 15);
                                self.trigger_shake(3.0);
                            }
                        }
                    }
                }
            }
        }
        if self.player.slashing > 0 { self.player.slashing -= 1; }
        if self.player.slash_cd > 0 { self.player.slash_cd -= 1; }

        // EMP (instant AOE, matching web)
        if self.ep && self.player.emps > 0 {
            self.player.emps -= 1;
            let emp_x = self.player.x + pw / 2.0 + self.player.facing * 40.0;
            let emp_y = self.player.y;
            spawn_particles(&mut self.particles, emp_x, emp_y, COL_BLUE, 20, 4.0, 25);
            for e in &mut self.enemies {
                if !e.alive { continue; }
                let dx = e.x - emp_x;
                let dy = e.y - emp_y;
                if (dx * dx + dy * dy).sqrt() < EMP_RADIUS {
                    e.stun = EMP_STUN;
                    spawn_particles(&mut self.particles, e.x + 6.0, e.y + 6.0, COL_BLUE, 5, 2.0, 15);
                }
            }
            if let Some(ref mut boss) = self.boss {
                if boss.alive {
                    let dx = boss.x - emp_x;
                    let dy = boss.y - emp_y;
                    if (dx * dx + dy * dy).sqrt() < EMP_RADIUS {
                        boss.stun = EMP_STUN;
                        boss.state = BossState::Stunned;
                        boss.timer = EMP_STUN;
                    }
                }
            }
            self.trigger_shake(5.0);
        }

        // Animation
        self.player.anim_timer += 1;
        if self.player.anim_timer > 6 {
            self.player.anim_timer = 0;
            self.player.anim_frame = 1 - self.player.anim_frame;
        }

        // Fall death
        if self.player.y > self.map_h as f32 * TILE + 50.0 {
            self.player.hp = 0;
            self.hurt_player_forced();
        }
    }

    fn hurt_player_forced(&mut self) {
        self.lives -= 1;
        self.trigger_shake(6.0);
        self.glitch_timer = 10;
        self.damage_flash = 8;
        if self.lives <= 0 {
            self.state = GameState::GameOver;
        } else {
            self.load_level(self.current_level);
            self.reset_player();
        }
    }

    fn update_enemies_logic(&mut self) {
        let px = self.player.x;
        let py = self.player.y;
        let gf = self.global_frame;

        for i in 0..self.enemies.len() {
            if !self.enemies[i].alive { continue; }
            if self.enemies[i].stun > 0 { self.enemies[i].stun -= 1; continue; }

            match self.enemies[i].etype {
                EnemyType::Drone => {
                    self.enemies[i].x += self.enemies[i].vx;
                    self.enemies[i].y = self.enemies[i].start_y + (gf as f32 * 0.04 + i as f32).sin() * 15.0;
                    // Reverse at walls
                    if tile_solid(&self.map, self.map_w, self.map_h, self.enemies[i].x + 14.0, self.enemies[i].y + 6.0)
                        || tile_solid(&self.map, self.map_w, self.map_h, self.enemies[i].x - 2.0, self.enemies[i].y + 6.0)
                    {
                        self.enemies[i].vx = -self.enemies[i].vx;
                    }
                    if self.enemies[i].x < self.enemies[i].start_x - 80.0 || self.enemies[i].x > self.enemies[i].start_x + 80.0 {
                        self.enemies[i].vx = -self.enemies[i].vx;
                    }
                    // Shoot
                    self.enemies[i].timer += 1;
                    if self.enemies[i].timer >= 90 {
                        self.enemies[i].timer = 0;
                        let dx = px - self.enemies[i].x;
                        let dy = py - self.enemies[i].y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist < 300.0 && dist > 0.0 {
                            self.bullets.push(Bullet {
                                x: self.enemies[i].x + 6.0,
                                y: self.enemies[i].y + 6.0,
                                vx: (dx / dist) * 3.0,
                                vy: (dy / dist) * 3.0,
                                life: 120,
                            });
                        }
                    }
                }
                EnemyType::Guard => {
                    self.enemies[i].x += self.enemies[i].dir;
                    let ahead_x = self.enemies[i].x + if self.enemies[i].dir > 0.0 { 14.0 } else { -2.0 };
                    let feet_y = self.enemies[i].y + 18.0;
                    if tile_solid(&self.map, self.map_w, self.map_h, ahead_x, self.enemies[i].y + 8.0)
                        || !tile_solid(&self.map, self.map_w, self.map_h, ahead_x, feet_y)
                    {
                        self.enemies[i].dir = -self.enemies[i].dir;
                    }
                    // Shoot
                    self.enemies[i].timer += 1;
                    if self.enemies[i].timer >= 120 {
                        self.enemies[i].timer = 0;
                        let dx = px - self.enemies[i].x;
                        let dy = py - self.enemies[i].y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist < 250.0 && dist > 0.0 {
                            self.bullets.push(Bullet {
                                x: self.enemies[i].x + 6.0,
                                y: self.enemies[i].y + 6.0,
                                vx: (dx / dist) * 2.5,
                                vy: (dy / dist) * 2.5,
                                life: 120,
                            });
                        }
                    }
                }
                EnemyType::Turret => {
                    if self.enemies[i].hacked {
                        self.enemies[i].hack_timer -= 1;
                        if self.enemies[i].hack_timer <= 0 {
                            self.enemies[i].hacked = false;
                            self.enemies[i].stun = 0;
                        }
                        continue;
                    }
                    self.enemies[i].angle += 0.02;
                    self.enemies[i].timer += 1;
                    if self.enemies[i].timer >= 60 {
                        self.enemies[i].timer = 0;
                        let bvx = self.enemies[i].angle.cos() * 3.0;
                        let bvy = self.enemies[i].angle.sin() * 3.0;
                        self.bullets.push(Bullet {
                            x: self.enemies[i].x + 6.0,
                            y: self.enemies[i].y + 6.0,
                            vx: bvx, vy: bvy, life: 90,
                        });
                    }
                }
            }

            // Contact damage
            if self.player.iframes <= 0 && self.player.dashing <= 0 {
                let ew: f32 = if self.enemies[i].etype == EnemyType::Guard { 12.0 } else { 12.0 };
                let eh: f32 = if self.enemies[i].etype == EnemyType::Guard { 16.0 } else { 12.0 };
                if rect_overlap(self.player.x, self.player.y, PW, PH, self.enemies[i].x, self.enemies[i].y, ew, eh) {
                    // Need to call hurt_player but can't borrow self -- set flag
                    self.player.hp -= 1;
                    self.player.iframes = INVULN_FRAMES;
                    self.trigger_shake(6.0);
                    self.glitch_timer = 10;
                    self.damage_flash = 8;
                    spawn_particles(&mut self.particles, self.player.x + PW / 2.0, self.player.y + PH / 2.0, RED, 8, 3.0, 15);
                    if self.player.hp <= 0 {
                        self.lives -= 1;
                        // Will be handled at end of frame
                    }
                    break;
                }
            }
        }

        // Check if player died from contact
        if self.player.hp <= 0 && self.lives <= 0 {
            self.state = GameState::GameOver;
        } else if self.player.hp <= 0 && self.lives > 0 {
            self.load_level(self.current_level);
            self.reset_player();
        }
    }

    fn update_boss_logic(&mut self) {
        let px = self.player.x;
        let _py = self.player.y;

        // Boss update -- extract values to avoid borrow issues
        let mut boss_landed = false;
        let mut boss_land_x = 0.0_f32;
        let mut boss_land_y = 0.0_f32;
        let mut boss_contact = false;

        if let Some(ref mut boss) = self.boss {
            if !boss.alive { /* skip */ }
            else {
                if boss.stun > 0 {
                    boss.stun -= 1;
                    if boss.stun <= 0 { boss.state = BossState::Patrol; }
                } else {
                    let spd = if boss.enraged { 3.5 } else { 2.0 };
                    boss.timer += 1;
                    match boss.state {
                        BossState::Patrol => {
                            boss.x += boss.dir * spd;
                            if tile_solid(&self.map, self.map_w, self.map_h, boss.x + 22.0, boss.y + 8.0)
                                || tile_solid(&self.map, self.map_w, self.map_h, boss.x - 2.0, boss.y + 8.0) {
                                boss.dir = -boss.dir;
                            }
                            if !tile_solid(&self.map, self.map_w, self.map_h, boss.x + 10.0, boss.y + 16.0) {
                                boss.y += 3.0;
                            }
                            let charge_threshold = if boss.enraged { 60 } else { 120 };
                            if boss.timer > charge_threshold {
                                boss.timer = 0;
                                let dx = px - boss.x;
                                if dx.abs() < 200.0 {
                                    boss.state = BossState::Charge;
                                    boss.dir = if dx > 0.0 { 1.0 } else { -1.0 };
                                } else if rand::gen_range(0.0_f32, 1.0) < 0.4 {
                                    boss.state = BossState::Leap;
                                    boss.vy = -10.0;
                                    boss.vx = if px > boss.x { 4.0 } else { -4.0 };
                                }
                            }
                        }
                        BossState::Charge => {
                            boss.x += boss.dir * spd * 2.0;
                            if !tile_solid(&self.map, self.map_w, self.map_h, boss.x + 10.0, boss.y + 16.0) {
                                boss.y += 3.0;
                            }
                            if tile_solid(&self.map, self.map_w, self.map_h, boss.x + 22.0, boss.y + 8.0)
                                || tile_solid(&self.map, self.map_w, self.map_h, boss.x - 2.0, boss.y + 8.0)
                                || boss.timer > 40
                            {
                                boss.state = BossState::Patrol;
                                boss.timer = 0;
                            }
                        }
                        BossState::Leap => {
                            boss.x += boss.vx;
                            boss.y += boss.vy;
                            boss.vy += 0.5;
                            if boss.vy > 0.0 && tile_solid(&self.map, self.map_w, self.map_h, boss.x + 10.0, boss.y + 16.0) {
                                boss.y = ((boss.y + 16.0) / TILE).floor() * TILE - 16.0;
                                boss.vy = 0.0;
                                boss.state = BossState::Patrol;
                                boss.timer = 0;
                                boss_landed = true;
                                boss_land_x = boss.x + 10.0;
                                boss_land_y = boss.y + 14.0;
                            }
                        }
                        BossState::Stunned => {
                            // Handled by stun counter above
                        }
                    }
                }
                if boss.flash_timer > 0 { boss.flash_timer -= 1; }

                // Boss contact with player
                if self.player.iframes <= 0 && self.player.dashing <= 0 && boss.alive {
                    if rect_overlap(self.player.x, self.player.y, PW, PH, boss.x, boss.y, 20.0, 14.0) {
                        boss_contact = true;
                    }
                }
            }
        }

        // Deferred effects from boss update
        if boss_landed {
            self.shake_mag = self.shake_mag.max(4.0);
            spawn_particles(&mut self.particles, boss_land_x, boss_land_y, Color::new(0.33, 0.33, 0.33, 1.0), 8, 3.0, 15);
        }
        if boss_contact {
            self.player.hp -= 1;
            self.player.iframes = INVULN_FRAMES;
            self.shake_mag = self.shake_mag.max(6.0);
            self.glitch_timer = 10;
            self.damage_flash = 8;
        }
    }

    fn update_bullets_logic(&mut self) {
        let mut i = self.bullets.len();
        while i > 0 {
            i -= 1;
            self.bullets[i].x += self.bullets[i].vx;
            self.bullets[i].y += self.bullets[i].vy;
            self.bullets[i].life -= 1;
            if self.bullets[i].life <= 0
                || tile_solid(&self.map, self.map_w, self.map_h, self.bullets[i].x, self.bullets[i].y)
            {
                self.bullets.swap_remove(i);
                continue;
            }
            // Hit player
            if self.player.iframes <= 0 && self.player.dashing <= 0 {
                if rect_overlap(self.player.x, self.player.y, PW, PH, self.bullets[i].x - 3.0, self.bullets[i].y - 3.0, 6.0, 6.0) {
                    self.player.hp -= 1;
                    self.player.iframes = INVULN_FRAMES;
                    self.shake_mag = self.shake_mag.max(6.0);
                    self.glitch_timer = 10;
                    self.damage_flash = 8;
                    spawn_particles(&mut self.particles, self.player.x + PW / 2.0, self.player.y + PH / 2.0, RED, 8, 3.0, 15);
                    self.bullets.swap_remove(i);
                    if self.player.hp <= 0 {
                        self.lives -= 1;
                        if self.lives <= 0 {
                            self.state = GameState::GameOver;
                        } else {
                            // Will reload at frame end check
                        }
                    }
                }
            }
        }
        // Check deferred death
        if self.player.hp <= 0 && self.lives > 0 && self.state == GameState::Playing {
            self.load_level(self.current_level);
            self.reset_player();
        }
    }

    fn update_lasers_logic(&mut self) {
        for l in &mut self.lasers {
            l.timer += 1;
            if l.timer >= 90 { l.timer = 0; l.active = !l.active; }
            if l.active && self.player.iframes <= 0 && self.player.dashing <= 0 {
                if rect_overlap(self.player.x, self.player.y, PW, PH, l.x + 7.0, l.y, 6.0, TILE) {
                    self.player.hp -= 1;
                    self.player.iframes = INVULN_FRAMES;
                    self.shake_mag = self.shake_mag.max(6.0);
                    self.glitch_timer = 10;
                    self.damage_flash = 8;
                }
            }
        }
    }

    fn update_falling_plats_logic(&mut self) {
        for fp in &mut self.falling_plats {
            match fp.state {
                FPState::Solid => {}
                FPState::Shaking => {
                    fp.timer -= 1;
                    if fp.timer <= 0 { fp.state = FPState::Falling; }
                }
                FPState::Falling => {
                    fp.y += 4.0;
                    if fp.y > self.map_h as f32 * TILE + 100.0 {
                        fp.state = FPState::Respawning;
                        fp.timer = 180;
                    }
                }
                FPState::Respawning => {
                    fp.timer -= 1;
                    if fp.timer <= 0 {
                        fp.state = FPState::Solid;
                        fp.y = fp.orig_y;
                    }
                }
            }
        }
    }

    fn update_terminals_logic(&mut self) {
        for term in &mut self.terminals {
            if term.hacked {
                term.hack_timer -= 1;
                if term.hack_timer <= 0 {
                    term.hacked = false;
                    // Re-enable nearby lasers
                    for l in &mut self.lasers {
                        let dist = (l.x - term.x).abs() + (l.y - term.y).abs();
                        if dist < TILE * 15.0 { l.active = true; }
                    }
                }
            }
        }
    }

    fn update_pickups_logic(&mut self) {
        for i in 0..self.pickups.len() {
            if !self.pickups[i].alive { continue; }
            match self.pickups[i].kind {
                PickupKind::Chip => {
                    if rect_overlap(self.player.x, self.player.y, PW, PH,
                        self.pickups[i].x - 4.0, self.pickups[i].y - 4.0, 8.0, 8.0) {
                        self.pickups[i].alive = false;
                        self.score += 50;
                        spawn_particles(&mut self.particles, self.pickups[i].x, self.pickups[i].y, COL_AMBER, 6, 2.0, 15);
                    }
                }
                PickupKind::Health => {
                    if rect_overlap(self.player.x, self.player.y, PW, PH,
                        self.pickups[i].x, self.pickups[i].y, TILE, TILE)
                        && self.pickups[i].alive
                    {
                        self.pickups[i].alive = false;
                        if self.player.hp < 3 { self.player.hp += 1; }
                        spawn_particles(&mut self.particles, self.pickups[i].x + 10.0, self.pickups[i].y + 10.0, COL_MAGENTA, 8, 2.0, 15);
                    }
                }
                PickupKind::Emp => {
                    if rect_overlap(self.player.x, self.player.y, PW, PH,
                        self.pickups[i].x, self.pickups[i].y, TILE, TILE)
                        && self.pickups[i].alive
                    {
                        self.pickups[i].alive = false;
                        self.player.emps += 1;
                        spawn_particles(&mut self.particles, self.pickups[i].x + 10.0, self.pickups[i].y + 10.0, COL_BLUE, 8, 2.0, 15);
                    }
                }
            }
        }
    }

    fn update_hazards_logic(&mut self) {
        // Electric floor & acid
        for ty in 0..self.map_h {
            for tx in 0..self.map_w {
                if self.map[ty][tx] == 2 {
                    if self.player.iframes <= 0 && self.player.dashing <= 0
                        && rect_overlap(self.player.x, self.player.y, PW, PH,
                            tx as f32 * TILE, ty as f32 * TILE, TILE, TILE)
                    {
                        self.player.hp -= 1;
                        self.player.iframes = INVULN_FRAMES;
                        self.shake_mag = self.shake_mag.max(6.0);
                        self.glitch_timer = 10;
                        self.damage_flash = 8;
                    }
                }
                if self.map[ty][tx] == 3 {
                    if rect_overlap(self.player.x, self.player.y, PW, PH,
                        tx as f32 * TILE, ty as f32 * TILE + 4.0, TILE, TILE - 4.0)
                    {
                        self.player.hp = 0;
                        // Instant kill
                    }
                }
            }
        }
    }

    fn update_exit_check(&mut self) {
        if rect_overlap(self.player.x, self.player.y, PW, PH, self.exit_x, self.exit_y, TILE, TILE) {
            if let Some(ref boss) = self.boss {
                if boss.alive { return; }
            }
            self.complete_level();
        }
    }

    fn update_particles_logic(&mut self) {
        let mut i = self.particles.len();
        while i > 0 {
            i -= 1;
            self.particles[i].x += self.particles[i].vx;
            self.particles[i].y += self.particles[i].vy;
            self.particles[i].vy += 0.05;
            self.particles[i].life -= 1;
            if self.particles[i].life <= 0 {
                self.particles.swap_remove(i);
            }
        }
    }

    fn update_afterimages(&mut self) {
        let mut i = self.afterimages.len();
        while i > 0 {
            i -= 1;
            self.afterimages[i].alpha -= 0.08;
            if self.afterimages[i].alpha <= 0.0 {
                self.afterimages.swap_remove(i);
            }
        }
    }

    fn update_rain_logic(&mut self) {
        for r in &mut self.raindrops {
            r.y += r.speed;
            r.x -= 0.5;
            if r.y > SCREEN_H + self.cam_y + 20.0 {
                r.y = self.cam_y - 20.0;
                r.x = self.cam_x + rand::gen_range(0.0_f32, SCREEN_W * 2.0);
            }
        }
    }

    fn update_camera_logic(&mut self) {
        let target_cx = self.player.x - SCREEN_W / 2.0 + self.player.facing * 40.0;
        let target_cy = self.player.y - SCREEN_H / 2.0 + 30.0;
        self.cam_x += (target_cx - self.cam_x) * 0.1;
        self.cam_y += (target_cy - self.cam_y) * 0.08;
        self.cam_x = self.cam_x.max(0.0).min((self.map_w as f32 * TILE - SCREEN_W).max(0.0));
        self.cam_y = self.cam_y.max(0.0).min((self.map_h as f32 * TILE - SCREEN_H).max(0.0));
    }

    // -- Draw -----------------------------------------------------------------

    fn draw(&self) {
        clear_background(COL_DARK);

        match self.state {
            GameState::Start => self.draw_start(),
            GameState::IntroStory | GameState::LevelStory | GameState::WinStory => self.draw_story(),
            GameState::Playing => self.draw_playing(),
            GameState::GameOver => self.draw_game_over(),
            GameState::Win => self.draw_win(),
        }

        // CRT scanlines
        for y in (0..SCREEN_H as i32).step_by(4) {
            draw_rectangle(0.0, y as f32, SCREEN_W, 1.0, Color::new(0.0, 0.0, 0.0, 0.12));
        }
        // Vignette
        for i in 0..8 {
            let alpha = 0.12 * (1.0 - i as f32 / 8.0);
            let c = Color::new(0.0, 0.0, 0.0, alpha);
            let fi = i as f32 * 8.0;
            draw_rectangle(0.0, fi, SCREEN_W, 2.0, c);
            draw_rectangle(0.0, SCREEN_H - fi - 2.0, SCREEN_W, 2.0, c);
            draw_rectangle(fi, 0.0, 2.0, SCREEN_H, c);
            draw_rectangle(SCREEN_W - fi - 2.0, 0.0, 2.0, SCREEN_H, c);
        }
    }

    fn draw_parallax_city(&self, cam_x: f32) {
        // Layer 1: far
        for i in 0..15 {
            let w = 30.0 + ((i * 37) % 50) as f32;
            let h = 40.0 + ((i * 53) % 200) as f32;
            let bx = ((i as f32 * 120.0 + 20.0 - cam_x * 0.05) % (SCREEN_W + 200.0) + SCREEN_W + 200.0) % (SCREEN_W + 200.0) - 100.0;
            let by = SCREEN_H - h;
            draw_rectangle(bx, by, w, h, Color::new(0.04, 0.04, 0.08, 1.0));
            // Windows
            let mut wy = by + 5.0;
            while wy < SCREEN_H - 5.0 {
                let mut wx = bx + 4.0;
                while wx < bx + w - 4.0 {
                    if ((wx as i32 * 7 + wy as i32 * 13 + i) % 3) != 0 {
                        let flicker = if (self.global_frame as f32 * 0.02 + wx * 0.1 + wy * 0.07).sin() > 0.0 { 1.0 } else { 0.3 };
                        draw_rectangle(wx, wy, 3.0, 4.0, Color::new(1.0, 0.0, 0.5, 0.08 * flicker));
                    }
                    wx += 8.0;
                }
                wy += 12.0;
            }
        }
        // Layer 2: mid
        for i in 0..12 {
            let w = 30.0 + ((i * 37) % 50) as f32;
            let h = 40.0 + ((i * 53) % 150) as f32;
            let bx = ((i as f32 * 120.0 + 20.0 - cam_x * 0.1) % (SCREEN_W + 200.0) + SCREEN_W + 200.0) % (SCREEN_W + 200.0) - 100.0;
            let by = SCREEN_H - h;
            draw_rectangle(bx, by, w, h, Color::new(0.05, 0.05, 0.12, 1.0));
            let mut wy = by + 5.0;
            while wy < SCREEN_H - 5.0 {
                let mut wx = bx + 4.0;
                while wx < bx + w - 4.0 {
                    if ((wx as i32 * 7 + wy as i32 * 13 + i) % 3) != 0 {
                        draw_rectangle(wx, wy, 3.0, 4.0, Color::new(0.0, 0.83, 1.0, 0.1));
                    }
                    wx += 8.0;
                }
                wy += 12.0;
            }
        }
        // Layer 3: near
        for i in 0..10 {
            let w = 30.0 + ((i * 37) % 50) as f32;
            let h = 40.0 + ((i * 53) % 100) as f32;
            let bx = ((i as f32 * 120.0 + 20.0 - cam_x * 0.2) % (SCREEN_W + 200.0) + SCREEN_W + 200.0) % (SCREEN_W + 200.0) - 100.0;
            let by = SCREEN_H - h;
            draw_rectangle(bx, by, w, h, Color::new(0.067, 0.067, 0.19, 1.0));
            let mut wy = by + 5.0;
            while wy < SCREEN_H - 5.0 {
                let mut wx = bx + 4.0;
                while wx < bx + w - 4.0 {
                    if ((wx as i32 * 7 + wy as i32 * 13 + i) % 3) != 0 {
                        draw_rectangle(wx, wy, 3.0, 4.0, Color::new(1.0, 0.72, 0.0, 0.12));
                    }
                    wx += 8.0;
                }
                wy += 12.0;
            }
        }
    }

    fn draw_start(&self) {
        self.draw_parallax_city(self.global_frame as f32 * 0.5);

        // Rain
        for r in &self.raindrops {
            draw_line(r.x, r.y, r.x - 0.3, r.y + r.len, 1.0, Color::new(0.39, 0.55, 0.78, r.alpha));
        }

        // Ground line
        draw_line(0.0, SCREEN_H - 80.0, SCREEN_W, SCREEN_H - 80.0, 2.0, Color::new(1.0, 0.0, 0.5, 0.5));

        // Title
        let title = "NEON RUNNER";
        let tw = measure_text(title, None, 32, 1.0).width;

        // Glitch
        let glitch = if (self.global_frame as f32 * 0.15).sin() > 0.9 { (rand::gen_range(0.0_f32, 1.0) - 0.5) * 6.0 } else { 0.0 };
        if glitch != 0.0 {
            draw_text(title, SCREEN_W / 2.0 - tw / 2.0 + glitch - 2.0, 180.0, 32.0, Color::new(0.0, 0.83, 1.0, 0.5));
            draw_text(title, SCREEN_W / 2.0 - tw / 2.0 + glitch + 2.0, 180.0, 32.0, Color::new(1.0, 0.0, 0.5, 0.5));
        }
        draw_text(title, SCREEN_W / 2.0 - tw / 2.0, 180.0, 32.0, WHITE);

        let sub = "GHOST PROTOCOL";
        let sw = measure_text(sub, None, 12, 1.0).width;
        draw_text(sub, SCREEN_W / 2.0 - sw / 2.0, 220.0, 12.0, COL_GREEN);

        if (self.global_frame / 30) % 2 != 0 {
            let prompt = "JACK IN";
            let pw = measure_text(prompt, None, 10, 1.0).width;
            draw_text(prompt, SCREEN_W / 2.0 - pw / 2.0, 320.0, 10.0, COL_AMBER);
        }

        let ctrl = "ARROWS:MOVE  SPACE:JUMP  X:ATTACK  Z:DASH  C:EMP";
        let cw = measure_text(ctrl, None, 7, 1.0).width;
        draw_text(ctrl, SCREEN_W / 2.0 - cw / 2.0, SCREEN_H - 30.0, 7.0, Color::new(0.27, 0.27, 0.33, 1.0));
    }

    fn draw_story(&self) {
        clear_background(Color::new(0.02, 0.03, 0.03, 1.0));
        draw_rectangle_lines(20.0, 20.0, SCREEN_W - 40.0, SCREEN_H - 40.0, 1.0, Color::new(0.0, 1.0, 0.255, 0.4));

        draw_text("NEO-KYOTO TERMINAL v3.7", 30.0, 40.0, 8.0, COL_GREEN);
        draw_rectangle(30.0, 45.0, SCREEN_W - 60.0, 1.0, Color::new(0.0, 0.2, 0.0, 1.0));

        let lines: Vec<&str> = self.story_revealed.split('\n').collect();
        for (i, line) in lines.iter().enumerate() {
            if i >= 22 { break; }
            draw_text(line, 35.0, 65.0 + i as f32 * 16.0, 8.0, COL_GREEN);
        }

        // Cursor
        if self.story_line_idx < self.story_queue.len() && (self.global_frame / 20) % 2 != 0 {
            let last = lines.last().unwrap_or(&"");
            let lw = measure_text(last, None, 8, 1.0).width;
            let cy = 65.0 + (lines.len() as f32 - 1.0) * 16.0;
            draw_rectangle(35.0 + lw + 2.0, cy - 8.0, 6.0, 10.0, COL_GREEN);
        }

        // Continue prompt
        if self.story_line_idx >= self.story_queue.len() && (self.global_frame / 30) % 2 != 0 {
            let prompt = "PRESS ANY KEY TO CONTINUE";
            let pw = measure_text(prompt, None, 10, 1.0).width;
            draw_text(prompt, SCREEN_W / 2.0 - pw / 2.0, SCREEN_H - 35.0, 10.0, COL_AMBER);
        }
    }

    fn draw_playing(&self) {
        let cam_x = self.cam_x;
        let cam_y = self.cam_y;
        let sx = self.shake_x;
        let sy = self.shake_y;
        let gf = self.global_frame;
        let level_color = match self.current_level {
            0 => COL_MAGENTA,
            1 => COL_BLUE,
            _ => COL_GREEN,
        };

        // Background
        self.draw_parallax_city(cam_x);

        // Rain
        for r in &self.raindrops {
            let rx = r.x - cam_x * 0.5;
            let ry = r.y - cam_y * 0.3;
            if rx > -10.0 && rx < SCREEN_W + 10.0 && ry > -10.0 && ry < SCREEN_H + 10.0 {
                draw_line(rx, ry, rx - 0.5, ry + r.len, 1.0, Color::new(0.39, 0.55, 0.78, r.alpha));
            }
        }

        // Tiles
        let start_col = (cam_x / TILE).max(0.0) as usize;
        let end_col = ((cam_x + SCREEN_W) / TILE) as usize + 2;
        let start_row = (cam_y / TILE).max(0.0) as usize;
        let end_row = ((cam_y + SCREEN_H) / TILE) as usize + 2;

        for r in start_row..end_row.min(self.map_h) {
            for c in start_col..end_col.min(self.map_w) {
                let tx = c as f32 * TILE - cam_x + sx;
                let ty = r as f32 * TILE - cam_y + sy;
                if self.map[r][c] == 1 {
                    draw_rectangle(tx, ty, TILE, TILE, Color::new(0.1, 0.1, 0.18, 1.0));
                    // Neon edge
                    if r == 0 || self.map[r - 1][c] != 1 {
                        draw_rectangle(tx, ty, TILE, 1.0, Color::new(level_color.r, level_color.g, level_color.b, 0.6));
                    }
                } else if self.map[r][c] == 2 {
                    // Electric floor
                    draw_rectangle(tx, ty, TILE, TILE, Color::new(0.1, 0.0, 0.2, 1.0));
                    let spark = (gf as f32 * 0.3 + c as f32).sin() > 0.5;
                    let col = if spark { Color::new(0.53, 0.0, 1.0, 1.0) } else { Color::new(0.27, 0.0, 0.67, 1.0) };
                    draw_rectangle(tx + 2.0, ty + 2.0, TILE - 4.0, TILE - 4.0, col);
                } else if self.map[r][c] == 3 {
                    // Acid pool
                    let bubble = (gf as f32 * 0.1 + c as f32 * 0.5).sin() * 0.3;
                    draw_rectangle(tx, ty + 4.0, TILE, TILE - 4.0, Color::new(0.0, 1.0, 0.255, 0.3 + bubble));
                    draw_rectangle(tx, ty, TILE, 4.0, Color::new(0.0, 1.0, 0.255, 0.15 + bubble));
                }
            }
        }

        // Falling platforms
        for fp in &self.falling_plats {
            if fp.state == FPState::Respawning { continue; }
            let shake_off = if fp.state == FPState::Shaking { (rand::gen_range(0.0_f32, 1.0) - 0.5) * 3.0 } else { 0.0 };
            let alpha = if fp.state == FPState::Falling { 0.6 } else { 1.0 };
            let fx = fp.x - cam_x + sx + shake_off;
            let fy = fp.y - cam_y + sy;
            draw_rectangle(fx, fy, TILE, TILE, Color::new(0.27, 0.2, 0.13, alpha));
            draw_rectangle(fx, fy, TILE, 2.0, Color::new(1.0, 0.72, 0.0, alpha * 0.6));
        }

        // Lasers
        for l in &self.lasers {
            let lx = l.x - cam_x + sx;
            let ly = l.y - cam_y + sy;
            if l.active {
                let alpha = 0.5 + (gf as f32 * 0.2).sin() * 0.3;
                draw_rectangle(lx + 8.0, ly, 4.0, TILE, Color::new(1.0, 0.0, 0.25, alpha));
                draw_rectangle(lx + 4.0, ly, 12.0, TILE, Color::new(1.0, 0.0, 0.25, 0.15));
            } else {
                draw_rectangle(lx + 9.0, ly, 2.0, TILE, Color::new(1.0, 0.0, 0.25, 0.08));
            }
        }

        // Terminals
        for term in &self.terminals {
            let tx = term.x - cam_x + sx;
            let ty = term.y - cam_y + sy;
            draw_texture_ex(&self.tex_terminal, tx, ty, WHITE, DrawTextureParams {
                dest_size: Some(Vec2::new(TILE, TILE)), ..Default::default()
            });
            if term.hacked {
                draw_rectangle_lines(tx, ty, TILE, TILE, 1.0, COL_GREEN);
            }
        }

        // Chips / Pickups
        for p in &self.pickups {
            if !p.alive { continue; }
            let bob = (gf as f32 * 0.08 + p.x * 0.1).sin() * 3.0;
            match p.kind {
                PickupKind::Chip => {
                    let px = p.x - 4.0 - cam_x + sx;
                    let py = p.y - 4.0 + bob - cam_y + sy;
                    draw_texture_ex(&self.tex_chip, px, py, WHITE, DrawTextureParams {
                        dest_size: Some(Vec2::new(8.0, 8.0)), ..Default::default()
                    });
                    draw_rectangle(px, py, 8.0, 8.0, Color::new(1.0, 0.72, 0.0, 0.2));
                }
                PickupKind::Health => {
                    let px = p.x + 2.0 - cam_x + sx;
                    let py = p.y + 2.0 + bob - cam_y + sy;
                    draw_texture_ex(&self.tex_health, px, py, WHITE, DrawTextureParams {
                        dest_size: Some(Vec2::new(TILE - 4.0, TILE - 4.0)), ..Default::default()
                    });
                }
                PickupKind::Emp => {
                    let px = p.x + 2.0 - cam_x + sx;
                    let py = p.y + 2.0 + bob - cam_y + sy;
                    draw_texture_ex(&self.tex_emp_ammo, px, py, WHITE, DrawTextureParams {
                        dest_size: Some(Vec2::new(TILE - 4.0, TILE - 4.0)), ..Default::default()
                    });
                }
            }
        }

        // Exit
        let exit_bob = (gf as f32 * 0.05).sin() * 3.0;
        let ex = self.exit_x + 2.0 - cam_x + sx;
        let ey = self.exit_y + 2.0 + exit_bob - cam_y + sy;
        draw_texture_ex(&self.tex_exit, ex, ey, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(TILE - 4.0, TILE - 4.0)), ..Default::default()
        });
        draw_rectangle(self.exit_x - cam_x + sx, self.exit_y + exit_bob - cam_y + sy, TILE, TILE, Color::new(0.0, 1.0, 0.255, 0.15));

        // Enemies
        for e in &self.enemies {
            if !e.alive { continue; }
            let ex = e.x - cam_x + sx;
            let ey = e.y - cam_y + sy;
            if e.stun > 0 && (gf / 3) % 2 != 0 { continue; }
            let (tex, tw, th) = match e.etype {
                EnemyType::Drone => (&self.tex_drone, 12.0, 12.0),
                EnemyType::Guard => (&self.tex_guard, 12.0, 16.0),
                EnemyType::Turret => (&self.tex_turret, 12.0, 12.0),
            };
            draw_texture_ex(tex, ex, ey, WHITE, DrawTextureParams {
                dest_size: Some(Vec2::new(tw, th)), ..Default::default()
            });
            if e.hacked {
                draw_rectangle_lines(ex, ey, tw, th, 1.0, COL_GREEN);
            }
        }

        // Boss
        if let Some(ref boss) = self.boss {
            if boss.alive {
                let bx = boss.x - cam_x + sx;
                let by = boss.y - cam_y + sy;
                if boss.stun > 0 && (gf / 3) % 2 != 0 {
                    // flash
                } else {
                    let alpha = if boss.flash_timer > 0 { 0.5 + (gf as f32).sin() * 0.5 } else { 1.0 };
                    draw_texture_ex(&self.tex_boss, bx, by, Color::new(1.0, 1.0, 1.0, alpha), DrawTextureParams {
                        dest_size: Some(Vec2::new(24.0, 16.0)), ..Default::default()
                    });
                }
                // HP bar
                let bar_w = 40.0;
                draw_rectangle(bx - 8.0, by - 8.0, bar_w, 4.0, Color::new(0.2, 0.0, 0.0, 1.0));
                let color = if boss.enraged { RED } else { COL_MAGENTA };
                draw_rectangle(bx - 8.0, by - 8.0, bar_w * (boss.hp as f32 / boss.max_hp as f32), 4.0, color);
            }
        }

        // Bullets
        for b in &self.bullets {
            let bx = b.x - 2.0 - cam_x + sx;
            let by = b.y - 2.0 - cam_y + sy;
            draw_rectangle(bx, by, 4.0, 4.0, COL_MAGENTA);
        }

        // Afterimages
        for ai in &self.afterimages {
            let ax = ai.x - cam_x + sx;
            let ay = ai.y - cam_y + sy;
            let flip = ai.facing < 0.0;
            draw_texture_ex(&self.tex_player_idle, ax, ay, Color::new(1.0, 0.0, 0.5, ai.alpha * 0.4), DrawTextureParams {
                dest_size: Some(Vec2::new(PW, PH)),
                flip_x: flip,
                ..Default::default()
            });
        }

        // Player
        if self.player.iframes <= 0 || (gf / 3) % 2 != 0 {
            let tex = if self.player.vx.abs() > 0.5 && self.player.anim_frame != 0 {
                &self.tex_player_run
            } else {
                &self.tex_player_idle
            };
            let px = self.player.x - cam_x + sx;
            let py = self.player.y - cam_y + sy;
            let flip = self.player.facing < 0.0;
            draw_texture_ex(tex, px, py, WHITE, DrawTextureParams {
                dest_size: Some(Vec2::new(PW, PH)),
                flip_x: flip,
                ..Default::default()
            });

            // Slash effect
            if self.player.slashing > 0 {
                let alpha = self.player.slashing as f32 / 8.0;
                let slash_x = if self.player.facing > 0.0 { px + 12.0 } else { px - 14.0 };
                draw_texture_ex(&self.tex_slash, slash_x, py - 4.0, Color::new(1.0, 1.0, 1.0, alpha), DrawTextureParams {
                    dest_size: Some(Vec2::new(14.0, 14.0)),
                    flip_x: self.player.facing < 0.0,
                    ..Default::default()
                });
            }
        }

        // Particles
        for p in &self.particles {
            let px = p.x - cam_x + sx;
            let py = p.y - cam_y + sy;
            let alpha = p.life as f32 / p.max_life as f32;
            draw_rectangle(px, py, p.size, p.size, Color::new(p.color.r, p.color.g, p.color.b, alpha));
        }

        // Glitch effect
        if self.glitch_timer > 0 {
            for _ in 0..5 {
                let gy = rand::gen_range(0.0_f32, SCREEN_H);
                let gh = 2.0 + rand::gen_range(0.0_f32, 6.0);
                let gdx = (rand::gen_range(0.0_f32, 1.0) - 0.5) * 10.0;
                draw_rectangle(gdx, gy, SCREEN_W, gh, Color::new(1.0, 0.0, 0.5, 0.1));
            }
        }

        // Damage flash
        if self.damage_flash > 0 {
            draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(1.0, 0.0, 0.0, self.damage_flash as f32 / 16.0));
        }

        // HUD
        self.draw_hud(level_color);
    }

    fn draw_hud(&self, level_color: Color) {
        // Health bars
        for i in 0..3 {
            let color = if i < self.player.hp { COL_MAGENTA } else { Color::new(0.2, 0.2, 0.2, 1.0) };
            draw_rectangle(10.0 + i as f32 * 22.0, 10.0, 18.0, 8.0, color);
        }

        // Lives
        draw_text(&format!("x{}", self.lives), 80.0, 19.0, 10.0, COL_BLUE);

        // Score
        let score_str = format!("{:06}", self.score);
        let sw = measure_text(&score_str, None, 10, 1.0).width;
        draw_text(&score_str, SCREEN_W - 10.0 - sw, 19.0, 10.0, COL_AMBER);

        // EMP count
        draw_text(&format!("EMP:{}", self.player.emps), 10.0, 32.0, 10.0, COL_BLUE);

        // Level name
        let name = LEVEL_NAMES[self.current_level.min(2)];
        let nw = measure_text(name, None, 8, 1.0).width;
        draw_text(name, SCREEN_W / 2.0 - nw / 2.0, 16.0, 8.0, level_color);

        // Dash cooldown
        if self.player.dash_cd > 0 {
            let frac = 1.0 - self.player.dash_cd as f32 / DASH_COOLDOWN as f32;
            draw_rectangle(10.0, 40.0, 40.0 * frac, 4.0, Color::new(0.0, 1.0, 0.255, 0.3));
        } else {
            draw_rectangle(10.0, 40.0, 40.0, 4.0, COL_GREEN);
        }
        draw_text("DASH", 12.0, 52.0, 6.0, COL_GREEN);
    }

    fn draw_game_over(&self) {
        // Static noise
        for _ in 0..300 {
            let nx = rand::gen_range(0.0_f32, SCREEN_W);
            let ny = rand::gen_range(0.0_f32, SCREEN_H);
            let ns = 1.0 + rand::gen_range(0.0_f32, 3.0);
            draw_rectangle(nx, ny, ns, ns, Color::new(1.0, 0.0, 0.5, 0.05 + rand::gen_range(0.0_f32, 0.1)));
        }

        let title1 = "CONNECTION";
        let tw1 = measure_text(title1, None, 24, 1.0).width;
        let gx = if (self.global_frame as f32 * 0.2).sin() > 0.8 { (rand::gen_range(0.0_f32, 1.0) - 0.5) * 8.0 } else { 0.0 };
        draw_text(title1, SCREEN_W / 2.0 - tw1 / 2.0 + gx - 2.0, 180.0, 24.0, Color::new(1.0, 0.0, 0.5, 0.5));
        draw_text(title1, SCREEN_W / 2.0 - tw1 / 2.0 + gx + 2.0, 180.0, 24.0, Color::new(0.0, 0.83, 1.0, 0.5));
        draw_text(title1, SCREEN_W / 2.0 - tw1 / 2.0 + gx, 180.0, 24.0, WHITE);

        let title2 = "LOST";
        let tw2 = measure_text(title2, None, 20, 1.0).width;
        draw_text(title2, SCREEN_W / 2.0 - tw2 / 2.0, 220.0, 20.0, COL_MAGENTA);

        let score_text = format!("SCORE: {:06}", self.score);
        let stw = measure_text(&score_text, None, 10, 1.0).width;
        draw_text(&score_text, SCREEN_W / 2.0 - stw / 2.0, 280.0, 10.0, COL_AMBER);

        if (self.global_frame / 30) % 2 != 0 {
            let prompt = "RECONNECT?";
            let pw = measure_text(prompt, None, 10, 1.0).width;
            draw_text(prompt, SCREEN_W / 2.0 - pw / 2.0, 340.0, 10.0, COL_GREEN);
        }
    }

    fn draw_win(&self) {
        clear_background(Color::new(0.008, 0.03, 0.03, 1.0));

        let title1 = "TRUTH";
        let tw1 = measure_text(title1, None, 20, 1.0).width;
        draw_text(title1, SCREEN_W / 2.0 - tw1 / 2.0, 160.0, 20.0, COL_GREEN);

        let title2 = "BROADCAST";
        let tw2 = measure_text(title2, None, 20, 1.0).width;
        draw_text(title2, SCREEN_W / 2.0 - tw2 / 2.0, 190.0, 20.0, COL_GREEN);

        let score_text = format!("FINAL SCORE: {:06}", self.score);
        let stw = measure_text(&score_text, None, 10, 1.0).width;
        draw_text(&score_text, SCREEN_W / 2.0 - stw / 2.0, 260.0, 10.0, COL_AMBER);

        let msg1 = "THE SIGNAL IS OUT.";
        let mw1 = measure_text(msg1, None, 8, 1.0).width;
        draw_text(msg1, SCREEN_W / 2.0 - mw1 / 2.0, 310.0, 8.0, COL_BLUE);

        let msg2 = "NEO-KYOTO IS FREE.";
        let mw2 = measure_text(msg2, None, 8, 1.0).width;
        draw_text(msg2, SCREEN_W / 2.0 - mw2 / 2.0, 330.0, 8.0, COL_BLUE);

        if (self.global_frame / 30) % 2 != 0 {
            let prompt = "JACK IN AGAIN?";
            let pw = measure_text(prompt, None, 10, 1.0).width;
            draw_text(prompt, SCREEN_W / 2.0 - pw / 2.0, 400.0, 10.0, COL_MAGENTA);
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

        if dt > 0.1 { dt = 0.1; }

        accumulator += dt;

        while accumulator >= TIME_STEP {
            game.update();
            accumulator -= TIME_STEP;
        }

        game.draw();
        next_frame().await;
    }
}
