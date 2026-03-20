// Dragon Fury: Streets of Vengeance - Miyoo Mini Plus Port
// Beat 'em up in the style of Streets of Rage / Final Fight
// A Dragon Fist Story - Rust/Macroquad 0.4 port targeting 640x480 @ 60fps

use macroquad::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------
const SCREEN_W: f32 = 640.0;
const SCREEN_H: f32 = 480.0;

const DEPTH_MIN: f32 = 200.0;
const DEPTH_MAX: f32 = 420.0;
const DEPTH_MATCH: f32 = 20.0;

const PLAYER_SPEED_X: f32 = 3.0;
const PLAYER_SPEED_Y: f32 = 2.0;
const PLAYER_MAX_HP: f32 = 100.0;
const PLAYER_START_LIVES: i32 = 3;
const RESPAWN_INV_FRAMES: i32 = 120;

const COMBO_WINDOW: f32 = 0.4; // seconds
const COMBO_DAMAGE: [f32; 3] = [10.0, 15.0, 25.0];
const JUMP_KICK_DAMAGE: f32 = 30.0;
const JUMP_DURATION: i32 = 30;
const JUMP_HEIGHT: f32 = 80.0;
const JUMP_INV_FRAMES: i32 = 10;
const SPECIAL_DAMAGE: f32 = 40.0;
const SPECIAL_RADIUS: f32 = 80.0;
const SPECIAL_COST_FRAC: f32 = 0.15;
const SPECIAL_FRAMES: i32 = 20;

const GRAB_DURATION: f32 = 1.0;
const GRAB_KNEE_DAMAGE: f32 = 20.0;
const GRAB_MAX_KNEES: i32 = 3;
const THROW_DAMAGE: f32 = 30.0;
const THROW_DISTANCE: f32 = 200.0;

const SCREEN_SHAKE_HEAVY: f32 = 4.0;
const SCREEN_SHAKE_LIGHT: f32 = 2.0;

const MAX_PARTICLES: usize = 200;
const CONTINUE_SECONDS: i32 = 9;
const MAX_FLOAT_TEXTS: usize = 16;
const FLOAT_TEXT_DURATION: i32 = 20;
const MAX_RAIN_DROPS: usize = 40;

const GO_ARROW_DISPLAY: f32 = 3.0; // seconds to show "GO >>>"

// Palette constants
const BG_DARK: Color = Color::new(0.04, 0.04, 0.10, 1.0);
const NEON_PINK: Color = Color::new(1.0, 0.0, 0.4, 1.0);
const NEON_CYAN: Color = Color::new(0.0, 1.0, 1.0, 1.0);
const NEON_ORANGE: Color = Color::new(1.0, 0.4, 0.0, 1.0);
const NEON_YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
const PLAYER_BLUE: Color = Color::new(0.0, 0.53, 1.0, 1.0);
const PLAYER_HAIR: Color = Color::new(1.0, 0.8, 0.0, 1.0);
const SKIN_COLOR: Color = Color::new(1.0, 0.82, 0.65, 1.0);
const THUG_RED: Color = Color::new(0.8, 0.2, 0.2, 1.0);
const THUG_BROWN: Color = Color::new(0.53, 0.27, 0.13, 1.0);
const KNIFE_PURPLE: Color = Color::new(0.53, 0.2, 0.67, 1.0);
const KNIFE_DARK: Color = Color::new(0.2, 0.2, 0.2, 1.0);
const BRAWLER_GREEN: Color = Color::new(0.2, 0.67, 0.2, 1.0);
const BOSS_GOLD: Color = Color::new(0.85, 0.65, 0.13, 1.0);

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Title,
    Story,
    BossIntro,
    Playing,
    Paused,
    StageTransition,
    GameOver,
    Victory,
}

#[derive(Clone, Copy, PartialEq)]
enum StoryPhase {
    Intro,
    PreStage,
    PostStage,
    VictoryStory,
}

struct BossIntroInfo {
    name: &'static str,
    title: &'static str,
    quote: &'static str,
}

// ---------------------------------------------------------------------------
// Story Data
// ---------------------------------------------------------------------------
const STORY_INTRO: &[&str] = &[
    "NEO-OSAKA, KABUKI DISTRICT -- 3:47 AM",
    "The Dragon Fist dojo burns. Grandmaster Sato is gone.",
    "A witness saw the Iron Serpents heading east toward the port district. Their tag is fresh on every wall.",
    "You crack your knuckles. Ten blocks east. That's where this starts.",
];

const STORY_PRE_STAGE: &[&[&str]] = &[
    &["The alleys behind Kabuki are Iron Serpent territory. Low-level thugs and dealers. Someone here knows where Sato is."],
    &["The Iron Serpents' base of operations. Crates of weapons, stolen goods, and something else -- medical equipment. What are they doing to Sato?"],
    &["The neon skyline of Neo-Osaka stretches before you. On the highest rooftop, Jin waits. Sato kneels beside him, alive but broken."],
];

const STORY_MID_STAGE: &[&str] = &[
    "A beaten thug spits blood and laughs: \"You think you're tough? Wait till you see what's waiting at the warehouse. The Viper's got plans for your old man...\"",
    "You find a room with a chair, restraints, and recording equipment. On a screen: footage of Sato being forced to demonstrate the Dragon's Breath technique. He refuses. They hurt him. He still refuses.",
    "Jin's voice echoes across the rooftops: \"Do you know WHY Sato refused to teach me the Dragon's Breath? Because he saw what I really am. But he taught YOU. His precious student. His chosen son. While I got NOTHING.\"",
];

const BOSS_INTROS: &[BossIntroInfo] = &[
    BossIntroInfo { name: "BLADE", title: "Iron Serpent Lieutenant", quote: "Nothing personal, Dragon boy. Just business." },
    BossIntroInfo { name: "CRUSHER", title: "Iron Serpent Enforcer", quote: "The Viper said to break every bone in your body. I always follow orders." },
    BossIntroInfo { name: "JIN TAKEDA -- THE VIPER", title: "Former Dragon Fist Disciple", quote: "Sato chose you over me. Now I'll take everything from both of you." },
];

const STORY_POST_STAGE: &[&[&str]] = &[
    &[
        "Blade falls. Through his radio, you hear a voice -- cold, familiar.",
        "\"Let him come. I want to see what Sato taught him.\"",
        "It's Jin. He knows you're coming.",
    ],
    &[
        "Behind the warehouse, you find a passage leading up -- to the rooftops.",
        "And a note in Sato's handwriting, hidden under a loose brick:",
        "\"Ryu -- Jin doesn't want the technique to sell. He wants it to destroy. The rage consumed him long ago. Forgive him if you can. Stop him if you must. --Sato\"",
    ],
    &[
        "Jin falls to his knees. His rage is spent.",
        "Sato limps forward and places a hand on Jin's shoulder.",
        "\"I didn't refuse you because you were weak, Jin. I refused because the Dragon's Breath amplifies what's in your heart. And your heart was full of anger. But anger fades.\"",
        "Jin looks up. For the first time in ten years, he doesn't look furious. He looks tired.",
    ],
];

const STORY_VICTORY: &[&str] = &[
    "The sun rises over Neo-Osaka. The Iron Serpents scatter without their leader.",
    "Grandmaster Sato begins rebuilding the dojo. It will take months. Jin Takeda turns himself in to the authorities.",
    "Three months later, you visit Jin in prison. He's thinner. Quieter. He asks about the dojo.",
    "\"We saved you a spot,\" you tell him. \"When you're ready.\"",
    "The Dragon Fist endures.",
];

#[derive(Clone, Copy, PartialEq)]
enum PlayerState {
    Idle,
    Walking,
    Punching,
    Jumping,
    JumpKicking,
    Grabbing,
    Throwing,
    Special,
    Hurt,
    Down,
    Dead,
}

#[derive(Clone, Copy, PartialEq)]
enum EnemyState {
    Idle,
    Walking,
    Attacking,
    Stunned,
    Grabbed,
    Thrown,
    KnockedDown,
    Dead,
}

#[derive(Clone, Copy, PartialEq)]
enum EnemyKind {
    Thug,
    KnifeWielder,
    Brawler,
    BossBlade,
    BossCrusher,
    BossDragonKing,
}

#[derive(Clone, Copy, PartialEq)]
enum WeaponKind {
    Pipe,
    Knife,
    Bottle,
}

#[derive(Clone, Copy, PartialEq)]
enum PickupKind {
    Chicken,
    Pizza,
    ExtraLife,
}

// ---------------------------------------------------------------------------
// Structs
// ---------------------------------------------------------------------------
#[derive(Clone)]
struct Player {
    x: f32,
    y: f32,       // ground Y (depth)
    vx: f32,
    vy: f32,
    jump_z: f32,  // height above ground during jump
    jump_vz: f32,
    facing: f32,  // 1.0 right, -1.0 left
    hp: f32,
    max_hp: f32,
    lives: i32,
    score: i32,
    state: PlayerState,
    state_timer: i32,
    combo_index: i32,
    combo_timer: f32,
    inv_frames: i32,
    weapon: Option<WeaponKind>,
    weapon_durability: i32,
    grab_target: Option<usize>,
    grab_timer: f32,
    grab_knees: i32,
    hit_this_attack: bool,
    anim_frame: i32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 100.0,
            y: 310.0,
            vx: 0.0,
            vy: 0.0,
            jump_z: 0.0,
            jump_vz: 0.0,
            facing: 1.0,
            hp: PLAYER_MAX_HP,
            max_hp: PLAYER_MAX_HP,
            lives: PLAYER_START_LIVES,
            score: 0,
            state: PlayerState::Idle,
            state_timer: 0,
            combo_index: 0,
            combo_timer: 0.0,
            inv_frames: 0,
            weapon: None,
            weapon_durability: 0,
            grab_target: None,
            grab_timer: 0.0,
            grab_knees: 0,
            hit_this_attack: false,
            anim_frame: 0,
        }
    }

    fn ground_y(&self) -> f32 {
        self.y
    }

    fn attack_rect(&self) -> HitRect {
        let range = if let Some(w) = self.weapon {
            match w {
                WeaponKind::Pipe => 50.0,
                WeaponKind::Knife => 35.0,
                WeaponKind::Bottle => 40.0,
            }
        } else {
            40.0
        };
        let ax = if self.facing > 0.0 {
            self.x + 16.0
        } else {
            self.x - 16.0 - range
        };
        HitRect::new(ax, self.y - 24.0, range, 48.0)
    }

    fn body_rect(&self) -> HitRect {
        HitRect::new(self.x - 16.0, self.y - 48.0 - self.jump_z, 32.0, 48.0)
    }

    fn attack_damage(&self) -> f32 {
        if let Some(w) = self.weapon {
            match w {
                WeaponKind::Pipe => 35.0,
                WeaponKind::Knife => 25.0,
                WeaponKind::Bottle => 40.0,
            }
        } else if self.state == PlayerState::JumpKicking {
            JUMP_KICK_DAMAGE
        } else {
            let idx = (self.combo_index - 1).clamp(0, 2) as usize;
            COMBO_DAMAGE[idx]
        }
    }

    fn is_attacking(&self) -> bool {
        matches!(
            self.state,
            PlayerState::Punching | PlayerState::JumpKicking
        )
    }

    fn is_invincible(&self) -> bool {
        self.inv_frames > 0
            || self.state == PlayerState::Special
            || (self.state == PlayerState::Jumping && self.state_timer < JUMP_INV_FRAMES)
    }
}

#[derive(Clone)]
struct Enemy {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    facing: f32,
    hp: f32,
    max_hp: f32,
    kind: EnemyKind,
    state: EnemyState,
    state_timer: i32,
    attack_cooldown: f32,
    speed: f32,
    damage: f32,
    stagger_hits: i32,
    active: bool,
    throw_vx: f32,
    throw_vy: f32,
    anim_frame: i32,
    boss_phase: i32,
    boss_special_cooldown: f32,
}

impl Enemy {
    fn new(kind: EnemyKind, x: f32, y: f32) -> Self {
        let (hp, speed, damage) = match kind {
            EnemyKind::Thug => (60.0, 1.5, 10.0),
            EnemyKind::KnifeWielder => (40.0, 2.0, 20.0),
            EnemyKind::Brawler => (150.0, 0.8, 25.0),
            EnemyKind::BossBlade => (300.0, 2.2, 20.0),
            EnemyKind::BossCrusher => (500.0, 0.6, 25.0),
            EnemyKind::BossDragonKing => (400.0, 2.0, 15.0),
        };
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            facing: -1.0,
            hp,
            max_hp: hp,
            kind,
            state: EnemyState::Idle,
            state_timer: 0,
            attack_cooldown: rand::gen_range(0.5, 1.5),
            speed,
            damage,
            stagger_hits: 0,
            active: true,
            throw_vx: 0.0,
            throw_vy: 0.0,
            anim_frame: 0,
            boss_phase: 0,
            boss_special_cooldown: 3.0,
        }
    }

    fn ground_y(&self) -> f32 {
        self.y
    }

    fn body_rect(&self) -> HitRect {
        let (w, h) = self.sprite_size();
        HitRect::new(self.x - w * 0.5, self.y - h, w, h)
    }

    fn attack_rect(&self) -> HitRect {
        let range = match self.kind {
            EnemyKind::KnifeWielder | EnemyKind::BossBlade => 35.0,
            EnemyKind::Brawler | EnemyKind::BossCrusher => 50.0,
            _ => 40.0,
        };
        let ax = if self.facing > 0.0 {
            self.x + 16.0
        } else {
            self.x - 16.0 - range
        };
        HitRect::new(ax, self.y - 24.0, range, 48.0)
    }

    fn sprite_size(&self) -> (f32, f32) {
        match self.kind {
            EnemyKind::Brawler | EnemyKind::BossCrusher => (40.0, 52.0),
            EnemyKind::BossBlade | EnemyKind::BossDragonKing => (36.0, 52.0),
            _ => (32.0, 48.0),
        }
    }

    fn is_boss(&self) -> bool {
        matches!(
            self.kind,
            EnemyKind::BossBlade | EnemyKind::BossCrusher | EnemyKind::BossDragonKing
        )
    }

    fn can_be_grabbed(&self) -> bool {
        self.state == EnemyState::Stunned
            && !matches!(
                self.kind,
                EnemyKind::Brawler | EnemyKind::BossCrusher
            )
    }

    fn has_super_armor(&self) -> bool {
        matches!(self.kind, EnemyKind::Brawler | EnemyKind::BossCrusher)
    }
}

#[derive(Clone)]
struct GroundWeapon {
    x: f32,
    y: f32,
    kind: WeaponKind,
    active: bool,
}

#[derive(Clone)]
struct Pickup {
    x: f32,
    y: f32,
    kind: PickupKind,
    active: bool,
}

#[derive(Clone)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    max_life: f32,
    color: Color,
    size: f32,
    active: bool,
}

impl Particle {
    fn inactive() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            life: 0.0,
            max_life: 1.0,
            color: WHITE,
            size: 4.0,
            active: false,
        }
    }
}

