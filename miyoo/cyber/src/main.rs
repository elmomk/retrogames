// Chrome Viper - Cyberpunk Spaceship Action Game
// Rust/Macroquad port for Miyoo Mini Plus
// Story: "Neon Abyss"

use macroquad::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------
const SCREEN_W: f32 = 640.0;
const SCREEN_H: f32 = 480.0;
const TIME_STEP: f64 = 1.0 / 60.0;

const PLAYER_SPEED: f32 = 4.0;
const BULLET_SPEED: f32 = 8.0;
const ENEMY_SPEED: f32 = 2.0;
const SCROLL_SPEED: f32 = 1.0;
const PLAYER_MOVE_BOUND: f32 = SCREEN_W * 0.4; // left 40% of screen

const STAR_COUNT: usize = 80;
const DROP_CHANCE: f32 = 0.12;

// Cyberpunk colors
const NEON_PINK: Color = Color::new(1.0, 0.08, 0.58, 1.0);
const NEON_CYAN: Color = Color::new(0.0, 1.0, 1.0, 1.0);
const NEON_GREEN: Color = Color::new(0.2, 1.0, 0.3, 1.0);
const NEON_PURPLE: Color = Color::new(0.7, 0.2, 1.0, 1.0);
const NEON_ORANGE: Color = Color::new(1.0, 0.5, 0.0, 1.0);
const DARK_BG: Color = Color::new(0.02, 0.01, 0.05, 1.0);
const TERMINAL_GREEN: Color = Color::new(0.0, 0.9, 0.3, 1.0);

// ---------------------------------------------------------------------------
// Game States
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

// ---------------------------------------------------------------------------
// Enemy Types
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq)]
enum EnemyType {
    Drone,
    Gunship,
    Turret,
    Boss,
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
// Wave System
// ---------------------------------------------------------------------------
struct LevelInfo {
    name: &'static str,
    intro_text: &'static str,
    clear_text: &'static str,
    boss_name: &'static str,
    waves: &'static [(f32, EnemyType, f32, f32, u32)],
    boss_hp: i32,
}

const LEVELS: &[LevelInfo] = &[
    LevelInfo {
        name: "SECTOR 1: ORBITAL PERIMETER",
        intro_text: "CHROME VIPER FLIGHT LOG -- 2187.09.14\n\n\
            AXIOM megacorp has seized control of every\n\
            orbital colony in the Sol system. Their\n\
            defense grid spans millions of kilometers.\n\n\
            You are callsign VIPER -- the last pilot\n\
            crazy enough to fly into AXIOM space.\n\n\
            Your target: the flagship LEVIATHAN.\n\
            First, punch through the orbital perimeter.\n\n\
            D-PAD: Move  X: Fire  SPACE: Special  ENTER: Start",
        clear_text: "The Defense Satellite collapses in a\n\
            cascade of blue fire. Debris tumbles into\n\
            the atmosphere below.\n\n\
            AXIOM COMMS [INTERCEPTED]:\n\
            \"Perimeter breach in Sector 7. Deploy the\n\
            Cyborg Carrier. All units converge.\"\n\n\
            They know you're coming. Good.",
        boss_name: "DEFENSE SATELLITE",
        waves: &[
            (2.0, EnemyType::Drone, 600.0, 80.0, 3),
            (5.0, EnemyType::Drone, 600.0, 200.0, 4),
            (8.0, EnemyType::Drone, 600.0, 350.0, 3),
            (12.0, EnemyType::Gunship, 650.0, 150.0, 1),
            (15.0, EnemyType::Drone, 600.0, 100.0, 5),
            (18.0, EnemyType::Gunship, 650.0, 300.0, 1),
            (20.0, EnemyType::Drone, 600.0, 250.0, 4),
            (24.0, EnemyType::Turret, 550.0, 50.0, 1),
            (26.0, EnemyType::Drone, 600.0, 400.0, 5),
            (30.0, EnemyType::Gunship, 650.0, 200.0, 2),
        ],
        boss_hp: 60,
    },
    LevelInfo {
        name: "SECTOR 2: THE SCRAPYARD",
        intro_text: "AXIOM NET -- PRIORITY ALERT\n\n\
            \"All units: the insurgent pilot designated\n\
            VIPER has breached the orbital perimeter.\n\
            Director Kaine has authorized lethal force.\"\n\n\
            The Scrapyard -- AXIOM's dumping ground for\n\
            decommissioned warships. But the drones here\n\
            aren't decommissioned. They're feral.\n\n\
            Somewhere in this maze of dead metal,\n\
            the Cyborg Carrier waits.",
        clear_text: "The Carrier's hull splits open, spewing\n\
            corrupted drones into the void. Its AI\n\
            screams on every frequency before going\n\
            silent.\n\n\
            AXIOM COMMS [INTERCEPTED]:\n\
            \"Director Kaine, the Carrier is down.\n\
            VIPER is approaching the Leviathan.\"\n\
            \"Then wake it up. ALL of it.\"\n\n\
            The Leviathan. The end of the line.",
        boss_name: "CYBORG CARRIER",
        waves: &[
            (2.0, EnemyType::Drone, 600.0, 60.0, 5),
            (4.0, EnemyType::Drone, 600.0, 300.0, 4),
            (7.0, EnemyType::Gunship, 650.0, 120.0, 2),
            (10.0, EnemyType::Turret, 520.0, 80.0, 1),
            (12.0, EnemyType::Drone, 600.0, 200.0, 6),
            (14.0, EnemyType::Gunship, 650.0, 350.0, 2),
            (17.0, EnemyType::Turret, 540.0, 400.0, 1),
            (19.0, EnemyType::Drone, 600.0, 150.0, 5),
            (22.0, EnemyType::Gunship, 650.0, 250.0, 3),
            (25.0, EnemyType::Drone, 600.0, 380.0, 6),
            (28.0, EnemyType::Turret, 500.0, 200.0, 1),
            (30.0, EnemyType::Gunship, 650.0, 100.0, 2),
        ],
        boss_hp: 100,
    },
    LevelInfo {
        name: "SECTOR 3: THE LEVIATHAN",
        intro_text: "CHROME VIPER -- FINAL APPROACH\n\n\
            The Leviathan fills your entire viewport.\n\
            Three kilometers of armored death, bristling\n\
            with weapons that could crack a moon.\n\n\
            AXIOM BROADCAST [ALL CHANNELS]:\n\
            \"This is Director Kaine. VIPER, you've\n\
            fought well. But the Leviathan has never\n\
            been defeated. Surrender now and I'll make\n\
            your death quick.\"\n\n\
            Your response: full throttle.",
        clear_text: "",
        boss_name: "THE LEVIATHAN",
        waves: &[
            (2.0, EnemyType::Drone, 600.0, 100.0, 6),
            (3.0, EnemyType::Drone, 600.0, 350.0, 6),
            (5.0, EnemyType::Gunship, 650.0, 200.0, 3),
            (7.0, EnemyType::Turret, 500.0, 60.0, 1),
            (8.0, EnemyType::Turret, 500.0, 420.0, 1),
            (10.0, EnemyType::Drone, 600.0, 240.0, 8),
            (13.0, EnemyType::Gunship, 650.0, 150.0, 3),
            (15.0, EnemyType::Gunship, 650.0, 350.0, 3),
            (18.0, EnemyType::Turret, 480.0, 150.0, 1),
            (20.0, EnemyType::Drone, 600.0, 200.0, 8),
            (22.0, EnemyType::Gunship, 650.0, 280.0, 4),
            (25.0, EnemyType::Turret, 520.0, 380.0, 1),
            (28.0, EnemyType::Drone, 600.0, 100.0, 10),
            (30.0, EnemyType::Gunship, 650.0, 200.0, 4),
        ],
        boss_hp: 150,
    },
];

const VICTORY_TEXT: &str = "The Leviathan erupts in a chain reaction\n\
    of neon fire. Director Kaine's last\n\
    transmission cuts to static mid-sentence.\n\n\
    Across the colonies, AXIOM's control\n\
    network goes dark. Defense grids shut down.\n\
    Prison doors unlock. Propaganda feeds\n\
    dissolve into white noise.\n\n\
    The colonies are free.\n\n\
    You bank the Chrome Viper toward the\n\
    nearest station, fuel nearly spent,\n\
    hull scarred with a hundred impacts.\n\n\
    They'll call you a hero. You know better.\n\
    You're just a pilot who was angry enough\n\
    to fly into hell.\n\n\
    But tonight, the neon burns a little\n\
    brighter.";

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
    last_shot: i64,
    shot_cooldown: i32,
    invulnerable: i32,
    emp_charges: i32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 80.0,
            y: SCREEN_H / 2.0,
            w: 32.0,
            h: 24.0,
            shields: 3,
            max_shields: 3,
            weapon: WeaponType::DualLaser,
            weapon_timer: 0,
            last_shot: -100,
            shot_cooldown: 10,
            invulnerable: 0,
            emp_charges: 1,
        }
    }
}

