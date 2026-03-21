// Chrome Viper - Cyberpunk Spaceship Action Game
// Rust/Macroquad port for Miyoo Mini Plus
// Story: "Neon Abyss"
// Synced with web version (800x600, shield generators, EMP waves, boss phases)

use macroquad::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------
const SCREEN_W: f32 = 800.0;
const SCREEN_H: f32 = 600.0;
const TIME_STEP: f64 = 1.0 / 60.0;

const PLAYER_SPEED: f32 = 4.0;
const BULLET_SPEED: f32 = 7.0;
const SCROLL_SPEED: f32 = 1.0;
const PLAYER_MOVE_BOUND: f32 = SCREEN_W * 0.4; // left 40% of screen

const DROP_CHANCE: f32 = 0.12;

// Cyberpunk colors
const NEON_PINK: Color = Color::new(1.0, 0.08, 0.58, 1.0);
const NEON_CYAN: Color = Color::new(0.0, 0.95, 1.0, 1.0);
const NEON_GREEN: Color = Color::new(0.22, 1.0, 0.08, 1.0);
const NEON_PURPLE: Color = Color::new(0.545, 0.0, 1.0, 1.0);
const NEON_ORANGE: Color = Color::new(1.0, 0.5, 0.0, 1.0);
const DARK_BG: Color = Color::new(0.04, 0.04, 0.1, 1.0);
const TERMINAL_GREEN: Color = Color::new(0.22, 1.0, 0.08, 1.0);

// ---------------------------------------------------------------------------
// Game States
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Start,
    LevelStory,
    Playing,
    GameOver,
    Win,
}

// ---------------------------------------------------------------------------
// Enemy Types
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq)]
enum EnemyType {
    Drone,
    Gunship,
    Turret,
    ShieldGen,
}

// ---------------------------------------------------------------------------
// Power-up Types
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq)]
enum PowerUpKind {
    Spread,
    Homing,
    Emp,
    Shield,
}

// ---------------------------------------------------------------------------
// Weapon Types
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq)]
enum WeaponType {
    DualLaser,
    SpreadShot,
    HomingMissile,
}

// ---------------------------------------------------------------------------
// Wave spawn pattern
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq)]
enum SpawnPattern {
    Line,
    Sine,
    Vee,
    Random,
}

// ---------------------------------------------------------------------------
// Wave event (frame-based like web version)
// ---------------------------------------------------------------------------
struct WaveEvent {
    time: i64,
    etype: Option<EnemyType>, // None means boss
    count: u32,
    pattern: SpawnPattern,
    y: f32,
    spacing: f32,
    boss_type: i32, // only used when etype is None
}

// ---------------------------------------------------------------------------
// Story texts (matching web version)
// ---------------------------------------------------------------------------
const STORY_TEXTS: &[&str] = &[
    // Before Level 1
    "AXIOM CORP CLASSIFIED -- 2187.03.15\n\n\
     > INTERCEPTED TRANSMISSION:\n\n\
     The orbital colonies are ours. Every\n\
     transit hub, every station, every\n\
     breathing human above the atmosphere\n\
     now answers to AXIOM.\n\n\
     But one asset has gone missing.\n\
     Prototype CV-7 'Chrome Viper' --\n\
     stolen from Hangar 9 by an unknown\n\
     pilot.\n\n\
     > MISSION: Breach the Orbital Ring.\n\
     > STATUS: LAUNCH READY",
    // After Level 1
    "CHROME VIPER FLIGHT LOG -- 2187.03.15\n\n\
     Outer defense ring breached. The\n\
     Defense Satellite is scrap metal.\n\n\
     But the data cores I recovered...\n\
     AXIOM isn't just controlling the\n\
     colonies. They're building something\n\
     in the Neon Corridor -- a weapon\n\
     that could glass every city on Earth.\n\n\
     Project LEVIATHAN.\n\n\
     > Proceeding to Sector 7-G.\n\
     > The Neon Corridor awaits.",
    // After Level 2
    "CHROME VIPER FLIGHT LOG -- 2187.03.15\n\n\
     The Carrier is down, but now I see it.\n\
     Through the debris field, past the\n\
     neon haze of the dying corridor...\n\n\
     The Leviathan.\n\n\
     It's massive. A dreadnought the size\n\
     of a colony. And it's powering up\n\
     its main cannon -- aimed at Earth.\n\n\
     This is it. No backup. No retreat.\n\
     Just me and the Chrome Viper.\n\n\
     > FINAL APPROACH: The Abyss.",
    // Victory
    "AXIOM EMERGENCY BROADCAST -- ALL FREQ\n\n\
     [SIGNAL LOST]\n\
     [SIGNAL LOST]\n\
     [SIGNAL LOST]\n\n\
     The Leviathan is destroyed.\n\
     AXIOM's orbital network is collapsing.\n\n\
     The colonies are free.\n\n\
     But as the Chrome Viper drifts through\n\
     the wreckage, its pilot knows the\n\
     truth: corporations don't die.\n\
     They rebrand.\n\n\
     Somewhere in the neon dark,\n\
     a new signal flickers to life.\n\n\
     > CHROME VIPER -- MISSION COMPLETE\n\
     > PILOT STATUS: ALIVE\n\
     > COST: EVERYTHING",
];

const LEVEL_NAMES: &[&str] = &["ORBITAL RING", "NEON CORRIDOR", "THE ABYSS"];
const BOSS_NAMES: &[&str] = &["DEFENSE SATELLITE", "CYBORG CARRIER", "LEVIATHAN"];

// ---------------------------------------------------------------------------
// Generate waves for a level (matching web version)
// ---------------------------------------------------------------------------
fn generate_waves(level: usize) -> Vec<WaveEvent> {
    let mut waves = Vec::new();
    match level {
        0 => {
            waves.push(WaveEvent { time: 60, etype: Some(EnemyType::Drone), count: 5, pattern: SpawnPattern::Line, y: 100.0, spacing: 30.0, boss_type: 0 });
            waves.push(WaveEvent { time: 180, etype: Some(EnemyType::Drone), count: 5, pattern: SpawnPattern::Line, y: 350.0, spacing: 30.0, boss_type: 0 });
            waves.push(WaveEvent { time: 300, etype: Some(EnemyType::Drone), count: 8, pattern: SpawnPattern::Sine, y: 240.0, spacing: 25.0, boss_type: 0 });
            waves.push(WaveEvent { time: 450, etype: Some(EnemyType::Gunship), count: 2, pattern: SpawnPattern::Line, y: 150.0, spacing: 80.0, boss_type: 0 });
            waves.push(WaveEvent { time: 550, etype: Some(EnemyType::Drone), count: 6, pattern: SpawnPattern::Vee, y: 200.0, spacing: 25.0, boss_type: 0 });
            waves.push(WaveEvent { time: 700, etype: Some(EnemyType::Gunship), count: 3, pattern: SpawnPattern::Line, y: 300.0, spacing: 60.0, boss_type: 0 });
            waves.push(WaveEvent { time: 850, etype: Some(EnemyType::Drone), count: 10, pattern: SpawnPattern::Sine, y: 240.0, spacing: 20.0, boss_type: 0 });
            waves.push(WaveEvent { time: 1000, etype: Some(EnemyType::Gunship), count: 2, pattern: SpawnPattern::Random, y: 0.0, spacing: 0.0, boss_type: 0 });
            waves.push(WaveEvent { time: 1100, etype: Some(EnemyType::Drone), count: 8, pattern: SpawnPattern::Line, y: 120.0, spacing: 25.0, boss_type: 0 });
            waves.push(WaveEvent { time: 1200, etype: Some(EnemyType::Drone), count: 8, pattern: SpawnPattern::Line, y: 360.0, spacing: 25.0, boss_type: 0 });
            waves.push(WaveEvent { time: 1400, etype: None, count: 0, pattern: SpawnPattern::Line, y: 0.0, spacing: 0.0, boss_type: 0 });
        }
        1 => {
            waves.push(WaveEvent { time: 60, etype: Some(EnemyType::Drone), count: 8, pattern: SpawnPattern::Sine, y: 150.0, spacing: 20.0, boss_type: 0 });
            waves.push(WaveEvent { time: 150, etype: Some(EnemyType::Turret), count: 3, pattern: SpawnPattern::Line, y: 80.0, spacing: 100.0, boss_type: 0 });
            waves.push(WaveEvent { time: 250, etype: Some(EnemyType::Gunship), count: 3, pattern: SpawnPattern::Line, y: 300.0, spacing: 50.0, boss_type: 0 });
            waves.push(WaveEvent { time: 350, etype: Some(EnemyType::Drone), count: 10, pattern: SpawnPattern::Vee, y: 240.0, spacing: 20.0, boss_type: 0 });
            waves.push(WaveEvent { time: 450, etype: Some(EnemyType::Turret), count: 4, pattern: SpawnPattern::Line, y: 400.0, spacing: 80.0, boss_type: 0 });
            waves.push(WaveEvent { time: 550, etype: Some(EnemyType::Gunship), count: 4, pattern: SpawnPattern::Random, y: 0.0, spacing: 0.0, boss_type: 0 });
            waves.push(WaveEvent { time: 650, etype: Some(EnemyType::Drone), count: 12, pattern: SpawnPattern::Sine, y: 200.0, spacing: 18.0, boss_type: 0 });
            waves.push(WaveEvent { time: 750, etype: Some(EnemyType::Turret), count: 2, pattern: SpawnPattern::Line, y: 120.0, spacing: 150.0, boss_type: 0 });
            waves.push(WaveEvent { time: 750, etype: Some(EnemyType::Turret), count: 2, pattern: SpawnPattern::Line, y: 360.0, spacing: 150.0, boss_type: 0 });
            waves.push(WaveEvent { time: 900, etype: Some(EnemyType::Gunship), count: 5, pattern: SpawnPattern::Line, y: 240.0, spacing: 40.0, boss_type: 0 });
            waves.push(WaveEvent { time: 1050, etype: Some(EnemyType::Drone), count: 15, pattern: SpawnPattern::Sine, y: 240.0, spacing: 15.0, boss_type: 0 });
            waves.push(WaveEvent { time: 1200, etype: Some(EnemyType::ShieldGen), count: 2, pattern: SpawnPattern::Line, y: 150.0, spacing: 200.0, boss_type: 0 });
            waves.push(WaveEvent { time: 1350, etype: None, count: 0, pattern: SpawnPattern::Line, y: 0.0, spacing: 0.0, boss_type: 1 });
        }
        _ => {
            waves.push(WaveEvent { time: 60, etype: Some(EnemyType::Drone), count: 10, pattern: SpawnPattern::Sine, y: 120.0, spacing: 18.0, boss_type: 0 });
            waves.push(WaveEvent { time: 60, etype: Some(EnemyType::Drone), count: 10, pattern: SpawnPattern::Sine, y: 360.0, spacing: 18.0, boss_type: 0 });
            waves.push(WaveEvent { time: 200, etype: Some(EnemyType::Gunship), count: 4, pattern: SpawnPattern::Random, y: 0.0, spacing: 0.0, boss_type: 0 });
            waves.push(WaveEvent { time: 300, etype: Some(EnemyType::Turret), count: 5, pattern: SpawnPattern::Line, y: 80.0, spacing: 70.0, boss_type: 0 });
            waves.push(WaveEvent { time: 400, etype: Some(EnemyType::Drone), count: 15, pattern: SpawnPattern::Vee, y: 240.0, spacing: 15.0, boss_type: 0 });
            waves.push(WaveEvent { time: 500, etype: Some(EnemyType::Gunship), count: 5, pattern: SpawnPattern::Line, y: 200.0, spacing: 40.0, boss_type: 0 });
            waves.push(WaveEvent { time: 500, etype: Some(EnemyType::Turret), count: 3, pattern: SpawnPattern::Line, y: 400.0, spacing: 100.0, boss_type: 0 });
            waves.push(WaveEvent { time: 650, etype: Some(EnemyType::Drone), count: 20, pattern: SpawnPattern::Sine, y: 240.0, spacing: 12.0, boss_type: 0 });
            waves.push(WaveEvent { time: 800, etype: Some(EnemyType::ShieldGen), count: 3, pattern: SpawnPattern::Line, y: 120.0, spacing: 120.0, boss_type: 0 });
            waves.push(WaveEvent { time: 900, etype: Some(EnemyType::Gunship), count: 6, pattern: SpawnPattern::Random, y: 0.0, spacing: 0.0, boss_type: 0 });
            waves.push(WaveEvent { time: 1000, etype: Some(EnemyType::Drone), count: 12, pattern: SpawnPattern::Line, y: 100.0, spacing: 20.0, boss_type: 0 });
            waves.push(WaveEvent { time: 1000, etype: Some(EnemyType::Drone), count: 12, pattern: SpawnPattern::Line, y: 380.0, spacing: 20.0, boss_type: 0 });
            waves.push(WaveEvent { time: 1150, etype: Some(EnemyType::Turret), count: 6, pattern: SpawnPattern::Line, y: 240.0, spacing: 50.0, boss_type: 0 });
            waves.push(WaveEvent { time: 1300, etype: None, count: 0, pattern: SpawnPattern::Line, y: 0.0, spacing: 0.0, boss_type: 2 });
        }
    }
    waves
}