#[derive(Clone)]
struct FloatingText {
    x: f32,
    y: f32,
    text: &'static str,
    timer: i32,       // counts down from FLOAT_TEXT_DURATION
    large: bool,      // true for combo finisher hits
    active: bool,
}

impl FloatingText {
    fn inactive() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            text: "",
            timer: 0,
            large: false,
            active: false,
        }
    }
}

#[derive(Clone)]
struct RainDrop {
    x: f32,
    y: f32,
    speed: f32,
    length: f32,
}

impl RainDrop {
    fn new_random() -> Self {
        Self {
            x: rand::gen_range(0.0, SCREEN_W + 100.0),
            y: rand::gen_range(-SCREEN_H, 0.0),
            speed: rand::gen_range(6.0, 12.0),
            length: rand::gen_range(8.0, 18.0),
        }
    }
}

#[derive(Clone)]
struct Projectile {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    damage: f32,
    life: f32,
    active: bool,
}

struct Wave {
    trigger_x: f32,
    enemies: Vec<(EnemyKind, f32, f32)>, // kind, x_offset, y
    triggered: bool,
}

struct StageData {
    name: &'static str,
    length: f32,
    waves: Vec<Wave>,
    pickups: Vec<(PickupKind, f32, f32)>,
    weapons: Vec<(WeaponKind, f32, f32)>,
    bg_far_color: Color,
    bg_mid_color: Color,
    bg_floor_color: Color,
    parallax_far: f32,
    parallax_mid: f32,
}

struct ScreenShake {
    intensity: f32,
    frames: i32,
}

struct Game {
    state: GameState,
    player: Player,
    enemies: Vec<Enemy>,
    ground_weapons: Vec<GroundWeapon>,
    pickups: Vec<Pickup>,
    particles: Vec<Particle>,
    projectiles: Vec<Projectile>,
    camera_x: f32,
    screen_locked: bool,
    lock_left: f32,
    lock_right: f32,
    stage_index: usize,
    stages: Vec<StageData>,
    current_wave_index: usize,
    all_waves_done: bool,
    shake: ScreenShake,
    transition_timer: f32,
    go_arrow_timer: f32,
    continue_timer: f32,
    victory_timer: f32,
    title_blink: f32,
    stage_name_timer: f32,
    flash_timer: f32,
    hit_stop: i32,
    float_texts: Vec<FloatingText>,
    rain_drops: Vec<RainDrop>,
    shockwave_radius: f32,
    shockwave_active: bool,
    frame_count: u64,
    stage_fade_timer: i32,   // counts 0..60 during stage transition fade
    stage_fade_dir: i32,     // -1 = fading out, 1 = fading in, 0 = showing name
    stage_fade_hold: i32,    // frames to hold on stage name
    // Story system
    story_lines: Vec<&'static str>,
    story_line_index: usize,
    story_char_index: usize,
    story_char_timer: i32,
    story_phase: StoryPhase,
    story_next: u8, // encoded next action after story: 0=none, 1=pre_stage, 2=stage_intro, 3=victory_story, 4=victory, 5=post_then_next_stage, 6=post_then_victory
    // Boss intro
    boss_intro_index: usize, // which boss intro to show
    boss_intro_timer: i32,
    // Mid-stage dialogue
    mid_stage_dialogue: Option<&'static str>,
    mid_stage_timer: i32,
    mid_stage_shown: [bool; 3], // per-stage tracking
    // Tracking boss intro shown per wave
    boss_intro_shown: Vec<bool>,
}

impl Game {
    fn new() -> Self {
        let stages = build_stages();
        Self {
            state: GameState::Title,
            player: Player::new(),
            enemies: Vec::new(),
            ground_weapons: Vec::new(),
            pickups: Vec::new(),
            particles: vec![Particle::inactive(); MAX_PARTICLES],
            projectiles: Vec::new(),
            camera_x: 0.0,
            screen_locked: false,
            lock_left: 0.0,
            lock_right: SCREEN_W,
            stage_index: 0,
            stages,
            current_wave_index: 0,
            all_waves_done: false,
            shake: ScreenShake {
                intensity: 0.0,
                frames: 0,
            },
            transition_timer: 0.0,
            go_arrow_timer: 0.0,
            continue_timer: CONTINUE_SECONDS as f32,
            victory_timer: 0.0,
            title_blink: 0.0,
            stage_name_timer: 0.0,
            flash_timer: 0.0,
            hit_stop: 0,
            float_texts: vec![FloatingText::inactive(); MAX_FLOAT_TEXTS],
            rain_drops: (0..MAX_RAIN_DROPS).map(|_| RainDrop::new_random()).collect(),
            shockwave_radius: 0.0,
            shockwave_active: false,
            frame_count: 0,
            stage_fade_timer: 0,
            stage_fade_dir: -1,
            stage_fade_hold: 0,
            story_lines: Vec::new(),
            story_line_index: 0,
            story_char_index: 0,
            story_char_timer: 0,
            story_phase: StoryPhase::Intro,
            story_next: 0,
            boss_intro_index: 0,
            boss_intro_timer: 0,
            mid_stage_dialogue: None,
            mid_stage_timer: 0,
            mid_stage_shown: [false; 3],
            boss_intro_shown: Vec::new(),
        }
    }