#[derive(Clone)]
struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    w: f32,
    h: f32,
    is_player: bool,
    alive: bool,
    color: Color,
    homing: bool,
}

#[derive(Clone)]
struct Enemy {
    x: f32,
    y: f32,
    vx: f32,
    _vy: f32,
    w: f32,
    h: f32,
    etype: EnemyType,
    hp: i32,
    max_hp: i32,
    score: u32,
    alive: bool,
    last_shot: i64,
    shoot_cooldown: i32,
    color: Color,
    spawn_flash: i32,
    angle: f32,       // for turret rotation
    move_timer: f32,   // for sine-wave movement
}

#[derive(Clone, Copy)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    decay: f32,
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
    layer: u8, // 0=far, 1=mid, 2=near (parallax)
}

impl Star {
    fn new(random_x: bool) -> Self {
        let layer: u8 = rand::gen_range(0u8, 3);
        let base_speed = match layer {
            0 => 0.3,
            1 => 0.8,
            _ => 1.5,
        };
        let size = match layer {
            0 => rand::gen_range(0.5, 1.0),
            1 => rand::gen_range(1.0, 2.0),
            _ => rand::gen_range(1.5, 3.0),
        };
        Self {
            x: if random_x { rand::gen_range(0.0, SCREEN_W) } else { SCREEN_W + 10.0 },
            y: rand::gen_range(0.0, SCREEN_H),
            size,
            speed: base_speed * SCROLL_SPEED,
            brightness: rand::gen_range(0.2, 1.0),
            layer,
        }
    }
}

#[derive(Clone)]
struct PowerUp {
    x: f32,
    y: f32,
    vx: f32,
    kind: PowerUpKind,
    alive: bool,
    color: Color,
    letter: char,
}

impl PowerUp {
    fn new(x: f32, y: f32) -> Self {
        let r = rand::gen_range(0.0f32, 1.0);
        let (kind, color, letter) = if r < 0.3 {
            (PowerUpKind::Spread, NEON_ORANGE, 'S')
        } else if r < 0.55 {
            (PowerUpKind::Homing, NEON_PURPLE, 'H')
        } else if r < 0.75 {
            (PowerUpKind::Emp, NEON_CYAN, 'E')
        } else {
            (PowerUpKind::Shield, NEON_GREEN, '+')
        };
        Self { x, y, vx: -1.5, kind, alive: true, color, letter }
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
struct DyingEnemy {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    _etype: EnemyType,
    color: Color,
    frames_left: i32,
}

// ---------------------------------------------------------------------------
// Main Game
// ---------------------------------------------------------------------------
struct Game {
    state: GameState,
    frame: i64,
    score: u32,
    level_idx: usize,
    level_time: f32, // seconds into current level
    wave_event_idx: usize, // next wave event to process
    boss_spawned: bool,
    boss_alive: bool,

    player: Player,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    particles: Vec<Particle>,
    stars: Vec<Star>,
    power_ups: Vec<PowerUp>,
    floating_texts: Vec<FloatingText>,
    dying_enemies: Vec<DyingEnemy>,

    // screen shake
    shake_mag: f32,
    shake_x: f32,
    shake_y: f32,

    // chain/combo
    chain_count: u32,
    chain_timer: i32,

    // hitstop
    hitstop_frames: i32,

    // muzzle flash
    muzzle_flash: i32,

    // story typewriter
    story_text: String,
    story_char_idx: usize,
    story_displayed: String,
    story_wait: i32,
    showing_clear_text: bool,
    victory_triggered: bool,

    // shoot held
    shoot_held: bool,

    // EMP flash
    emp_flash: i32,
}

impl Game {
    fn new() -> Self {
        let mut stars = Vec::with_capacity(STAR_COUNT);
        for _ in 0..STAR_COUNT {
            stars.push(Star::new(true));
        }
        Self {
            state: GameState::Start,
            frame: 0,
            score: 0,
            level_idx: 0,
            level_time: 0.0,
            wave_event_idx: 0,
            boss_spawned: false,
            boss_alive: false,

            player: Player::new(),
            bullets: Vec::new(),
            enemies: Vec::new(),
            particles: Vec::new(),
            stars,
            power_ups: Vec::new(),
            floating_texts: Vec::new(),
            dying_enemies: Vec::new(),

            shake_mag: 0.0,
            shake_x: 0.0,
            shake_y: 0.0,

            chain_count: 0,
            chain_timer: 0,

            hitstop_frames: 0,

            muzzle_flash: 0,

            story_text: String::new(),
            story_char_idx: 0,
            story_displayed: String::new(),
            story_wait: 0,
            showing_clear_text: false,
            victory_triggered: false,

            shoot_held: false,
            emp_flash: 0,
        }
    }

    fn reset_for_new_game(&mut self) {
        self.score = 0;
        self.level_idx = 0;
        self.player = Player::new();
        self.bullets.clear();
        self.enemies.clear();
        self.power_ups.clear();
        self.floating_texts.clear();
        self.dying_enemies.clear();
        self.chain_count = 0;
        self.chain_timer = 0;
        self.shake_mag = 0.0;
        self.shake_x = 0.0;
        self.shake_y = 0.0;
        self.muzzle_flash = 0;
        self.victory_triggered = false;
        self.boss_spawned = false;
        self.boss_alive = false;
        self.hitstop_frames = 0;
        self.emp_flash = 0;
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
            let spd: f32 = rand::gen_range(0.5, 4.0) * speed_scale;
            self.particles.push(Particle {
                x,
                y,
                vx: angle.cos() * spd,
                vy: angle.sin() * spd,
                life: 1.0,
                decay: rand::gen_range(0.02, 0.06),
                color,
                alive: true,
                size: rand::gen_range(1.5, 3.5),
            });
        }
    }

    fn overlaps(ax: f32, ay: f32, aw: f32, ah: f32, bx: f32, by: f32, bw: f32, bh: f32) -> bool {
        ax - aw / 2.0 < bx + bw / 2.0
            && ax + aw / 2.0 > bx - bw / 2.0
            && ay - ah / 2.0 < by + bh / 2.0
            && ay + ah / 2.0 > by - bh / 2.0
    }

    fn current_level(&self) -> &'static LevelInfo {
        &LEVELS[self.level_idx.min(LEVELS.len() - 1)]
    }

    // ------------------------------------------------------------------
    // State transitions
    // ------------------------------------------------------------------
    fn start_intro_story(&mut self) {
        self.reset_for_new_game();
        self.state = GameState::Story;
        self.showing_clear_text = false;
        let level = self.current_level();
        self.story_text = level.intro_text.to_string();
        self.story_char_idx = 0;
        self.story_displayed.clear();
        self.story_wait = 0;
    }

    fn start_level_story(&mut self, text: &str) {
        self.state = GameState::LevelStory;
        self.showing_clear_text = true;
        self.story_text = text.to_string();
        self.story_char_idx = 0;
        self.story_displayed.clear();
        self.story_wait = 0;
        self.bullets.clear();
        self.enemies.clear();
        self.power_ups.clear();
        self.dying_enemies.clear();
        self.floating_texts.clear();
    }

    fn start_level(&mut self) {
        self.state = GameState::Playing;
        self.frame = 0;
        self.level_time = 0.0;
        self.wave_event_idx = 0;
        self.boss_spawned = false;
        self.boss_alive = false;
        self.bullets.clear();
        self.enemies.clear();
        self.power_ups.clear();
        self.dying_enemies.clear();
        self.floating_texts.clear();
        self.chain_count = 0;
        self.chain_timer = 0;
        self.player.last_shot = -100;
    }