// ---------------------------------------------------------------------------
// Structs
// ---------------------------------------------------------------------------
#[derive(Clone)]
struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    shields: i32,
    max_shields: i32,
    weapon: WeaponType,
    weapon_timer: i32,
    fire_timer: i32,
    invulnerable: i32,
    emp_charges: i32,
    emp_cooldown: i32,
    alive: bool,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 80.0,
            y: SCREEN_H / 2.0,
            w: 32.0,
            h: 32.0,
            shields: 3,
            max_shields: 3,
            weapon: WeaponType::DualLaser,
            weapon_timer: 0,
            fire_timer: 0,
            invulnerable: 0,
            emp_charges: 1,
            emp_cooldown: 0,
            alive: true,
        }
    }
}

#[derive(Clone)]
struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: i32,
    is_player: bool,
    alive: bool,
    btype: BulletType,
}

#[derive(Clone, Copy, PartialEq)]
enum BulletType {
    Laser,
    Spread,
    Homing,
    Enemy,
}

#[derive(Clone)]
struct Enemy {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    etype: EnemyType,
    hp: i32,
    max_hp: i32,
    speed: f32,
    points: u32,
    alive: bool,
    shoot_timer: f32,
    shoot_rate: f32,
    sine_offset: f32,
    flash_timer: i32,
}

#[derive(Clone, Copy)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    max_life: f32,
    color: Color,
    alive: bool,
    size: f32,
}

#[derive(Clone, Copy)]
struct Star {
    x: f32,
    y: f32,
    size: f32,
    speed: f32,
    brightness: f32,
    layer: u8,
}

impl Star {
    fn new_random_x(layer: u8) -> Self {
        Self::new_at(rand::gen_range(0.0, SCREEN_W), layer)
    }

    fn new_at(x: f32, layer: u8) -> Self {
        let (speed, size, bright_base) = match layer {
            0 => (0.3 + rand::gen_range(0.0f32, 0.5), 1.0f32, 0.3 + rand::gen_range(0.0f32, 0.3)),
            1 => (0.8 + rand::gen_range(0.0f32, 0.7), 1.5f32, 0.4 + rand::gen_range(0.0f32, 0.3)),
            _ => (1.5 + rand::gen_range(0.0f32, 1.0), 2.0f32, 0.6 + rand::gen_range(0.0f32, 0.4)),
        };
        Self {
            x,
            y: rand::gen_range(0.0, SCREEN_H),
            size,
            speed: speed * SCROLL_SPEED,
            brightness: bright_base,
            layer,
        }
    }
}

#[derive(Clone)]
struct PowerUp {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    kind: PowerUpKind,
    alive: bool,
    angle: f32,
}

impl PowerUp {
    fn new(x: f32, y: f32) -> Self {
        let r = rand::gen_range(0.0f32, 1.0);
        let kind = if r < 0.25 {
            PowerUpKind::Spread
        } else if r < 0.5 {
            PowerUpKind::Homing
        } else if r < 0.75 {
            PowerUpKind::Shield
        } else {
            PowerUpKind::Emp
        };
        Self { x, y, w: 12.0, h: 12.0, kind, alive: true, angle: 0.0 }
    }
}

#[derive(Clone)]
struct FloatingText {
    x: f32,
    y: f32,
    text: String,
    life: i32,
    color: Color,
}

#[derive(Clone)]
struct EmpWave {
    x: f32,
    y: f32,
    radius: f32,
    max_radius: f32,
    speed: f32,
    alive: bool,
}

// ---------------------------------------------------------------------------
// Boss (separate entity like web version)
// ---------------------------------------------------------------------------
#[derive(Clone)]
struct Boss {
    boss_type: i32,
    x: f32,
    y: f32,
    target_x: f32,
    w: f32,
    h: f32,
    hp: i32,
    max_hp: i32,
    fire_timer: i64,
    drone_timer: i64,
    sweep_angle: f32,
    angle: f32,
    alive: bool,
    enter_phase: bool,
}

// ---------------------------------------------------------------------------
// City building for background
// ---------------------------------------------------------------------------
#[derive(Clone)]
struct CityBuilding {
    x: f32,
    w: f32,
    h: f32,
    color: Color,
    has_windows: bool,
}

// ---------------------------------------------------------------------------
// Digital rain column
// ---------------------------------------------------------------------------
#[derive(Clone)]
struct RainColumn {
    x: f32,
    y: f32,
    speed: f32,
    len: usize,
}

// ---------------------------------------------------------------------------
// Main Game
// ---------------------------------------------------------------------------
struct Game {
    state: GameState,
    frame: i64,
    score: u32,
    current_level: usize,

    // Wave system (frame-based like web)
    level_waves: Vec<WaveEvent>,
    level_timer: i64,
    wave_index: usize,

    player: Player,
    bullets: Vec<Bullet>,
    enemy_bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    particles: Vec<Particle>,
    stars: Vec<Star>,
    power_ups: Vec<PowerUp>,
    floating_texts: Vec<FloatingText>,
    emp_waves: Vec<EmpWave>,

    // Boss (separate like web version)
    boss: Option<Boss>,
    boss_active: bool,
    boss_warning_shown: bool,

    // screen shake
    shake_mag: f32,
    shake_x: f32,
    shake_y: f32,

    // chain/combo
    chain_timer: i32,
    chain_multiplier: u32,

    // aberration timer
    aberration_timer: i32,

    // story typewriter
    story_text: String,
    story_char_idx: usize,
    story_timer: i32,
    story_phase: usize,

    // city background
    city_buildings: Vec<CityBuilding>,
    city_scroll_x: f32,

    // digital rain
    rain_columns: Vec<RainColumn>,

    // boss defeat delay
    boss_defeat_timer: i32,
}

impl Game {
    fn new() -> Self {
        let mut stars = Vec::new();
        for _ in 0..80 {
            stars.push(Star::new_random_x(0));
        }
        for _ in 0..50 {
            stars.push(Star::new_random_x(1));
        }
        for _ in 0..30 {
            stars.push(Star::new_random_x(2));
        }

        let city_buildings = Self::init_city();
        let rain_columns = Self::init_rain();

        Self {
            state: GameState::Start,
            frame: 0,
            score: 0,
            current_level: 0,

            level_waves: Vec::new(),
            level_timer: 0,
            wave_index: 0,

            player: Player::new(),
            bullets: Vec::new(),
            enemy_bullets: Vec::new(),
            enemies: Vec::new(),
            particles: Vec::new(),
            stars,
            power_ups: Vec::new(),
            floating_texts: Vec::new(),
            emp_waves: Vec::new(),

            boss: None,
            boss_active: false,
            boss_warning_shown: false,

            shake_mag: 0.0,
            shake_x: 0.0,
            shake_y: 0.0,

            chain_timer: 0,
            chain_multiplier: 1,

            aberration_timer: 0,

            story_text: String::new(),
            story_char_idx: 0,
            story_timer: 0,
            story_phase: 0,

            city_buildings,
            city_scroll_x: 0.0,

            rain_columns,

            boss_defeat_timer: 0,
        }
    }

    fn init_city() -> Vec<CityBuilding> {
        let mut buildings = Vec::new();
        for i in 0..30 {
            buildings.push(CityBuilding {
                x: i as f32 * 60.0 + rand::gen_range(0.0f32, 20.0),
                w: 15.0 + rand::gen_range(0.0f32, 30.0),
                h: 30.0 + rand::gen_range(0.0f32, 80.0),
                color: if rand::gen_range(0.0f32, 1.0) < 0.5 {
                    Color::new(0.0, 0.95, 1.0, 0.03)
                } else {
                    Color::new(1.0, 0.18, 0.47, 0.02)
                },
                has_windows: rand::gen_range(0.0f32, 1.0) < 0.6,
            });
        }
        buildings
    }

    fn init_rain() -> Vec<RainColumn> {
        let mut cols = Vec::new();
        for _ in 0..40 {
            cols.push(RainColumn {
                x: rand::gen_range(0.0, SCREEN_W),
                y: rand::gen_range(0.0, SCREEN_H),
                speed: 1.0 + rand::gen_range(0.0f32, 3.0),
                len: 5 + rand::gen_range(0u32, 15) as usize,
            });
        }
        cols
    }