    fn start_stage(&mut self, idx: usize) {
        self.stage_index = idx;
        self.current_wave_index = 0;
        self.all_waves_done = false;
        self.enemies.clear();
        self.ground_weapons.clear();
        self.pickups.clear();
        self.projectiles.clear();
        self.camera_x = 0.0;
        self.screen_locked = false;
        self.go_arrow_timer = 0.0;
        self.player.x = 100.0;
        self.player.y = 310.0;
        self.player.state = PlayerState::Idle;
        self.player.state_timer = 0;
        self.player.inv_frames = RESPAWN_INV_FRAMES;
        self.player.jump_z = 0.0;
        self.player.grab_target = None;
        self.stage_name_timer = 2.5;

        // Reset wave triggers
        for wave in &mut self.stages[idx].waves {
            wave.triggered = false;
        }

        // Spawn stage pickups
        let pickup_defs: Vec<(PickupKind, f32, f32)> =
            self.stages[idx].pickups.clone();
        for (kind, px, py) in pickup_defs {
            self.pickups.push(Pickup {
                x: px,
                y: py,
                kind,
                active: true,
            });
        }

        // Spawn stage weapons
        let weapon_defs: Vec<(WeaponKind, f32, f32)> =
            self.stages[idx].weapons.clone();
        for (kind, wx, wy) in weapon_defs {
            self.ground_weapons.push(GroundWeapon {
                x: wx,
                y: wy,
                kind,
                active: true,
            });
        // Reset boss intro tracking
        let num_waves = self.stages[idx].waves.len();
        self.boss_intro_shown = vec![false; num_waves];
    }

    fn start_story(&mut self, lines: &[&'static str], phase: StoryPhase, next: u8) {
        self.story_lines = lines.to_vec();
        self.story_line_index = 0;
        self.story_char_index = 0;
        self.story_char_timer = 0;
        self.story_phase = phase;
        self.story_next = next;
        self.state = GameState::Story;
    }

    fn spawn_particle(&mut self, x: f32, y: f32, color: Color, count: i32, spread: f32) {
        let mut spawned = 0;
        for p in &mut self.particles {
            if !p.active && spawned < count {
                p.active = true;
                p.x = x + rand::gen_range(-4.0, 4.0);
                p.y = y + rand::gen_range(-4.0, 4.0);
                p.vx = rand::gen_range(-spread, spread);
                p.vy = rand::gen_range(-spread * 1.5, -spread * 0.3);
                p.life = rand::gen_range(0.15, 0.4);
                p.max_life = p.life;
                p.color = color;
                p.size = rand::gen_range(3.0, 8.0);
                spawned += 1;
            }
        }
    }

    fn add_shake(&mut self, intensity: f32, frames: i32) {
        self.shake.intensity = intensity;
        self.shake.frames = frames;
    }

    fn add_flash(&mut self) {
        self.flash_timer = 0.1;
    }

    fn spawn_float_text(&mut self, x: f32, y: f32, large: bool) {
        let texts: [&'static str; 3] = ["POW", "BAM", "CRACK"];
        let text = texts[rand::gen_range(0, 3) as usize];
        for ft in &mut self.float_texts {
            if !ft.active {
                ft.active = true;
                ft.x = x;
                ft.y = y;
                ft.text = text;
                ft.timer = FLOAT_TEXT_DURATION;
                ft.large = large;
                return;
            }
        }
    }

    fn spawn_wham_text(&mut self, x: f32, y: f32) {
        for ft in &mut self.float_texts {
            if !ft.active {
                ft.active = true;
                ft.x = x;
                ft.y = y;
                ft.text = "WHAM!";
                ft.timer = FLOAT_TEXT_DURATION;
                ft.large = true;
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Stage definitions
// ---------------------------------------------------------------------------
fn build_stages() -> Vec<StageData> {
    vec![
        // Stage 1: Back Alley
        StageData {
            name: "BACK ALLEY",
            length: 3000.0,
            waves: vec![
                Wave {
                    trigger_x: 200.0,
                    enemies: vec![
                        (EnemyKind::Thug, 500.0, 280.0),
                        (EnemyKind::Thug, 550.0, 340.0),
                        (EnemyKind::Thug, 580.0, 380.0),
                    ],
                    triggered: false,
                },
                Wave {
                    trigger_x: 800.0,
                    enemies: vec![
                        (EnemyKind::Thug, 500.0, 260.0),
                        (EnemyKind::Thug, 540.0, 320.0),
                        (EnemyKind::KnifeWielder, 560.0, 390.0),
                    ],
                    triggered: false,
                },
                Wave {
                    trigger_x: 1400.0,
                    enemies: vec![
                        (EnemyKind::Thug, 500.0, 300.0),
                        (EnemyKind::Thug, 530.0, 360.0),
                        (EnemyKind::KnifeWielder, 550.0, 250.0),
                        (EnemyKind::Thug, 580.0, 400.0),
                    ],
                    triggered: false,
                },
                Wave {
                    trigger_x: 2000.0,
                    enemies: vec![
                        (EnemyKind::Thug, 500.0, 280.0),
                        (EnemyKind::KnifeWielder, 550.0, 350.0),
                        (EnemyKind::KnifeWielder, 520.0, 400.0),
                    ],
                    triggered: false,
                },
                Wave {
                    trigger_x: 2600.0,
                    enemies: vec![(EnemyKind::BossBlade, 500.0, 310.0)],
                    triggered: false,
                },
            ],
            pickups: vec![
                (PickupKind::Chicken, 600.0, 350.0),
                (PickupKind::ExtraLife, 1800.0, 300.0),
            ],
            weapons: vec![(WeaponKind::Pipe, 400.0, 330.0)],
            bg_far_color: Color::new(0.06, 0.06, 0.15, 1.0),
            bg_mid_color: Color::new(0.08, 0.08, 0.18, 1.0),
            bg_floor_color: Color::new(0.12, 0.12, 0.14, 1.0),
            parallax_far: 0.2,
            parallax_mid: 0.5,
        },
        // Stage 2: Warehouse
        StageData {
            name: "WAREHOUSE",
            length: 3500.0,
            waves: vec![
                Wave {
                    trigger_x: 200.0,
                    enemies: vec![
                        (EnemyKind::Thug, 500.0, 280.0),
                        (EnemyKind::Thug, 550.0, 370.0),
                    ],
                    triggered: false,
                },
                Wave {
                    trigger_x: 700.0,
                    enemies: vec![
                        (EnemyKind::Thug, 500.0, 300.0),
                        (EnemyKind::KnifeWielder, 540.0, 260.0),
                        (EnemyKind::KnifeWielder, 560.0, 390.0),
                    ],
                    triggered: false,
                },
                Wave {
                    trigger_x: 1300.0,
                    enemies: vec![
                        (EnemyKind::Thug, 500.0, 290.0),
                        (EnemyKind::KnifeWielder, 530.0, 350.0),
                        (EnemyKind::Thug, 560.0, 400.0),
                    ],
                    triggered: false,
                },
                Wave {
                    trigger_x: 1900.0,
                    enemies: vec![
                        (EnemyKind::Brawler, 520.0, 320.0),
                        (EnemyKind::Thug, 560.0, 260.0),
                        (EnemyKind::Thug, 580.0, 400.0),
                    ],
                    triggered: false,
                },
                Wave {
                    trigger_x: 2500.0,
                    enemies: vec![
                        (EnemyKind::KnifeWielder, 500.0, 280.0),
                        (EnemyKind::Brawler, 550.0, 360.0),
                    ],
                    triggered: false,
                },
                Wave {
                    trigger_x: 3100.0,
                    enemies: vec![(EnemyKind::BossCrusher, 500.0, 310.0)],
                    triggered: false,
                },
            ],
            pickups: vec![
                (PickupKind::Pizza, 900.0, 350.0),
                (PickupKind::ExtraLife, 2200.0, 300.0),
            ],
            weapons: vec![
                (WeaponKind::Bottle, 500.0, 340.0),
                (WeaponKind::Knife, 1600.0, 380.0),
            ],
            bg_far_color: Color::new(0.1, 0.08, 0.06, 1.0),
            bg_mid_color: Color::new(0.14, 0.11, 0.08, 1.0),
            bg_floor_color: Color::new(0.18, 0.16, 0.14, 1.0),
            parallax_far: 0.3,
            parallax_mid: 0.6,
        },
        // Stage 3: Rooftop Showdown
        StageData {
            name: "ROOFTOP SHOWDOWN",
            length: 4000.0,
            waves: vec![
                Wave {
                    trigger_x: 200.0,
                    enemies: vec![
                        (EnemyKind::Thug, 500.0, 280.0),
                        (EnemyKind::KnifeWielder, 540.0, 340.0),
                        (EnemyKind::Thug, 570.0, 400.0),
                    ],
                    triggered: false,
                },
                Wave {
                    trigger_x: 800.0,
                    enemies: vec![
                        (EnemyKind::KnifeWielder, 500.0, 260.0),
                        (EnemyKind::Brawler, 550.0, 330.0),
                        (EnemyKind::KnifeWielder, 580.0, 400.0),
                    ],
                    triggered: false,
                },
                Wave {
                    trigger_x: 1500.0,
                    enemies: vec![
                        (EnemyKind::Thug, 500.0, 280.0),
                        (EnemyKind::Thug, 520.0, 320.0),
                        (EnemyKind::KnifeWielder, 540.0, 370.0),
                        (EnemyKind::Brawler, 570.0, 410.0),
                    ],
                    triggered: false,
                },
                Wave {
                    trigger_x: 2200.0,
                    enemies: vec![
                        (EnemyKind::Brawler, 500.0, 290.0),
                        (EnemyKind::KnifeWielder, 530.0, 350.0),
                        (EnemyKind::KnifeWielder, 560.0, 400.0),
                        (EnemyKind::Thug, 590.0, 260.0),
                    ],
                    triggered: false,
                },
                Wave {
                    trigger_x: 2900.0,
                    enemies: vec![
                        (EnemyKind::Brawler, 500.0, 300.0),
                        (EnemyKind::Brawler, 560.0, 380.0),
                        (EnemyKind::KnifeWielder, 530.0, 260.0),
                    ],
                    triggered: false,
                },
                Wave {
                    trigger_x: 3600.0,
                    enemies: vec![(EnemyKind::BossDragonKing, 500.0, 310.0)],
                    triggered: false,
                },
            ],
            pickups: vec![
                (PickupKind::Chicken, 600.0, 350.0),
                (PickupKind::Pizza, 1800.0, 300.0),
                (PickupKind::ExtraLife, 2800.0, 370.0),
            ],
            weapons: vec![
                (WeaponKind::Pipe, 400.0, 330.0),
                (WeaponKind::Knife, 1200.0, 360.0),
                (WeaponKind::Bottle, 2400.0, 300.0),
            ],
            bg_far_color: Color::new(0.04, 0.04, 0.12, 1.0),
            bg_mid_color: Color::new(0.06, 0.06, 0.16, 1.0),
            bg_floor_color: Color::new(0.1, 0.1, 0.12, 1.0),
            parallax_far: 0.1,
            parallax_mid: 0.4,
        },
    ]
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------
fn window_conf() -> Conf {
    Conf {
        window_title: "Dragon Fury: Streets of Vengeance".to_string(),
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
        let dt = get_frame_time().min(0.05);

        game.frame_count = game.frame_count.wrapping_add(1);

        // Hit stop: skip game updates but still draw
        if game.hit_stop > 0 {
            game.hit_stop -= 1;
        } else {
            match game.state {
                GameState::Title => update_title(&mut game),
                GameState::Story => update_story(&mut game),
                GameState::BossIntro => update_boss_intro(&mut game),
                GameState::Playing => update_playing(&mut game, dt),
                GameState::Paused => update_paused(&mut game),
                GameState::StageTransition => update_stage_transition(&mut game, dt),
                GameState::GameOver => update_game_over(&mut game, dt),
                GameState::Victory => update_victory(&mut game, dt),
            }
        }

        // Render
        clear_background(BG_DARK);

        let shake_offset = if game.shake.frames > 0 {
            vec2(
                rand::gen_range(-game.shake.intensity, game.shake.intensity),
                rand::gen_range(-game.shake.intensity, game.shake.intensity),
            )
        } else {
            vec2(0.0, 0.0)
        };

        match game.state {
            GameState::Title => draw_title(&game),
            GameState::Story => draw_story_screen(&game),
            GameState::BossIntro => draw_boss_intro_screen(&game),
            GameState::Playing | GameState::Paused => {
                draw_game(&game, shake_offset);
                if game.state == GameState::Paused {
                    draw_pause_overlay();
                }
                // Mid-stage dialogue overlay
                if game.mid_stage_dialogue.is_some() {
                    draw_mid_stage_dialogue(&game);
                }
            }
            GameState::StageTransition => draw_stage_transition(&game),
            GameState::GameOver => draw_game_over(&game),
            GameState::Victory => draw_victory(&game),
        }

        // Screen flash
        if game.flash_timer > 0.0 {
            let alpha = (game.flash_timer / 0.1).min(1.0);
            draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(1.0, 1.0, 1.0, alpha * 0.7));
            game.flash_timer -= dt;
        }

        // CRT scanline overlay
        {
            let scanline_color = Color::new(0.0, 0.0, 0.0, 0.12);
            let mut scan_y = 0.0;
            while scan_y < SCREEN_H {
                draw_rectangle(0.0, scan_y, SCREEN_W, 1.0, scanline_color);
                scan_y += 4.0;
            }
        }

        next_frame().await;
    }
}

// ---------------------------------------------------------------------------
// Update: Title
// ---------------------------------------------------------------------------
fn update_title(game: &mut Game) {
    game.title_blink += get_frame_time();
    if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::X) {
        game.player = Player::new();
        game.stage_index = 0;
        game.mid_stage_shown = [false; 3];
        // Start intro story -> pre_stage story -> stage transition
        // story_next: 1 = show pre_stage after this
        game.start_story(STORY_INTRO, StoryPhase::Intro, 1);
    }
}

// ---------------------------------------------------------------------------
// Update: Story (typewriter text)
// ---------------------------------------------------------------------------
fn update_story(game: &mut Game) {
    // Advance typewriter
    game.story_char_timer += 1;
    if game.story_char_timer >= 2 {
        game.story_char_timer = 0;
        if game.story_line_index < game.story_lines.len() {
            let char_count = game.story_lines[game.story_line_index].chars().count();
            if game.story_char_index < char_count {
                game.story_char_index += 1;
            }
        }
    }

    if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::X) {
        if game.story_line_index < game.story_lines.len() {
            let char_count = game.story_lines[game.story_line_index].chars().count();
            if game.story_char_index < char_count {
                // Show full current line
                game.story_char_index = char_count;
            } else {
                // Advance to next line
                game.story_line_index += 1;
                game.story_char_index = 0;
                game.story_char_timer = 0;
                if game.story_line_index >= game.story_lines.len() {
                    // Story complete - handle next action
                    story_complete(game);
                }
            }
        }
    }
}

fn story_complete(game: &mut Game) {
    match game.story_next {
        1 => {
            // Show pre-stage story, then go to stage transition
            let idx = game.stage_index;
            if idx < STORY_PRE_STAGE.len() {
                game.start_story(STORY_PRE_STAGE[idx], StoryPhase::PreStage, 2);
            } else {
                game.state = GameState::StageTransition;
                game.transition_timer = 2.5;
            }
        }
        2 => {
            // Go to stage transition
            game.state = GameState::StageTransition;
            game.transition_timer = 2.5;
        }
        3 => {
            // Show victory story, then go to victory screen
            game.start_story(STORY_VICTORY, StoryPhase::VictoryStory, 4);
        }
        4 => {
            // Go to victory screen
            game.state = GameState::Victory;
            game.victory_timer = 0.0;
        }
        5 => {
            // Post-stage done, advance to next stage's pre-stage
            game.stage_index += 1;
            let idx = game.stage_index;
            if idx < STORY_PRE_STAGE.len() {
                game.start_story(STORY_PRE_STAGE[idx], StoryPhase::PreStage, 2);
            } else {
                game.state = GameState::StageTransition;
                game.transition_timer = 2.5;
            }
        }
        6 => {
            // Post-stage done for final boss, show victory story
            game.start_story(STORY_VICTORY, StoryPhase::VictoryStory, 4);
        }
        _ => {
            game.state = GameState::Playing;
        }
    }
}

// ---------------------------------------------------------------------------
// Update: Boss Intro
// ---------------------------------------------------------------------------
fn update_boss_intro(game: &mut Game) {
    game.boss_intro_timer += 1;
    if (is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::X)) && game.boss_intro_timer > 15 {
        game.state = GameState::Playing;
        // The wave will be triggered on next check_wave_triggers call
    }
}

// ---------------------------------------------------------------------------
// Update: Stage Transition
// ---------------------------------------------------------------------------
fn update_stage_transition(game: &mut Game, dt: f32) {
    game.transition_timer -= dt;

    // Frame-based fade: fade out 30 frames, hold 30 frames, fade in 30 frames
    // Total ~90 frames mapped over the 2.5s timer
    if game.transition_timer > 1.8 {
        // Fading to black (first 30 frames)
        game.stage_fade_dir = -1;
        game.stage_fade_timer = ((2.5 - game.transition_timer) / 0.7 * 30.0).min(30.0) as i32;
    } else if game.transition_timer > 0.7 {
        // Holding on stage name
        game.stage_fade_dir = 0;
        game.stage_fade_timer = 30;
    } else {
        // Fading back in
        game.stage_fade_dir = 1;
        game.stage_fade_timer = ((0.7 - game.transition_timer) / 0.7 * 30.0).min(30.0) as i32;
    }

    if game.transition_timer <= 0.0 {
        game.state = GameState::Playing;
        game.start_stage(game.stage_index);
    }
}

// ---------------------------------------------------------------------------
// Update: Paused
// ---------------------------------------------------------------------------
fn update_paused(game: &mut Game) {
    if is_key_pressed(KeyCode::Enter) {
        game.state = GameState::Playing;
    }
}

// ---------------------------------------------------------------------------
// Update: Game Over
// ---------------------------------------------------------------------------
fn update_game_over(game: &mut Game, dt: f32) {
    game.continue_timer -= dt;
    if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::X) {
        // Continue - restart current stage
        game.player.lives = PLAYER_START_LIVES;
        game.player.hp = PLAYER_MAX_HP;
        game.state = GameState::StageTransition;
        game.transition_timer = 2.5;
    }
    if game.continue_timer <= 0.0 {
        // True game over - back to title
        game.state = GameState::Title;
        game.player = Player::new();
    }
}

// ---------------------------------------------------------------------------
// Update: Victory
// ---------------------------------------------------------------------------
fn update_victory(game: &mut Game, dt: f32) {
    game.victory_timer += dt;
    if game.victory_timer > 5.0 && (is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::X)) {
        game.state = GameState::Title;
        game.player = Player::new();
    }
}

// ---------------------------------------------------------------------------
// Update: Playing (main gameplay)
// ---------------------------------------------------------------------------
fn update_playing(game: &mut Game, dt: f32) {
    if is_key_pressed(KeyCode::Enter) {
        game.state = GameState::Paused;
        return;
    }

    // Shake countdown
    if game.shake.frames > 0 {
        game.shake.frames -= 1;
    }

    // Stage name timer
    if game.stage_name_timer > 0.0 {
        game.stage_name_timer -= dt;
    }

    // GO arrow timer
    if game.go_arrow_timer > 0.0 {
        game.go_arrow_timer -= dt;
    }

    // Mid-stage dialogue timer
    if game.mid_stage_dialogue.is_some() {
        game.mid_stage_timer -= 1;
        if game.mid_stage_timer <= 0 || is_key_pressed(KeyCode::Z) {
            game.mid_stage_dialogue = None;
        }
    }

    update_player(game, dt);
    update_enemies(game, dt);
    update_projectiles(game, dt);
    update_particles(game, dt);
    update_float_texts(game);
    update_rain(game);
    check_wave_triggers(game);
    update_camera(game);
    check_screen_lock(game);
    check_stage_complete(game);
}

// ---------------------------------------------------------------------------
// Player Update
// ---------------------------------------------------------------------------
fn update_player(game: &mut Game, dt: f32) {
    // Invincibility countdown
    if game.player.inv_frames > 0 {
        game.player.inv_frames -= 1;
    }

    // Combo timer
    if game.player.combo_timer > 0.0 {
        game.player.combo_timer -= dt;
        if game.player.combo_timer <= 0.0 {
            game.player.combo_index = 0;
        }
    }

    game.player.anim_frame += 1;

    let cur_state = game.player.state;
    match cur_state {
        PlayerState::Idle | PlayerState::Walking => {
            let mut dx = 0.0f32;
            let mut dy = 0.0f32;
            if is_key_down(KeyCode::Right) {
                dx += PLAYER_SPEED_X;
            }
            if is_key_down(KeyCode::Left) {
                dx -= PLAYER_SPEED_X;
            }
            if is_key_down(KeyCode::Up) {
                dy -= PLAYER_SPEED_Y;
            }
            if is_key_down(KeyCode::Down) {
                dy += PLAYER_SPEED_Y;
            }

            if dx != 0.0 {
                game.player.facing = dx.signum();
            }

            let new_x = game.player.x + dx;
            let new_y = (game.player.y + dy).clamp(DEPTH_MIN, DEPTH_MAX);

            // Screen lock boundaries
            if game.screen_locked {
                game.player.x = new_x.clamp(game.lock_left + 20.0, game.lock_right - 20.0);
            } else {
                game.player.x = new_x.clamp(20.0, game.stages[game.stage_index].length - 20.0);
            }
            game.player.y = new_y;

            game.player.state = if dx != 0.0 || dy != 0.0 {
                PlayerState::Walking
            } else {
                PlayerState::Idle
            };

            // Drop weapon: Down + X attack key when holding weapon
            if is_key_down(KeyCode::Down)
                && is_key_pressed(KeyCode::X)
                && game.player.weapon.is_some()
            {
                let kind = game.player.weapon.unwrap();
                let px = game.player.x;
                let py = game.player.y;
                game.ground_weapons.push(GroundWeapon {
                    x: px,
                    y: py,
                    kind,
                    active: true,
                });
                game.player.weapon = None;
                game.player.weapon_durability = 0;
            }

            // Punch / attack
            if is_key_pressed(KeyCode::X)
                && !(is_key_down(KeyCode::Down) && game.player.weapon.is_some())
            {
                game.player.combo_index = if game.player.combo_timer > 0.0 {
                    (game.player.combo_index + 1).min(3)
                } else {
                    1
                };
                game.player.combo_timer = COMBO_WINDOW;
                game.player.state = PlayerState::Punching;
                game.player.state_timer = 12;
                game.player.hit_this_attack = false;
            }

            // Jump
            if is_key_pressed(KeyCode::Space) {
                game.player.state = PlayerState::Jumping;
                game.player.state_timer = 0;
                game.player.jump_z = 0.0;
                game.player.jump_vz = 8.0;
            }

            // Special move
            if is_key_pressed(KeyCode::Z) {
                let cost = PLAYER_MAX_HP * SPECIAL_COST_FRAC;
                if game.player.hp > cost {
                    game.player.hp -= cost;
                    game.player.state = PlayerState::Special;
                    game.player.state_timer = SPECIAL_FRAMES;
                }
            }

            // Try to grab stunned enemy
            if dx != 0.0 || dy != 0.0 {
                try_grab_enemy(game);
            }

            // Pick up weapons and pickups
            pick_up_items(game);
        }

        PlayerState::Punching => {
            game.player.state_timer -= 1;
            // Hit detection in attack frames 4-8
            if game.player.state_timer >= 4
                && game.player.state_timer <= 8
                && !game.player.hit_this_attack
            {
                check_player_attack_hits(game);
            }
            if game.player.state_timer <= 0 {
                game.player.state = PlayerState::Idle;
                // Consume weapon durability
                if game.player.weapon.is_some() {
                    game.player.weapon_durability -= 1;
                    if game.player.weapon_durability <= 0 {
                        // Weapon breaks
                        let px = game.player.x + game.player.facing * 30.0;
                        let py = game.player.y - 24.0;
                        game.spawn_particle(px, py, NEON_ORANGE, 8, 3.0);
                        game.player.weapon = None;
                    }
                }
            }
        }

        PlayerState::Jumping => {
            game.player.state_timer += 1;
            game.player.jump_z += game.player.jump_vz;
            game.player.jump_vz -=
                (2.0 * JUMP_HEIGHT) / ((JUMP_DURATION as f32 / 2.0).powi(2));

            // Move while jumping
            if is_key_down(KeyCode::Right) {
                game.player.x += PLAYER_SPEED_X;
                game.player.facing = 1.0;
            }
            if is_key_down(KeyCode::Left) {
                game.player.x -= PLAYER_SPEED_X;
                game.player.facing = -1.0;
            }

            // Jump kick
            if is_key_pressed(KeyCode::X) {
                game.player.state = PlayerState::JumpKicking;
                game.player.hit_this_attack = false;
            }

            if game.player.jump_z <= 0.0 && game.player.state_timer > 5 {
                game.player.jump_z = 0.0;
                game.player.state = PlayerState::Idle;
            }
        }

        PlayerState::JumpKicking => {
            game.player.state_timer += 1;
            game.player.jump_z += game.player.jump_vz;
            game.player.jump_vz -=
                (2.0 * JUMP_HEIGHT) / ((JUMP_DURATION as f32 / 2.0).powi(2));

            if is_key_down(KeyCode::Right) {
                game.player.x += PLAYER_SPEED_X;
            }
            if is_key_down(KeyCode::Left) {
                game.player.x -= PLAYER_SPEED_X;
            }

            if !game.player.hit_this_attack {
                check_player_attack_hits(game);
            }

            if game.player.jump_z <= 0.0 && game.player.state_timer > 5 {
                game.player.jump_z = 0.0;
                game.player.state = PlayerState::Idle;
            }
        }

        PlayerState::Grabbing => {
            game.player.grab_timer -= dt;

            // Position grabbed enemy
            if let Some(gi) = game.player.grab_target {
                if gi < game.enemies.len() && game.enemies[gi].active {
                    let gx = game.player.x + game.player.facing * 30.0;
                    let gy = game.player.y;
                    game.enemies[gi].x = gx;
                    game.enemies[gi].y = gy;
                    game.enemies[gi].state = EnemyState::Grabbed;
                }
            }

            // Knee strikes
            if is_key_pressed(KeyCode::X) && game.player.grab_knees < GRAB_MAX_KNEES {
                game.player.grab_knees += 1;
                if let Some(gi) = game.player.grab_target {
                    if gi < game.enemies.len() {
                        game.enemies[gi].hp -= GRAB_KNEE_DAMAGE;
                        game.player.score += 100;
                        let ex = game.enemies[gi].x;
                        let ey = game.enemies[gi].y - 24.0;
                        game.spawn_particle(ex, ey, NEON_YELLOW, 4, 2.0);
                    }
                }
            }

            // Throw
            let do_throw = is_key_pressed(KeyCode::Space)
                || (is_key_pressed(KeyCode::X) && game.player.grab_knees >= GRAB_MAX_KNEES);

            if do_throw || game.player.grab_timer <= 0.0 {
                if let Some(gi) = game.player.grab_target {
                    if gi < game.enemies.len() {
                        let dir = if is_key_down(KeyCode::Left) {
                            -1.0
                        } else {
                            game.player.facing
                        };
                        game.enemies[gi].state = EnemyState::Thrown;
                        game.enemies[gi].throw_vx = dir * 6.0;
                        game.enemies[gi].throw_vy = 0.0;
                        game.enemies[gi].state_timer = (THROW_DISTANCE / 6.0) as i32;
                    }
                }
                game.player.state = PlayerState::Idle;
                game.player.grab_target = None;
            }
        }

        PlayerState::Special => {
            game.player.state_timer -= 1;
            // Shockwave expands over move duration
            game.shockwave_active = true;
            let progress = 1.0 - (game.player.state_timer as f32 / SPECIAL_FRAMES as f32);
            game.shockwave_radius = progress * 80.0;
            // Hit all enemies in radius on first frame
            if game.player.state_timer == SPECIAL_FRAMES - 1 {
                game.add_flash();
                let px = game.player.x;
                let py = game.player.y;
                for e in &mut game.enemies {
                    if !e.active || e.state == EnemyState::Dead {
                        continue;
                    }
                    let dist = ((e.x - px).powi(2) + (e.y - py).powi(2)).sqrt();
                    if dist < SPECIAL_RADIUS {
                        e.hp -= SPECIAL_DAMAGE;
                        e.state = EnemyState::Stunned;
                        e.state_timer = 40;
                        e.stagger_hits = 0;
                    }
                }
                game.spawn_particle(px, py - 20.0, NEON_CYAN, 15, 5.0);
                game.add_shake(SCREEN_SHAKE_HEAVY, 10);
            }
            if game.player.state_timer <= 0 {
                game.player.state = PlayerState::Idle;
                game.shockwave_active = false;
                game.shockwave_radius = 0.0;
            }
        }

        PlayerState::Hurt => {
            game.player.state_timer -= 1;
            if game.player.state_timer <= 0 {
                game.player.state = PlayerState::Idle;
            }
        }

        PlayerState::Down => {
            game.player.state_timer -= 1;
            if game.player.state_timer <= 0 {
                if game.player.hp > 0.0 {
                    game.player.state = PlayerState::Idle;
                    game.player.inv_frames = 60;
                } else {
                    player_die(game);
                }
            }
        }

        PlayerState::Dead => {
            // Handled by game over transition
        }

        _ => {}
    }

    // Clamp to screen lock
    if game.screen_locked {
        game.player.x = game.player.x.clamp(game.lock_left + 20.0, game.lock_right - 20.0);
    }
}

fn try_grab_enemy(game: &mut Game) {
    if game.player.state != PlayerState::Walking && game.player.state != PlayerState::Idle {
        return;
    }
    let pr = game.player.body_rect();
    let py = game.player.y;
    let mut grab_index: Option<usize> = None;
    for (i, e) in game.enemies.iter().enumerate() {
        if !e.active || !e.can_be_grabbed() {
            continue;
        }
        if (e.y - py).abs() > DEPTH_MATCH {
            continue;
        }
        let er = e.body_rect();
        if rects_overlap(&pr, &er) {
            grab_index = Some(i);
            break;
        }
    }
    if let Some(i) = grab_index {
        game.player.state = PlayerState::Grabbing;
        game.player.grab_target = Some(i);
        game.player.grab_timer = GRAB_DURATION;
        game.player.grab_knees = 0;
    }
}

fn pick_up_items(game: &mut Game) {
    let px = game.player.x;
    let py = game.player.y;

    // Weapons
    for gw in &mut game.ground_weapons {
        if !gw.active {
            continue;
        }
        if (gw.x - px).abs() < 24.0 && (gw.y - py).abs() < 20.0 && game.player.weapon.is_none() {
            game.player.weapon = Some(gw.kind);
            game.player.weapon_durability = match gw.kind {
                WeaponKind::Pipe => 8,
                WeaponKind::Knife => 12,
                WeaponKind::Bottle => 3,
            };
            gw.active = false;
        }
    }

    // Pickups
    for pu in &mut game.pickups {
        if !pu.active {
            continue;
        }
        if (pu.x - px).abs() < 20.0 && (pu.y - py).abs() < 20.0 {
            match pu.kind {
                PickupKind::Chicken => {
                    game.player.hp = (game.player.hp + PLAYER_MAX_HP * 0.3).min(PLAYER_MAX_HP);
                }
                PickupKind::Pizza => {
                    game.player.hp = (game.player.hp + PLAYER_MAX_HP * 0.6).min(PLAYER_MAX_HP);
                }
                PickupKind::ExtraLife => {
                    game.player.lives += 1;
                }
            }
            game.player.score += 500;
            pu.active = false;
        }
    }
}

fn check_player_attack_hits(game: &mut Game) {
    let atk = game.player.attack_rect();
    let dmg = game.player.attack_damage();
    let py = game.player.y;
    let facing = game.player.facing;
    let is_jump_kick = game.player.state == PlayerState::JumpKicking;
    let combo_idx = game.player.combo_index;
    let player_x = game.player.x;

    let num = game.enemies.len();
    for i in 0..num {
        if !game.enemies[i].active
            || game.enemies[i].state == EnemyState::Dead
            || game.enemies[i].state == EnemyState::Grabbed
            || game.enemies[i].state == EnemyState::Thrown
        {
            continue;
        }
        if (game.enemies[i].y - py).abs() > DEPTH_MATCH {
            continue;
        }
        let er = game.enemies[i].body_rect();
        if rects_overlap(&atk, &er) {
            game.enemies[i].hp -= dmg;
            game.enemies[i].stagger_hits += 1;
            game.player.hit_this_attack = true;
            game.player.score += 50;

            // Knockback
            let kb = if is_jump_kick { 60.0 } else { 15.0 };
            if !game.enemies[i].has_super_armor() || game.enemies[i].stagger_hits >= 3 {
                game.enemies[i].state = EnemyState::Stunned;
                game.enemies[i].state_timer = if combo_idx >= 3 { 60 } else { 20 };
                game.enemies[i].x += facing * kb;
                if game.enemies[i].stagger_hits >= 3 {
                    game.enemies[i].stagger_hits = 0;
                }
            }

            // Spawn hit sparks
            let spark_x = (player_x + game.enemies[i].x) * 0.5;
            let spark_y = game.enemies[i].y - 24.0;
            game.spawn_particle(spark_x, spark_y, NEON_YELLOW, 5, 3.0);
            game.add_shake(SCREEN_SHAKE_LIGHT, 4);

            // POW/BAM floating text on every hit
            let is_combo_finisher = combo_idx >= 3;
            game.spawn_float_text(spark_x, spark_y, is_combo_finisher);

            // Hit stop on combo finisher (3rd punch) or jump kick
            if is_combo_finisher || is_jump_kick {
                game.hit_stop = 5;
            }

            if game.enemies[i].hp <= 0.0 {
                game.enemies[i].state = EnemyState::Dead;
                game.enemies[i].state_timer = 60;
                let score = match game.enemies[i].kind {
                    EnemyKind::Thug => 200,
                    EnemyKind::KnifeWielder => 300,
                    EnemyKind::Brawler => 500,
                    EnemyKind::BossBlade => 2000,
                    EnemyKind::BossCrusher => 3000,
                    EnemyKind::BossDragonKing => 5000,
                };
                game.player.score += score;

                let ex = game.enemies[i].x;
                let ey = game.enemies[i].y;

                game.spawn_particle(ex, ey - 24.0, NEON_PINK, 10, 4.0);
                game.add_shake(SCREEN_SHAKE_HEAVY, 8);

                // Knife wielder may drop weapon
                if game.enemies[i].kind == EnemyKind::KnifeWielder
                    && rand::gen_range(0.0, 1.0) < 0.5
                {
                    game.ground_weapons.push(GroundWeapon {
                        x: ex,
                        y: ey,
                        kind: WeaponKind::Knife,
                        active: true,
                    });
                }

                // Random pickup drop (10%)
                if rand::gen_range(0.0, 1.0) < 0.1 {
                    game.pickups.push(Pickup {
                        x: ex,
                        y: ey,
                        kind: PickupKind::Chicken,
                        active: true,
                    });
                }
            }

            break; // one hit per attack frame
        }
    }
}

fn player_die(game: &mut Game) {
    game.player.lives -= 1;
    if game.player.lives <= 0 {
        game.player.state = PlayerState::Dead;
        game.state = GameState::GameOver;
        game.continue_timer = CONTINUE_SECONDS as f32;
    } else {
        // Respawn
        game.player.hp = PLAYER_MAX_HP;
        game.player.state = PlayerState::Idle;
        game.player.inv_frames = RESPAWN_INV_FRAMES;
        game.player.jump_z = 0.0;
        game.player.weapon = None;
        game.player.weapon_durability = 0;
        game.player.grab_target = None;
    }
}

fn hurt_player(game: &mut Game, damage: f32) {
    if game.player.is_invincible() {
        return;
    }
    game.player.hp -= damage;
    game.spawn_particle(game.player.x, game.player.y - 30.0, NEON_PINK, 4, 2.0);
    game.add_shake(SCREEN_SHAKE_LIGHT, 4);
    if game.player.hp <= 0.0 {
        game.player.hp = 0.0;
        game.player.state = PlayerState::Down;
        game.player.state_timer = 60;
    } else {
        game.player.state = PlayerState::Hurt;
        game.player.state_timer = 15;
    }
}

// ---------------------------------------------------------------------------
// Enemy Update
// ---------------------------------------------------------------------------
fn update_enemies(game: &mut Game, dt: f32) {
    let px = game.player.x;
    let py = game.player.y;
    let player_state = game.player.state;

    let num = game.enemies.len();
    for i in 0..num {
        if !game.enemies[i].active {
            continue;
        }

        game.enemies[i].anim_frame += 1;

        match game.enemies[i].state {
            EnemyState::Dead => {
                game.enemies[i].state_timer -= 1;
                if game.enemies[i].state_timer <= 0 {
                    game.enemies[i].active = false;
                }
            }

            EnemyState::Stunned => {
                game.enemies[i].state_timer -= 1;
                if game.enemies[i].state_timer <= 0 {
                    game.enemies[i].state = EnemyState::Idle;
                }
            }

            EnemyState::Grabbed => {
                // Controlled by player grab logic
            }

            EnemyState::Thrown => {
                game.enemies[i].x += game.enemies[i].throw_vx;
                game.enemies[i].y += game.enemies[i].throw_vy;
                game.enemies[i].state_timer -= 1;

                // Check collision with other enemies
                let ei_x = game.enemies[i].x;
                let ei_y = game.enemies[i].y;
                for j in 0..num {
                    if i == j
                        || !game.enemies[j].active
                        || game.enemies[j].state == EnemyState::Dead
                    {
                        continue;
                    }
                    if (ei_x - game.enemies[j].x).abs() < 40.0
                        && (ei_y - game.enemies[j].y).abs() < DEPTH_MATCH
                    {
                        game.enemies[j].hp -= THROW_DAMAGE;
                        game.enemies[j].state = EnemyState::Stunned;
                        game.enemies[j].state_timer = 30;
                        game.player.score += 100;

                        // Thrown enemy collision: big burst of white/yellow particles + WHAM text
                        let cx = (ei_x + game.enemies[j].x) * 0.5;
                        let cy = (ei_y + game.enemies[j].y) * 0.5 - 20.0;
                        game.spawn_particle(cx, cy, WHITE, 8, 5.0);
                        game.spawn_particle(cx, cy, NEON_YELLOW, 7, 4.0);
                        game.spawn_wham_text(cx, cy);
                        game.add_shake(SCREEN_SHAKE_HEAVY, 8);
                    }
                }

                if game.enemies[i].state_timer <= 0 {
                    game.enemies[i].state = EnemyState::KnockedDown;
                    game.enemies[i].state_timer = 40;
                    if game.enemies[i].hp <= 0.0 {
                        game.enemies[i].state = EnemyState::Dead;
                        game.enemies[i].state_timer = 60;
                    }
                }
            }

            EnemyState::KnockedDown => {
                game.enemies[i].state_timer -= 1;
                if game.enemies[i].state_timer <= 0 {
                    if game.enemies[i].hp <= 0.0 {
                        game.enemies[i].state = EnemyState::Dead;
                        game.enemies[i].state_timer = 60;
                    } else {
                        game.enemies[i].state = EnemyState::Idle;
                    }
                }
            }

            EnemyState::Attacking => {
                game.enemies[i].state_timer -= 1;
                // Hit player on frame 5-8 of attack
                if game.enemies[i].state_timer >= 4
                    && game.enemies[i].state_timer <= 8
                    && player_state != PlayerState::Dead
                    && player_state != PlayerState::Down
                {
                    let atk = game.enemies[i].attack_rect();
                    let pr = game.player.body_rect();
                    if rects_overlap(&atk, &pr)
                        && (game.enemies[i].y - py).abs() < DEPTH_MATCH
                    {
                        let dmg = game.enemies[i].damage;
                        hurt_player(game, dmg);
                    }
                }
                if game.enemies[i].state_timer <= 0 {
                    game.enemies[i].state = EnemyState::Idle;
                    game.enemies[i].attack_cooldown = match game.enemies[i].kind {
                        EnemyKind::KnifeWielder | EnemyKind::BossBlade => 1.2,
                        EnemyKind::Brawler | EnemyKind::BossCrusher => 1.8,
                        EnemyKind::BossDragonKing => 0.8,
                        _ => 1.0,
                    };
                }
            }

            EnemyState::Idle | EnemyState::Walking => {
                game.enemies[i].attack_cooldown -= dt;
                let ex = game.enemies[i].x;
                let ey = game.enemies[i].y;
                let espeed = game.enemies[i].speed;
                let kind = game.enemies[i].kind;

                // Face player
                game.enemies[i].facing = if px > ex { 1.0 } else { -1.0 };

                let dx = px - ex;
                let dy = py - ey;
                let dist = (dx * dx + dy * dy).sqrt().max(1.0);

                match kind {
                    EnemyKind::KnifeWielder => {
                        // Circle and dash
                        if dist < 60.0 && game.enemies[i].attack_cooldown <= 0.0 {
                            game.enemies[i].state = EnemyState::Attacking;
                            game.enemies[i].state_timer = 12;
                        } else if dist < 100.0 {
                            // Circle around (move perpendicular)
                            let perp_x = -dy / dist;
                            let perp_y = dx / dist;
                            let sign = if (game.enemies[i].anim_frame / 60) % 2 == 0 {
                                1.0
                            } else {
                                -1.0
                            };
                            game.enemies[i].x += perp_x * espeed * sign;
                            game.enemies[i].y += perp_y * espeed * sign;
                            game.enemies[i].state = EnemyState::Walking;
                        } else {
                            // Approach
                            game.enemies[i].x += (dx / dist) * espeed;
                            game.enemies[i].y += (dy / dist) * espeed * 0.6;
                            game.enemies[i].state = EnemyState::Walking;
                        }
                    }

                    EnemyKind::Brawler => {
                        // Slow approach, charge attack
                        if dist < 50.0 && game.enemies[i].attack_cooldown <= 0.0 {
                            game.enemies[i].state = EnemyState::Attacking;
                            game.enemies[i].state_timer = 18;
                            game.enemies[i].damage = 25.0;
                        } else if dist < 120.0
                            && game.enemies[i].attack_cooldown <= 0.0
                        {
                            // Charge attack
                            game.enemies[i].state = EnemyState::Attacking;
                            game.enemies[i].state_timer = 25;
                            game.enemies[i].damage = 35.0;
                            game.enemies[i].x += (dx / dist) * espeed * 4.0;
                        } else {
                            game.enemies[i].x += (dx / dist) * espeed;
                            game.enemies[i].y += (dy / dist) * espeed * 0.6;
                            game.enemies[i].state = EnemyState::Walking;
                        }
                    }

                    EnemyKind::BossBlade => {
                        update_boss_blade(game, i, dx, dy, dist, dt);
                    }

                    EnemyKind::BossCrusher => {
                        update_boss_crusher(game, i, dx, dy, dist, dt);
                    }

                    EnemyKind::BossDragonKing => {
                        update_boss_dragon_king(game, i, dx, dy, dist, dt);
                    }

                    _ => {
                        // Thug: walk toward player, attack when close
                        if dist < 45.0 && game.enemies[i].attack_cooldown <= 0.0 {
                            game.enemies[i].state = EnemyState::Attacking;
                            game.enemies[i].state_timer = 12;
                        } else {
                            game.enemies[i].x += (dx / dist) * espeed;
                            game.enemies[i].y += (dy / dist) * espeed * 0.6;
                            game.enemies[i].state = EnemyState::Walking;
                        }
                    }
                }

                // Clamp Y to depth band
                game.enemies[i].y = game.enemies[i].y.clamp(DEPTH_MIN, DEPTH_MAX);
            }
        }

        // Kill check
        if game.enemies[i].hp <= 0.0
            && game.enemies[i].state != EnemyState::Dead
            && game.enemies[i].state != EnemyState::Thrown
        {
            game.enemies[i].state = EnemyState::Dead;
            game.enemies[i].state_timer = 60;
        }
    }
}

fn update_boss_blade(game: &mut Game, i: usize, dx: f32, dy: f32, dist: f32, dt: f32) {
    let espeed = game.enemies[i].speed;
    game.enemies[i].boss_special_cooldown -= dt;

    // Throw knife projectile
    if game.enemies[i].boss_special_cooldown <= 0.0 {
        game.enemies[i].boss_special_cooldown = 2.5;
        let dir = game.enemies[i].facing;
        game.projectiles.push(Projectile {
            x: game.enemies[i].x + dir * 20.0,
            y: game.enemies[i].y - 20.0,
            vx: dir * 5.0,
            vy: 0.0,
            damage: 15.0,
            life: 2.0,
            active: true,
        });
    }

    if dist < 50.0 && game.enemies[i].attack_cooldown <= 0.0 {
        game.enemies[i].state = EnemyState::Attacking;
        game.enemies[i].state_timer = 10;
    } else {
        game.enemies[i].x += (dx / dist) * espeed;
        game.enemies[i].y += (dy / dist) * espeed * 0.6;
        game.enemies[i].state = EnemyState::Walking;
    }
}

fn update_boss_crusher(game: &mut Game, i: usize, dx: f32, dy: f32, dist: f32, dt: f32) {
    let espeed = game.enemies[i].speed;
    game.enemies[i].boss_special_cooldown -= dt;

    // Ground pound AoE
    if game.enemies[i].boss_special_cooldown <= 0.0 && dist < 100.0 {
        game.enemies[i].boss_special_cooldown = 4.0;
        game.enemies[i].state = EnemyState::Attacking;
        game.enemies[i].state_timer = 30;
        game.enemies[i].damage = 35.0;
        game.add_shake(SCREEN_SHAKE_HEAVY, 10);
        // AoE damage handled in attack frames
    } else if dist < 50.0 && game.enemies[i].attack_cooldown <= 0.0 {
        game.enemies[i].state = EnemyState::Attacking;
        game.enemies[i].state_timer = 18;
        game.enemies[i].damage = 25.0;
    } else {
        game.enemies[i].x += (dx / dist) * espeed;
        game.enemies[i].y += (dy / dist) * espeed * 0.6;
        game.enemies[i].state = EnemyState::Walking;
    }
}

fn update_boss_dragon_king(game: &mut Game, i: usize, dx: f32, dy: f32, dist: f32, dt: f32) {
    let espeed = game.enemies[i].speed;
    game.enemies[i].boss_special_cooldown -= dt;

    // Uses player-like moveset: combo, jump kick patterns
    if game.enemies[i].boss_special_cooldown <= 0.0 && dist < 80.0 {
        game.enemies[i].boss_special_cooldown = 3.0;
        // Special spinning attack
        game.enemies[i].state = EnemyState::Attacking;
        game.enemies[i].state_timer = 20;
        game.enemies[i].damage = 30.0;
        game.add_shake(SCREEN_SHAKE_LIGHT, 6);
    } else if dist < 45.0 && game.enemies[i].attack_cooldown <= 0.0 {
        game.enemies[i].state = EnemyState::Attacking;
        game.enemies[i].state_timer = 10;
        game.enemies[i].damage = 15.0;
    } else {
        game.enemies[i].x += (dx / dist) * espeed;
        game.enemies[i].y += (dy / dist) * espeed * 0.6;
        game.enemies[i].state = EnemyState::Walking;
    }
}

// ---------------------------------------------------------------------------
// Projectiles
// ---------------------------------------------------------------------------
fn update_projectiles(game: &mut Game, dt: f32) {
    for proj in &mut game.projectiles {
        if !proj.active {
            continue;
        }
        proj.x += proj.vx;
        proj.y += proj.vy;
        proj.life -= dt;
        if proj.life <= 0.0 {
            proj.active = false;
            continue;
        }

        // Hit player
        let pr = game.player.body_rect();
        let proj_rect = HitRect::new(proj.x - 6.0, proj.y - 6.0, 12.0, 12.0);
        if rects_overlap(&pr, &proj_rect)
            && (game.player.y - proj.y - 6.0).abs() < DEPTH_MATCH
        {
            hurt_player(game, proj.damage);
            proj.active = false;
        }
    }
    game.projectiles.retain(|p| p.active);
}

// ---------------------------------------------------------------------------
// Particles
// ---------------------------------------------------------------------------
fn update_particles(game: &mut Game, dt: f32) {
    for p in &mut game.particles {
        if !p.active {
            continue;
        }
        p.x += p.vx;
        p.y += p.vy;
        p.vy += 12.0 * dt; // gravity
        p.life -= dt;
        if p.life <= 0.0 {
            p.active = false;
        }
    }
}

fn update_float_texts(game: &mut Game) {
    for ft in &mut game.float_texts {
        if !ft.active {
            continue;
        }
        ft.timer -= 1;
        ft.y -= 1.5; // float upward
        if ft.timer <= 0 {
            ft.active = false;
        }
    }
}

fn update_rain(game: &mut Game) {
    // Only rain in stage 0 (Back Alley)
    if game.stage_index != 0 {
        return;
    }
    for drop in &mut game.rain_drops {
        drop.x -= drop.speed * 0.3; // diagonal
        drop.y += drop.speed;
        if drop.y > SCREEN_H || drop.x < -20.0 {
            drop.x = rand::gen_range(0.0, SCREEN_W + 100.0);
            drop.y = rand::gen_range(-SCREEN_H, -10.0);
            drop.speed = rand::gen_range(6.0, 12.0);
            drop.length = rand::gen_range(8.0, 18.0);
        }
    }
}

// ---------------------------------------------------------------------------
// Wave triggers, screen lock, stage completion
// ---------------------------------------------------------------------------
fn check_wave_triggers(game: &mut Game) {
    let player_world_x = game.player.x;
    let cam = game.camera_x;
    let stage_idx = game.stage_index;

    let num_waves = game.stages[stage_idx].waves.len();
    // Ensure boss_intro_shown is properly sized
    if game.boss_intro_shown.len() < num_waves {
        game.boss_intro_shown.resize(num_waves, false);
    }

    let mut wave_index_to_spawn: Option<usize> = None;

    for (wi, wave) in game.stages[stage_idx].waves.iter_mut().enumerate() {
        if wave.triggered {
            continue;
        }
        if player_world_x >= wave.trigger_x {
            // Check if this is a boss wave
            let is_boss_wave = wave.enemies.iter().any(|(kind, _, _)| {
                matches!(
                    kind,
                    EnemyKind::BossBlade | EnemyKind::BossCrusher | EnemyKind::BossDragonKing
                )
            });

            if is_boss_wave && !game.boss_intro_shown[wi] {
                game.boss_intro_shown[wi] = true;
                game.boss_intro_index = stage_idx;
                game.boss_intro_timer = 0;
                game.state = GameState::BossIntro;
                // Don't spawn yet -- will spawn after boss intro
                return;
            }

            wave.triggered = true;
            wave_index_to_spawn = Some(wi);
            break; // only process one wave per frame
        }
    }

    if let Some(wi) = wave_index_to_spawn {
        let enemies: Vec<(EnemyKind, f32, f32)> = game.stages[stage_idx].waves[wi].enemies.clone();
        for (kind, ox, ey) in enemies {
            let ex = cam + ox;
            game.enemies.push(Enemy::new(kind, ex, ey));
        }

        // Count triggered waves to determine mid-stage dialogue timing
        let triggered_count = game.stages[stage_idx]
            .waves
            .iter()
            .filter(|w| w.triggered)
            .count();

        // Show mid-stage dialogue after wave 3 (triggered_count == 3)
        if triggered_count == 3 && !game.mid_stage_shown[stage_idx] && stage_idx < STORY_MID_STAGE.len() {
            game.mid_stage_shown[stage_idx] = true;
            game.mid_stage_dialogue = Some(STORY_MID_STAGE[stage_idx]);
            game.mid_stage_timer = 240; // 4 seconds at 60fps
        }
    }

    // Check if all waves are triggered
    let all_triggered = game.stages[stage_idx]
        .waves
        .iter()
        .all(|w| w.triggered);
    let all_dead = game
        .enemies
        .iter()
        .all(|e| !e.active || e.state == EnemyState::Dead);
    game.all_waves_done = all_triggered && all_dead;
}

fn check_screen_lock(game: &mut Game) {
    let has_active_enemies = game
        .enemies
        .iter()
        .any(|e| e.active && e.state != EnemyState::Dead);

    if has_active_enemies {
        if !game.screen_locked {
            game.screen_locked = true;
            game.lock_left = game.camera_x;
            game.lock_right = game.camera_x + SCREEN_W;
        }
    } else {
        if game.screen_locked {
            game.go_arrow_timer = GO_ARROW_DISPLAY;
        }
        game.screen_locked = false;
    }
}

fn check_stage_complete(game: &mut Game) {
    if !game.all_waves_done {
        return;
    }
    let stage_len = game.stages[game.stage_index].length;
    if game.player.x >= stage_len - 100.0 {
        let si = game.stage_index;
        if si < 2 {
            // Show post-stage story, then advance to next stage
            // story_next=5 means "post-stage done, go to next stage's pre-stage"
            if si < STORY_POST_STAGE.len() {
                game.start_story(STORY_POST_STAGE[si], StoryPhase::PostStage, 5);
            } else {
                game.stage_index += 1;
                game.state = GameState::StageTransition;
                game.transition_timer = 2.5;
            }
        } else {
            // Final boss beaten - show post-stage then victory story
            // story_next=6 means "post-stage done for final boss, show victory story"
            if si < STORY_POST_STAGE.len() {
                game.start_story(STORY_POST_STAGE[si], StoryPhase::PostStage, 6);
            } else {
                game.start_story(STORY_VICTORY, StoryPhase::VictoryStory, 4);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Camera
// ---------------------------------------------------------------------------
fn update_camera(game: &mut Game) {
    if game.screen_locked {
        return;
    }
    let target = game.player.x - SCREEN_W * 0.4;
    let stage_len = game.stages[game.stage_index].length;
    let max_cam = (stage_len - SCREEN_W).max(0.0);
    game.camera_x = target.clamp(0.0, max_cam);
}

// ---------------------------------------------------------------------------
// Drawing
// ---------------------------------------------------------------------------
fn draw_game(game: &Game, shake: Vec2) {
    let cam = game.camera_x + shake.x;
    let sy = shake.y;

    draw_background(game, cam);
    draw_ground(game, cam, sy);

    // Collect and depth-sort all drawable entities
    struct Drawable {
        ground_y: f32,
        kind: u8, // 0=player, 1=enemy, 2=weapon, 3=pickup, 4=projectile
        index: usize,
    }
    let mut drawables: Vec<Drawable> = Vec::new();

    drawables.push(Drawable {
        ground_y: game.player.ground_y(),
        kind: 0,
        index: 0,
    });

    for (i, e) in game.enemies.iter().enumerate() {
        if e.active {
            drawables.push(Drawable {
                ground_y: e.ground_y(),
                kind: 1,
                index: i,
            });
        }
    }

    for (i, w) in game.ground_weapons.iter().enumerate() {
        if w.active {
            drawables.push(Drawable {
                ground_y: w.y,
                kind: 2,
                index: i,
            });
        }
    }

    for (i, p) in game.pickups.iter().enumerate() {
        if p.active {
            drawables.push(Drawable {
                ground_y: p.y,
                kind: 3,
                index: i,
            });
        }
    }

    for (i, p) in game.projectiles.iter().enumerate() {
        if p.active {
            drawables.push(Drawable {
                ground_y: p.y,
                kind: 4,
                index: i,
            });
        }
    }

    drawables.sort_by(|a, b| a.ground_y.partial_cmp(&b.ground_y).unwrap());

    for d in &drawables {
        match d.kind {
            0 => draw_player(&game.player, cam, sy),
            1 => draw_enemy(&game.enemies[d.index], cam, sy),
            2 => draw_ground_weapon(&game.ground_weapons[d.index], cam, sy),
            3 => draw_pickup(&game.pickups[d.index], cam, sy),
            4 => draw_projectile(&game.projectiles[d.index], cam, sy),
            _ => {}
        }
    }

    // Particles on top
    for p in &game.particles {
        if p.active {
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            let c = Color::new(p.color.r, p.color.g, p.color.b, alpha);
            draw_rectangle(p.x - cam, p.y + sy, p.size, p.size, c);
        }
    }

    // Floating text effects (POW, BAM, CRACK, WHAM!)
    for ft in &game.float_texts {
        if !ft.active {
            continue;
        }
        let progress = 1.0 - (ft.timer as f32 / FLOAT_TEXT_DURATION as f32);
        let alpha = 1.0 - progress;
        let base_size = if ft.large { 40.0 } else { 28.0 };
        let scale_up = 1.0 + progress * 0.3; // scales up slightly
        let font_size = base_size * scale_up;
        let c = Color::new(1.0, 1.0, 0.0, alpha); // bold yellow
        let sx = ft.x - cam;
        let sy_ft = ft.y + sy;
        // Shadow for readability
        draw_text(ft.text, sx + 2.0, sy_ft + 2.0, font_size, Color::new(0.0, 0.0, 0.0, alpha * 0.6));
        draw_text(ft.text, sx, sy_ft, font_size, c);
    }

    // Special move shockwave
    if game.shockwave_active && game.shockwave_radius > 0.0 {
        let px = game.player.x - cam;
        let py_sw = game.player.y + sy - game.player.jump_z;
        let fade = if game.player.state_timer < 5 {
            game.player.state_timer as f32 / 5.0
        } else {
            1.0
        };
        let alpha = 0.3 * fade;
        draw_circle(
            px,
            py_sw - 24.0,
            game.shockwave_radius,
            Color::new(1.0, 0.0, 1.0, alpha),
        );
        // Ring outline
        draw_circle_lines(
            px,
            py_sw - 24.0,
            game.shockwave_radius,
            2.0,
            Color::new(1.0, 0.0, 1.0, alpha * 1.5),
        );
    }

    // Background rain for Back Alley (stage 0) - draw on top for visibility
    if game.stage_index == 0 {
        for drop in &game.rain_drops {
            let alpha = 0.25;
            draw_line(
                drop.x,
                drop.y + sy,
                drop.x - drop.length * 0.3,
                drop.y + drop.length + sy,
                1.0,
                Color::new(0.8, 0.8, 1.0, alpha),
            );
        }
    }

    draw_hud(game);

    // "GO >>>" arrow
    if game.go_arrow_timer > 0.0 && !game.screen_locked {
        let blink = ((game.go_arrow_timer * 4.0).sin() > 0.0) as u8 as f32;
        if blink > 0.5 {
            let txt = "GO >>>";
            draw_text(txt, SCREEN_W - 140.0, SCREEN_H * 0.5, 36.0, NEON_YELLOW);
        }
    }

    // Stage name display
    if game.stage_name_timer > 0.0 {
        let alpha = (game.stage_name_timer / 0.5).min(1.0);
        let name = format!(
            "STAGE {}: {}",
            game.stage_index + 1,
            game.stages[game.stage_index].name
        );
        let c = Color::new(1.0, 1.0, 1.0, alpha);
        let w = measure_text(&name, None, 40, 1.0).width;
        draw_text(&name, (SCREEN_W - w) * 0.5, SCREEN_H * 0.4, 40.0, c);
    }
}

fn draw_background(game: &Game, cam: f32) {
    let stage = &game.stages[game.stage_index];

    // Far layer
    let far_offset = cam * stage.parallax_far;
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H * 0.6, stage.bg_far_color);

    // Far buildings
    for i in 0..12 {
        let bx = (i as f32) * 120.0 - (far_offset % 120.0);
        let bh = 60.0 + ((i * 37) % 80) as f32;
        let c = Color::new(
            stage.bg_far_color.r + 0.03,
            stage.bg_far_color.g + 0.03,
            stage.bg_far_color.b + 0.05,
            1.0,
        );
        draw_rectangle(bx, SCREEN_H * 0.6 - bh, 80.0, bh, c);

        // Neon sign on some buildings
        if i % 3 == 0 {
            let neon = match i % 4 {
                0 => NEON_PINK,
                1 => NEON_CYAN,
                2 => NEON_ORANGE,
                _ => NEON_YELLOW,
            };
            let nc = Color::new(neon.r, neon.g, neon.b, 0.6);
            draw_rectangle(bx + 10.0, SCREEN_H * 0.6 - bh + 10.0, 30.0, 8.0, nc);
        }

        // Windows
        for wy in 0..((bh / 15.0) as i32).min(4) {
            for wx in 0..3 {
                let lit = ((i + wx + wy as usize) % 3 != 0) as u8 as f32;
                let wc = Color::new(0.8 * lit, 0.7 * lit, 0.3 * lit, 0.5 * lit);
                draw_rectangle(
                    bx + 10.0 + wx as f32 * 22.0,
                    SCREEN_H * 0.6 - bh + 25.0 + wy as f32 * 15.0,
                    12.0,
                    10.0,
                    wc,
                );
            }
        }
    }

    // Mid layer
    let mid_offset = cam * stage.parallax_mid;
    for i in 0..10 {
        let bx = (i as f32) * 140.0 - (mid_offset % 140.0);
        let bh = 40.0 + ((i * 53) % 60) as f32;
        let c = Color::new(
            stage.bg_mid_color.r + 0.02,
            stage.bg_mid_color.g + 0.02,
            stage.bg_mid_color.b + 0.04,
            1.0,
        );
        draw_rectangle(bx, SCREEN_H * 0.6 - bh + 10.0, 100.0, bh, c);
    }
}

fn draw_ground(game: &Game, cam: f32, sy: f32) {
    let stage = &game.stages[game.stage_index];
    // Ground plane
    draw_rectangle(0.0, SCREEN_H * 0.55 + sy, SCREEN_W, SCREEN_H * 0.45, stage.bg_floor_color);

    // Ground lines for perspective
    for i in 0..8 {
        let gy = SCREEN_H * 0.55 + (i as f32) * 20.0 + sy;
        let alpha = 0.1 - (i as f32) * 0.01;
        draw_line(
            0.0,
            gy,
            SCREEN_W,
            gy,
            1.0,
            Color::new(0.3, 0.3, 0.4, alpha.max(0.02)),
        );
    }

    // Ambient stage-specific details
    match game.stage_index {
        0 => {
            // Rain particles (simple)
            for i in 0..30 {
                let rx = ((i as f32 * 97.3 + cam * 0.5 + get_time() as f32 * 200.0) % SCREEN_W)
                    as f32;
                let ry = ((i as f32 * 41.7 + get_time() as f32 * 300.0) % (SCREEN_H * 0.55))
                    as f32;
                draw_line(
                    rx,
                    ry + sy,
                    rx - 2.0,
                    ry + 8.0 + sy,
                    1.0,
                    Color::new(0.5, 0.5, 0.8, 0.3),
                );
            }
        }
        1 => {
            // Warehouse: crate silhouettes in background
            for i in 0..5 {
                let cx = (i as f32) * 200.0 - (cam * 0.8 % 200.0);
                draw_rectangle(
                    cx,
                    SCREEN_H * 0.5 + sy,
                    40.0,
                    30.0,
                    Color::new(0.15, 0.12, 0.08, 0.5),
                );
            }
        }
        2 => {
            // Rooftop: stars
            for i in 0..40 {
                let sx = ((i as f32) * 73.1 + cam * 0.02) % SCREEN_W;
                let star_y = ((i as f32) * 37.9) % (SCREEN_H * 0.4);
                let twinkle =
                    (0.5 + 0.5 * ((get_time() as f32 * 2.0 + i as f32).sin())) as f32;
                draw_rectangle(
                    sx,
                    star_y + sy,
                    2.0,
                    2.0,
                    Color::new(1.0, 1.0, 1.0, twinkle * 0.6),
                );
            }
        }
        _ => {}
    }
}

fn depth_scale(y: f32) -> f32 {
    let t = (y - DEPTH_MIN) / (DEPTH_MAX - DEPTH_MIN);
    0.95 + t * 0.05
}

fn draw_shadow(x: f32, y: f32, w: f32, cam: f32, sy: f32) {
    let scale = depth_scale(y);
    draw_ellipse(
        x - cam,
        y + sy + 2.0,
        w * 0.6 * scale,
        6.0 * scale,
        0.0,
        Color::new(0.0, 0.0, 0.0, 0.3),
    );
}

fn draw_player(player: &Player, cam: f32, sy: f32) {
    let x = player.x - cam;
    let y = player.y + sy;
    let jz = player.jump_z;
    let scale = depth_scale(player.y);
    let facing = player.facing;

    // Shadow (always on ground)
    draw_shadow(player.x, player.y, 32.0, cam, sy);

    // Invincibility flash
    if player.inv_frames > 0 && player.inv_frames % 4 < 2 {
        return;
    }

    let body_y = y - jz;

    // Special move: spinning glow
    if player.state == PlayerState::Special {
        let alpha = 0.4 + 0.3 * ((player.state_timer as f32 * 0.5).sin());
        draw_circle(
            x,
            body_y - 24.0 * scale,
            SPECIAL_RADIUS * scale * 0.5,
            Color::new(0.0, 1.0, 1.0, alpha),
        );
    }

    // Legs
    let leg_spread = if player.state == PlayerState::Walking {
        ((player.anim_frame as f32 * 0.3).sin() * 6.0)
    } else if player.state == PlayerState::Jumping || player.state == PlayerState::JumpKicking {
        8.0
    } else {
        0.0
    };

    // Left leg
    draw_rectangle(
        x - 8.0 * scale - leg_spread * scale * 0.5,
        body_y - 12.0 * scale,
        6.0 * scale,
        14.0 * scale,
        Color::new(0.2, 0.2, 0.6, 1.0), // dark blue pants
    );
    // Right leg
    draw_rectangle(
        x + 2.0 * scale + leg_spread * scale * 0.5,
        body_y - 12.0 * scale,
        6.0 * scale,
        14.0 * scale,
        Color::new(0.2, 0.2, 0.6, 1.0),
    );

    // Body (jacket)
    draw_rectangle(
        x - 10.0 * scale,
        body_y - 34.0 * scale,
        20.0 * scale,
        22.0 * scale,
        PLAYER_BLUE,
    );

    // Arms
    let arm_angle = match player.state {
        PlayerState::Punching => {
            let ext = if player.state_timer > 6 { 20.0 } else { 10.0 };
            ext * facing
        }
        PlayerState::JumpKicking => 15.0 * facing,
        PlayerState::Grabbing => 12.0 * facing,
        _ => ((player.anim_frame as f32 * 0.15).sin() * 4.0) * facing,
    };

    // Back arm
    draw_rectangle(
        x - 12.0 * scale * facing.abs() - arm_angle * scale * 0.3,
        body_y - 32.0 * scale,
        6.0 * scale,
        12.0 * scale,
        Color::new(0.0, 0.4, 0.8, 1.0),
    );
    // Front arm
    draw_rectangle(
        x + arm_angle * scale,
        body_y - 32.0 * scale,
        6.0 * scale,
        12.0 * scale,
        SKIN_COLOR,
    );

    // Weapon in hand
    if let Some(wk) = player.weapon {
        let wx = x + (arm_angle + 6.0 * facing) * scale;
        let wy = body_y - 30.0 * scale;
        match wk {
            WeaponKind::Pipe => {
                draw_rectangle(wx, wy - 4.0 * scale, 20.0 * scale * facing, 4.0 * scale, GRAY);
            }
            WeaponKind::Knife => {
                draw_rectangle(wx, wy - 2.0 * scale, 14.0 * scale * facing, 3.0 * scale, Color::new(0.7, 0.7, 0.8, 1.0));
            }
            WeaponKind::Bottle => {
                draw_rectangle(wx, wy - 6.0 * scale, 6.0 * scale, 10.0 * scale, Color::new(0.3, 0.6, 0.3, 1.0));
            }
        }
    }

    // Head
    draw_rectangle(
        x - 6.0 * scale,
        body_y - 46.0 * scale,
        12.0 * scale,
        12.0 * scale,
        SKIN_COLOR,
    );

    // Hair
    draw_rectangle(
        x - 7.0 * scale,
        body_y - 48.0 * scale,
        14.0 * scale,
        6.0 * scale,
        PLAYER_HAIR,
    );

    // Eyes
    let eye_x = x + facing * 2.0 * scale;
    draw_rectangle(
        eye_x - 1.0 * scale,
        body_y - 42.0 * scale,
        2.0 * scale,
        2.0 * scale,
        BLACK,
    );
}

fn draw_enemy(enemy: &Enemy, cam: f32, sy: f32) {
    if enemy.state == EnemyState::Dead && enemy.state_timer < 30 {
        // Fade out
        let alpha = enemy.state_timer as f32 / 30.0;
        if alpha < 0.05 {
            return;
        }
    }

    let x = enemy.x - cam;
    let y = enemy.y + sy;
    let scale = depth_scale(enemy.y);
    let (sw, sh) = enemy.sprite_size();
    let facing = enemy.facing;

    draw_shadow(enemy.x, enemy.y, sw, cam, sy);

    // Knocked down = draw flat
    if enemy.state == EnemyState::KnockedDown || enemy.state == EnemyState::Dead {
        let alpha = if enemy.state == EnemyState::Dead {
            (enemy.state_timer as f32 / 60.0).clamp(0.0, 1.0)
        } else {
            1.0
        };
        let c = enemy_body_color(enemy.kind);
        let c = Color::new(c.r, c.g, c.b, alpha);
        draw_rectangle(x - sw * 0.5, y - 8.0, sw, 8.0, c);
        return;
    }

    // Stunned flash
    if enemy.state == EnemyState::Stunned && enemy.state_timer % 4 < 2 {
        // flash effect - draw slightly brighter
    }

    // Stun stars orbiting above head
    if enemy.state == EnemyState::Stunned {
        let (_, sh) = enemy.sprite_size();
        let star_cx = x;
        let star_cy = y - sh * scale - 10.0;
        let t = enemy.anim_frame as f32 * 0.08;
        for s in 0..3 {
            let angle = t + (s as f32) * std::f32::consts::TAU / 3.0;
            let sr = 10.0 * scale;
            let sx = star_cx + angle.cos() * sr;
            let sy_star = star_cy + angle.sin() * sr * 0.4; // elliptical orbit
            // Draw small yellow star (diamond shape)
            let star_size = 3.0 * scale;
            draw_rectangle(
                sx - star_size * 0.5,
                sy_star - star_size * 0.5,
                star_size,
                star_size,
                NEON_YELLOW,
            );
        }
    }

    let body_color = enemy_body_color(enemy.kind);
    let accent_color = enemy_accent_color(enemy.kind);

    // Legs
    let leg_anim = if enemy.state == EnemyState::Walking {
        (enemy.anim_frame as f32 * 0.25).sin() * 5.0
    } else {
        0.0
    };

    draw_rectangle(
        x - 7.0 * scale - leg_anim * scale * 0.5,
        y - 10.0 * scale,
        5.0 * scale,
        12.0 * scale,
        Color::new(0.2, 0.2, 0.2, 1.0),
    );
    draw_rectangle(
        x + 2.0 * scale + leg_anim * scale * 0.5,
        y - 10.0 * scale,
        5.0 * scale,
        12.0 * scale,
        Color::new(0.2, 0.2, 0.2, 1.0),
    );

    // Body
    let body_w = sw * 0.6 * scale;
    let body_h = sh * 0.4 * scale;
    draw_rectangle(x - body_w * 0.5, y - 10.0 * scale - body_h, body_w, body_h, body_color);

    // Arms
    let arm_ext = if enemy.state == EnemyState::Attacking {
        18.0 * facing
    } else {
        (enemy.anim_frame as f32 * 0.12).sin() * 3.0 * facing
    };

    draw_rectangle(
        x + arm_ext * scale,
        y - (sh * 0.55) * scale,
        5.0 * scale,
        10.0 * scale,
        accent_color,
    );
    draw_rectangle(
        x - 10.0 * scale * facing,
        y - (sh * 0.55) * scale,
        5.0 * scale,
        10.0 * scale,
        accent_color,
    );

    // Head
    let head_size = if enemy.kind == EnemyKind::Brawler || enemy.kind == EnemyKind::BossCrusher {
        14.0
    } else {
        10.0
    };
    draw_rectangle(
        x - head_size * 0.5 * scale,
        y - sh * scale + 2.0 * scale,
        head_size * scale,
        head_size * scale,
        SKIN_COLOR,
    );

    // Boss indicator: crown / special accessory
    if enemy.is_boss() {
        draw_rectangle(
            x - 8.0 * scale,
            y - sh * scale - 2.0 * scale,
            16.0 * scale,
            4.0 * scale,
            BOSS_GOLD,
        );
    }

    // Knife wielder shows knife
    if enemy.kind == EnemyKind::KnifeWielder || enemy.kind == EnemyKind::BossBlade {
        let kx = x + (arm_ext + 6.0 * facing) * scale;
        let ky = y - (sh * 0.5) * scale;
        draw_rectangle(kx, ky, 10.0 * scale * facing, 2.0 * scale, Color::new(0.7, 0.7, 0.8, 1.0));
    }

    // Boss HP bar
    if enemy.is_boss() && enemy.hp > 0.0 {
        let bar_w = 200.0;
        let bar_h = 8.0;
        let bx = (SCREEN_W - bar_w) * 0.5;
        let by = 50.0;
        draw_rectangle(bx - 1.0, by - 1.0, bar_w + 2.0, bar_h + 2.0, WHITE);
        draw_rectangle(bx, by, bar_w, bar_h, Color::new(0.2, 0.0, 0.0, 1.0));
        let frac = (enemy.hp / enemy.max_hp).clamp(0.0, 1.0);
        draw_rectangle(bx, by, bar_w * frac, bar_h, RED);
        let boss_name = match enemy.kind {
            EnemyKind::BossBlade => "BLADE",
            EnemyKind::BossCrusher => "CRUSHER",
            EnemyKind::BossDragonKing => "JIN TAKEDA",
            _ => "BOSS",
        };
        let tw = measure_text(boss_name, None, 18, 1.0).width;
        draw_text(
            boss_name,
            (SCREEN_W - tw) * 0.5,
            by - 4.0,
            18.0,
            NEON_ORANGE,
        );
    }
}

fn enemy_body_color(kind: EnemyKind) -> Color {
    match kind {
        EnemyKind::Thug => THUG_RED,
        EnemyKind::KnifeWielder => KNIFE_PURPLE,
        EnemyKind::Brawler => BRAWLER_GREEN,
        EnemyKind::BossBlade => KNIFE_DARK,
        EnemyKind::BossCrusher => BRAWLER_GREEN,
        EnemyKind::BossDragonKing => Color::new(0.6, 0.1, 0.1, 1.0),
    }
}

fn enemy_accent_color(kind: EnemyKind) -> Color {
    match kind {
        EnemyKind::Thug => THUG_BROWN,
        EnemyKind::KnifeWielder => KNIFE_DARK,
        EnemyKind::Brawler => Color::new(1.0, 0.67, 0.4, 1.0),
        EnemyKind::BossBlade => KNIFE_PURPLE,
        EnemyKind::BossCrusher => Color::new(1.0, 0.67, 0.4, 1.0),
        EnemyKind::BossDragonKing => BOSS_GOLD,
    }
}

fn draw_ground_weapon(w: &GroundWeapon, cam: f32, sy: f32) {
    let x = w.x - cam;
    let y = w.y + sy;
    // Glow
    draw_circle(x, y - 4.0, 10.0, Color::new(1.0, 1.0, 0.5, 0.15));
    match w.kind {
        WeaponKind::Pipe => {
            draw_rectangle(x - 12.0, y - 3.0, 24.0, 6.0, GRAY);
        }
        WeaponKind::Knife => {
            draw_rectangle(x - 8.0, y - 2.0, 16.0, 4.0, Color::new(0.7, 0.7, 0.8, 1.0));
            draw_rectangle(x - 3.0, y - 4.0, 6.0, 8.0, Color::new(0.4, 0.3, 0.2, 1.0));
        }
        WeaponKind::Bottle => {
            draw_rectangle(x - 4.0, y - 8.0, 8.0, 12.0, Color::new(0.3, 0.6, 0.3, 0.8));
            draw_rectangle(x - 2.0, y - 12.0, 4.0, 4.0, Color::new(0.3, 0.6, 0.3, 0.8));
        }
    }
}

fn draw_pickup(p: &Pickup, cam: f32, sy: f32) {
    let x = p.x - cam;
    let y = p.y + sy;
    // Glow
    let t = get_time() as f32;
    let bob = (t * 3.0).sin() * 3.0;
    draw_circle(x, y - 8.0 + bob, 12.0, Color::new(1.0, 1.0, 0.0, 0.1));
    match p.kind {
        PickupKind::Chicken => {
            // Drumstick
            draw_circle(x - 2.0, y - 8.0 + bob, 6.0, Color::new(0.8, 0.5, 0.2, 1.0));
            draw_rectangle(x + 2.0, y - 10.0 + bob, 8.0, 4.0, Color::new(0.9, 0.85, 0.7, 1.0));
        }
        PickupKind::Pizza => {
            // Pizza slice triangle approximation
            draw_triangle(
                vec2(x, y - 16.0 + bob),
                vec2(x - 8.0, y - 2.0 + bob),
                vec2(x + 8.0, y - 2.0 + bob),
                Color::new(0.9, 0.7, 0.1, 1.0),
            );
            // Red dots (pepperoni)
            draw_circle(x - 2.0, y - 8.0 + bob, 2.0, Color::new(0.8, 0.1, 0.1, 1.0));
            draw_circle(x + 2.0, y - 6.0 + bob, 2.0, Color::new(0.8, 0.1, 0.1, 1.0));
        }
        PickupKind::ExtraLife => {
            // Star token
            draw_circle(x, y - 8.0 + bob, 8.0, NEON_YELLOW);
            draw_text("1UP", x - 9.0, y - 5.0 + bob, 14.0, BLACK);
        }
    }
}

fn draw_projectile(proj: &Projectile, cam: f32, sy: f32) {
    let x = proj.x - cam;
    let y = proj.y + sy;
    // Knife projectile
    draw_rectangle(x - 6.0, y - 2.0, 12.0, 4.0, Color::new(0.7, 0.7, 0.8, 1.0));
    // Trail
    draw_rectangle(
        x - 6.0 - proj.vx * 3.0,
        y - 1.0,
        8.0,
        2.0,
        Color::new(0.7, 0.7, 0.8, 0.4),
    );
}

fn draw_hud(game: &Game) {
    let p = &game.player;

    // Player name and health bar
    draw_text("DRAGON", 16.0, 24.0, 20.0, NEON_CYAN);
    let bar_w = 150.0;
    let bar_h = 12.0;
    let bx = 16.0;
    let by = 30.0;
    draw_rectangle(bx - 1.0, by - 1.0, bar_w + 2.0, bar_h + 2.0, WHITE);
    draw_rectangle(bx, by, bar_w, bar_h, Color::new(0.15, 0.0, 0.0, 1.0));
    let frac = (p.hp / p.max_hp).clamp(0.0, 1.0);
    let hp_color = if frac > 0.5 {
        Color::new(0.0, 1.0, 0.0, 1.0)
    } else if frac > 0.25 {
        Color::new(1.0, 1.0, 0.0, 1.0)
    } else {
        Color::new(1.0, 0.0, 0.0, 1.0)
    };
    draw_rectangle(bx, by, bar_w * frac, bar_h, hp_color);

    // Score
    let score_txt = format!("SCORE: {}", p.score);
    let tw = measure_text(&score_txt, None, 20, 1.0).width;
    draw_text(&score_txt, SCREEN_W - tw - 16.0, 24.0, 20.0, WHITE);

    // Lives
    for i in 0..p.lives {
        draw_rectangle(
            16.0 + i as f32 * 20.0,
            SCREEN_H - 30.0,
            14.0,
            14.0,
            PLAYER_BLUE,
        );
        draw_rectangle(
            19.0 + i as f32 * 20.0,
            SCREEN_H - 34.0,
            8.0,
            6.0,
            PLAYER_HAIR,
        );
    }

    // Weapon indicator
    if let Some(wk) = p.weapon {
        let name = match wk {
            WeaponKind::Pipe => "PIPE",
            WeaponKind::Knife => "KNIFE",
            WeaponKind::Bottle => "BOTTLE",
        };
        let txt = format!("{}: {}", name, p.weapon_durability);
        let tw = measure_text(&txt, None, 18, 1.0).width;
        draw_text(
            &txt,
            SCREEN_W - tw - 16.0,
            SCREEN_H - 20.0,
            18.0,
            NEON_ORANGE,
        );
    }
}

// ---------------------------------------------------------------------------
// Screen draws
// ---------------------------------------------------------------------------
fn draw_title(game: &Game) {
    // Background gradient
    for i in 0..48 {
        let y = i as f32 * 10.0;
        let t = i as f32 / 48.0;
        let c = Color::new(0.04 + t * 0.06, 0.02, 0.1 + t * 0.08, 1.0);
        draw_rectangle(0.0, y, SCREEN_W, 10.0, c);
    }

    // Neon border lines
    draw_line(0.0, 0.0, SCREEN_W, 0.0, 2.0, NEON_PINK);
    draw_line(0.0, SCREEN_H - 1.0, SCREEN_W, SCREEN_H - 1.0, 2.0, NEON_PINK);

    // Title text with glow
    let title1 = "DRAGON";
    let title2 = "FURY";
    let t1w = measure_text(title1, None, 72, 1.0).width;
    let t2w = measure_text(title2, None, 72, 1.0).width;

    // Glow behind text
    let glow_alpha = 0.3 + 0.15 * (get_time() as f32 * 2.0).sin();
    draw_text(
        title1,
        (SCREEN_W - t1w) * 0.5 + 2.0,
        170.0,
        72.0,
        Color::new(1.0, 0.3, 0.0, glow_alpha),
    );
    draw_text(
        title2,
        (SCREEN_W - t2w) * 0.5 + 2.0,
        240.0,
        72.0,
        Color::new(1.0, 0.0, 0.3, glow_alpha),
    );

    // Main text
    draw_text(
        title1,
        (SCREEN_W - t1w) * 0.5,
        168.0,
        72.0,
        NEON_ORANGE,
    );
    draw_text(
        title2,
        (SCREEN_W - t2w) * 0.5,
        238.0,
        72.0,
        NEON_PINK,
    );

    // Subtitle: STREETS OF VENGEANCE
    let sub = "STREETS OF VENGEANCE";
    let sub_w = measure_text(sub, None, 18, 1.0).width;
    draw_text(sub, (SCREEN_W - sub_w) * 0.5, 262.0, 18.0, NEON_CYAN);

    // Tagline
    let tag = "A Dragon Fist Story";
    let tag_w = measure_text(tag, None, 14, 1.0).width;
    draw_text(tag, (SCREEN_W - tag_w) * 0.5, 284.0, 14.0, NEON_PINK);

    // Procedural character preview
    draw_title_character(SCREEN_W * 0.5, 340.0);

    // Blinking "PRESS START"
    if (game.title_blink * 2.5).sin() > 0.0 {
        let txt = "PRESS START";
        let tw = measure_text(txt, None, 28, 1.0).width;
        draw_text(txt, (SCREEN_W - tw) * 0.5, 400.0, 28.0, WHITE);
    }

    // Controls
    let ctrl = "Arrows:Move  X:Punch  Space:Jump  Z:Special  Enter:Start";
    let cw = measure_text(ctrl, None, 14, 1.0).width;
    draw_text(
        ctrl,
        (SCREEN_W - cw) * 0.5,
        450.0,
        14.0,
        Color::new(0.6, 0.6, 0.6, 1.0),
    );
}

fn draw_title_character(cx: f32, cy: f32) {
    // Simple standing character
    // Legs
    draw_rectangle(cx - 8.0, cy, 6.0, 16.0, Color::new(0.2, 0.2, 0.6, 1.0));
    draw_rectangle(cx + 2.0, cy, 6.0, 16.0, Color::new(0.2, 0.2, 0.6, 1.0));
    // Body
    draw_rectangle(cx - 10.0, cy - 24.0, 20.0, 24.0, PLAYER_BLUE);
    // Arms
    draw_rectangle(cx - 16.0, cy - 22.0, 6.0, 14.0, Color::new(0.0, 0.4, 0.8, 1.0));
    draw_rectangle(cx + 10.0, cy - 22.0, 6.0, 14.0, SKIN_COLOR);
    // Head
    draw_rectangle(cx - 6.0, cy - 36.0, 12.0, 12.0, SKIN_COLOR);
    // Hair
    draw_rectangle(cx - 7.0, cy - 38.0, 14.0, 6.0, PLAYER_HAIR);
    // Eyes
    draw_rectangle(cx + 1.0, cy - 33.0, 2.0, 2.0, BLACK);
    // Fist glow
    let t = get_time() as f32;
    let glow = 0.3 + 0.2 * (t * 4.0).sin();
    draw_circle(cx + 13.0, cy - 15.0, 8.0, Color::new(1.0, 0.5, 0.0, glow));
}

fn draw_stage_transition(game: &Game) {
    // Fade alpha: during fade-out, black goes from 0->1;
    // during hold, black = 1; during fade-in, black goes from 1->0
    let fade_alpha = if game.stage_fade_dir == -1 {
        // Fading to black
        (game.stage_fade_timer as f32 / 30.0).clamp(0.0, 1.0)
    } else if game.stage_fade_dir == 0 {
        // Holding
        1.0
    } else {
        // Fading in from black
        1.0 - (game.stage_fade_timer as f32 / 30.0).clamp(0.0, 1.0)
    };

    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.0, 0.0, 0.0, fade_alpha));

    // Only show text when screen is mostly dark
    let text_alpha = if game.stage_fade_dir == 0 {
        1.0
    } else if game.stage_fade_dir == -1 && game.stage_fade_timer > 20 {
        (game.stage_fade_timer as f32 - 20.0) / 10.0
    } else if game.stage_fade_dir == 1 && game.stage_fade_timer < 10 {
        (10.0 - game.stage_fade_timer as f32) / 10.0
    } else if game.stage_fade_dir == -1 {
        0.0
    } else {
        1.0
    };

    let c = Color::new(1.0, 1.0, 1.0, text_alpha.clamp(0.0, 1.0));

    let stage_name = format!(
        "STAGE {}: {}",
        game.stage_index + 1,
        game.stages[game.stage_index].name
    );
    let tw = measure_text(&stage_name, None, 40, 1.0).width;
    draw_text(
        &stage_name,
        (SCREEN_W - tw) * 0.5,
        SCREEN_H * 0.45,
        40.0,
        c,
    );

    let ready = "GET READY!";
    let rw = measure_text(ready, None, 24, 1.0).width;
    draw_text(
        ready,
        (SCREEN_W - rw) * 0.5,
        SCREEN_H * 0.55,
        24.0,
        Color::new(NEON_CYAN.r, NEON_CYAN.g, NEON_CYAN.b, text_alpha.clamp(0.0, 1.0)),
    );
}

fn draw_game_over(game: &Game) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.0, 0.0, 0.0, 0.85));