    fn show_victory(&mut self) {
        self.state = GameState::Story;
        self.victory_triggered = true;
        self.bullets.clear();
        self.enemies.clear();
        self.power_ups.clear();
        self.dying_enemies.clear();
        self.story_text = VICTORY_TEXT.to_string();
        self.story_char_idx = 0;
        self.story_displayed.clear();
        self.story_wait = 0;
    }

    fn game_over(&mut self) {
        let px = self.player.x;
        let py = self.player.y;
        self.spawn_particles(px, py, 80, NEON_CYAN, 4.0);
        self.spawn_particles(px, py, 40, NEON_PINK, 3.0);
        self.trigger_shake(10.0);
        self.state = GameState::GameOver;
    }

    fn hit_player(&mut self) {
        if self.player.invulnerable > 0 {
            return;
        }
        self.trigger_shake(5.0);
        let px = self.player.x;
        let py = self.player.y;

        self.player.shields -= 1;
        self.spawn_particles(px, py, 20, NEON_CYAN, 2.5);

        if self.player.shields <= 0 {
            self.game_over();
        } else {
            self.player.invulnerable = 90;
        }
    }

    // ------------------------------------------------------------------
    // Player shooting
    // ------------------------------------------------------------------
    fn player_shoot(&mut self) {
        let bx = self.player.x + self.player.w / 2.0;
        let by = self.player.y;

        match self.player.weapon {
            WeaponType::DualLaser => {
                self.player.shot_cooldown = 10;
                self.bullets.push(mk_bullet(bx, by - 6.0, BULLET_SPEED, 0.0, NEON_CYAN, true, false));
                self.bullets.push(mk_bullet(bx, by + 6.0, BULLET_SPEED, 0.0, NEON_CYAN, true, false));
            }
            WeaponType::SpreadShot => {
                self.player.shot_cooldown = 14;
                let angles: [f32; 5] = [-0.3, -0.15, 0.0, 0.15, 0.3];
                for a in &angles {
                    let vx = BULLET_SPEED * a.cos();
                    let vy = BULLET_SPEED * a.sin();
                    self.bullets.push(mk_bullet(bx, by, vx, vy, NEON_ORANGE, true, false));
                }
            }
            WeaponType::HomingMissile => {
                self.player.shot_cooldown = 18;
                self.bullets.push(mk_bullet(bx, by - 4.0, BULLET_SPEED * 0.7, -1.0, NEON_PURPLE, true, true));
                self.bullets.push(mk_bullet(bx, by + 4.0, BULLET_SPEED * 0.7, 1.0, NEON_PURPLE, true, true));
            }
        }

        let frame = self.frame;
        self.player.last_shot = frame;
        self.muzzle_flash = 4;
        self.spawn_particles(bx, by, 3, NEON_CYAN, 1.0);
    }

    fn fire_emp(&mut self) {
        if self.player.emp_charges <= 0 {
            return;
        }
        self.player.emp_charges -= 1;
        self.emp_flash = 15;
        self.trigger_shake(6.0);

        // Damage all enemies on screen
        for i in 0..self.enemies.len() {
            self.enemies[i].hp -= 3;
            let ex = self.enemies[i].x;
            let ey = self.enemies[i].y;
            self.spawn_particles(ex, ey, 8, NEON_CYAN, 2.0);
        }
        // Clear enemy bullets
        for b in self.bullets.iter_mut() {
            if !b.is_player {
                b.alive = false;
            }
        }
    }

    // ------------------------------------------------------------------
    // Spawn wave enemies
    // ------------------------------------------------------------------
    fn process_wave_events(&mut self) {
        let level = self.current_level();
        let waves = level.waves;

        while self.wave_event_idx < waves.len() {
            let (time, etype, x, y, count) = waves[self.wave_event_idx];
            if self.level_time < time {
                break;
            }
            for i in 0..count {
                let offset_y = i as f32 * 40.0;
                self.spawn_enemy(etype, x, y + offset_y);
            }
            self.wave_event_idx += 1;
        }

        // Spawn boss after all waves done
        if self.wave_event_idx >= waves.len() && !self.boss_spawned && self.enemies.is_empty() {
            self.spawn_boss();
            self.boss_spawned = true;
            self.boss_alive = true;
        }
    }

    fn spawn_enemy(&mut self, etype: EnemyType, x: f32, y: f32) {
        let level_mult = 1.0 + self.level_idx as f32 * 0.15;
        let (color, vx, _vy_unused, hp, score_val, w, h, shoot_cd) = match etype {
            EnemyType::Drone => (
                NEON_PINK, -ENEMY_SPEED * level_mult, 0.0f32,
                1, 100u32, 16.0f32, 16.0f32, 9999i32,
            ),
            EnemyType::Gunship => (
                NEON_ORANGE, -ENEMY_SPEED * 0.7 * level_mult, 0.0f32,
                3, 250u32, 24.0f32, 20.0f32, 90i32,
            ),
            EnemyType::Turret => (
                NEON_PURPLE, -ENEMY_SPEED * 0.3 * level_mult, 0.0f32,
                5, 400u32, 20.0f32, 20.0f32, 60i32,
            ),
            EnemyType::Boss => {
                // handled by spawn_boss
                (WHITE, 0.0, 0.0, 1, 0, 0.0, 0.0, 0)
            }
        };

        self.enemies.push(Enemy {
            x,
            y: y.clamp(20.0, SCREEN_H - 20.0),
            vx,
            _vy: 0.0,
            w,
            h,
            etype,
            hp,
            max_hp: hp,
            score: score_val,
            alive: true,
            last_shot: self.frame + rand::gen_range(0, 30) as i64,
            shoot_cooldown: shoot_cd,
            color,
            spawn_flash: 6,
            angle: 0.0,
            move_timer: rand::gen_range(0.0f32, std::f32::consts::TAU),
        });
    }

    fn spawn_boss(&mut self) {
        let level = self.current_level();
        let boss_hp = level.boss_hp;
        self.enemies.push(Enemy {
            x: SCREEN_W + 40.0,
            y: SCREEN_H / 2.0,
            vx: -1.0,
            _vy: 0.0,
            w: 64.0,
            h: 48.0,
            etype: EnemyType::Boss,
            hp: boss_hp,
            max_hp: boss_hp,
            score: 2000,
            alive: true,
            last_shot: self.frame,
            shoot_cooldown: 30,
            color: match self.level_idx {
                0 => NEON_CYAN,
                1 => NEON_ORANGE,
                _ => NEON_PINK,
            },
            spawn_flash: 10,
            angle: 0.0,
            move_timer: 0.0,
        });
    }

    // ------------------------------------------------------------------
    // Update
    // ------------------------------------------------------------------
    fn update(&mut self) {
        // Stars always update (horizontal scrolling)
        let playing = self.state == GameState::Playing;
        let star_mult = if playing { 2.0 } else { 0.5 };
        for s in self.stars.iter_mut() {
            s.x -= s.speed * star_mult;
            if s.x < -5.0 {
                *s = Star::new(false);
            }
        }

        // Particles always update
        for p in self.particles.iter_mut() {
            p.x += p.vx;
            p.y += p.vy;
            p.life -= p.decay;
            if p.life <= 0.0 {
                p.alive = false;
            }
        }
        self.particles.retain(|p| p.alive);

        // Input
        let left = is_key_down(KeyCode::Left);
        let right = is_key_down(KeyCode::Right);
        let up = is_key_down(KeyCode::Up);
        let down = is_key_down(KeyCode::Down);
        self.shoot_held = is_key_down(KeyCode::X);
        let special_pressed = is_key_pressed(KeyCode::Space);
        let enter = is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter);