    fn reset_game(&mut self) {
        self.player = Player::new();
        self.bullets.clear();
        self.enemy_bullets.clear();
        self.enemies.clear();
        self.power_ups.clear();
        self.floating_texts.clear();
        self.emp_waves.clear();
        self.particles.clear();
        self.score = 0;
        self.chain_timer = 0;
        self.chain_multiplier = 1;
        self.current_level = 0;
        self.story_phase = 0;
        self.level_timer = 0;
        self.wave_index = 0;
        self.boss_active = false;
        self.boss = None;
        self.boss_warning_shown = false;
        self.shake_mag = 0.0;
        self.shake_x = 0.0;
        self.shake_y = 0.0;
        self.aberration_timer = 0;
        self.boss_defeat_timer = 0;
    }

    // ------------------------------------------------------------------
    // Helpers
    // ------------------------------------------------------------------
    fn trigger_shake(&mut self, mag: f32) {
        if mag > self.shake_mag {
            self.shake_mag = mag;
        }
    }

    fn spawn_particles(&mut self, x: f32, y: f32, count: usize, color: Color, speed_scale: f32) {
        for _ in 0..count {
            let angle: f32 = rand::gen_range(0.0, std::f32::consts::TAU);
            let spd: f32 = (0.5 + rand::gen_range(0.0f32, 1.0)) * speed_scale;
            self.particles.push(Particle {
                x,
                y,
                vx: angle.cos() * spd,
                vy: angle.sin() * spd,
                life: 30.0 + rand::gen_range(0.0f32, 30.0),
                max_life: 60.0,
                color,
                alive: true,
                size: 1.0 + rand::gen_range(0.0f32, 2.0),
            });
        }
    }

    fn aabb_overlap(ax: f32, ay: f32, aw: f32, ah: f32, bx: f32, by: f32, bw: f32, bh: f32) -> bool {
        ax + aw > bx && ax < bx + bw && ay + ah > by && ay < by + bh
    }

    // ------------------------------------------------------------------
    // State transitions
    // ------------------------------------------------------------------
    fn start_level_story(&mut self, phase: usize) {
        self.state = GameState::LevelStory;
        self.story_phase = phase;
        self.story_text = STORY_TEXTS[phase.min(STORY_TEXTS.len() - 1)].to_string();
        self.story_char_idx = 0;
        self.story_timer = 0;
    }

    fn start_level(&mut self, lvl: usize) {
        self.current_level = lvl;
        self.level_waves = generate_waves(lvl);
        self.level_timer = 0;
        self.wave_index = 0;
        self.boss_active = false;
        self.boss = None;
        self.boss_warning_shown = false;
        self.enemies.clear();
        self.enemy_bullets.clear();
        self.power_ups.clear();
        self.emp_waves.clear();
        self.player.x = 80.0;
        self.player.y = SCREEN_H / 2.0;
        self.player.fire_timer = 0;
        self.boss_defeat_timer = 0;
        self.state = GameState::Playing;
    }

    fn hit_player(&mut self) {
        if self.player.invulnerable > 0 || !self.player.alive {
            return;
        }
        self.player.shields -= 1;
        self.player.invulnerable = 90;
        self.aberration_timer = 15;
        self.trigger_shake(5.0);
        let px = self.player.x + self.player.w / 2.0;
        let py = self.player.y + self.player.h / 2.0;
        self.spawn_particles(px, py, 10, NEON_CYAN, 2.0);

        if self.player.shields <= 0 {
            self.player.alive = false;
            self.spawn_particles(px, py, 30, NEON_PINK, 3.0);
            // Delay game over
            self.boss_defeat_timer = 90; // reuse for death delay
        }
    }

    // ------------------------------------------------------------------
    // Create boss (matching web version)
    // ------------------------------------------------------------------
    fn create_boss(&mut self, boss_type: i32) {
        self.boss_active = true;
        self.boss_warning_shown = false;
        let (target_x, w, h, hp) = match boss_type {
            0 => (SCREEN_W - 100.0, 54.0f32, 48.0f32, 50),
            1 => (SCREEN_W - 110.0, 60.0f32, 54.0f32, 80),
            _ => (SCREEN_W - 130.0, 72.0f32, 60.0f32, 120),
        };
        self.boss = Some(Boss {
            boss_type,
            x: SCREEN_W + 50.0 + boss_type as f32 * 10.0,
            y: SCREEN_H / 2.0,
            target_x,
            w,
            h,
            hp,
            max_hp: hp,
            fire_timer: 0,
            drone_timer: 0,
            sweep_angle: 0.0,
            angle: 0.0,
            alive: true,
            enter_phase: true,
        });
    }

    // ------------------------------------------------------------------
    // Spawn wave enemies (with patterns like web)
    // ------------------------------------------------------------------
    fn spawn_wave_enemies(&mut self, etype: EnemyType, count: u32, pattern: SpawnPattern, base_y: f32, spacing: f32) {
        for i in 0..count {
            let mut ex = SCREEN_W + 20.0 + i as f32 * spacing.max(30.0);
            let mut ey = base_y;
            match pattern {
                SpawnPattern::Random => {
                    ey = 40.0 + rand::gen_range(0.0f32, SCREEN_H - 80.0);
                    ex = SCREEN_W + 20.0 + i as f32 * 60.0;
                }
                SpawnPattern::Vee => {
                    let mid = count / 2;
                    ey = base_y + (i as i32 - mid as i32).unsigned_abs() as f32 * 25.0;
                }
                SpawnPattern::Sine => {
                    ey = base_y + (i as f32 * 0.8).sin() * 60.0;
                }
                SpawnPattern::Line => {}
            }
            ey = ey.clamp(20.0, SCREEN_H - 20.0);

            let (hp, speed, shoot_rate, w, h, points) = match etype {
                EnemyType::Drone => (1, 2.0 + rand::gen_range(0.0f32, 1.5), 9999.0f32, 16.0f32, 16.0f32, 100u32),
                EnemyType::Gunship => (3, 1.2f32, 90.0f32, 24.0f32, 24.0f32, 250u32),
                EnemyType::Turret => (5, 0.8f32, 60.0f32, 20.0f32, 20.0f32, 300u32),
                EnemyType::ShieldGen => (8, 0.6f32, 120.0f32, 24.0f32, 24.0f32, 500u32),
            };

            self.enemies.push(Enemy {
                x: ex,
                y: ey,
                w,
                h,
                etype,
                hp,
                max_hp: hp,
                speed,
                points,
                alive: true,
                shoot_timer: rand::gen_range(0.0f32, shoot_rate.min(999.0)),
                shoot_rate,
                sine_offset: rand::gen_range(0.0f32, std::f32::consts::TAU),
                flash_timer: 0,
            });
        }
    }

    // ------------------------------------------------------------------
    // Player shooting (matching web fire rates)
    // ------------------------------------------------------------------
    fn player_shoot(&mut self) {
        let px = self.player.x + self.player.w;
        let py_mid = self.player.y + self.player.h / 2.0;

        match self.player.weapon {
            WeaponType::DualLaser => {
                self.player.fire_timer = 8;
                self.bullets.push(Bullet {
                    x: px, y: self.player.y + 6.0,
                    vx: BULLET_SPEED, vy: 0.0, life: 120,
                    is_player: true, alive: true, btype: BulletType::Laser,
                });
                self.bullets.push(Bullet {
                    x: px, y: self.player.y + self.player.h - 6.0,
                    vx: BULLET_SPEED, vy: 0.0, life: 120,
                    is_player: true, alive: true, btype: BulletType::Laser,
                });
            }
            WeaponType::SpreadShot => {
                self.player.fire_timer = 12;
                for vy in &[-1.5f32, 0.0, 1.5] {
                    self.bullets.push(Bullet {
                        x: px, y: py_mid,
                        vx: BULLET_SPEED, vy: *vy, life: 100,
                        is_player: true, alive: true, btype: BulletType::Spread,
                    });
                }
            }
            WeaponType::HomingMissile => {
                self.player.fire_timer = 18;
                self.bullets.push(Bullet {
                    x: px, y: py_mid,
                    vx: 5.0, vy: 0.0, life: 180,
                    is_player: true, alive: true, btype: BulletType::Homing,
                });
            }
        }
        let bx = px;
        let by = py_mid;
        self.spawn_particles(bx, by, 3, NEON_CYAN, 1.0);
    }