    let go = "GAME OVER";
    let gow = measure_text(go, None, 56, 1.0).width;
    draw_text(go, (SCREEN_W - gow) * 0.5, 180.0, 56.0, RED);

    // Subtitle
    let sub = "STREETS OF VENGEANCE";
    let sub_w = measure_text(sub, None, 14, 1.0).width;
    draw_text(sub, (SCREEN_W - sub_w) * 0.5, 210.0, 14.0, NEON_CYAN);

    let score_txt = format!("SCORE: {}", game.player.score);
    let sw = measure_text(&score_txt, None, 28, 1.0).width;
    draw_text(&score_txt, (SCREEN_W - sw) * 0.5, 260.0, 28.0, WHITE);

    let cont = "CONTINUE?";
    let cw = measure_text(cont, None, 22, 1.0).width;
    draw_text(cont, (SCREEN_W - cw) * 0.5, 310.0, 22.0, WHITE);

    let secs = format!("{}", game.continue_timer.ceil() as i32);
    let secw = measure_text(&secs, None, 44, 1.0).width;
    draw_text(&secs, (SCREEN_W - secw) * 0.5, 360.0, 44.0, NEON_YELLOW);

    let hint = "PRESS START TO CONTINUE";
    let hw = measure_text(hint, None, 16, 1.0).width;
    draw_text(
        hint,
        (SCREEN_W - hw) * 0.5,
        410.0,
        16.0,
        Color::new(0.5, 0.5, 0.5, 1.0),
    );
}