        // ----- Start screen -----
        if self.state == GameState::Start {
            if enter || is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Space) {
                self.start_intro_story();
            }
            self.frame += 1;
            return;
        }

        // ----- Story / LevelStory -----
        if self.state == GameState::Story || self.state == GameState::LevelStory {
            let txt_len = self.story_text.len();
            if self.story_char_idx < txt_len {
                if self.frame % 2 == 0 {
                    let ch = self.story_text.as_bytes()[self.story_char_idx] as char;
                    self.story_displayed.push(ch);
                    self.story_char_idx += 1;
                }
                if enter || is_key_pressed(KeyCode::X) {
                    self.story_displayed = self.story_text.clone();
                    self.story_char_idx = txt_len;
                }
            } else {
                if self.story_wait == 0 {
                    self.story_wait = 120;
                }
                self.story_wait -= 1;
                if self.story_wait <= 0 || enter || is_key_pressed(KeyCode::X) {
                    if self.victory_triggered {
                        self.state = GameState::Win;
                    } else if self.state == GameState::LevelStory && self.showing_clear_text {
                        // After level-clear text, advance to next level intro
                        self.level_idx += 1;
                        if self.level_idx >= LEVELS.len() {
                            self.show_victory();
                        } else {
                            self.state = GameState::Story;
                            self.showing_clear_text = false;
                            let level = self.current_level();
                            self.story_text = level.intro_text.to_string();
                            self.story_char_idx = 0;
                            self.story_displayed.clear();
                            self.story_wait = 0;
                        }
                    } else {
                        // Story intro done, start level
                        self.start_level();
                    }
                }
            }
            self.frame += 1;
            return;
        }

        // ----- Game Over / Win -----
        if self.state == GameState::GameOver || self.state == GameState::Win {
            if enter || is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Space) {
                self.start_intro_story();
            }
            self.frame += 1;
            return;
        }

        // ----- Playing -----
        // Hitstop
        if self.hitstop_frames > 0 {
            self.hitstop_frames -= 1;
            self.frame += 1;
            return;
        }

        self.frame += 1;
        self.level_time += 1.0 / 60.0;

        // Player movement (8-directional, constrained to left portion)
        let spd = PLAYER_SPEED;
        if left  { self.player.x -= spd; }
        if right { self.player.x += spd; }
        if up    { self.player.y -= spd; }
        if down  { self.player.y += spd; }

        let pw2 = self.player.w / 2.0;
        let ph2 = self.player.h / 2.0;
        self.player.x = self.player.x.clamp(pw2, PLAYER_MOVE_BOUND);
        self.player.y = self.player.y.clamp(ph2, SCREEN_H - ph2);

        // Weapon timer
        if self.player.weapon_timer > 0 {
            self.player.weapon_timer -= 1;
            if self.player.weapon_timer <= 0 {
                self.player.weapon = WeaponType::DualLaser;
            }
        }

        // Shoot
        if self.shoot_held && (self.frame - self.player.last_shot) as i32 > self.player.shot_cooldown {
            self.player_shoot();
        }

        // Special weapon (EMP)
        if special_pressed {
            self.fire_emp();
        }

        if self.player.invulnerable > 0 {
            self.player.invulnerable -= 1;
        }

        // Chain timer
        if self.chain_timer > 0 {
            self.chain_timer -= 1;
            if self.chain_timer <= 0 {
                self.chain_count = 0;
            }
        }

        // Muzzle flash
        if self.muzzle_flash > 0 {
            self.muzzle_flash -= 1;
        }

        // EMP flash
        if self.emp_flash > 0 {
            self.emp_flash -= 1;
        }

        // Engine trail particles
        {
            let ex = self.player.x - self.player.w / 2.0;
            let ey = self.player.y;
            for _ in 0..2 {
                let oy: f32 = rand::gen_range(-3.0, 3.0);
                let c = if rand::gen_range(0.0f32, 1.0) > 0.5 {
                    Color::new(0.0, 0.8, 1.0, 0.7)
                } else {
                    Color::new(0.0, 0.4, 1.0, 0.7)
                };
                self.particles.push(Particle {
                    x: ex,
                    y: ey + oy,
                    vx: rand::gen_range(-3.0, -1.0),
                    vy: rand::gen_range(-0.3, 0.3),
                    life: 0.6,
                    decay: rand::gen_range(0.06, 0.12),
                    color: c,
                    alive: true,
                    size: rand::gen_range(1.5, 3.0),
                });
            }
        }

        // Screen shake decay
        if self.shake_mag > 0.1 {
            self.shake_x = rand::gen_range(-1.0f32, 1.0) * self.shake_mag;
            self.shake_y = rand::gen_range(-1.0f32, 1.0) * self.shake_mag;
            self.shake_mag *= 0.85;
        } else {
            self.shake_x = 0.0;
            self.shake_y = 0.0;
            self.shake_mag = 0.0;
        }

        // Floating texts
        for ft in self.floating_texts.iter_mut() {
            ft.y -= 0.8;
            ft.life -= 1;
        }
        self.floating_texts.retain(|ft| ft.life > 0);

        // Dying enemies
        for de in self.dying_enemies.iter_mut() {
            de.frames_left -= 1;
        }
        self.dying_enemies.retain(|de| de.frames_left > 0);

        // Process wave events
        self.process_wave_events();

        // Update bullets
        for b in self.bullets.iter_mut() {
            if b.homing && b.is_player {
                // Simple homing: steer toward nearest enemy (not implemented as full search for perf,
                // just adjust vy toward center area)
                b.vy *= 0.95;
                b.vx = b.vx.abs().max(BULLET_SPEED * 0.5); // always move right
            }
            b.x += b.vx;
            b.y += b.vy;
            if b.x < -50.0 || b.x > SCREEN_W + 50.0 || b.y < -50.0 || b.y > SCREEN_H + 50.0 {
                b.alive = false;
            }
        }

        // Homing: find nearest enemy and steer
        {
            let enemies_snapshot: Vec<(f32, f32, bool)> = self.enemies.iter()
                .filter(|e| e.alive)
                .map(|e| (e.x, e.y, true))
                .collect();

            for b in self.bullets.iter_mut() {
                if !b.homing || !b.is_player || !b.alive {
                    continue;
                }
                let mut best_dist = f32::MAX;
                let mut best_y = b.y;
                for &(ex, ey, _) in &enemies_snapshot {
                    let dx = ex - b.x;
                    let dy = ey - b.y;
                    let dist = dx * dx + dy * dy;
                    if dist < best_dist {
                        best_dist = dist;
                        best_y = ey;
                    }
                }
                if best_dist < f32::MAX {
                    let dy = best_y - b.y;
                    b.vy += dy.signum() * 0.5;
                    b.vy = b.vy.clamp(-4.0, 4.0);
                }
            }
        }

        // Bullet trail particles (player bullets)
        {
            let mut trails = Vec::new();
            for b in &self.bullets {
                if !b.alive || !b.is_player { continue; }
                if rand::gen_range(0u32, 3) == 0 {
                    trails.push(Particle {
                        x: b.x + rand::gen_range(-1.5, 1.5),
                        y: b.y + rand::gen_range(-1.0, 1.0),
                        vx: rand::gen_range(-0.5, 0.0),
                        vy: rand::gen_range(-0.2, 0.2),
                        life: rand::gen_range(0.2, 0.4),
                        decay: rand::gen_range(0.05, 0.1),
                        color: Color::new(b.color.r, b.color.g, b.color.b, 0.4),
                        alive: true,
                        size: 1.5,
                    });
                }
            }
            self.particles.extend(trails);
        }

        // Update enemies
        let mut new_bullets: Vec<Bullet> = Vec::new();
        let px = self.player.x;
        let py = self.player.y;
        let frame = self.frame;
        for e in self.enemies.iter_mut() {
            // Spawn flash
            if e.spawn_flash > 0 {
                e.spawn_flash -= 1;
            }

            match e.etype {
                EnemyType::Drone => {
                    e.x += e.vx;
                    e.move_timer += 0.05;
                    e.y += (e.move_timer).sin() * 1.5;
                }
                EnemyType::Gunship => {
                    e.x += e.vx;
                    e.move_timer += 0.03;
                    e.y += (e.move_timer).sin() * 2.0;
                    // Shoot at player
                    if (frame - e.last_shot) as i32 > e.shoot_cooldown {
                        let dx = px - e.x;
                        let dy = py - e.y;
                        let dist = (dx * dx + dy * dy).sqrt().max(1.0);
                        let bvx = dx / dist * 4.0;
                        let bvy = dy / dist * 4.0;
                        new_bullets.push(mk_bullet(e.x, e.y, bvx, bvy, NEON_PINK, false, false));
                        e.last_shot = frame;
                    }
                }
                EnemyType::Turret => {
                    e.x += e.vx;
                    // Rotate and fire
                    e.angle += 0.02;
                    if (frame - e.last_shot) as i32 > e.shoot_cooldown {
                        let a = e.angle;
                        let bvx = a.cos() * 3.5;
                        let bvy = a.sin() * 3.5;
                        new_bullets.push(mk_bullet(e.x, e.y, bvx, bvy, NEON_PURPLE, false, false));
                        // Also fire opposite
                        new_bullets.push(mk_bullet(e.x, e.y, -bvx, -bvy, NEON_PURPLE, false, false));
                        e.last_shot = frame;
                    }
                }
                EnemyType::Boss => {
                    // Move into position, then bob
                    if e.x > SCREEN_W * 0.7 {
                        e.x -= 1.0;
                    }
                    e.move_timer += 0.02;
                    e.y = SCREEN_H / 2.0 + (e.move_timer).sin() * 100.0;

                    // Boss attack patterns
                    if (frame - e.last_shot) as i32 > e.shoot_cooldown {
                        let pattern = (frame / 120) % 3;
                        match pattern {
                            0 => {
                                // Spread shot toward player
                                for i in 0..5 {
                                    let a: f32 = -0.4 + i as f32 * 0.2;
                                    let bvx = -5.0 * (1.0 + a * 0.3);
                                    let bvy = a * 3.0;
                                    new_bullets.push(mk_bullet(
                                        e.x - e.w / 2.0, e.y, bvx, bvy, e.color, false, false,
                                    ));
                                }
                            }
                            1 => {
                                // Aimed shot
                                let dx = px - e.x;
                                let dy = py - e.y;
                                let dist = (dx * dx + dy * dy).sqrt().max(1.0);
                                for i in 0..3 {
                                    let spd = 4.0 + i as f32 * 0.5;
                                    new_bullets.push(mk_bullet(
                                        e.x - e.w / 2.0, e.y,
                                        dx / dist * spd, dy / dist * spd,
                                        e.color, false, false,
                                    ));
                                }
                            }
                            _ => {
                                // Circle burst
                                for i in 0..8 {
                                    let a: f32 = i as f32 * std::f32::consts::TAU / 8.0;
                                    new_bullets.push(mk_bullet(
                                        e.x, e.y,
                                        a.cos() * 3.0, a.sin() * 3.0,
                                        e.color, false, false,
                                    ));
                                }
                            }
                        }
                        e.last_shot = frame;
                    }
                }
            }

            // Off-screen removal (non-boss)
            if e.etype != EnemyType::Boss && e.x < -60.0 {
                e.alive = false;
            }
        }
        self.bullets.extend(new_bullets);

        // Update power-ups (scroll left)
        for p in self.power_ups.iter_mut() {
            p.x += p.vx;
            if p.x < -20.0 {
                p.alive = false;
            }
        }

        // ----- Collisions -----
        // Player bullets vs enemies
        for i in 0..self.bullets.len() {
            if !self.bullets[i].alive || !self.bullets[i].is_player {
                continue;
            }
            let bx = self.bullets[i].x;
            let by = self.bullets[i].y;
            let bw = self.bullets[i].w;
            let bh = self.bullets[i].h;

            for j in 0..self.enemies.len() {
                if !self.enemies[j].alive {
                    continue;
                }
                let ex = self.enemies[j].x;
                let ey = self.enemies[j].y;
                let ew = self.enemies[j].w;
                let eh = self.enemies[j].h;

                if Self::overlaps(bx, by, bw, bh, ex, ey, ew, eh) {
                    self.bullets[i].alive = false;
                    self.enemies[j].hp -= 1;

                    // Hit particles
                    self.spawn_particles(bx, by, 4, self.enemies[j].color, 1.5);

                    if self.enemies[j].hp <= 0 {
                        self.enemies[j].alive = false;

                        // Chain multiplier
                        self.chain_count += 1;
                        self.chain_timer = 120; // 2 seconds
                        let multiplier = 1.0 + self.chain_count as f32 * 0.5;
                        let points = (self.enemies[j].score as f32 * multiplier) as u32;
                        self.score += points;

                        // Explosion
                        let ecolor = self.enemies[j].color;
                        self.spawn_particles(ex, ey, 25, ecolor, 3.0);
                        self.spawn_particles(ex, ey, 10, WHITE, 2.0);
                        self.trigger_shake(3.0);

                        // Hitstop on boss damage
                        if self.enemies[j].etype == EnemyType::Boss {
                            self.hitstop_frames = 6;
                            self.trigger_shake(8.0);
                            self.boss_alive = false;
                        }

                        // Dying animation
                        self.dying_enemies.push(DyingEnemy {
                            x: ex,
                            y: ey,
                            w: ew,
                            h: eh,
                            _etype: self.enemies[j].etype,
                            color: ecolor,
                            frames_left: 15,
                        });

                        // Floating score
                        self.floating_texts.push(FloatingText {
                            x: ex,
                            y: ey,
                            text: format!("+{}", points),
                            life: 45,
                            color: if self.chain_count >= 3 { YELLOW } else { WHITE },
                        });

                        // Drop power-up
                        if rand::gen_range(0.0f32, 1.0) < DROP_CHANCE {
                            self.power_ups.push(PowerUp::new(ex, ey));
                        }
                    } else if self.enemies[j].etype == EnemyType::Boss {
                        // Boss hit flash
                        self.hitstop_frames = 2;
                    }
                    break;
                }
            }
        }

        // Enemy bullets vs player
        if self.player.invulnerable <= 0 && self.state == GameState::Playing {
            let ppx = self.player.x;
            let ppy = self.player.y;
            let ppw = self.player.w * 0.6; // smaller hitbox
            let pph = self.player.h * 0.6;

            let mut was_hit = false;
            for b in self.bullets.iter_mut() {
                if !b.alive || b.is_player { continue; }
                if Self::overlaps(ppx, ppy, ppw, pph, b.x, b.y, b.w, b.h) {
                    b.alive = false;
                    was_hit = true;
                    break;
                }
            }

            // Enemy body collision
            if !was_hit {
                for e in self.enemies.iter() {
                    if !e.alive { continue; }
                    if Self::overlaps(ppx, ppy, ppw, pph, e.x, e.y, e.w, e.h) {
                        was_hit = true;
                        break;
                    }
                }
            }

            if was_hit {
                self.hit_player();
            }
        }

        // Player vs power-ups
        {
            let ppx = self.player.x;
            let ppy = self.player.y;
            let ppw = self.player.w;
            let pph = self.player.h;

            for i in 0..self.power_ups.len() {
                if !self.power_ups[i].alive { continue; }
                let pu = &self.power_ups[i];
                if Self::overlaps(ppx, ppy, ppw, pph, pu.x, pu.y, 16.0, 16.0) {
                    let kind = self.power_ups[i].kind;
                    let pux = self.power_ups[i].x;
                    let puy = self.power_ups[i].y;
                    let puc = self.power_ups[i].color;
                    self.power_ups[i].alive = false;

                    self.spawn_particles(pux, puy, 12, puc, 2.0);

                    match kind {
                        PowerUpKind::Spread => {
                            self.player.weapon = WeaponType::SpreadShot;
                            self.player.weapon_timer = 600; // 10 seconds
                            self.floating_texts.push(FloatingText {
                                x: pux, y: puy,
                                text: "SPREAD SHOT".to_string(),
                                life: 50,
                                color: NEON_ORANGE,
                            });
                        }
                        PowerUpKind::Homing => {
                            self.player.weapon = WeaponType::HomingMissile;
                            self.player.weapon_timer = 600;
                            self.floating_texts.push(FloatingText {
                                x: pux, y: puy,
                                text: "HOMING".to_string(),
                                life: 50,
                                color: NEON_PURPLE,
                            });
                        }
                        PowerUpKind::Emp => {
                            self.player.emp_charges += 1;
                            self.floating_texts.push(FloatingText {
                                x: pux, y: puy,
                                text: "EMP +1".to_string(),
                                life: 50,
                                color: NEON_CYAN,
                            });
                        }
                        PowerUpKind::Shield => {
                            if self.player.shields < self.player.max_shields {
                                self.player.shields += 1;
                            }
                            self.floating_texts.push(FloatingText {
                                x: pux, y: puy,
                                text: "SHIELD +1".to_string(),
                                life: 50,
                                color: NEON_GREEN,
                            });
                        }
                    }
                }
            }
        }

        // Cleanup
        self.bullets.retain(|b| b.alive);
        self.enemies.retain(|e| e.alive);
        self.power_ups.retain(|p| p.alive);

        // Check boss defeated -> level clear
        if self.boss_spawned && !self.boss_alive && self.enemies.iter().all(|e| e.etype != EnemyType::Boss) {
            let clear_text = self.current_level().clear_text;
            if clear_text.is_empty() {
                // Final level, no clear text -> victory
                self.show_victory();
            } else {
                self.start_level_story(clear_text);
            }
        }
    }

    // ------------------------------------------------------------------
    // Draw
    // ------------------------------------------------------------------
    fn draw(&self) {
        clear_background(DARK_BG);

        let sx = self.shake_x;
        let sy = self.shake_y;

        // Grid lines (cyberpunk aesthetic)
        {
            let grid_col = Color::new(0.0, 1.0, 1.0, 0.03);
            let step = 40.0;
            let mut gx = 0.0;
            while gx < SCREEN_W {
                draw_line(gx + sx, 0.0, gx + sx, SCREEN_H, 1.0, grid_col);
                gx += step;
            }
            let mut gy = 0.0;
            while gy < SCREEN_H {
                draw_line(0.0, gy + sy, SCREEN_W, gy + sy, 1.0, grid_col);
                gy += step;
            }
        }

        // Parallax starfield
        for (i, s) in self.stars.iter().enumerate() {
            let twinkle = 0.6 + 0.4 * ((self.frame as f32 * 0.04 + i as f32 * 2.1).sin());
            let alpha = s.brightness * twinkle;
            let alpha = alpha.clamp(0.2, 1.0);
            let tint = match s.layer {
                0 => Color::new(0.4, 0.4, 0.6, alpha),
                1 => Color::new(0.6, 0.7, 1.0, alpha),
                _ => Color::new(0.8, 0.9, 1.0, alpha),
            };
            draw_rectangle(s.x + sx, s.y + sy, s.size, s.size, tint);
        }

        // EMP flash
        if self.emp_flash > 0 {
            let alpha = self.emp_flash as f32 / 15.0 * 0.3;
            draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.0, 1.0, 1.0, alpha));
        }

        // Power-ups
        for p in &self.power_ups {
            let pulse = 1.0 + (self.frame as f32 * 0.1).sin() * 0.15;
            let r = 10.0 * pulse;
            draw_circle_lines(p.x + sx, p.y + sy, r, 2.0, p.color);
            let glow = Color::new(p.color.r, p.color.g, p.color.b, 0.15);
            draw_circle(p.x + sx, p.y + sy, r + 3.0, glow);
            let txt = &p.letter.to_string();
            let tw = measure_text(txt, None, 14, 1.0).width;
            draw_text(txt, p.x + sx - tw / 2.0, p.y + sy + 5.0, 14.0, p.color);
        }

        // Particles
        for p in &self.particles {
            let c = Color::new(p.color.r, p.color.g, p.color.b, p.life);
            draw_rectangle(p.x + sx - p.size / 2.0, p.y + sy - p.size / 2.0, p.size, p.size, c);
        }

        if self.state == GameState::Playing || self.state == GameState::GameOver || self.state == GameState::Win {
            // Bullets
            for b in &self.bullets {
                let gc = Color::new(b.color.r, b.color.g, b.color.b, 0.3);
                if b.is_player {
                    // Horizontal laser style
                    draw_rectangle(b.x + sx - b.w / 2.0, b.y + sy - b.h, b.w, b.h * 2.0, gc);
                    draw_rectangle(b.x + sx - b.w / 2.0, b.y + sy - b.h / 2.0, b.w, b.h, WHITE);
                    draw_rectangle_lines(b.x + sx - b.w / 2.0, b.y + sy - b.h / 2.0, b.w, b.h, 1.0, b.color);
                } else {
                    // Enemy bullets: small circles
                    draw_circle(b.x + sx, b.y + sy, 4.0, gc);
                    draw_circle(b.x + sx, b.y + sy, 2.5, b.color);
                }
            }

            // Enemies
            for e in &self.enemies {
                self.draw_enemy(e.x + sx, e.y + sy, e.w, e.h, e.etype, e.color, e.hp, e.max_hp);
                if e.spawn_flash > 0 {
                    let t = e.spawn_flash as f32 / 6.0;
                    let radius = e.w * (1.0 + (1.0 - t) * 1.5);
                    let alpha = t * 0.6;
                    draw_circle(e.x + sx, e.y + sy, radius, Color::new(1.0, 1.0, 1.0, alpha));
                }
            }

            // Dying enemies
            for de in &self.dying_enemies {
                let t = de.frames_left as f32 / 15.0;
                let c = Color::new(de.color.r, de.color.g, de.color.b, t);
                draw_circle(de.x + sx, de.y + sy, de.w * (2.0 - t), Color::new(1.0, 1.0, 1.0, t * 0.3));
                draw_rectangle(
                    de.x + sx - de.w * t / 2.0,
                    de.y + sy - de.h * t / 2.0,
                    de.w * t, de.h * t, c,
                );
            }
        }

        // Player
        if self.state == GameState::Playing {
            let p = &self.player;
            if p.invulnerable > 0 && (self.frame / 4) % 2 == 0 {
                // blink
            } else {
                self.draw_player(p);
            }
        }

        // Chain display
        if self.state == GameState::Playing && self.chain_count >= 3 {
            let chain_txt = format!("CHAIN x{}", self.chain_count);
            let tw = measure_text(&chain_txt, None, 20, 1.0).width;
            draw_text(&chain_txt, SCREEN_W / 2.0 - tw / 2.0, 50.0, 20.0, YELLOW);
            let mult_txt = format!("x{:.1} SCORE", 1.0 + self.chain_count as f32 * 0.5);
            let tw2 = measure_text(&mult_txt, None, 14, 1.0).width;
            draw_text(&mult_txt, SCREEN_W / 2.0 - tw2 / 2.0, 68.0, 14.0, WHITE);
        }

        // Floating texts
        for ft in &self.floating_texts {
            let alpha = ft.life as f32 / 45.0;
            let c = Color::new(ft.color.r, ft.color.g, ft.color.b, alpha);
            let tw = measure_text(&ft.text, None, 14, 1.0).width;
            draw_text(&ft.text, ft.x + sx - tw / 2.0, ft.y + sy, 14.0, c);
        }

        // HUD
        if self.state == GameState::Playing {
            self.draw_hud();
        }

        // Overlay screens
        match self.state {
            GameState::Start => self.draw_title(),
            GameState::Story | GameState::LevelStory => self.draw_story(),
            GameState::GameOver => self.draw_game_over(),
            GameState::Win => self.draw_win(),
            GameState::Playing => {}
        }

        // CRT scanline overlay
        {
            let scanline_color = Color::new(0.0, 0.0, 0.0, 0.15);
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
    fn draw_player(&self, p: &Player) {
        let sx = self.shake_x;
        let sy = self.shake_y;
        let cx = p.x + sx;
        let cy = p.y + sy;
        let hw = p.w / 2.0;
        let hh = p.h / 2.0;

        // Shield shimmer
        if p.shields > 0 {
            let shimmer = (self.frame as f32 * 0.08).sin() * 0.15 + 0.2;
            let shield_r = p.w * 0.8;
            draw_circle_lines(cx, cy, shield_r, 1.5, Color::new(0.0, 1.0, 1.0, shimmer));
            if p.shields >= 2 {
                draw_circle_lines(cx, cy, shield_r + 3.0, 1.0, Color::new(0.0, 0.8, 1.0, shimmer * 0.5));
            }
        }

        // Ship body (arrow shape pointing right)
        let nose = Vec2::new(cx + hw, cy);
        let top_wing = Vec2::new(cx - hw, cy - hh);
        let bot_wing = Vec2::new(cx - hw, cy + hh);
        let notch_top = Vec2::new(cx - hw * 0.3, cy - hh * 0.3);
        let notch_bot = Vec2::new(cx - hw * 0.3, cy + hh * 0.3);
        let tail = Vec2::new(cx - hw * 0.6, cy);

        // Filled dark body
        draw_triangle(nose, top_wing, tail, Color::new(0.0, 0.05, 0.1, 0.9));
        draw_triangle(nose, tail, bot_wing, Color::new(0.0, 0.05, 0.1, 0.9));

        // Outline in cyan
        let col = NEON_CYAN;
        draw_line(nose.x, nose.y, top_wing.x, top_wing.y, 2.0, col);
        draw_line(top_wing.x, top_wing.y, notch_top.x, notch_top.y, 2.0, col);
        draw_line(notch_top.x, notch_top.y, tail.x, tail.y, 2.0, col);
        draw_line(tail.x, tail.y, notch_bot.x, notch_bot.y, 2.0, col);
        draw_line(notch_bot.x, notch_bot.y, bot_wing.x, bot_wing.y, 2.0, col);
        draw_line(bot_wing.x, bot_wing.y, nose.x, nose.y, 2.0, col);

        // Engine glow
        draw_circle(cx - hw * 0.6, cy, 3.0, WHITE);
        draw_circle(cx - hw * 0.6, cy, 5.0, Color::new(0.0, 0.8, 1.0, 0.4));

        // Muzzle flash
        if self.muzzle_flash > 0 {
            let r = 5.0 + self.muzzle_flash as f32 * 2.0;
            let a = self.muzzle_flash as f32 / 4.0;
            draw_circle(cx + hw, cy, r, Color::new(1.0, 1.0, 1.0, a * 0.5));
        }
    }

    fn draw_enemy(&self, x: f32, y: f32, w: f32, h: f32, etype: EnemyType, color: Color, hp: i32, max_hp: i32) {
        let hw = w / 2.0;
        let hh = h / 2.0;
        let fill = Color::new(0.0, 0.0, 0.0, 0.8 * color.a);

        match etype {
            EnemyType::Drone => {
                // Small diamond
                draw_triangle(Vec2::new(x, y - hh), Vec2::new(x + hw, y), Vec2::new(x, y + hh), fill);
                draw_triangle(Vec2::new(x, y - hh), Vec2::new(x, y + hh), Vec2::new(x - hw, y), fill);
                draw_line(x, y - hh, x + hw, y, 1.5, color);
                draw_line(x + hw, y, x, y + hh, 1.5, color);
                draw_line(x, y + hh, x - hw, y, 1.5, color);
                draw_line(x - hw, y, x, y - hh, 1.5, color);
                draw_circle(x, y, 2.0, color);
            }
            EnemyType::Gunship => {
                // Hexagonal shape
                draw_triangle(
                    Vec2::new(x - hw, y),
                    Vec2::new(x - hw * 0.5, y - hh),
                    Vec2::new(x + hw * 0.5, y - hh),
                    fill,
                );
                draw_triangle(
                    Vec2::new(x - hw, y),
                    Vec2::new(x + hw * 0.5, y - hh),
                    Vec2::new(x + hw, y),
                    fill,
                );
                draw_triangle(
                    Vec2::new(x - hw, y),
                    Vec2::new(x + hw, y),
                    Vec2::new(x + hw * 0.5, y + hh),
                    fill,
                );
                draw_triangle(
                    Vec2::new(x - hw, y),
                    Vec2::new(x + hw * 0.5, y + hh),
                    Vec2::new(x - hw * 0.5, y + hh),
                    fill,
                );
                // Outline
                draw_line(x - hw, y, x - hw * 0.5, y - hh, 2.0, color);
                draw_line(x - hw * 0.5, y - hh, x + hw * 0.5, y - hh, 2.0, color);
                draw_line(x + hw * 0.5, y - hh, x + hw, y, 2.0, color);
                draw_line(x + hw, y, x + hw * 0.5, y + hh, 2.0, color);
                draw_line(x + hw * 0.5, y + hh, x - hw * 0.5, y + hh, 2.0, color);
                draw_line(x - hw * 0.5, y + hh, x - hw, y, 2.0, color);
                // Cockpit
                draw_circle(x, y, 3.0, color);
            }
            EnemyType::Turret => {
                // Square with rotating cross
                draw_rectangle(x - hw, y - hh, w, h, fill);
                draw_rectangle_lines(x - hw, y - hh, w, h, 2.0, color);
                // Rotating barrel indicator
                let a = self.frame as f32 * 0.02;
                let bx = a.cos() * hw * 0.8;
                let by = a.sin() * hh * 0.8;
                draw_line(x, y, x + bx, y + by, 2.0, color);
                draw_line(x, y, x - bx, y - by, 2.0, color);
                draw_circle(x, y, 3.0, color);
            }
            EnemyType::Boss => {
                // Large menacing shape
                // Main body
                draw_rectangle(x - hw, y - hh * 0.6, w, h * 0.6, fill);
                draw_rectangle_lines(x - hw, y - hh * 0.6, w, h * 0.6, 2.0, color);
                // Wings
                draw_triangle(
                    Vec2::new(x - hw, y - hh * 0.6),
                    Vec2::new(x - hw * 0.5, y - hh),
                    Vec2::new(x, y - hh * 0.6),
                    fill,
                );
                draw_triangle(
                    Vec2::new(x - hw, y + hh * 0.6),
                    Vec2::new(x - hw * 0.5, y + hh),
                    Vec2::new(x, y + hh * 0.6),
                    fill,
                );
                // Wing outlines
                draw_line(x - hw, y - hh * 0.6, x - hw * 0.5, y - hh, 2.0, color);
                draw_line(x - hw * 0.5, y - hh, x, y - hh * 0.6, 2.0, color);
                draw_line(x - hw, y + hh * 0.6, x - hw * 0.5, y + hh, 2.0, color);
                draw_line(x - hw * 0.5, y + hh, x, y + hh * 0.6, 2.0, color);
                // Nose weapon
                draw_line(x + hw, y - 4.0, x + hw + 8.0, y, 2.0, color);
                draw_line(x + hw, y + 4.0, x + hw + 8.0, y, 2.0, color);
                // Eye
                let pulse = (self.frame as f32 * 0.1).sin() * 0.3 + 0.7;
                draw_circle(x, y, 6.0, Color::new(color.r, color.g, color.b, pulse));
                draw_circle(x, y, 3.0, WHITE);

                // HP bar
                if hp < max_hp {
                    let bar_w = w * 1.2;
                    let bar_h = 4.0;
                    let bar_x = x - bar_w / 2.0;
                    let bar_y = y - hh - 12.0;
                    draw_rectangle(bar_x, bar_y, bar_w, bar_h, Color::new(0.3, 0.0, 0.0, 0.8));
                    let fill_w = bar_w * (hp as f32 / max_hp as f32);
                    draw_rectangle(bar_x, bar_y, fill_w, bar_h, NEON_PINK);
                    draw_rectangle_lines(bar_x, bar_y, bar_w, bar_h, 1.0, color);
                }
            }
        }
    }

    fn draw_hud(&self) {
        // Score
        let score_txt = format!("SCORE: {}", self.score);
        draw_text(&score_txt, 10.0, 20.0, 16.0, NEON_CYAN);

        // Level name
        let level = self.current_level();
        draw_text(level.name, 10.0, 38.0, 12.0, Color::new(0.5, 0.5, 0.5, 0.8));

        // Shields
        let shield_txt = format!("SHIELDS: {}", self.player.shields);
        let shield_col = if self.player.shields <= 1 { NEON_PINK } else { NEON_GREEN };
        draw_text(&shield_txt, SCREEN_W - 140.0, 20.0, 16.0, shield_col);

        // EMP charges
        let emp_txt = format!("EMP: {}", self.player.emp_charges);
        draw_text(&emp_txt, SCREEN_W - 140.0, 38.0, 14.0, NEON_CYAN);

        // Weapon indicator
        let weapon_name = match self.player.weapon {
            WeaponType::DualLaser => "DUAL LASER",
            WeaponType::SpreadShot => "SPREAD SHOT",
            WeaponType::HomingMissile => "HOMING",
        };
        let wc = match self.player.weapon {
            WeaponType::DualLaser => NEON_CYAN,
            WeaponType::SpreadShot => NEON_ORANGE,
            WeaponType::HomingMissile => NEON_PURPLE,
        };
        draw_text(weapon_name, 10.0, SCREEN_H - 10.0, 14.0, wc);

        // Weapon timer bar
        if self.player.weapon_timer > 0 {
            let bar_w = 100.0;
            let bar_h = 4.0;
            let fill = bar_w * (self.player.weapon_timer as f32 / 600.0);
            draw_rectangle(10.0, SCREEN_H - 26.0, bar_w, bar_h, Color::new(0.2, 0.2, 0.2, 0.6));
            draw_rectangle(10.0, SCREEN_H - 26.0, fill, bar_h, wc);
        }

        // Boss indicator
        for e in &self.enemies {
            if e.etype == EnemyType::Boss {
                let boss_name = self.current_level().boss_name;
                let txt = format!(">> {} <<", boss_name);
                let tw = measure_text(&txt, None, 18, 1.0).width;
                let blink = if (self.frame / 20) % 2 == 0 { 1.0 } else { 0.5 };
                draw_text(&txt, SCREEN_W / 2.0 - tw / 2.0, 30.0, 18.0,
                    Color::new(1.0, 0.1, 0.3, blink));
            }
        }
    }

    fn draw_title(&self) {
        let overlay = Color::new(0.0, 0.0, 0.0, 0.7);
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, overlay);

        // Title
        let title = "CHROME VIPER";
        let tw = measure_text(title, None, 40, 1.0).width;
        let pulse = (self.frame as f32 * 0.05).sin() * 0.2 + 0.8;
        draw_text(title, SCREEN_W / 2.0 - tw / 2.0 + 2.0, SCREEN_H * 0.3 + 2.0, 40.0,
            Color::new(0.0, 0.0, 0.0, 0.5));
        draw_text(title, SCREEN_W / 2.0 - tw / 2.0, SCREEN_H * 0.3, 40.0,
            Color::new(NEON_CYAN.r, NEON_CYAN.g, NEON_CYAN.b, pulse));

        // Subtitle
        let sub = "NEON ABYSS";
        let sw = measure_text(sub, None, 18, 1.0).width;
        draw_text(sub, SCREEN_W / 2.0 - sw / 2.0, SCREEN_H * 0.3 + 30.0, 18.0, NEON_PINK);

        // Tagline
        let tag = "2187 -- AXIOM MUST FALL";
        let tgw = measure_text(tag, None, 12, 1.0).width;
        draw_text(tag, SCREEN_W / 2.0 - tgw / 2.0, SCREEN_H * 0.45, 12.0, NEON_GREEN);

        // Controls
        let controls = [
            "D-PAD: Move Ship",
            "X: Fire Weapon",
            "SPACE: EMP Blast",
            "ENTER: Start",
        ];
        for (i, ctrl) in controls.iter().enumerate() {
            let cw = measure_text(ctrl, None, 12, 1.0).width;
            draw_text(ctrl, SCREEN_W / 2.0 - cw / 2.0, SCREEN_H * 0.55 + i as f32 * 18.0, 12.0,
                Color::new(0.6, 0.6, 0.6, 0.8));
        }

        // Start prompt
        if (self.frame / 30) % 2 == 0 {
            let prompt = "PRESS ENTER TO BEGIN";
            let pw = measure_text(prompt, None, 18, 1.0).width;
            draw_text(prompt, SCREEN_W / 2.0 - pw / 2.0, SCREEN_H * 0.80, 18.0, NEON_CYAN);
        }
    }

    fn draw_story(&self) {
        let overlay = Color::new(0.0, 0.02, 0.0, 0.85);
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, overlay);

        // Terminal header
        let header = if self.victory_triggered {
            "[ MISSION COMPLETE ]"
        } else {
            self.current_level().name
        };
        let hw_text = measure_text(header, None, 16, 1.0).width;
        draw_text(header, SCREEN_W / 2.0 - hw_text / 2.0, 40.0, 16.0, NEON_CYAN);

        // Separator line
        draw_line(40.0, 52.0, SCREEN_W - 40.0, 52.0, 1.0, Color::new(0.0, 0.5, 0.2, 0.5));

        // Typewriter text
        let lines: Vec<&str> = self.story_displayed.split('\n').collect();
        for (i, line) in lines.iter().enumerate() {
            draw_text(line, 50.0, 80.0 + i as f32 * 18.0, 14.0, TERMINAL_GREEN);
        }

        // Cursor blink
        if self.story_char_idx < self.story_text.len() {
            let last_line_idx = lines.len().saturating_sub(1);
            let last_line = if last_line_idx < lines.len() { lines[last_line_idx] } else { "" };
            let lw = measure_text(last_line, None, 14, 1.0).width;
            if (self.frame / 8) % 2 == 0 {
                draw_rectangle(
                    50.0 + lw + 2.0,
                    80.0 + last_line_idx as f32 * 18.0 - 12.0,
                    8.0, 14.0,
                    TERMINAL_GREEN,
                );
            }
        }

        // Skip hint
        if self.story_char_idx < self.story_text.len() {
            let hint = "[PRESS X TO SKIP]";
            let hw2 = measure_text(hint, None, 10, 1.0).width;
            draw_text(hint, SCREEN_W / 2.0 - hw2 / 2.0, SCREEN_H - 30.0, 10.0,
                Color::new(0.3, 0.5, 0.3, 0.6));
        } else if (self.frame / 30) % 2 == 0 {
            let hint = "[PRESS ENTER TO CONTINUE]";
            let hw2 = measure_text(hint, None, 12, 1.0).width;
            draw_text(hint, SCREEN_W / 2.0 - hw2 / 2.0, SCREEN_H - 30.0, 12.0, TERMINAL_GREEN);
        }
    }

    fn draw_game_over(&self) {
        let overlay = Color::new(0.0, 0.0, 0.0, 0.75);
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, overlay);

        let title = "MISSION FAILED";
        let tw = measure_text(title, None, 32, 1.0).width;
        draw_text(title, SCREEN_W / 2.0 - tw / 2.0, SCREEN_H * 0.3, 32.0, NEON_PINK);

        let sc = format!("FINAL SCORE: {}", self.score);
        let sw = measure_text(&sc, None, 18, 1.0).width;
        draw_text(&sc, SCREEN_W / 2.0 - sw / 2.0, SCREEN_H * 0.45, 18.0, WHITE);

        let level_name = self.current_level().name;
        let wv = format!("DESTROYED AT: {}", level_name);
        let ww = measure_text(&wv, None, 14, 1.0).width;
        draw_text(&wv, SCREEN_W / 2.0 - ww / 2.0, SCREEN_H * 0.45 + 26.0, 14.0, NEON_CYAN);

        let lore = "The Chrome Viper drifts into the void. AXIOM prevails.";
        let lw = measure_text(lore, None, 10, 1.0).width;
        draw_text(lore, SCREEN_W / 2.0 - lw / 2.0, SCREEN_H * 0.45 + 52.0, 10.0, GRAY);

        if (self.frame / 30) % 2 == 0 {
            let prompt = "PRESS ENTER TO RETRY";
            let pw = measure_text(prompt, None, 16, 1.0).width;
            draw_text(prompt, SCREEN_W / 2.0 - pw / 2.0, SCREEN_H * 0.70, 16.0, NEON_CYAN);
        }
    }

    fn draw_win(&self) {
        let overlay = Color::new(0.0, 0.0, 0.0, 0.75);
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, overlay);

        let title = "AXIOM HAS FALLEN";
        let tw = measure_text(title, None, 32, 1.0).width;
        let pulse = (self.frame as f32 * 0.05).sin() * 0.2 + 0.8;
        draw_text(title, SCREEN_W / 2.0 - tw / 2.0, SCREEN_H * 0.25, 32.0,
            Color::new(NEON_GREEN.r, NEON_GREEN.g, NEON_GREEN.b, pulse));

        let sub = "THE COLONIES ARE FREE";
        let sw = measure_text(sub, None, 14, 1.0).width;
        draw_text(sub, SCREEN_W / 2.0 - sw / 2.0, SCREEN_H * 0.35, 14.0, NEON_CYAN);

        let sc = format!("FINAL SCORE: {}", self.score);
        let sw2 = measure_text(&sc, None, 20, 1.0).width;
        draw_text(&sc, SCREEN_W / 2.0 - sw2 / 2.0, SCREEN_H * 0.5, 20.0, YELLOW);

        if (self.frame / 30) % 2 == 0 {
            let prompt = "PRESS ENTER TO PLAY AGAIN";
            let pw = measure_text(prompt, None, 16, 1.0).width;
            draw_text(prompt, SCREEN_W / 2.0 - pw / 2.0, SCREEN_H * 0.65, 16.0, NEON_PINK);
        }
    }
}

// ---------------------------------------------------------------------------
// Bullet helper
// ---------------------------------------------------------------------------
fn mk_bullet(x: f32, y: f32, vx: f32, vy: f32, color: Color, is_player: bool, homing: bool) -> Bullet {
    let (w, h) = if is_player { (12.0, 3.0) } else { (5.0, 5.0) };
    Bullet {
        x, y, vx, vy, w, h,
        is_player,
        alive: true,
        color,
        homing,
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