    // ------------------------------------------------------------------
    // Update
    // ------------------------------------------------------------------
    fn update(&mut self) {
        self.frame += 1;

        // Stars always update
        for s in self.stars.iter_mut() {
            s.x -= s.speed;
            if s.x < -2.0 {
                s.x = SCREEN_W + 2.0;
                s.y = rand::gen_range(0.0, SCREEN_H);
            }
        }

        // Rain always update
        for col in self.rain_columns.iter_mut() {
            col.y += col.speed;
            if col.y > SCREEN_H + col.len as f32 * 14.0 {
                col.y = -(col.len as f32 * 14.0);
                col.x = rand::gen_range(0.0, SCREEN_W);
            }
        }

        // Particles always update
        for p in self.particles.iter_mut() {
            p.x += p.vx;
            p.y += p.vy;
            p.vx *= 0.97;
            p.vy *= 0.97;
            p.life -= 1.0;
            if p.life <= 0.0 {
                p.alive = false;
            }
        }
        self.particles.retain(|p| p.alive);

        // Input
        let enter = is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter);
        let fire_pressed = is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Space);

        // ----- Start screen -----
        if self.state == GameState::Start {
            if enter || fire_pressed {
                self.reset_game();
                self.start_level_story(0);
            }
            return;
        }

        // ----- Level Story -----
        if self.state == GameState::LevelStory {
            self.story_timer += 1;
            if self.story_timer % 2 == 0 && self.story_char_idx < self.story_text.len() {
                self.story_char_idx += 1;
            }
            if (fire_pressed || enter) && self.story_timer > 30 {
                if self.story_char_idx < self.story_text.len() {
                    self.story_char_idx = self.story_text.len();
                } else if self.story_phase <= 2 {
                    self.start_level(self.story_phase);
                }
            }
            return;
        }

        // ----- Game Over -----
        if self.state == GameState::GameOver {
            if (enter || fire_pressed) && self.frame > 60 {
                self.state = GameState::Start;
            }
            return;
        }

        // ----- Win -----
        if self.state == GameState::Win {
            self.story_timer += 1;
            if self.story_timer % 2 == 0 && self.story_char_idx < self.story_text.len() {
                self.story_char_idx += 1;
            }
            if (fire_pressed || enter) && self.story_timer > 30 {
                if self.story_char_idx < self.story_text.len() {
                    self.story_char_idx = self.story_text.len();
                } else {
                    self.state = GameState::Start;
                }
            }
            return;
        }

        // ----- Playing -----
        // Check for death delay / boss defeat delay
        if self.boss_defeat_timer > 0 {
            self.boss_defeat_timer -= 1;
            if self.boss_defeat_timer <= 0 {
                if !self.player.alive {
                    self.state = GameState::GameOver;
                    self.frame = 0;
                    return;
                }
                // Boss defeat: advance level
                self.boss_active = false;
                if self.current_level < 2 {
                    let next_phase = self.current_level + 1;
                    self.start_level_story(next_phase);
                } else {
                    // Victory!
                    self.story_text = STORY_TEXTS[3].to_string();
                    self.story_char_idx = 0;
                    self.story_timer = 0;
                    self.state = GameState::Win;
                }
                return;
            }
        }

        // City scroll
        self.city_scroll_x += 0.3;

        // Level timer & wave spawning
        self.level_timer += 1;
        while self.wave_index < self.level_waves.len() && self.level_waves[self.wave_index].time <= self.level_timer {
            let wave = &self.level_waves[self.wave_index];
            if let Some(etype) = wave.etype {
                let count = wave.count;
                let pattern = wave.pattern;
                let y = wave.y;
                let spacing = wave.spacing;
                self.wave_index += 1;
                self.spawn_wave_enemies(etype, count, pattern, y, spacing);
            } else {
                let bt = wave.boss_type;
                self.wave_index += 1;
                self.create_boss(bt);
            }
        }

        // Player input
        let left = is_key_down(KeyCode::Left);
        let right = is_key_down(KeyCode::Right);
        let up = is_key_down(KeyCode::Up);
        let down = is_key_down(KeyCode::Down);
        let shoot_held = is_key_down(KeyCode::X) || is_key_down(KeyCode::Space);
        let special_pressed = is_key_pressed(KeyCode::Space);

        if self.player.alive {
            let mut dx: f32 = 0.0;
            let mut dy: f32 = 0.0;
            if left { dx -= 1.0; }
            if right { dx += 1.0; }
            if up { dy -= 1.0; }
            if down { dy += 1.0; }

            self.player.x += dx * self.player.w.min(PLAYER_SPEED);
            self.player.y += dy * PLAYER_SPEED;

            self.player.x = self.player.x.clamp(10.0, PLAYER_MOVE_BOUND);
            self.player.y = self.player.y.clamp(10.0, SCREEN_H - 10.0 - self.player.h);
        }

        if self.player.invulnerable > 0 {
            self.player.invulnerable -= 1;
        }
        if self.player.emp_cooldown > 0 {
            self.player.emp_cooldown -= 1;
        }
        if self.aberration_timer > 0 {
            self.aberration_timer -= 1;
        }

        // Firing
        if shoot_held && self.player.alive {
            self.player.fire_timer -= 1;
            if self.player.fire_timer <= 0 {
                self.player_shoot();
            }
        } else {
            if self.player.fire_timer > 0 {
                self.player.fire_timer = 0;
            }
        }

        // EMP (special key = space, but only if not also used for fire... use separate logic)
        // In Miyoo: Space = B button for EMP
        if special_pressed && self.player.emp_charges > 0 && self.player.emp_cooldown <= 0 && self.player.alive {
            let px = self.player.x + self.player.w / 2.0;
            let py = self.player.y + self.player.h / 2.0;
            self.emp_waves.push(EmpWave {
                x: px, y: py,
                radius: 0.0,
                max_radius: 400.0,
                speed: 8.0,
                alive: true,
            });
            self.player.emp_charges -= 1;
            self.player.emp_cooldown = 600;
            self.trigger_shake(6.0);
        }

        // Weapon timer
        if self.player.weapon_timer > 0 {
            self.player.weapon_timer -= 1;
            if self.player.weapon_timer <= 0 {
                self.player.weapon = WeaponType::DualLaser;
            }
        }

        // Update bullets
        for i in 0..self.bullets.len() {
            if !self.bullets[i].alive { continue; }

            // Homing logic
            if self.bullets[i].btype == BulletType::Homing && self.bullets[i].life > 150 {
                let bx = self.bullets[i].x;
                let by = self.bullets[i].y;
                let mut nearest_x = bx;
                let mut nearest_y = by;
                let mut min_dist = 9999.0f32;

                for e in &self.enemies {
                    if !e.alive { continue; }
                    let d = ((e.x - bx).powi(2) + (e.y - by).powi(2)).sqrt();
                    if d < min_dist { min_dist = d; nearest_x = e.x; nearest_y = e.y + e.h / 2.0; }
                }
                if let Some(ref boss) = self.boss {
                    if boss.alive {
                        let d = ((boss.x - bx).powi(2) + (boss.y - by).powi(2)).sqrt();
                        if d < min_dist { nearest_x = boss.x; nearest_y = boss.y; }
                    }
                }

                if min_dist < 9999.0 {
                    let angle: f32 = (nearest_y - by).atan2(nearest_x - bx);
                    self.bullets[i].vx += angle.cos() * 0.5;
                    self.bullets[i].vy += angle.sin() * 0.5;
                    let spd = (self.bullets[i].vx.powi(2) + self.bullets[i].vy.powi(2)).sqrt();
                    if spd > 6.0 {
                        self.bullets[i].vx = self.bullets[i].vx / spd * 6.0;
                        self.bullets[i].vy = self.bullets[i].vy / spd * 6.0;
                    }
                }
            }

            self.bullets[i].x += self.bullets[i].vx;
            self.bullets[i].y += self.bullets[i].vy;
            self.bullets[i].life -= 1;
            if self.bullets[i].x > SCREEN_W + 10.0 || self.bullets[i].x < -10.0
                || self.bullets[i].y < -10.0 || self.bullets[i].y > SCREEN_H + 10.0
                || self.bullets[i].life <= 0
            {
                self.bullets[i].alive = false;
            }
        }
        self.bullets.retain(|b| b.alive);

        // Update enemy bullets
        for i in 0..self.enemy_bullets.len() {
            self.enemy_bullets[i].x += self.enemy_bullets[i].vx;
            self.enemy_bullets[i].y += self.enemy_bullets[i].vy;
            self.enemy_bullets[i].life -= 1;
            if self.enemy_bullets[i].x < -10.0 || self.enemy_bullets[i].x > SCREEN_W + 10.0
                || self.enemy_bullets[i].y < -10.0 || self.enemy_bullets[i].y > SCREEN_H + 10.0
                || self.enemy_bullets[i].life <= 0
            {
                self.enemy_bullets[i].alive = false;
                continue;
            }
            // Hit player
            if self.player.alive && self.player.invulnerable <= 0 {
                let bx = self.enemy_bullets[i].x;
                let by = self.enemy_bullets[i].y;
                if bx > self.player.x && bx < self.player.x + self.player.w
                    && by > self.player.y && by < self.player.y + self.player.h
                {
                    self.enemy_bullets[i].alive = false;
                    self.hit_player();
                }
            }
        }
        self.enemy_bullets.retain(|b| b.alive);

        // Update enemies
        let frame = self.frame;
        let px = self.player.x;
        let py = self.player.y;
        let ph = self.player.h;
        let mut new_enemy_bullets: Vec<Bullet> = Vec::new();

        for i in 0..self.enemies.len() {
            if !self.enemies[i].alive { continue; }
            if self.enemies[i].flash_timer > 0 {
                self.enemies[i].flash_timer -= 1;
            }

            // Movement
            match self.enemies[i].etype {
                EnemyType::Drone => {
                    self.enemies[i].x -= self.enemies[i].speed;
                    self.enemies[i].y += (frame as f32 * 0.05 + self.enemies[i].sine_offset).sin() * 1.2;
                }
                EnemyType::Gunship => {
                    self.enemies[i].x -= self.enemies[i].speed;
                    self.enemies[i].y += (frame as f32 * 0.03 + self.enemies[i].sine_offset).sin() * 0.8;
                }
                EnemyType::Turret | EnemyType::ShieldGen => {
                    self.enemies[i].x -= self.enemies[i].speed;
                }
            }

            // Shooting
            if self.enemies[i].shoot_rate < 9000.0 && self.enemies[i].x < SCREEN_W - 20.0 {
                self.enemies[i].shoot_timer += 1.0;
                if self.enemies[i].shoot_timer >= self.enemies[i].shoot_rate {
                    self.enemies[i].shoot_timer = 0.0;
                    let ex = self.enemies[i].x;
                    let ey = self.enemies[i].y;
                    let ew = self.enemies[i].w;
                    let eh = self.enemies[i].h;

                    match self.enemies[i].etype {
                        EnemyType::Turret => {
                            let angle: f32 = (py + ph / 2.0 - ey - eh / 2.0).atan2(px - ex);
                            new_enemy_bullets.push(Bullet {
                                x: ex, y: ey + eh / 2.0,
                                vx: angle.cos() * 3.0, vy: angle.sin() * 3.0,
                                life: 200, is_player: false, alive: true, btype: BulletType::Enemy,
                            });
                        }
                        EnemyType::Gunship => {
                            new_enemy_bullets.push(Bullet {
                                x: ex - 4.0, y: ey + eh / 2.0,
                                vx: -3.5, vy: 0.0,
                                life: 150, is_player: false, alive: true, btype: BulletType::Enemy,
                            });
                        }
                        EnemyType::ShieldGen => {
                            for a in 0..4 {
                                let ang: f32 = a as f32 * std::f32::consts::FRAC_PI_2 + frame as f32 * 0.02;
                                new_enemy_bullets.push(Bullet {
                                    x: ex + ew / 2.0, y: ey + eh / 2.0,
                                    vx: ang.cos() * 2.0, vy: ang.sin() * 2.0,
                                    life: 120, is_player: false, alive: true, btype: BulletType::Enemy,
                                });
                            }
                        }
                        EnemyType::Drone => {}
                    }
                }
            }

            // Off screen
            if self.enemies[i].x < -50.0 {
                self.enemies[i].alive = false;
                continue;
            }

            // Bullet collision with enemies
            for j in 0..self.bullets.len() {
                if !self.bullets[j].alive || !self.bullets[j].is_player { continue; }
                let bx = self.bullets[j].x;
                let by = self.bullets[j].y;
                let ex = self.enemies[i].x;
                let ey = self.enemies[i].y;
                let ew = self.enemies[i].w;
                let eh = self.enemies[i].h;

                if bx + 4.0 > ex && bx < ex + ew && by + 4.0 > ey && by < ey + eh {
                    self.enemies[i].hp -= 1;
                    self.enemies[i].flash_timer = 4;
                    self.bullets[j].alive = false;
                    if self.enemies[i].hp <= 0 {
                        self.enemies[i].alive = false;
                        let cx = ex + ew / 2.0;
                        let cy = ey + eh / 2.0;
                        self.spawn_particles(cx, cy, 15, NEON_PINK, 2.5);
                        self.trigger_shake(3.0);
                        let pts = self.enemies[i].points * self.chain_multiplier;
                        self.score += pts;
                        self.floating_texts.push(FloatingText {
                            x: ex, y: ey,
                            text: format!("+{}", pts),
                            life: 60,
                            color: YELLOW,
                        });
                        self.chain_timer = 120;
                        self.chain_multiplier = (self.chain_multiplier + 1).min(8);
                        if rand::gen_range(0.0f32, 1.0) < DROP_CHANCE {
                            self.power_ups.push(PowerUp::new(ex, ey));
                        }
                    }
                    break;
                }
            }

            // Enemy body collision with player
            if self.player.alive && self.player.invulnerable <= 0 && self.enemies[i].alive {
                let pw = self.player.w;
                if Self::aabb_overlap(
                    self.player.x, self.player.y, pw, self.player.h,
                    self.enemies[i].x, self.enemies[i].y, self.enemies[i].w, self.enemies[i].h,
                ) {
                    self.enemies[i].hp -= 2;
                    if self.enemies[i].hp <= 0 {
                        self.enemies[i].alive = false;
                        let cx = self.enemies[i].x + self.enemies[i].w / 2.0;
                        let cy = self.enemies[i].y + self.enemies[i].h / 2.0;
                        self.spawn_particles(cx, cy, 10, NEON_PINK, 2.0);
                    }
                    self.hit_player();
                }
            }
        }
        self.enemy_bullets.extend(new_enemy_bullets);
        self.bullets.retain(|b| b.alive);
        self.enemies.retain(|e| e.alive);

        // EMP waves
        for i in 0..self.emp_waves.len() {
            self.emp_waves[i].radius += self.emp_waves[i].speed;
            if self.emp_waves[i].radius >= self.emp_waves[i].max_radius {
                self.emp_waves[i].alive = false;
                continue;
            }
            let emp_x = self.emp_waves[i].x;
            let emp_y = self.emp_waves[i].y;
            let emp_r = self.emp_waves[i].radius;

            // Damage enemies in ring
            for j in 0..self.enemies.len() {
                if !self.enemies[j].alive { continue; }
                let d = ((self.enemies[j].x + self.enemies[j].w / 2.0 - emp_x).powi(2)
                    + (self.enemies[j].y + self.enemies[j].h / 2.0 - emp_y).powi(2)).sqrt();
                if d < emp_r + 10.0 && d > emp_r - 20.0 {
                    self.enemies[j].hp -= 3;
                    if self.enemies[j].hp <= 0 {
                        self.enemies[j].alive = false;
                        let cx = self.enemies[j].x + self.enemies[j].w / 2.0;
                        let cy = self.enemies[j].y + self.enemies[j].h / 2.0;
                        self.spawn_particles(cx, cy, 12, NEON_PURPLE, 3.0);
                        self.score += self.enemies[j].points;
                    }
                }
            }
            // Clear enemy bullets in radius
            for b in self.enemy_bullets.iter_mut() {
                if !b.alive { continue; }
                let d = ((b.x - emp_x).powi(2) + (b.y - emp_y).powi(2)).sqrt();
                if d < emp_r + 5.0 {
                    b.alive = false;
                }
            }
            // EMP vs boss
            if let Some(ref mut boss) = self.boss {
                if boss.alive {
                    let d = ((boss.x + boss.w / 2.0 - emp_x).powi(2)
                        + (boss.y + boss.h / 2.0 - emp_y).powi(2)).sqrt();
                    if d < emp_r + 20.0 && d > emp_r - 25.0 {
                        boss.hp -= 5;
                        let bx = boss.x + boss.w / 2.0;
                        let by = boss.y + boss.h / 2.0;
                        self.spawn_particles(bx, by, 10, NEON_PURPLE, 3.0);
                    }
                }
            }
        }
        self.emp_waves.retain(|e| e.alive);
        self.enemy_bullets.retain(|b| b.alive);
        self.enemies.retain(|e| e.alive);

        // Boss update - extract to avoid borrow checker issues
        let mut boss_died = false;
        let mut boss_new_bullets: Vec<Bullet> = Vec::new();
        let mut boss_spawn_drones: Vec<Enemy> = Vec::new();
        let mut boss_hit_particles: Vec<(f32, f32, usize, Color, f32)> = Vec::new();
        let mut boss_hit_bullets: Vec<usize> = Vec::new(); // indices to mark dead
        let mut boss_score_add: u32 = 0;
        let mut boss_score_text: Option<(f32, f32, u32)> = None;
        let mut boss_shake: f32 = 0.0;
        let mut boss_hit_player = false;

        if let Some(ref mut boss) = self.boss {
            if boss.alive {
                // Enter animation
                if boss.enter_phase {
                    boss.x += (boss.target_x - boss.x) * 0.02;
                    if (boss.x - boss.target_x).abs() < 2.0 {
                        boss.enter_phase = false;
                        boss.x = boss.target_x;
                    }
                } else {
                    boss.y = SCREEN_H / 2.0 + (frame as f32 * 0.015).sin() * 60.0 - boss.h / 2.0;
                }

                boss.fire_timer += 1;

                // Boss attacks
                if !boss.enter_phase {
                    match boss.boss_type {
                        0 => {
                            boss.angle += 0.02;
                            if boss.fire_timer % 30 == 0 {
                                for a in 0..4 {
                                    let ang: f32 = boss.angle + a as f32 * std::f32::consts::FRAC_PI_2;
                                    boss_new_bullets.push(Bullet {
                                        x: boss.x + boss.w / 2.0, y: boss.y + boss.h / 2.0,
                                        vx: ang.cos() * 2.5, vy: ang.sin() * 2.5,
                                        life: 200, is_player: false, alive: true, btype: BulletType::Enemy,
                                    });
                                }
                            }
                        }
                        1 => {
                            boss.drone_timer += 1;
                            if boss.drone_timer % 180 == 0 && self.enemies.len() < 15 {
                                let by_drone = boss.y + boss.h / 2.0;
                                for k in 0..3u32 {
                                    boss_spawn_drones.push(Enemy {
                                        x: SCREEN_W + 20.0 + k as f32 * 20.0,
                                        y: by_drone, w: 16.0, h: 16.0,
                                        etype: EnemyType::Drone, hp: 1, max_hp: 1,
                                        speed: 2.0 + rand::gen_range(0.0f32, 1.5),
                                        points: 100, alive: true,
                                        shoot_timer: 0.0, shoot_rate: 9999.0,
                                        sine_offset: rand::gen_range(0.0f32, std::f32::consts::TAU),
                                        flash_timer: 0,
                                    });
                                }
                            }
                            if boss.fire_timer % 45 == 0 {
                                let angle: f32 = (py + ph / 2.0 - boss.y - boss.h / 2.0)
                                    .atan2(px - boss.x);
                                boss_new_bullets.push(Bullet {
                                    x: boss.x, y: boss.y + boss.h / 2.0,
                                    vx: angle.cos() * 3.0, vy: angle.sin() * 3.0,
                                    life: 180, is_player: false, alive: true, btype: BulletType::Enemy,
                                });
                            }
                        }
                        _ => {
                            let hp_pct = boss.hp as f32 / boss.max_hp as f32;
                            if hp_pct > 0.66 {
                                if boss.fire_timer % 20 == 0 {
                                    for k in -2i32..=2 {
                                        boss_new_bullets.push(Bullet {
                                            x: boss.x, y: boss.y + boss.h / 2.0 + k as f32 * 12.0,
                                            vx: -3.0, vy: k as f32 * 0.3,
                                            life: 200, is_player: false, alive: true, btype: BulletType::Enemy,
                                        });
                                    }
                                }
                            } else if hp_pct > 0.33 {
                                boss.sweep_angle += 0.03;
                                if boss.fire_timer % 10 == 0 {
                                    let ang: f32 = boss.sweep_angle.sin() * 1.2;
                                    let base: f32 = std::f32::consts::PI + ang;
                                    boss_new_bullets.push(Bullet {
                                        x: boss.x, y: boss.y + boss.h / 2.0,
                                        vx: base.cos() * 4.0, vy: base.sin() * 4.0,
                                        life: 150, is_player: false, alive: true, btype: BulletType::Enemy,
                                    });
                                }
                                if boss.fire_timer % 60 == 0 && self.enemies.len() < 10 {
                                    for k in 0..4u32 {
                                        let ey_rand = 40.0 + rand::gen_range(0.0f32, SCREEN_H - 80.0);
                                        boss_spawn_drones.push(Enemy {
                                            x: SCREEN_W + 20.0 + k as f32 * 60.0,
                                            y: ey_rand, w: 16.0, h: 16.0,
                                            etype: EnemyType::Drone, hp: 1, max_hp: 1,
                                            speed: 2.0 + rand::gen_range(0.0f32, 1.5),
                                            points: 100, alive: true,
                                            shoot_timer: 0.0, shoot_rate: 9999.0,
                                            sine_offset: rand::gen_range(0.0f32, std::f32::consts::TAU),
                                            flash_timer: 0,
                                        });
                                    }
                                }
                            } else {
                                if boss.fire_timer % 6 == 0 {
                                    let ang: f32 = (py + ph / 2.0 - boss.y - boss.h / 2.0)
                                        .atan2(px - boss.x) + (rand::gen_range(0.0f32, 1.0) - 0.5) * 0.5;
                                    boss_new_bullets.push(Bullet {
                                        x: boss.x, y: boss.y + boss.h / 2.0,
                                        vx: ang.cos() * 3.5, vy: ang.sin() * 3.5,
                                        life: 180, is_player: false, alive: true, btype: BulletType::Enemy,
                                    });
                                }
                                if boss.fire_timer % 90 == 0 {
                                    for a in 0..8 {
                                        let ang: f32 = a as f32 * std::f32::consts::FRAC_PI_4;
                                        boss_new_bullets.push(Bullet {
                                            x: boss.x + boss.w / 2.0, y: boss.y + boss.h / 2.0,
                                            vx: ang.cos() * 2.0, vy: ang.sin() * 2.0,
                                            life: 200, is_player: false, alive: true, btype: BulletType::Enemy,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }

                // Boss bullet collision - collect results
                for j in 0..self.bullets.len() {
                    if !self.bullets[j].alive || !self.bullets[j].is_player { continue; }
                    let bx = self.bullets[j].x;
                    let by = self.bullets[j].y;
                    if bx + 4.0 > boss.x && bx < boss.x + boss.w
                        && by + 4.0 > boss.y && by < boss.y + boss.h
                    {
                        boss.hp -= 1;
                        boss_hit_bullets.push(j);
                        boss_hit_particles.push((bx, by, 3, NEON_CYAN, 1.0));
                        if boss.hp <= 0 {
                            boss.alive = false;
                            boss_died = true;
                            let bx2 = boss.x + boss.w / 2.0;
                            let by2 = boss.y + boss.h / 2.0;
                            boss_hit_particles.push((bx2, by2, 40, NEON_PINK, 4.0));
                            boss_hit_particles.push((bx2, by2, 25, NEON_CYAN, 3.0));
                            boss_hit_particles.push((bx2, by2, 20, NEON_PURPLE, 3.5));
                            boss_shake = 10.0;
                            let pts = 5000 * self.chain_multiplier;
                            boss_score_add = pts;
                            boss_score_text = Some((boss.x, boss.y, pts));
                        }
                    }
                }

                // Boss body collision with player
                if self.player.alive && self.player.invulnerable <= 0 && !boss.enter_phase {
                    if Self::aabb_overlap(
                        self.player.x, self.player.y, self.player.w, self.player.h,
                        boss.x, boss.y, boss.w, boss.h,
                    ) {
                        boss_hit_player = true;
                    }
                }
            }
        }

        // Apply deferred boss results
        self.enemy_bullets.extend(boss_new_bullets);
        self.enemies.extend(boss_spawn_drones);
        for &idx in boss_hit_bullets.iter().rev() {
            if idx < self.bullets.len() {
                self.bullets[idx].alive = false;
            }
        }
        self.bullets.retain(|b| b.alive);
        for (px2, py2, count, color, spd) in boss_hit_particles {
            self.spawn_particles(px2, py2, count, color, spd);
        }
        if boss_shake > 0.0 {
            self.trigger_shake(boss_shake);
        }
        self.score += boss_score_add;
        if let Some((fx, fy, pts)) = boss_score_text {
            self.floating_texts.push(FloatingText {
                x: fx, y: fy,
                text: format!("+{}", pts),
                life: 60,
                color: YELLOW,
            });
        }
        if boss_hit_player {
            self.hit_player();
        }

        if boss_died {
            self.boss_defeat_timer = 120;
        }

        // Update power-ups
        for i in 0..self.power_ups.len() {
            self.power_ups[i].x -= 1.0;
            self.power_ups[i].angle += 0.05;
            if self.power_ups[i].x < -20.0 {
                self.power_ups[i].alive = false;
                continue;
            }
            // Collect
            if self.player.alive {
                let pu = &self.power_ups[i];
                if Self::aabb_overlap(
                    self.player.x, self.player.y, self.player.w, self.player.h,
                    pu.x, pu.y, pu.w, pu.h,
                ) {
                    let kind = self.power_ups[i].kind;
                    let pux = self.power_ups[i].x;
                    let puy = self.power_ups[i].y;
                    self.power_ups[i].alive = false;

                    match kind {
                        PowerUpKind::Spread => {
                            self.player.weapon = WeaponType::SpreadShot;
                            self.player.weapon_timer = 600;
                            self.floating_texts.push(FloatingText {
                                x: pux, y: puy, text: "SPREAD".to_string(),
                                life: 60, color: NEON_PINK,
                            });
                        }
                        PowerUpKind::Homing => {
                            self.player.weapon = WeaponType::HomingMissile;
                            self.player.weapon_timer = 600;
                            self.floating_texts.push(FloatingText {
                                x: pux, y: puy, text: "HOMING".to_string(),
                                life: 60, color: NEON_GREEN,
                            });
                        }
                        PowerUpKind::Shield => {
                            self.player.shields = self.player.shields.min(self.player.max_shields - 1) + 1;
                            self.floating_texts.push(FloatingText {
                                x: pux, y: puy, text: "SHIELD+".to_string(),
                                life: 60, color: NEON_CYAN,
                            });
                        }
                        PowerUpKind::Emp => {
                            self.player.emp_charges = (self.player.emp_charges + 1).min(3);
                            self.floating_texts.push(FloatingText {
                                x: pux, y: puy, text: "EMP+".to_string(),
                                life: 60, color: NEON_PURPLE,
                            });
                        }
                    }
                    self.spawn_particles(pux, puy, 10, NEON_CYAN, 2.0);
                }
            }
        }
        self.power_ups.retain(|p| p.alive);

        // Floating texts
        for ft in self.floating_texts.iter_mut() {
            ft.y -= 0.8;
            ft.life -= 1;
        }
        self.floating_texts.retain(|ft| ft.life > 0);

        // Chain decay
        if self.chain_timer > 0 {
            self.chain_timer -= 1;
            if self.chain_timer <= 0 {
                self.chain_multiplier = 1;
            }
        }

        // Screen shake decay
        if self.shake_mag > 0.5 {
            self.shake_x = (rand::gen_range(0.0f32, 1.0) - 0.5) * self.shake_mag * 2.0;
            self.shake_y = (rand::gen_range(0.0f32, 1.0) - 0.5) * self.shake_mag * 2.0;
            self.shake_mag *= 0.85;
        } else {
            self.shake_mag = 0.0;
            self.shake_x = 0.0;
            self.shake_y = 0.0;
        }
    }

    // ------------------------------------------------------------------
    // Draw
    // ------------------------------------------------------------------
    fn draw(&self) {
        clear_background(DARK_BG);

        let sx = self.shake_x;
        let sy = self.shake_y;

        // Stars
        for (idx, s) in self.stars.iter().enumerate() {
            let twinkle = s.brightness + (self.frame as f32 * 0.03 + s.x + idx as f32 * 0.1).sin() * 0.15;
            let alpha = twinkle.clamp(0.1, 1.0);
            let tint = match s.layer {
                0 => Color::new(0.39, 0.47, 0.78, alpha),
                1 => Color::new(0.59, 0.63, 0.86, alpha),
                _ => Color::new(0.78, 0.82, 1.0, alpha),
            };
            draw_rectangle(s.x + sx, s.y + sy, s.size, s.size, tint);
        }

        // Chromatic aberration flash
        if self.aberration_timer > 0 {
            let a = self.aberration_timer as f32 / 15.0 * 0.1;
            draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(1.0, 0.0, 0.0, a));
        }

        match self.state {
            GameState::Start => {
                self.draw_rain(0.3);
                self.draw_title();
            }
            GameState::LevelStory | GameState::Win => {
                self.draw_rain(0.6);
                self.draw_story();
            }
            GameState::GameOver => {
                self.draw_rain(0.4);
                self.draw_game_over();
            }
            GameState::Playing => {
                self.draw_city(sx, sy);
                self.draw_playing(sx, sy);
            }
        }

        // CRT scanline overlay
        {
            let scanline_color = Color::new(0.0, 0.0, 0.0, 0.12);
            let mut y = 0.0;
            while y < SCREEN_H {
                draw_rectangle(0.0, y, SCREEN_W, 2.0, scanline_color);
                y += 4.0;
            }
        }

        // Vignette
        {
            let depth = 60.0;
            let steps = 12;
            let step_size = depth / steps as f32;
            for i in 0..steps {
                let t = 1.0 - (i as f32 / steps as f32);
                let alpha = t * 0.4;
                let c = Color::new(0.0, 0.0, 0.0, alpha);
                let offset = i as f32 * step_size;
                draw_rectangle(0.0, offset, SCREEN_W, step_size, c);
                draw_rectangle(0.0, SCREEN_H - offset - step_size, SCREEN_W, step_size, c);
                draw_rectangle(offset, 0.0, step_size, SCREEN_H, c);
                draw_rectangle(SCREEN_W - offset - step_size, 0.0, step_size, SCREEN_H, c);
            }
        }
    }

    // ------------------------------------------------------------------
    // Draw helpers
    // ------------------------------------------------------------------
    fn draw_rain(&self, alpha: f32) {
        for col in &self.rain_columns {
            for j in 0..col.len {
                let cy = col.y + j as f32 * 14.0;
                if cy < 0.0 || cy > SCREEN_H { continue; }
                let a = if j == 0 { 1.0 } else { 1.0 - j as f32 / col.len as f32 };
                let c = Color::new(0.22, 1.0, 0.08, a * 0.5 * alpha);
                draw_rectangle(col.x, cy, 6.0, 10.0, c);
            }
        }
    }

    fn draw_city(&self, sx: f32, sy: f32) {
        let wrap = 30.0 * 60.0 + 200.0;
        for b in &self.city_buildings {
            let bx = ((b.x - self.city_scroll_x * 0.5) % wrap + wrap) % wrap - 50.0;
            if bx < -b.w || bx > SCREEN_W + b.w { continue; }
            draw_rectangle(bx + sx, SCREEN_H - b.h + sy, b.w, b.h, b.color);
            if b.has_windows {
                let win_color = Color::new(0.0, 0.95, 1.0, 0.06);
                let mut wy = SCREEN_H - b.h + 5.0;
                while wy < SCREEN_H - 5.0 {
                    let mut wx = bx + 3.0;
                    while wx < bx + b.w - 3.0 {
                        if (wx * 3.7 + wy * 2.1).sin() > 0.3 {
                            draw_rectangle(wx + sx, wy + sy, 3.0, 4.0, win_color);
                        }
                        wx += 6.0;
                    }
                    wy += 10.0;
                }
            }
        }
    }

    fn draw_playing(&self, sx: f32, sy: f32) {
        // Power-ups
        for p in &self.power_ups {
            let glow_color = match p.kind {
                PowerUpKind::Spread => NEON_PINK,
                PowerUpKind::Homing => NEON_GREEN,
                PowerUpKind::Shield => NEON_CYAN,
                PowerUpKind::Emp => NEON_PURPLE,
            };
            let pulse = 1.0 + (self.frame as f32 * 0.1).sin() * 0.15;
            let r = (p.w / 2.0) * pulse;
            draw_circle_lines(p.x + p.w / 2.0 + sx, p.y + p.h / 2.0 + sy, r + 2.0, 1.5, glow_color);
            draw_circle(p.x + p.w / 2.0 + sx, p.y + p.h / 2.0 + sy, r, Color::new(glow_color.r, glow_color.g, glow_color.b, 0.3));
            let letter = match p.kind {
                PowerUpKind::Spread => "S",
                PowerUpKind::Homing => "H",
                PowerUpKind::Shield => "+",
                PowerUpKind::Emp => "E",
            };
            let tw = measure_text(letter, None, 10, 1.0).width;
            draw_text(letter, p.x + p.w / 2.0 - tw / 2.0 + sx, p.y + p.h / 2.0 + 4.0 + sy, 10.0, glow_color);
        }

        // Enemies
        for e in &self.enemies {
            if !e.alive { continue; }
            if e.flash_timer > 0 {
                // Flash white when hit
                draw_rectangle(e.x + sx, e.y + sy, e.w, e.h, WHITE);
            } else {
                let color = match e.etype {
                    EnemyType::Drone => NEON_PINK,
                    EnemyType::Gunship => NEON_PURPLE,
                    EnemyType::Turret => NEON_ORANGE,
                    EnemyType::ShieldGen => NEON_GREEN,
                };
                self.draw_enemy_shape(e.x + sx, e.y + sy, e.w, e.h, e.etype, color);
            }
            // HP bar for tough enemies
            if e.max_hp > 1 {
                let hp_pct = e.hp as f32 / e.max_hp as f32;
                let bar_color = if hp_pct > 0.5 { NEON_GREEN } else if hp_pct > 0.25 { YELLOW } else { NEON_PINK };
                draw_rectangle(e.x + sx, e.y - 4.0 + sy, e.w, 2.0, Color::new(0.0, 0.0, 0.0, 0.5));
                draw_rectangle(e.x + sx, e.y - 4.0 + sy, e.w * hp_pct, 2.0, bar_color);
            }
        }

        // Boss
        self.draw_boss(sx, sy);

        // Player bullets
        for b in &self.bullets {
            if !b.alive { continue; }
            match b.btype {
                BulletType::Laser => {
                    draw_rectangle(b.x + sx, b.y - 1.0 + sy, 8.0, 2.0, NEON_CYAN);
                    // Trail
                    draw_rectangle(b.x - 6.0 + sx, b.y - 0.5 + sy, 6.0, 1.0, Color::new(0.0, 0.95, 1.0, 0.2));
                }
                BulletType::Spread => {
                    draw_rectangle(b.x + sx, b.y - 1.0 + sy, 6.0, 2.0, NEON_PINK);
                }
                BulletType::Homing => {
                    draw_circle(b.x + sx, b.y + sy, 3.0, NEON_GREEN);
                    draw_rectangle(b.x - 8.0 + sx, b.y - 0.5 + sy, 8.0, 1.0, Color::new(0.22, 1.0, 0.08, 0.15));
                }
                BulletType::Enemy => {}
            }
        }

        // Enemy bullets
        for b in &self.enemy_bullets {
            if !b.alive { continue; }
            draw_circle(b.x + sx, b.y + sy, 2.5, Color::new(1.0, 0.4, 0.2, 1.0));
        }

        // EMP waves
        for emp in &self.emp_waves {
            let alpha = 1.0 - emp.radius / emp.max_radius;
            draw_circle_lines(emp.x + sx, emp.y + sy, emp.radius, 3.0,
                Color::new(0.545, 0.0, 1.0, alpha * 0.7));
        }

        // Particles
        for p in &self.particles {
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            let c = Color::new(p.color.r, p.color.g, p.color.b, alpha);
            draw_rectangle(p.x + sx, p.y + sy, p.size, p.size, c);
        }

        // Player
        if self.player.alive {
            if self.player.invulnerable > 0 && (self.player.invulnerable / 4) % 2 == 0 {
                // blink - skip drawing
            } else {
                self.draw_player_ship(sx, sy);
            }
        }

        // Floating texts
        for ft in &self.floating_texts {
            let alpha = ft.life as f32 / 60.0;
            let c = Color::new(ft.color.r, ft.color.g, ft.color.b, alpha);
            let tw = measure_text(&ft.text, None, 12, 1.0).width;
            draw_text(&ft.text, ft.x - tw / 2.0 + sx, ft.y + sy, 12.0, c);
        }

        // HUD
        self.draw_hud();
    }

    fn draw_player_ship(&self, sx: f32, sy: f32) {
        let p = &self.player;
        let px = p.x + sx;
        let py = p.y + sy;

        // Thruster glow
        let thr_pulse = if self.frame % 8 < 4 { 0.8 } else { 0.5 };
        draw_rectangle(px - 6.0, py + p.h / 2.0 - 3.0, 8.0, 6.0,
            Color::new(1.0, 0.4, 0.0, thr_pulse));

        // Ship body (triangular arrow pointing right)
        let nose = Vec2::new(px + p.w, py + p.h / 2.0);
        let top = Vec2::new(px, py);
        let bot = Vec2::new(px, py + p.h);
        draw_triangle(nose, top, bot, Color::new(0.0, 0.05, 0.1, 0.9));
        // Outline
        draw_line(nose.x, nose.y, top.x, top.y, 2.0, NEON_CYAN);
        draw_line(top.x, top.y, bot.x, bot.y, 2.0, NEON_CYAN);
        draw_line(bot.x, bot.y, nose.x, nose.y, 2.0, NEON_CYAN);

        // Cockpit
        draw_circle(px + p.w * 0.5, py + p.h * 0.5, 3.0, NEON_CYAN);

        // Shield shimmer
        if p.shields > 0 {
            let shimmer = 0.15 + (self.frame as f32 * 0.1).sin() * 0.08;
            draw_circle_lines(px + p.w / 2.0, py + p.h / 2.0, p.w / 2.0 + 4.0, 1.5,
                Color::new(0.0, 0.95, 1.0, shimmer));
        }
    }

    fn draw_enemy_shape(&self, x: f32, y: f32, w: f32, h: f32, etype: EnemyType, color: Color) {
        let fill = Color::new(0.0, 0.0, 0.0, 0.8);
        let hw = w / 2.0;
        let hh = h / 2.0;
        let cx = x + hw;
        let cy = y + hh;

        match etype {
            EnemyType::Drone => {
                // Diamond
                draw_triangle(Vec2::new(cx, cy - hh), Vec2::new(cx + hw, cy), Vec2::new(cx, cy + hh), fill);
                draw_triangle(Vec2::new(cx, cy - hh), Vec2::new(cx, cy + hh), Vec2::new(cx - hw, cy), fill);
                draw_line(cx, cy - hh, cx + hw, cy, 1.5, color);
                draw_line(cx + hw, cy, cx, cy + hh, 1.5, color);
                draw_line(cx, cy + hh, cx - hw, cy, 1.5, color);
                draw_line(cx - hw, cy, cx, cy - hh, 1.5, color);
                draw_circle(cx, cy, 2.0, color);
            }
            EnemyType::Gunship => {
                // Hexagonal shape
                draw_rectangle(x, y, w, h, fill);
                draw_rectangle_lines(x, y, w, h, 1.5, color);
                draw_circle(cx, cy, 3.0, color);
            }
            EnemyType::Turret => {
                // Square with rotating indicator
                draw_rectangle(x, y, w, h, fill);
                draw_rectangle_lines(x, y, w, h, 2.0, color);
                let a = self.frame as f32 * 0.02;
                let bx2 = a.cos() * hw * 0.8;
                let by2 = a.sin() * hh * 0.8;
                draw_line(cx, cy, cx + bx2, cy + by2, 2.0, color);
                draw_line(cx, cy, cx - bx2, cy - by2, 2.0, color);
                draw_circle(cx, cy, 3.0, color);
            }
            EnemyType::ShieldGen => {
                // Circle with radial lines
                draw_circle(cx, cy, hw, fill);
                draw_circle_lines(cx, cy, hw, 2.0, color);
                for a_idx in 0..4 {
                    let ang: f32 = a_idx as f32 * std::f32::consts::FRAC_PI_2 + self.frame as f32 * 0.02;
                    draw_line(cx, cy, cx + ang.cos() * hw * 0.8, cy + ang.sin() * hh * 0.8, 1.5, color);
                }
                draw_circle(cx, cy, 3.0, Color::new(color.r, color.g, color.b, 0.8));
            }
        }
    }

    fn draw_boss(&self, sx: f32, sy: f32) {
        let boss = match &self.boss {
            Some(b) if b.alive => b,
            _ => return,
        };

        // Warning text during enter phase
        if boss.enter_phase {
            let blink = 0.5 + (self.frame as f32 * 0.15).sin() * 0.5;
            let warn_txt = "WARNING";
            let tw = measure_text(warn_txt, None, 24, 1.0).width;
            draw_text(warn_txt, SCREEN_W / 2.0 - tw / 2.0, SCREEN_H / 2.0 - 50.0, 24.0,
                Color::new(1.0, 0.18, 0.47, blink));
            let boss_name = BOSS_NAMES[boss.boss_type as usize];
            let tw2 = measure_text(boss_name, None, 16, 1.0).width;
            draw_text(boss_name, SCREEN_W / 2.0 - tw2 / 2.0, SCREEN_H / 2.0 - 25.0, 16.0,
                Color::new(0.0, 0.95, 1.0, blink));
        }

        let bx = boss.x + sx;
        let by = boss.y + sy;

        // Boss body
        let fill = Color::new(0.13, 0.0, 0.0, 0.8);
        draw_rectangle(bx, by, boss.w, boss.h, fill);
        let boss_color = match boss.boss_type {
            0 => NEON_PINK,
            1 => NEON_ORANGE,
            _ => NEON_PINK,
        };
        draw_rectangle_lines(bx, by, boss.w, boss.h, 2.0, boss_color);

        // Boss-specific decorations
        match boss.boss_type {
            0 => {
                // Rotating arms
                let bcx = bx + boss.w / 2.0;
                let bcy = by + boss.h / 2.0;
                for a in 0..4 {
                    let ang: f32 = boss.angle + a as f32 * std::f32::consts::FRAC_PI_2;
                    let ex = bcx + ang.cos() * 30.0;
                    let ey = bcy + ang.sin() * 30.0;
                    draw_line(bcx, bcy, ex, ey, 2.0, NEON_PINK);
                    draw_rectangle(ex - 3.0, ey - 3.0, 6.0, 6.0, Color::new(1.0, 0.27, 0.0, 1.0));
                }
            }
            2 => {
                // Leviathan bulk
                draw_rectangle(bx - 15.0, by - 10.0, boss.w + 30.0, boss.h + 20.0,
                    Color::new(0.13, 0.0, 0.07, 0.4));
                // Glowing eyes
                let pulse = (self.frame as f32 * 0.1).sin() * 0.3 + 0.7;
                draw_rectangle(bx + 10.0, by + boss.h / 2.0 - 8.0, 4.0, 4.0,
                    Color::new(1.0, 0.18, 0.47, pulse));
                draw_rectangle(bx + 10.0, by + boss.h / 2.0 + 4.0, 4.0, 4.0,
                    Color::new(1.0, 0.18, 0.47, pulse));
            }
            _ => {}
        }

        // Eye/core
        draw_circle(bx + boss.w / 2.0, by + boss.h / 2.0, 4.0, boss_color);
        draw_circle(bx + boss.w / 2.0, by + boss.h / 2.0, 2.0, WHITE);

        // HP bar at top
        let bar_w = 250.0;
        let bar_x = (SCREEN_W - bar_w) / 2.0;
        let hp_pct = (boss.hp as f32 / boss.max_hp as f32).max(0.0);
        draw_rectangle(bar_x - 2.0, 10.0, bar_w + 4.0, 12.0, Color::new(0.0, 0.0, 0.0, 0.6));
        draw_rectangle(bar_x, 12.0, bar_w * hp_pct, 8.0, NEON_PINK);
        draw_rectangle_lines(bar_x - 2.0, 10.0, bar_w + 4.0, 12.0, 1.0, Color::new(1.0, 0.18, 0.47, 0.5));
        let boss_name = BOSS_NAMES[boss.boss_type as usize];
        let tw = measure_text(boss_name, None, 12, 1.0).width;
        draw_text(boss_name, SCREEN_W / 2.0 - tw / 2.0, 9.0, 12.0, WHITE);
    }

    fn draw_hud(&self) {
        // Score
        let score_txt = format!("SCORE {}", self.score);
        draw_text(&score_txt, 12.0, 30.0, 16.0, NEON_CYAN);

        // Chain
        if self.chain_multiplier > 1 {
            let chain_txt = format!("x{}", self.chain_multiplier);
            draw_text(&chain_txt, 12.0, 50.0, 16.0, YELLOW);
        }

        // Level name
        let lv_name = LEVEL_NAMES[self.current_level.min(2)];
        let lv_txt = format!("LV{} {}", self.current_level + 1, lv_name);
        let tw = measure_text(&lv_txt, None, 16, 1.0).width;
        draw_text(&lv_txt, SCREEN_W / 2.0 - tw / 2.0, SCREEN_H - 12.0, 16.0, NEON_CYAN);

        // Shields
        let shield_txt = "SHIELD";
        let stw = measure_text(shield_txt, None, 16, 1.0).width;
        draw_text(shield_txt, SCREEN_W - 12.0 - stw, 30.0, 16.0, NEON_CYAN);
        for i in 0..self.player.max_shields {
            let c = if i < self.player.shields {
                NEON_CYAN
            } else {
                Color::new(0.0, 0.95, 1.0, 0.2)
            };
            draw_rectangle(SCREEN_W - 75.0 + i as f32 * 20.0, 38.0, 14.0, 7.0, c);
        }

        // Weapon name
        let (weapon_name, weapon_color) = match self.player.weapon {
            WeaponType::DualLaser => ("DUAL LASER", NEON_CYAN),
            WeaponType::SpreadShot => ("SPREAD", NEON_PINK),
            WeaponType::HomingMissile => ("HOMING", NEON_GREEN),
        };
        let wtw = measure_text(weapon_name, None, 12, 1.0).width;
        draw_text(weapon_name, SCREEN_W - 12.0 - wtw, 60.0, 12.0, weapon_color);

        // EMP charges
        let emp_txt = format!("EMP:{}", self.player.emp_charges);
        let etw = measure_text(&emp_txt, None, 12, 1.0).width;
        draw_text(&emp_txt, SCREEN_W - 12.0 - etw, 75.0, 12.0, NEON_PURPLE);
    }

    fn draw_title(&self) {
        // Title
        let title1 = "CHROME";
        let tw1 = measure_text(title1, None, 56, 1.0).width;
        draw_text(title1, SCREEN_W / 2.0 - tw1 / 2.0, 175.0, 56.0, NEON_CYAN);

        let title2 = "VIPER";
        let tw2 = measure_text(title2, None, 56, 1.0).width;
        draw_text(title2, SCREEN_W / 2.0 - tw2 / 2.0, 230.0, 56.0, NEON_PINK);

        // Subtitle
        let sub = "NEON ABYSS";
        let sw = measure_text(sub, None, 16, 1.0).width;
        draw_text(sub, SCREEN_W / 2.0 - sw / 2.0, 268.0, 16.0, Color::new(1.0, 1.0, 1.0, 0.5));

        // Glitch effect
        if rand::gen_range(0.0f32, 1.0) < 0.05 {
            let gy = 150.0 + rand::gen_range(0.0f32, 100.0);
            draw_rectangle(125.0 + rand::gen_range(0.0f32, 125.0), gy,
                250.0 + rand::gen_range(0.0f32, 250.0), 2.0, Color::new(1.0, 0.18, 0.47, 0.3));
        }

        // Story blurb
        let lines = [
            "Year 2187. Megacorp AXIOM controls",
            "the orbital colonies. You pilot the",
            "Chrome Viper, a stolen prototype.",
            "",
            "Breach their defenses. End their reign.",
        ];
        for (i, line) in lines.iter().enumerate() {
            let lw = measure_text(line, None, 12, 1.0).width;
            draw_text(line, SCREEN_W / 2.0 - lw / 2.0, 325.0 + i as f32 * 20.0, 12.0,
                Color::new(0.7, 0.7, 0.78, 0.6));
        }

        // Controls
        let ctrl = "D-PAD: MOVE  |  X: FIRE  |  B: EMP  |  START: BEGIN";
        let cw = measure_text(ctrl, None, 10, 1.0).width;
        draw_text(ctrl, SCREEN_W / 2.0 - cw / 2.0, 475.0, 10.0, Color::new(0.0, 0.95, 1.0, 0.4));

        // Press start blink
        if (self.frame / 30) % 2 == 0 {
            let prompt = "PRESS START";
            let pw = measure_text(prompt, None, 20, 1.0).width;
            draw_text(prompt, SCREEN_W / 2.0 - pw / 2.0, 538.0, 20.0, NEON_CYAN);
        }
    }

    fn draw_story(&self) {
        // Terminal border
        let term_x = 50.0;
        let term_y = 38.0;
        let term_w = SCREEN_W - 100.0;
        let term_h = SCREEN_H - 76.0;
        draw_rectangle(term_x, term_y, term_w, term_h, Color::new(0.0, 0.04, 0.0, 0.7));
        draw_rectangle_lines(term_x, term_y, term_w, term_h, 1.0, Color::new(0.22, 1.0, 0.08, 0.3));

        // Header
        let header = "CHROME VIPER TERMINAL v2.187";
        draw_text(header, term_x + 12.0, term_y + 19.0, 12.0, TERMINAL_GREEN);
        draw_line(term_x, term_y + 28.0, term_x + term_w, term_y + 28.0, 1.0,
            Color::new(0.22, 1.0, 0.08, 0.3));

        // Typewriter text
        let visible = if self.story_char_idx <= self.story_text.len() {
            &self.story_text[..self.story_char_idx]
        } else {
            &self.story_text
        };

        let lines: Vec<&str> = visible.split('\n').collect();
        let mut line_y = term_y + 56.0;
        for line in &lines {
            draw_text(line, term_x + 19.0, line_y, 14.0, TERMINAL_GREEN);
            line_y += 18.0;
        }

        // Blinking cursor
        if self.story_char_idx < self.story_text.len() && (self.frame / 15) % 2 == 0 {
            let last_line = lines.last().unwrap_or(&"");
            let lw = measure_text(last_line, None, 14, 1.0).width;
            draw_rectangle(term_x + 19.0 + lw + 2.0, line_y - 18.0 + 2.0, 6.0, 8.0, TERMINAL_GREEN);
        }

        // Continue prompt
        if self.story_char_idx >= self.story_text.len() {
            if (self.frame / 25) % 2 == 0 {
                let prompt = "[PRESS FIRE TO CONTINUE]";
                let pw = measure_text(prompt, None, 14, 1.0).width;
                draw_text(prompt, SCREEN_W / 2.0 - pw / 2.0, SCREEN_H - 62.0, 14.0, NEON_CYAN);
            }
        }
    }

    fn draw_game_over(&self) {
        // Glitch bars
        for _ in 0..5 {
            if rand::gen_range(0.0f32, 1.0) < 0.3 {
                let gy = rand::gen_range(0.0f32, SCREEN_H);
                draw_rectangle(0.0, gy, SCREEN_W, 2.0 + rand::gen_range(0.0f32, 4.0),
                    Color::new(1.0, 0.18, 0.47, rand::gen_range(0.0f32, 0.15)));
            }
        }

        let title1 = "SYSTEM";
        let tw1 = measure_text(title1, None, 40, 1.0).width;
        draw_text(title1, SCREEN_W / 2.0 - tw1 / 2.0, 212.0, 40.0, NEON_PINK);
        let title2 = "FAILURE";
        let tw2 = measure_text(title2, None, 40, 1.0).width;
        draw_text(title2, SCREEN_W / 2.0 - tw2 / 2.0, 262.0, 40.0, NEON_PINK);

        let sub = "CHROME VIPER DESTROYED";
        let sw = measure_text(sub, None, 16, 1.0).width;
        draw_text(sub, SCREEN_W / 2.0 - sw / 2.0, 325.0, 16.0, Color::new(1.0, 1.0, 1.0, 0.6));

        let sc = format!("SCORE: {}", self.score);
        let scw = measure_text(&sc, None, 20, 1.0).width;
        draw_text(&sc, SCREEN_W / 2.0 - scw / 2.0, 375.0, 20.0, NEON_CYAN);

        let lore1 = "AXIOM prevails. The colonies remain";
        let lw1 = measure_text(lore1, None, 12, 1.0).width;
        draw_text(lore1, SCREEN_W / 2.0 - lw1 / 2.0, 462.0, 12.0, Color::new(0.78, 0.78, 0.86, 0.5));
        let lore2 = "under corporate control.";
        let lw2 = measure_text(lore2, None, 12, 1.0).width;
        draw_text(lore2, SCREEN_W / 2.0 - lw2 / 2.0, 488.0, 12.0, Color::new(0.78, 0.78, 0.86, 0.5));

        if (self.frame / 30) % 2 == 0 {
            let prompt = "PRESS START";
            let pw = measure_text(prompt, None, 16, 1.0).width;
            draw_text(prompt, SCREEN_W / 2.0 - pw / 2.0, 550.0, 16.0, NEON_CYAN);
        }
    }
}

// ---------------------------------------------------------------------------
// Window config
// ---------------------------------------------------------------------------
fn window_conf() -> Conf {
    Conf {
        window_title: "Chrome Viper - Neon Abyss".to_owned(),
        window_width: SCREEN_W as i32,
        window_height: SCREEN_H as i32,
        window_resizable: false,
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------
#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    let mut accumulator: f64 = 0.0;
    let mut last_time = get_time();

    loop {
        let now = get_time();
        let mut elapsed = now - last_time;
        last_time = now;

        // Death spiral prevention
        if elapsed > 0.1 {
            elapsed = 0.1;
        }

        accumulator += elapsed;

        while accumulator >= TIME_STEP {
            game.update();
            accumulator -= TIME_STEP;
        }

        game.draw();
        next_frame().await;
    }
}