fn draw_victory(game: &Game) {
    // Background: dark with sunrise gradient
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.04, 0.04, 0.10, 1.0));
    // Sunrise gradient at top
    for i in 0..12 {
        let y = i as f32 * 10.0;
        let alpha = 0.2 * (1.0 - y / 120.0);
        draw_rectangle(0.0, y, SCREEN_W, 10.0, Color::new(1.0, 0.4, 0.0, alpha));
    }

    let v1 = "VICTORY";
    let vw = measure_text(v1, None, 48, 1.0).width;
    draw_text(v1, (SCREEN_W - vw) * 0.5, 70.0, 48.0, NEON_YELLOW);

    let sub = "STREETS OF VENGEANCE";
    let sub_w = measure_text(sub, None, 16, 1.0).width;
    draw_text(sub, (SCREEN_W - sub_w) * 0.5, 100.0, 16.0, NEON_CYAN);

    let score_txt = format!("SCORE: {:07}", game.player.score);
    let sw = measure_text(&score_txt, None, 22, 1.0).width;
    draw_text(&score_txt, (SCREEN_W - sw) * 0.5, 140.0, 22.0, NEON_CYAN);

    // THE DRAGON FIST ENDURES
    let endures = "THE DRAGON FIST ENDURES";
    let ew = measure_text(endures, None, 16, 1.0).width;
    draw_text(endures, (SCREEN_W - ew) * 0.5, 175.0, 16.0, NEON_PINK);

    // Character preview
    draw_title_character(SCREEN_W * 0.5, 250.0);

    if game.victory_timer > 5.0 {
        if ((get_time() as f32) * 2.5).sin() > 0.0 {
            let txt = "PRESS START";
            let tw = measure_text(txt, None, 20, 1.0).width;
            draw_text(txt, (SCREEN_W - tw) * 0.5, 440.0, 20.0, WHITE);
        }
    }
}

// ---------------------------------------------------------------------------
// Story / Dialogue Drawing
// ---------------------------------------------------------------------------
fn draw_story_screen(game: &Game) {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, BLACK);

    // Subtle background effect (floating circles)
    let fc = game.frame_count as f32;
    for i in 0..5 {
        let rx = (fc * 0.01 + i as f32).sin() * 100.0 + SCREEN_W * 0.5;
        let ry = (fc * 0.008 + i as f32 * 2.0).cos() * 80.0 + SCREEN_H * 0.5;
        draw_circle(rx, ry, 100.0 + i as f32 * 20.0, Color::new(1.0, 0.0, 0.4, 0.03));
    }

    // Title bar based on phase
    let header = match game.story_phase {
        StoryPhase::Intro => "STREETS OF VENGEANCE".to_string(),
        StoryPhase::PreStage => format!(
            "STAGE {} -- {}",
            game.stage_index + 1,
            game.stages[game.stage_index].name
        ),
        StoryPhase::PostStage => format!("STAGE {} CLEAR", game.stage_index + 1),
        StoryPhase::VictoryStory => "EPILOGUE".to_string(),
    };
    let hw = measure_text(&header, None, 16, 1.0).width;
    draw_text(&header, (SCREEN_W - hw) * 0.5, 40.0, 16.0, NEON_PINK);

    // Horizontal divider
    draw_line(SCREEN_W * 0.15, 55.0, SCREEN_W * 0.85, 55.0, 1.0, Color::new(1.0, 0.0, 0.4, 0.4));

    // Draw story lines with typewriter effect
    let line_height = 28.0_f32;
    let start_y = 100.0_f32;
    let max_width = SCREEN_W - 100.0;
    let font_size = 16.0_f32;

    let mut y_offset = 0.0_f32;
    for i in 0..=game.story_line_index {
        if i >= game.story_lines.len() {
            break;
        }
        let full_line = game.story_lines[i];
        let text = if i < game.story_line_index {
            full_line.to_string()
        } else {
            // Current line: show chars up to story_char_index (char-safe)
            full_line.chars().take(game.story_char_index).collect::<String>()
        };

        let color = if i == game.story_line_index {
            WHITE
        } else {
            Color::new(0.8, 0.8, 0.8, 0.7)
        };

        // Simple word-wrap drawing
        let wrapped_lines = wrap_text_lines(&text, font_size, max_width);
        for wl in &wrapped_lines {
            draw_text(wl, 50.0, start_y + y_offset, font_size, color);
            y_offset += line_height;
        }
        y_offset += 12.0; // gap between story lines
    }

    // Blinking cursor on current line
    if game.story_line_index < game.story_lines.len() {
        let char_count = game.story_lines[game.story_line_index].chars().count();
        if game.story_char_index < char_count {
            // Show blinking cursor
            if (game.frame_count / 8) % 2 == 0 {
                let partial: String = game.story_lines[game.story_line_index].chars().take(game.story_char_index).collect();
                let wrapped = wrap_text_lines(&partial, font_size, max_width);
                let cursor_line = wrapped.last().unwrap_or(&String::new()).clone();
                let cursor_x = 50.0 + measure_text(&cursor_line, None, font_size as u16, 1.0).width;
                let num_prev_lines: f32 = {
                    let mut total = 0.0;
                    for j in 0..game.story_line_index {
                        if j < game.story_lines.len() {
                            let wl = wrap_text_lines(game.story_lines[j], font_size, max_width);
                            total += wl.len() as f32 * line_height + 12.0;
                        }
                    }
                    total += (wrapped.len() as f32 - 1.0).max(0.0) * line_height;
                    total
                };
                let cursor_y = start_y + num_prev_lines - 10.0;
                draw_rectangle(cursor_x, cursor_y, 8.0, 14.0, WHITE);
            }
        } else {
            // Line complete, show "press to continue" hint
            if (game.frame_count / 20) % 2 == 0 {
                let hint = "PRESS START TO CONTINUE";
                let hw2 = measure_text(hint, None, 14, 1.0).width;
                draw_text(hint, (SCREEN_W - hw2) * 0.5, SCREEN_H - 30.0, 14.0, Color::new(0.5, 0.5, 0.5, 1.0));
            }
        }
    } else {
        // All lines shown
        if (game.frame_count / 20) % 2 == 0 {
            let hint = "PRESS START TO CONTINUE";
            let hw2 = measure_text(hint, None, 14, 1.0).width;
            draw_text(hint, (SCREEN_W - hw2) * 0.5, SCREEN_H - 30.0, 14.0, Color::new(0.5, 0.5, 0.5, 1.0));
        }
    }
}

fn draw_boss_intro_screen(game: &Game) {
    // Dark overlay
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.0, 0.0, 0.0, 0.9));

    let idx = game.boss_intro_index;
    if idx >= BOSS_INTROS.len() {
        return;
    }
    let info = &BOSS_INTROS[idx];

    let t = (game.boss_intro_timer as f32 / 30.0).min(1.0); // fade in

    // Warning flash
    if game.boss_intro_timer < 60 && (game.boss_intro_timer / 4) % 2 == 0 {
        let warn = "!! WARNING !!";
        let ww = measure_text(warn, None, 16, 1.0).width;
        draw_text(warn, (SCREEN_W - ww) * 0.5, SCREEN_H * 0.5 - 100.0, 16.0, Color::new(1.0, 0.0, 0.0, t));
    }

    // Boss name
    let nw = measure_text(info.name, None, 36, 1.0).width;
    draw_text(info.name, (SCREEN_W - nw) * 0.5, SCREEN_H * 0.5 - 40.0, 36.0, Color::new(1.0, 0.0, 0.4, t));

    // Boss title
    let tw = measure_text(info.title, None, 16, 1.0).width;
    draw_text(info.title, (SCREEN_W - tw) * 0.5, SCREEN_H * 0.5, 16.0, Color::new(1.0, 1.0, 0.0, t));

    // Divider
    draw_line(
        SCREEN_W * 0.2, SCREEN_H * 0.5 + 20.0,
        SCREEN_W * 0.8, SCREEN_H * 0.5 + 20.0,
        2.0,
        Color::new(1.0, 0.0, 0.4, 0.4 * t),
    );

    // Quote with word wrap
    let quote = format!("\"{}\"", info.quote);
    let wrapped = wrap_text_lines(&quote, 16.0, SCREEN_W * 0.7);
    let mut qy = SCREEN_H * 0.5 + 50.0;
    for line in &wrapped {
        draw_text(line, SCREEN_W * 0.15, qy, 16.0, Color::new(1.0, 1.0, 1.0, t));
        qy += 22.0;
    }

    // Prompt
    if game.boss_intro_timer > 30 && (game.frame_count / 20) % 2 == 0 {
        let prompt = "PRESS START TO FIGHT";
        let pw = measure_text(prompt, None, 14, 1.0).width;
        draw_text(prompt, (SCREEN_W - pw) * 0.5, SCREEN_H - 40.0, 14.0, Color::new(0.5, 0.5, 0.5, 1.0));
    }
}

fn draw_mid_stage_dialogue(game: &Game) {
    if let Some(text) = game.mid_stage_dialogue {
        // Semi-transparent overlay at bottom
        draw_rectangle(0.0, SCREEN_H - 120.0, SCREEN_W, 120.0, Color::new(0.0, 0.0, 0.0, 0.85));
        draw_line(0.0, SCREEN_H - 120.0, SCREEN_W, SCREEN_H - 120.0, 2.0, NEON_PINK);

        let wrapped = wrap_text_lines(text, 14.0, SCREEN_W - 60.0);
        let mut y = SCREEN_H - 100.0;
        for line in &wrapped {
            draw_text(line, 30.0, y, 14.0, WHITE);
            y += 22.0;
        }
    }
}

/// Simple word-wrap: splits text into lines that fit within max_width pixels.
fn wrap_text_lines(text: &str, font_size: f32, max_width: f32) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();
    for word in text.split_whitespace() {
        let test = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        let w = measure_text(&test, None, font_size as u16, 1.0).width;
        if w > max_width && !current_line.is_empty() {
            lines.push(current_line);
            current_line = word.to_string();
        } else {
            current_line = test;
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

fn draw_pause_overlay() {
    draw_rectangle(
        0.0,
        0.0,
        SCREEN_W,
        SCREEN_H,
        Color::new(0.0, 0.0, 0.0, 0.5),
    );
    let txt = "PAUSED";
    let tw = measure_text(txt, None, 48, 1.0).width;
    draw_text(txt, (SCREEN_W - tw) * 0.5, SCREEN_H * 0.45, 48.0, WHITE);

    let hint = "Press Enter to resume";
    let hw = measure_text(hint, None, 20, 1.0).width;
    draw_text(
        hint,
        (SCREEN_W - hw) * 0.5,
        SCREEN_H * 0.55,
        20.0,
        Color::new(0.7, 0.7, 0.7, 1.0),
    );
}

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------
fn rects_overlap(a: &HitRect, b: &HitRect) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

struct HitRect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl HitRect {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}
