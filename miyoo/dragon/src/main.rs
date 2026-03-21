// Dragon Fury: Streets of Vengeance - Miyoo Mini Plus Port
// Beat 'em up in the style of Streets of Rage / Final Fight
// A Dragon Fist Story - Rust/Macroquad 0.4 port targeting 800x600 @ 60fps
// Synced with web version gameplay, physics, entity sizes, and stage data.

use macroquad::prelude::*;

// ---------------------------------------------------------------------------
// Constants (synced with web version: 800x600, gravity-based jumps)
// ---------------------------------------------------------------------------
const SCREEN_W: f32 = 800.0;
const SCREEN_H: f32 = 600.0;

const GROUND_MIN: f32 = 375.0;
const GROUND_MAX: f32 = 550.0;
const DEPTH_MATCH: f32 = 25.0;

const PLAYER_SPEED: f32 = 3.0;
const PLAYER_DEPTH_SPEED: f32 = 2.0;
const PLAYER_MAX_HP: f32 = 100.0;
const PLAYER_START_LIVES: i32 = 3;

const COMBO_DAMAGE: [f32; 3] = [12.0, 18.0, 30.0];
const JUMP_KICK_DAMAGE: f32 = 30.0;
const GRAVITY: f32 = 0.7;
const JUMP_VELOCITY: f32 = 12.0;
const SPECIAL_DAMAGE: f32 = 40.0;
const SPECIAL_COST: f32 = 15.0;
const SPECIAL_FRAMES: i32 = 25;

const MAX_PARTICLES: usize = 200;
const CONTINUE_FRAMES: i32 = 540; // 9 seconds at 60fps
const MAX_FLOAT_TEXTS: usize = 16;
const FLOAT_TEXT_DURATION: i32 = 40;

const SCREEN_SHAKE_HEAVY: f32 = 4.0;
const SCREEN_SHAKE_LIGHT: f32 = 2.0;

// Palette
const BG_DARK: Color = Color::new(0.04, 0.04, 0.10, 1.0);
const NEON_PINK: Color = Color::new(1.0, 0.0, 0.4, 1.0);
const NEON_CYAN: Color = Color::new(0.0, 1.0, 1.0, 1.0);
const NEON_ORANGE: Color = Color::new(1.0, 0.4, 0.0, 1.0);
const NEON_YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
const PLAYER_BLUE: Color = Color::new(0.0, 0.53, 1.0, 1.0);
const PLAYER_HAIR: Color = Color::new(1.0, 0.8, 0.0, 1.0);
const SKIN_COLOR: Color = Color::new(1.0, 0.82, 0.65, 1.0);
const THUG_RED: Color = Color::new(0.8, 0.2, 0.2, 1.0);
const KNIFE_PURPLE: Color = Color::new(0.53, 0.2, 0.67, 1.0);
const BRAWLER_GREEN: Color = Color::new(0.2, 0.67, 0.2, 1.0);
const BOSS_GOLD: Color = Color::new(0.85, 0.65, 0.13, 1.0);

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq)]
enum GameState { Title, Story, BossIntro, StageIntro, Playing, Paused, GameOver, Victory }

#[derive(Clone, Copy, PartialEq)]
enum StoryPhase { Intro, PreStage, PostStage, VictoryStory }

#[derive(Clone, Copy, PartialEq)]
enum PlayerState { Idle, Walking, Punching, Jumping, JumpKicking, Grabbing, Special, Hurt, Dead }

#[derive(Clone, Copy, PartialEq)]
enum EnemyState { Idle, Walking, Attacking, Stunned, Grabbed, Thrown, Dead }

#[derive(Clone, Copy, PartialEq)]
enum EnemyKind { Thug, KnifeWielder, Brawler, BossBlade, BossCrusher, BossDragonKing }

#[derive(Clone, Copy, PartialEq)]
enum WeaponKind { Pipe, Knife, Bottle }

#[derive(Clone, Copy, PartialEq)]
enum PickupKind { Chicken, Pizza, ExtraLife }

struct BossIntroInfo { name: &'static str, title: &'static str, quote: &'static str }

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
    &["Blade falls. Through his radio, you hear a voice -- cold, familiar.",
      "\"Let him come. I want to see what Sato taught him.\"",
      "It's Jin. He knows you're coming."],
    &["Behind the warehouse, you find a passage leading up -- to the rooftops.",
      "And a note in Sato's handwriting, hidden under a loose brick:",
      "\"Ryu -- Jin doesn't want the technique to sell. He wants it to destroy. The rage consumed him long ago. Forgive him if you can. Stop him if you must. --Sato\""],
    &["Jin falls to his knees. His rage is spent.",
      "Sato limps forward and places a hand on Jin's shoulder.",
      "\"I didn't refuse you because you were weak, Jin. I refused because the Dragon's Breath amplifies what's in your heart. And your heart was full of anger. But anger fades.\"",
      "Jin looks up. For the first time in ten years, he doesn't look furious. He looks tired."],
];
const STORY_VICTORY: &[&str] = &[
    "The sun rises over Neo-Osaka. The Iron Serpents scatter without their leader.",
    "Grandmaster Sato begins rebuilding the dojo. It will take months. Jin Takeda turns himself in to the authorities.",
    "Three months later, you visit Jin in prison. He's thinner. Quieter. He asks about the dojo.",
    "\"We saved you a spot,\" you tell him. \"When you're ready.\"",
    "The Dragon Fist endures.",
];

// ---------------------------------------------------------------------------
// Structs
// ---------------------------------------------------------------------------
#[derive(Clone)]
struct Player {
    x: f32, ground_y: f32, z: f32, vz: f32,
    w: f32, h: f32, facing: f32,
    hp: f32, max_hp: f32, lives: i32,
    state: PlayerState, anim_frame: i32, anim_timer: i32,
    combo_count: i32, combo_timer: i32, attack_cooldown: i32,
    inv_frames: i32, hurt_timer: i32,
    knockback_vx: f32, knockback_vy: f32,
    weapon: Option<WeaponKind>, weapon_durability: i32,
    grab_target: Option<usize>, grab_timer: i32, grab_knees: i32,
    jump_kicking: bool,
}
impl Player {
    fn new() -> Self {
        Self {
            x: 80.0, ground_y: 460.0, z: 0.0, vz: 0.0,
            w: 32.0, h: 48.0, facing: 1.0,
            hp: PLAYER_MAX_HP, max_hp: PLAYER_MAX_HP, lives: PLAYER_START_LIVES,
            state: PlayerState::Idle, anim_frame: 0, anim_timer: 0,
            combo_count: 0, combo_timer: 0, attack_cooldown: 0,
            inv_frames: 0, hurt_timer: 0,
            knockback_vx: 0.0, knockback_vy: 0.0,
            weapon: None, weapon_durability: 0,
            grab_target: None, grab_timer: 0, grab_knees: 0,
            jump_kicking: false,
        }
    }
    fn body_rect(&self) -> HitRect {
        HitRect { x: self.x, y: self.ground_y - self.z - self.h, w: self.w, h: self.h }
    }
    fn attack_damage(&self) -> f32 {
        if let Some(w) = self.weapon {
            match w { WeaponKind::Pipe => 35.0, WeaponKind::Knife => 25.0, WeaponKind::Bottle => 40.0 }
        } else if self.state == PlayerState::JumpKicking { JUMP_KICK_DAMAGE }
        else if self.state == PlayerState::Special { SPECIAL_DAMAGE }
        else { COMBO_DAMAGE[(self.combo_count - 1).clamp(0, 2) as usize] }
    }
    fn is_invincible(&self) -> bool {
        self.inv_frames > 0 || self.state == PlayerState::Special || self.state == PlayerState::Dead || self.hurt_timer > 0
    }
}

#[derive(Clone)]
struct Enemy {
    x: f32, ground_y: f32, w: f32, h: f32, facing: f32,
    hp: f32, max_hp: f32, damage: f32,
    kind: EnemyKind, state: EnemyState,
    speed: f32, score_value: i32, can_grab: bool, armor: i32, armor_hits: i32,
    attack_cooldown: i32, attack_timer: i32, stun_timer: i32,
    hurt_timer: i32, knockback_vx: f32, knockback_vy: f32,
    dead: bool, death_timer: i32, to_remove: bool, anim_frame: i32,
    boss_name: Option<&'static str>,
}
impl Enemy {
    fn new(kind: EnemyKind, x: f32, y: f32) -> Self {
        let (hp, speed, damage, w, h, score, can_grab, armor) = match kind {
            EnemyKind::Thug => (60.0, 1.2, 10.0, 32.0, 48.0, 100, true, 0),
            EnemyKind::KnifeWielder => (40.0, 1.8, 18.0, 32.0, 48.0, 200, true, 0),
            EnemyKind::Brawler => (150.0, 0.7, 25.0, 42.0, 56.0, 500, false, 3),
            EnemyKind::BossBlade => (350.0, 1.0, 30.0, 42.0, 56.0, 2000, false, 2),
            EnemyKind::BossCrusher => (350.0, 1.0, 30.0, 42.0, 56.0, 2000, false, 2),
            EnemyKind::BossDragonKing => (350.0, 1.0, 30.0, 42.0, 56.0, 2000, false, 2),
        };
        let boss_name = match kind {
            EnemyKind::BossBlade => Some("BLADE"),
            EnemyKind::BossCrusher => Some("CRUSHER"),
            EnemyKind::BossDragonKing => Some("JIN TAKEDA"),
            _ => None,
        };
        Self {
            x, ground_y: y, w, h, facing: -1.0,
            hp, max_hp: hp, damage, kind, state: EnemyState::Idle,
            speed, score_value: score, can_grab, armor, armor_hits: 0,
            attack_cooldown: (60.0 + rand::gen_range(0.0, 60.0)) as i32,
            attack_timer: 0, stun_timer: 0,
            hurt_timer: 0, knockback_vx: 0.0, knockback_vy: 0.0,
            dead: false, death_timer: 0, to_remove: false, anim_frame: 0,
            boss_name,
        }
    }
    fn body_rect(&self) -> HitRect {
        HitRect { x: self.x, y: self.ground_y - self.h, w: self.w, h: self.h }
    }
    fn is_boss(&self) -> bool {
        matches!(self.kind, EnemyKind::BossBlade | EnemyKind::BossCrusher | EnemyKind::BossDragonKing)
    }
}

#[derive(Clone)] struct GroundWeapon { x: f32, y: f32, kind: WeaponKind, durability: i32, active: bool }
#[derive(Clone)] struct Pickup { x: f32, y: f32, kind: PickupKind, active: bool }
#[derive(Clone)] struct Particle { x: f32, y: f32, vx: f32, vy: f32, life: f32, max_life: f32, color: Color, size: f32, active: bool }
impl Particle { fn inactive() -> Self { Self { x:0.0,y:0.0,vx:0.0,vy:0.0,life:0.0,max_life:1.0,color:WHITE,size:4.0,active:false } } }
#[derive(Clone)] struct FloatingText { x: f32, y: f32, text: &'static str, timer: i32, large: bool, active: bool }
impl FloatingText { fn inactive() -> Self { Self { x:0.0,y:0.0,text:"",timer:0,large:false,active:false } } }
#[derive(Clone)] struct Projectile { x: f32, y: f32, vx: f32, damage: f32, life: i32, active: bool }
struct ScreenShake { intensity: f32, frames: i32 }

struct WaveGroup { kind: EnemyKind, count: usize, boss_name: Option<&'static str> }
struct Wave { trigger_x: f32, groups: Vec<WaveGroup>, triggered: bool, boss_intro_shown: bool }
struct StageData { name: &'static str, length: f32, waves: Vec<Wave>, pickups: Vec<(PickupKind, f32)>, weapons: Vec<(WeaponKind, f32, i32)>, bg_color: Color }

// ---------------------------------------------------------------------------
// Game
// ---------------------------------------------------------------------------
struct Game {
    state: GameState,
    player: Player,
    enemies: Vec<Enemy>,
    ground_weapons: Vec<GroundWeapon>,
    pickups: Vec<Pickup>,
    particles: Vec<Particle>,
    projectiles: Vec<Projectile>,
    float_texts: Vec<FloatingText>,
    camera_x: f32, scroll_locked: bool,
    stage_index: usize, stages: Vec<StageData>,
    wave_index: usize, wave_clear_timer: i32,
    shake: ScreenShake,
    stage_intro_timer: i32, continue_timer: i32, game_over_timer: i32,
    title_blink: i32, flash_timer: i32,
    frame_count: u64, score: i32,
    // Story
    story_lines: Vec<&'static str>, story_line_index: usize, story_char_index: usize,
    story_char_timer: i32, story_phase: StoryPhase, story_next: u8,
    // Boss intro
    boss_intro_timer: i32,
    // Mid-stage dialogue
    mid_stage_dialogue: Option<&'static str>, mid_stage_timer: i32,
    mid_stage_shown: [bool; 10],
}

impl Game {
    fn new() -> Self {
        Self {
            state: GameState::Title,
            player: Player::new(),
            enemies: Vec::new(), ground_weapons: Vec::new(), pickups: Vec::new(),
            particles: vec![Particle::inactive(); MAX_PARTICLES],
            projectiles: Vec::new(),
            float_texts: vec![FloatingText::inactive(); MAX_FLOAT_TEXTS],
            camera_x: 0.0, scroll_locked: false,
            stage_index: 0, stages: build_stages(),
            wave_index: 0, wave_clear_timer: 0,
            shake: ScreenShake { intensity: 0.0, frames: 0 },
            stage_intro_timer: 0, continue_timer: 0, game_over_timer: 0,
            title_blink: 0, flash_timer: 0,
            frame_count: 0, score: 0,
            story_lines: Vec::new(), story_line_index: 0, story_char_index: 0,
            story_char_timer: 0, story_phase: StoryPhase::Intro, story_next: 0,
            boss_intro_timer: 0,
            mid_stage_dialogue: None, mid_stage_timer: 0,
            mid_stage_shown: [false; 10],
        }
    }

    fn init_stage(&mut self, idx: usize) {
        self.stage_index = idx;
        self.camera_x = 0.0; self.wave_index = 0; self.wave_clear_timer = 0;
        self.scroll_locked = false;
        self.enemies.clear(); self.ground_weapons.clear(); self.pickups.clear(); self.projectiles.clear();
        self.mid_stage_dialogue = None; self.mid_stage_timer = 0;
        self.shake.frames = 0; self.flash_timer = 0;
        for wave in &mut self.stages[idx].waves { wave.triggered = false; wave.boss_intro_shown = false; }
        let pds: Vec<(PickupKind, f32)> = self.stages[idx].pickups.clone();
        for (kind, px) in pds {
            let py = GROUND_MIN + rand::gen_range(0.0, GROUND_MAX - GROUND_MIN);
            self.pickups.push(Pickup { x: px, y: py, kind, active: true });
        }
        let wds: Vec<(WeaponKind, f32, i32)> = self.stages[idx].weapons.clone();
        for (kind, wx, dur) in wds {
            let wy = GROUND_MIN + rand::gen_range(0.0, GROUND_MAX - GROUND_MIN);
            self.ground_weapons.push(GroundWeapon { x: wx, y: wy, kind, durability: dur, active: true });
        }
        self.player.x = 80.0; self.player.ground_y = 460.0;
        self.player.z = 0.0; self.player.vz = 0.0;
        self.player.state = PlayerState::Idle; self.player.weapon = None;
        self.player.grab_target = None; self.player.facing = 1.0;
        self.player.hurt_timer = 0; self.player.knockback_vx = 0.0; self.player.knockback_vy = 0.0;
        self.player.combo_count = 0; self.player.combo_timer = 0;
        self.player.attack_cooldown = 0; self.player.jump_kicking = false;
    }

    fn start_story(&mut self, lines: &[&'static str], phase: StoryPhase, next: u8) {
        self.story_lines = lines.to_vec();
        self.story_line_index = 0; self.story_char_index = 0; self.story_char_timer = 0;
        self.story_phase = phase; self.story_next = next;
        self.state = GameState::Story;
    }

    fn spawn_particle(&mut self, x: f32, y: f32, color: Color, count: i32, speed: f32, life: i32) {
        let mut spawned = 0;
        for p in &mut self.particles {
            if !p.active && spawned < count {
                let angle: f32 = rand::gen_range(0.0, std::f32::consts::TAU);
                let spd: f32 = speed * (0.5 + rand::gen_range(0.0, 1.0));
                *p = Particle { x, y, vx: angle.cos()*spd, vy: angle.sin()*spd - 1.0,
                    life: life as f32, max_life: life as f32,
                    color, size: 2.0 + rand::gen_range(0.0, 3.0), active: true };
                spawned += 1;
            }
        }
    }

    fn spawn_hit_spark(&mut self, x: f32, y: f32) {
        self.spawn_particle(x, y, NEON_YELLOW, 5, 3.0, 12);
        self.spawn_particle(x, y, NEON_ORANGE, 3, 2.0, 15);
        self.spawn_particle(x, y, WHITE, 2, 4.0, 8);
    }

    fn spawn_float_text(&mut self, x: f32, y: f32, large: bool) {
        let texts = ["POW", "BAM", "CRACK"];
        let text = texts[rand::gen_range(0, 3) as usize];
        for ft in &mut self.float_texts {
            if !ft.active { *ft = FloatingText { x, y, text, timer: FLOAT_TEXT_DURATION, large, active: true }; return; }
        }
    }
}

// ---------------------------------------------------------------------------
// Stage definitions (synced with web)
// ---------------------------------------------------------------------------
fn build_stages() -> Vec<StageData> {
    vec![
        StageData { name: "BACK ALLEY", length: 3200.0, bg_color: Color::new(0.04,0.04,0.10,1.0),
            waves: vec![
                Wave { trigger_x: 200.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::Thug, count: 3, boss_name: None }] },
                Wave { trigger_x: 700.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::Thug, count: 3, boss_name: None },
                    WaveGroup { kind: EnemyKind::KnifeWielder, count: 1, boss_name: None }] },
                Wave { trigger_x: 1300.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::Thug, count: 2, boss_name: None },
                    WaveGroup { kind: EnemyKind::KnifeWielder, count: 2, boss_name: None }] },
                Wave { trigger_x: 1900.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::Thug, count: 3, boss_name: None },
                    WaveGroup { kind: EnemyKind::Brawler, count: 1, boss_name: None }] },
                Wave { trigger_x: 2500.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::BossBlade, count: 1, boss_name: Some("BLADE") }] },
            ],
            pickups: vec![(PickupKind::Chicken,500.0),(PickupKind::Chicken,1600.0),(PickupKind::Pizza,2200.0)],
            weapons: vec![(WeaponKind::Pipe,1100.0,8)],
        },
        StageData { name: "WAREHOUSE", length: 3600.0, bg_color: Color::new(0.06,0.04,0.04,1.0),
            waves: vec![
                Wave { trigger_x: 200.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::Thug, count: 2, boss_name: None },
                    WaveGroup { kind: EnemyKind::KnifeWielder, count: 2, boss_name: None }] },
                Wave { trigger_x: 800.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::Brawler, count: 1, boss_name: None },
                    WaveGroup { kind: EnemyKind::Thug, count: 2, boss_name: None }] },
                Wave { trigger_x: 1400.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::KnifeWielder, count: 3, boss_name: None },
                    WaveGroup { kind: EnemyKind::Thug, count: 2, boss_name: None }] },
                Wave { trigger_x: 2000.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::Brawler, count: 1, boss_name: None },
                    WaveGroup { kind: EnemyKind::KnifeWielder, count: 2, boss_name: None },
                    WaveGroup { kind: EnemyKind::Thug, count: 1, boss_name: None }] },
                Wave { trigger_x: 2600.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::Thug, count: 3, boss_name: None },
                    WaveGroup { kind: EnemyKind::Brawler, count: 1, boss_name: None }] },
                Wave { trigger_x: 3000.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::BossCrusher, count: 1, boss_name: Some("CRUSHER") }] },
            ],
            pickups: vec![(PickupKind::Chicken,900.0),(PickupKind::Pizza,2100.0),(PickupKind::Chicken,2700.0),(PickupKind::ExtraLife,2800.0)],
            weapons: vec![(WeaponKind::Knife,500.0,12),(WeaponKind::Pipe,1600.0,8)],
        },
        StageData { name: "ROOFTOP SHOWDOWN", length: 4000.0, bg_color: Color::new(0.04,0.04,0.13,1.0),
            waves: vec![
                Wave { trigger_x: 200.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::Thug, count: 3, boss_name: None },
                    WaveGroup { kind: EnemyKind::KnifeWielder, count: 1, boss_name: None }] },
                Wave { trigger_x: 800.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::KnifeWielder, count: 3, boss_name: None },
                    WaveGroup { kind: EnemyKind::Brawler, count: 1, boss_name: None }] },
                Wave { trigger_x: 1400.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::Brawler, count: 2, boss_name: None },
                    WaveGroup { kind: EnemyKind::Thug, count: 2, boss_name: None }] },
                Wave { trigger_x: 2000.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::KnifeWielder, count: 3, boss_name: None },
                    WaveGroup { kind: EnemyKind::Brawler, count: 1, boss_name: None },
                    WaveGroup { kind: EnemyKind::Thug, count: 2, boss_name: None }] },
                Wave { trigger_x: 2800.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::Thug, count: 2, boss_name: None },
                    WaveGroup { kind: EnemyKind::KnifeWielder, count: 2, boss_name: None },
                    WaveGroup { kind: EnemyKind::Brawler, count: 1, boss_name: None }] },
                Wave { trigger_x: 3400.0, triggered: false, boss_intro_shown: false, groups: vec![
                    WaveGroup { kind: EnemyKind::BossDragonKing, count: 1, boss_name: Some("JIN TAKEDA") }] },
            ],
            pickups: vec![(PickupKind::Chicken,900.0),(PickupKind::Pizza,1700.0),(PickupKind::Chicken,2300.0),(PickupKind::ExtraLife,2600.0),(PickupKind::Pizza,3100.0)],
            weapons: vec![(WeaponKind::Pipe,400.0,8),(WeaponKind::Knife,1200.0,12)],
        },
    ]
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------
fn window_conf() -> Conf {
    Conf { window_title: "Dragon Fury: Streets of Vengeance".to_string(), window_width: 800, window_height: 600, window_resizable: false, ..Default::default() }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        game.frame_count = game.frame_count.wrapping_add(1);
        let start_pressed = is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Space);
        let punch_pressed = is_key_pressed(KeyCode::X);
        let jump_pressed = is_key_pressed(KeyCode::Space);
        let special_pressed = is_key_pressed(KeyCode::Z);

        match game.state {
            GameState::Title => {
                game.title_blink += 1;
                if start_pressed {
                    game.player = Player::new();
                    game.score = 0;
                    game.mid_stage_shown = [false; 10];
                    game.init_stage(0);
                    game.start_story(STORY_INTRO, StoryPhase::Intro, 1);
                }
            }
            GameState::Story => {
                game.story_char_timer += 1;
                if game.story_char_timer >= 2 {
                    game.story_char_timer = 0;
                    if game.story_line_index < game.story_lines.len() {
                        let cc = game.story_lines[game.story_line_index].chars().count();
                        if game.story_char_index < cc { game.story_char_index += 1; }
                    }
                }
                if start_pressed {
                    if game.story_line_index < game.story_lines.len() {
                        let cc = game.story_lines[game.story_line_index].chars().count();
                        if game.story_char_index < cc {
                            game.story_char_index = cc;
                        } else {
                            game.story_line_index += 1;
                            game.story_char_index = 0;
                            game.story_char_timer = 0;
                            if game.story_line_index >= game.story_lines.len() {
                                story_complete(&mut game);
                            }
                        }
                    }
                }
            }
            GameState::BossIntro => {
                game.boss_intro_timer += 1;
                if start_pressed && game.boss_intro_timer > 15 {
                    game.state = GameState::Playing;
                    check_wave_spawn(&mut game);
                }
            }
            GameState::StageIntro => {
                game.stage_intro_timer -= 1;
                if game.stage_intro_timer <= 0 { game.state = GameState::Playing; }
            }
            GameState::Playing => {
                if is_key_pressed(KeyCode::Enter) { game.state = GameState::Paused; }
                else { update_playing(&mut game, punch_pressed, jump_pressed, special_pressed); }
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Enter) { game.state = GameState::Playing; }
            }
            GameState::GameOver => {
                game.game_over_timer += 1;
                game.continue_timer -= 1;
                if start_pressed && game.game_over_timer > 30 {
                    // Continue in-place (web behavior)
                    game.player.lives = 2;
                    game.player.hp = game.player.max_hp;
                    game.player.state = PlayerState::Idle;
                    game.player.inv_frames = 120;
                    game.player.hurt_timer = 0;
                    game.player.z = 0.0; game.player.vz = 0.0;
                    game.player.weapon = None; game.player.grab_target = None;
                    game.player.knockback_vx = 0.0; game.player.knockback_vy = 0.0;
                    for e in &mut game.enemies { if !e.dead { e.state = EnemyState::Idle; } }
                    game.state = GameState::Playing;
                }
                if game.continue_timer <= 0 { game.state = GameState::Title; }
            }
            GameState::Victory => {
                if start_pressed { game.state = GameState::Title; }
            }
        }

        // --- Render ---
        clear_background(BG_DARK);
        let shake_x = if game.shake.frames > 0 { rand::gen_range(-game.shake.intensity, game.shake.intensity) } else { 0.0 };
        let shake_y = if game.shake.frames > 0 { rand::gen_range(-game.shake.intensity, game.shake.intensity) } else { 0.0 };

        match game.state {
            GameState::Title => draw_title(&game),
            GameState::Story => draw_story(&game),
            GameState::BossIntro => draw_boss_intro(&game),
            GameState::StageIntro => draw_stage_intro(&game),
            GameState::Playing | GameState::Paused => {
                draw_game(&game, shake_x, shake_y);
                if game.state == GameState::Paused { draw_pause(); }
                if game.mid_stage_dialogue.is_some() { draw_mid_dialogue(&game); }
            }
            GameState::GameOver => { draw_game(&game, shake_x, shake_y); draw_game_over(&game); }
            GameState::Victory => draw_victory(&game),
        }

        // Screen flash
        if game.flash_timer > 0 {
            draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(1.0,1.0,1.0,0.15));
            game.flash_timer -= 1;
        }
        // CRT scanlines
        { let c = Color::new(0.0,0.0,0.0,0.12); let mut sy = 0.0; while sy < SCREEN_H { draw_rectangle(0.0, sy, SCREEN_W, 1.0, c); sy += 4.0; } }

        next_frame().await;
    }
}

fn story_complete(game: &mut Game) {
    match game.story_next {
        1 => { // Intro done -> pre-stage
            let idx = game.stage_index;
            if idx < STORY_PRE_STAGE.len() { game.start_story(STORY_PRE_STAGE[idx], StoryPhase::PreStage, 2); }
            else { game.state = GameState::StageIntro; game.stage_intro_timer = 180; }
        }
        2 => { game.state = GameState::StageIntro; game.stage_intro_timer = 180; }
        3 => { game.start_story(STORY_VICTORY, StoryPhase::VictoryStory, 4); }
        4 => { game.state = GameState::Victory; }
        5 => { // Post-stage done -> next stage pre-stage
            game.stage_index += 1;
            let idx = game.stage_index;
            game.init_stage(idx);
            if idx < STORY_PRE_STAGE.len() { game.start_story(STORY_PRE_STAGE[idx], StoryPhase::PreStage, 2); }
            else { game.state = GameState::StageIntro; game.stage_intro_timer = 180; }
        }
        6 => { game.start_story(STORY_VICTORY, StoryPhase::VictoryStory, 4); }
        _ => { game.state = GameState::Playing; }
    }
}

// ---------------------------------------------------------------------------
// Update: Playing
// ---------------------------------------------------------------------------
fn update_playing(game: &mut Game, punch_pressed: bool, jump_pressed: bool, special_pressed: bool) {
    if game.shake.frames > 0 { game.shake.frames -= 1; }

    // Mid-stage dialogue
    if game.mid_stage_dialogue.is_some() {
        game.mid_stage_timer -= 1;
        if game.mid_stage_timer <= 0 || is_key_pressed(KeyCode::Enter) { game.mid_stage_dialogue = None; }
    }

    update_player(game, punch_pressed, jump_pressed, special_pressed);

    // Update enemies
    let num_enemies = game.enemies.len();
    let px = game.player.x;
    let py = game.player.ground_y;
    let player_dead = game.player.state == PlayerState::Dead;
    for i in 0..num_enemies {
        if game.enemies[i].dead {
            game.enemies[i].death_timer += 1;
            if game.enemies[i].death_timer > 60 { game.enemies[i].to_remove = true; }
            continue;
        }
        // Hurt knockback
        if game.enemies[i].hurt_timer > 0 {
            game.enemies[i].hurt_timer -= 1;
            game.enemies[i].x += game.enemies[i].knockback_vx;
            game.enemies[i].ground_y += game.enemies[i].knockback_vy;
            game.enemies[i].knockback_vx *= 0.85;
            game.enemies[i].knockback_vy *= 0.85;
            game.enemies[i].ground_y = game.enemies[i].ground_y.clamp(GROUND_MIN, GROUND_MAX);
            // Thrown enemy hits others
            if game.enemies[i].state == EnemyState::Thrown {
                let ex = game.enemies[i].x;
                let ey = game.enemies[i].ground_y;
                for j in 0..num_enemies {
                    if i == j || game.enemies[j].dead { continue; }
                    if (ex - game.enemies[j].x).abs() < 30.0 && (ey - game.enemies[j].ground_y).abs() < 20.0 {
                        game.enemies[j].hp -= 30.0;
                        game.enemies[j].hurt_timer = 15;
                        game.enemies[j].knockback_vx = game.enemies[i].knockback_vx * 0.5;
                        let sx = game.enemies[j].x + game.enemies[j].w * 0.5;
                        let sy = game.enemies[j].ground_y - game.enemies[j].h * 0.5;
                        game.spawn_hit_spark(sx, sy);
                        if game.enemies[j].hp <= 0.0 { kill_enemy_at(game, j); }
                    }
                }
            }
            if game.enemies[i].hurt_timer <= 0 {
                if game.enemies[i].hp <= 0.0 { game.enemies[i].dead = true; game.enemies[i].death_timer = 0; game.enemies[i].state = EnemyState::Dead; }
                else { game.enemies[i].state = EnemyState::Idle; }
            }
            continue;
        }
        // Stunned
        if game.enemies[i].state == EnemyState::Stunned {
            game.enemies[i].stun_timer -= 1;
            if game.enemies[i].stun_timer <= 0 { game.enemies[i].state = EnemyState::Idle; game.enemies[i].armor_hits = 0; }
            continue;
        }
        if game.enemies[i].state == EnemyState::Grabbed { continue; }
        // Attack cooldown
        if game.enemies[i].attack_cooldown > 0 { game.enemies[i].attack_cooldown -= 1; }
        // Attacking
        if game.enemies[i].state == EnemyState::Attacking {
            game.enemies[i].attack_timer -= 1;
            if game.enemies[i].attack_timer > 3 && game.enemies[i].attack_timer < 10 && !player_dead {
                // Check hit player
                let hx = if game.enemies[i].facing > 0.0 { game.enemies[i].x + game.enemies[i].w } else { game.enemies[i].x - 35.0 };
                let atk = HitRect { x: hx, y: game.enemies[i].ground_y - game.enemies[i].h, w: 35.0, h: game.enemies[i].h };
                let pr = game.player.body_rect();
                if rects_overlap(&atk, &pr) && (game.enemies[i].ground_y - game.player.ground_y).abs() < DEPTH_MATCH {
                    hurt_player(game, game.enemies[i].damage, game.enemies[i].x);
                }
            }
            if game.enemies[i].attack_timer <= 0 {
                game.enemies[i].state = EnemyState::Idle;
                game.enemies[i].attack_cooldown = (40.0 + rand::gen_range(0.0, 40.0)) as i32;
            }
            continue;
        }
        // AI
        if player_dead { continue; }
        let dx = px - game.enemies[i].x;
        let dy = py - game.enemies[i].ground_y;
        let dist = (dx*dx + dy*dy).sqrt();
        if !dist.is_finite() || dist < 0.01 { continue; }
        game.enemies[i].facing = if dx > 0.0 { 1.0 } else { -1.0 };
        game.enemies[i].anim_frame += 1;

        match game.enemies[i].kind {
            EnemyKind::KnifeWielder => {
                if dist < 45.0 && game.enemies[i].attack_cooldown <= 0 && dy.abs() < 20.0 {
                    game.enemies[i].state = EnemyState::Attacking;
                    game.enemies[i].attack_timer = 14;
                    game.enemies[i].attack_cooldown = (60.0 + rand::gen_range(0.0, 30.0)) as i32;
                } else if dist < 80.0 && dist > 45.0 {
                    let circle_dir: f32 = if (game.frame_count as f32 * 0.02 + game.enemies[i].x).sin() > 0.0 { 1.0 } else { -1.0 };
                    game.enemies[i].ground_y += circle_dir * game.enemies[i].speed * 0.7;
                    game.enemies[i].x += dx.signum() * game.enemies[i].speed * 0.3;
                    game.enemies[i].state = EnemyState::Walking;
                } else {
                    game.enemies[i].x += dx.signum() * game.enemies[i].speed;
                    game.enemies[i].ground_y += dy.signum() * dy.abs().min(game.enemies[i].speed * 0.7);
                    game.enemies[i].state = EnemyState::Walking;
                }
            }
            EnemyKind::Brawler => {
                if dist < 50.0 && game.enemies[i].attack_cooldown <= 0 && dy.abs() < 20.0 {
                    game.enemies[i].state = EnemyState::Attacking; game.enemies[i].attack_timer = 20;
                    game.enemies[i].attack_cooldown = 80;
                } else {
                    game.enemies[i].x += dx.signum() * game.enemies[i].speed;
                    game.enemies[i].ground_y += dy.signum() * dy.abs().min(game.enemies[i].speed * 0.5);
                    game.enemies[i].state = EnemyState::Walking;
                }
            }
            EnemyKind::BossBlade | EnemyKind::BossCrusher | EnemyKind::BossDragonKing => {
                if dist < 50.0 && game.enemies[i].attack_cooldown <= 0 && dy.abs() < 20.0 {
                    game.enemies[i].state = EnemyState::Attacking; game.enemies[i].attack_timer = 16;
                    game.enemies[i].attack_cooldown = (30.0 + rand::gen_range(0.0, 20.0)) as i32;
                } else if dist > 200.0 {
                    game.enemies[i].x += dx.signum() * game.enemies[i].speed * 2.0;
                    game.enemies[i].ground_y += dy.signum() * dy.abs().min(game.enemies[i].speed);
                    game.enemies[i].state = EnemyState::Walking;
                } else {
                    game.enemies[i].x += dx.signum() * game.enemies[i].speed * 1.2;
                    game.enemies[i].ground_y += dy.signum() * dy.abs().min(game.enemies[i].speed * 0.7);
                    game.enemies[i].state = EnemyState::Walking;
                }
            }
            _ => { // Thug
                if dist < 40.0 && game.enemies[i].attack_cooldown <= 0 && dy.abs() < 20.0 {
                    game.enemies[i].state = EnemyState::Attacking; game.enemies[i].attack_timer = 16;
                    game.enemies[i].attack_cooldown = (50.0 + rand::gen_range(0.0, 40.0)) as i32;
                } else {
                    game.enemies[i].x += dx.signum() * game.enemies[i].speed;
                    game.enemies[i].ground_y += dy.signum() * dy.abs().min(game.enemies[i].speed * 0.6);
                    game.enemies[i].state = EnemyState::Walking;
                }
            }
        }
        game.enemies[i].ground_y = game.enemies[i].ground_y.clamp(GROUND_MIN, GROUND_MAX);
        if game.scroll_locked {
            game.enemies[i].x = game.enemies[i].x.clamp(game.camera_x - 20.0, game.camera_x + SCREEN_W + 20.0);
        }
    }
    game.enemies.retain(|e| !e.to_remove);

    check_combat(game);
    check_wave_spawn(game);

    // Wave clear check
    if game.scroll_locked {
        let alive = game.enemies.iter().filter(|e| !e.dead).count();
        if alive == 0 {
            game.wave_clear_timer += 1;
            if game.wave_clear_timer > 30 {
                game.scroll_locked = false;
                game.wave_clear_timer = 0;
                // Check stage complete
                if game.wave_index >= game.stages[game.stage_index].waves.len() {
                    let si = game.stage_index;
                    if si < game.stages.len() - 1 {
                        if si < STORY_POST_STAGE.len() {
                            game.start_story(STORY_POST_STAGE[si], StoryPhase::PostStage, 5);
                        } else { game.stage_index += 1; game.init_stage(game.stage_index); game.state = GameState::StageIntro; game.stage_intro_timer = 180; }
                    } else {
                        if si < STORY_POST_STAGE.len() {
                            game.start_story(STORY_POST_STAGE[si], StoryPhase::PostStage, 6);
                        } else { game.start_story(STORY_VICTORY, StoryPhase::VictoryStory, 4); }
                    }
                }
            }
        } else { game.wave_clear_timer = 0; }
    }

    update_camera(game);

    // Particles
    for p in &mut game.particles {
        if !p.active { continue; }
        p.x += p.vx; p.y += p.vy; p.vy += 0.15;
        p.life -= 1.0;
        if p.life <= 0.0 { p.active = false; }
    }
    // Float texts
    for ft in &mut game.float_texts {
        if !ft.active { continue; }
        ft.y -= 1.2; ft.timer -= 1;
        if ft.timer <= 0 { ft.active = false; }
    }
    // Projectiles
    for pr in &mut game.projectiles {
        if !pr.active { continue; }
        pr.x += pr.vx; pr.life -= 1;
        if pr.life <= 0 { pr.active = false; }
    }
    game.projectiles.retain(|p| p.active);
}

fn update_player(game: &mut Game, punch_pressed: bool, jump_pressed: bool, special_pressed: bool) {
    if game.player.state == PlayerState::Dead { return; }

    // Invincibility
    if game.player.inv_frames > 0 { game.player.inv_frames -= 1; }

    // Hurt state
    if game.player.hurt_timer > 0 {
        game.player.hurt_timer -= 1;
        game.player.x += game.player.knockback_vx;
        game.player.ground_y += game.player.knockback_vy;
        game.player.knockback_vx *= 0.85;
        game.player.knockback_vy *= 0.85;
        game.player.ground_y = game.player.ground_y.clamp(GROUND_MIN, GROUND_MAX);
        if game.player.hurt_timer <= 0 { game.player.state = PlayerState::Idle; }
        return;
    }

    // Combo timer
    if game.player.combo_timer > 0 { game.player.combo_timer -= 1; }
    if game.player.combo_timer <= 0 { game.player.combo_count = 0; }
    // Attack cooldown
    if game.player.attack_cooldown > 0 { game.player.attack_cooldown -= 1; }

    game.player.anim_frame += 1;

    // Grab update
    if game.player.state == PlayerState::Grabbing {
        game.player.grab_timer -= 1;
        if let Some(gi) = game.player.grab_target {
            if gi < game.enemies.len() && !game.enemies[gi].dead {
                game.enemies[gi].x = game.player.x + game.player.facing * 35.0;
                game.enemies[gi].ground_y = game.player.ground_y;
            } else {
                game.player.state = PlayerState::Idle; game.player.grab_target = None; return;
            }
        }
        if game.player.grab_timer <= 0 || game.player.grab_target.is_none() {
            if let Some(gi) = game.player.grab_target {
                if gi < game.enemies.len() { game.enemies[gi].state = EnemyState::Stunned; game.enemies[gi].stun_timer = 60; }
            }
            game.player.state = PlayerState::Idle; game.player.grab_target = None; return;
        }
        // Knee strike
        if punch_pressed && game.player.grab_knees < 3 {
            game.player.grab_knees += 1;
            if let Some(gi) = game.player.grab_target {
                if gi < game.enemies.len() {
                    game.enemies[gi].hp -= 20.0;
                    let sx = game.enemies[gi].x + game.enemies[gi].w * 0.5;
                    let sy = game.enemies[gi].ground_y - game.enemies[gi].h * 0.5;
                    game.spawn_hit_spark(sx, sy);
                    game.score += 50;
                }
            }
            return;
        }
        // Throw
        if jump_pressed {
            if let Some(gi) = game.player.grab_target {
                if gi < game.enemies.len() {
                    game.enemies[gi].state = EnemyState::Thrown;
                    game.enemies[gi].knockback_vx = game.player.facing * 8.0;
                    game.enemies[gi].knockback_vy = 0.0;
                    game.enemies[gi].hurt_timer = 30;
                    game.enemies[gi].hp -= 30.0;
                    game.score += 100;
                }
            }
            game.player.state = PlayerState::Idle; game.player.grab_target = None; return;
        }
        return;
    }

    // Punching
    if game.player.state == PlayerState::Punching {
        game.player.anim_timer -= 1;
        if game.player.anim_timer <= 0 {
            game.player.state = PlayerState::Idle;
            game.player.anim_frame = 0;
        }
        return;
    }

    // Special
    if game.player.state == PlayerState::Special {
        game.player.anim_timer -= 1;
        if game.player.anim_timer <= 0 {
            game.player.state = PlayerState::Idle;
            game.player.inv_frames = 0;
        }
        return;
    }

    // Jump physics
    if game.player.z > 0.0 || game.player.vz > 0.0 {
        game.player.vz -= GRAVITY;
        game.player.z += game.player.vz;

        // Jump kick
        if game.player.state == PlayerState::Jumping && punch_pressed && !game.player.jump_kicking {
            game.player.jump_kicking = true;
            game.player.state = PlayerState::JumpKicking;
        }

        if game.player.z <= 0.0 {
            game.player.z = 0.0; game.player.vz = 0.0;
            game.player.state = PlayerState::Idle;
            game.player.jump_kicking = false;
        }
        // Horizontal movement during jump
        if is_key_down(KeyCode::Left) { game.player.x -= PLAYER_SPEED; }
        if is_key_down(KeyCode::Right) { game.player.x += PLAYER_SPEED; }
        clamp_player(game);
        return;
    }

    // --- Grounded actions ---
    // Special move
    if special_pressed && game.player.hp > SPECIAL_COST && game.player.attack_cooldown <= 0 {
        game.player.state = PlayerState::Special;
        game.player.anim_timer = SPECIAL_FRAMES;
        game.player.hp -= SPECIAL_COST;
        game.player.inv_frames = SPECIAL_FRAMES;
        game.shake = ScreenShake { intensity: SCREEN_SHAKE_HEAVY, frames: 10 };
        let cx = game.player.x + game.player.w * 0.5;
        let cy = game.player.ground_y - game.player.h * 0.5;
        game.spawn_particle(cx, cy, Color::new(1.0,0.0,1.0,1.0), 12, 5.0, 15);
        game.spawn_particle(cx, cy, WHITE, 6, 4.0, 10);
        return;
    }

    // Jump
    if jump_pressed && game.player.state != PlayerState::Jumping {
        game.player.state = PlayerState::Jumping;
        game.player.vz = JUMP_VELOCITY;
        game.player.jump_kicking = false;
        return;
    }

    // Punch
    if punch_pressed && game.player.attack_cooldown <= 0 {
        if game.player.combo_count < 3 && game.player.combo_timer > 0 {
            game.player.combo_count += 1;
        } else { game.player.combo_count = 1; }
        game.player.state = PlayerState::Punching;
        game.player.anim_timer = if game.player.weapon.is_some() { 14 } else { 16 };
        game.player.anim_frame = game.player.combo_count;
        game.player.combo_timer = 24;
        game.player.attack_cooldown = 8;
        return;
    }

    // Movement
    let mut mx: f32 = 0.0;
    let mut my: f32 = 0.0;
    if is_key_down(KeyCode::Left) { mx -= PLAYER_SPEED; game.player.facing = -1.0; }
    if is_key_down(KeyCode::Right) { mx += PLAYER_SPEED; game.player.facing = 1.0; }
    if is_key_down(KeyCode::Up) { my -= PLAYER_DEPTH_SPEED; }
    if is_key_down(KeyCode::Down) { my += PLAYER_DEPTH_SPEED; }

    game.player.state = if mx != 0.0 || my != 0.0 { PlayerState::Walking } else { PlayerState::Idle };
    game.player.x += mx;
    game.player.ground_y = (game.player.ground_y + my).clamp(GROUND_MIN, GROUND_MAX);
    clamp_player(game);

    // Pick up weapons
    for i in 0..game.ground_weapons.len() {
        if !game.ground_weapons[i].active { continue; }
        if (game.player.x + game.player.w * 0.5 - game.ground_weapons[i].x).abs() < 25.0
            && (game.player.ground_y - game.ground_weapons[i].y).abs() < 20.0
            && game.player.weapon.is_none()
        {
            game.player.weapon = Some(game.ground_weapons[i].kind);
            game.player.weapon_durability = game.ground_weapons[i].durability;
            game.ground_weapons[i].active = false;
        }
    }

    // Pick up items
    for i in 0..game.pickups.len() {
        if !game.pickups[i].active { continue; }
        if (game.player.x + game.player.w * 0.5 - game.pickups[i].x).abs() < 25.0
            && (game.player.ground_y - game.pickups[i].y).abs() < 20.0
        {
            match game.pickups[i].kind {
                PickupKind::Chicken => { game.player.hp = (game.player.hp + 30.0).min(game.player.max_hp); game.score += 500; }
                PickupKind::Pizza => { game.player.hp = (game.player.hp + 60.0).min(game.player.max_hp); game.score += 500; }
                PickupKind::ExtraLife => { game.player.lives += 1; game.score += 1000; }
            }
            let px = game.pickups[i].x;
            let py = game.pickups[i].y - 10.0;
            game.pickups[i].active = false;
            game.spawn_particle(px, py, WHITE, 6, 3.0, 15);
        }
    }

    // Try grab stunned enemy
    for ei in 0..game.enemies.len() {
        if game.enemies[ei].state == EnemyState::Stunned && game.enemies[ei].can_grab && !game.enemies[ei].dead {
            if (game.player.x + game.player.w * 0.5 - (game.enemies[ei].x + game.enemies[ei].w * 0.5)).abs() < 30.0
                && (game.player.ground_y - game.enemies[ei].ground_y).abs() < 20.0
            {
                game.player.state = PlayerState::Grabbing;
                game.player.grab_target = Some(ei);
                game.player.grab_timer = 90;
                game.player.grab_knees = 0;
                game.enemies[ei].state = EnemyState::Grabbed;
                game.player.facing = if game.enemies[ei].x > game.player.x { 1.0 } else { -1.0 };
                break;
            }
        }
    }
}

fn clamp_player(game: &mut Game) {
    if game.scroll_locked {
        game.player.x = game.player.x.clamp(game.camera_x, game.camera_x + SCREEN_W - game.player.w);
    } else {
        game.player.x = game.player.x.clamp(game.camera_x - 20.0, game.stages[game.stage_index].length - game.player.w);
    }
}

fn hurt_player(game: &mut Game, damage: f32, from_x: f32) {
    if game.player.is_invincible() || game.player.state == PlayerState::Dead { return; }
    game.player.hp -= damage;
    game.player.state = PlayerState::Hurt;
    game.player.hurt_timer = 20;
    game.player.knockback_vx = if from_x < game.player.x { 4.0 } else { -4.0 };
    game.player.knockback_vy = 0.0;
    if game.player.grab_target.is_some() {
        if let Some(gi) = game.player.grab_target {
            if gi < game.enemies.len() { game.enemies[gi].state = EnemyState::Idle; }
        }
        game.player.grab_target = None;
    }
    game.shake = ScreenShake { intensity: 3.0, frames: 8 };
    let cx = game.player.x + game.player.w * 0.5;
    let cy = game.player.ground_y - game.player.h * 0.5;
    game.spawn_particle(cx, cy, Color::new(1.0,0.0,0.0,1.0), 4, 2.0, 10);

    if game.player.hp <= 0.0 {
        game.player.hp = 0.0;
        game.player.lives -= 1;
        if game.player.lives >= 0 {
            game.player.hp = game.player.max_hp;
            game.player.inv_frames = 120;
            game.player.state = PlayerState::Idle;
            game.player.hurt_timer = 0;
            game.player.weapon = None;
            game.player.z = 0.0; game.player.vz = 0.0;
        } else {
            game.player.state = PlayerState::Dead;
            game.state = GameState::GameOver;
            game.game_over_timer = 0;
            game.continue_timer = CONTINUE_FRAMES;
        }
    }
}

fn kill_enemy_at(game: &mut Game, idx: usize) {
    if game.enemies[idx].dead { return; }
    game.enemies[idx].dead = true;
    game.enemies[idx].death_timer = 0;
    game.enemies[idx].state = EnemyState::Dead;
    game.score += game.enemies[idx].score_value;
    let cx = game.enemies[idx].x + game.enemies[idx].w * 0.5;
    let cy = game.enemies[idx].ground_y - game.enemies[idx].h * 0.5;
    game.spawn_particle(cx, cy, NEON_ORANGE, 8, 3.0, 20);
    game.spawn_particle(cx, cy, NEON_YELLOW, 5, 2.0, 15);
    // Drop food 12% chance
    if rand::gen_range(0.0_f32, 1.0) < 0.12 {
        game.pickups.push(Pickup { x: cx, y: game.enemies[idx].ground_y, kind: PickupKind::Chicken, active: true });
    }
    // Knife wielders drop knives 50%
    if game.enemies[idx].kind == EnemyKind::KnifeWielder && rand::gen_range(0.0_f32, 1.0) < 0.5 {
        game.ground_weapons.push(GroundWeapon { x: cx, y: game.enemies[idx].ground_y, kind: WeaponKind::Knife, durability: 10, active: true });
    }
}

fn check_combat(game: &mut Game) {
    // Player attack hitbox
    let hitbox = if game.player.state == PlayerState::Punching && game.player.anim_timer > 4 && game.player.anim_timer < 12 {
        let hx = if game.player.facing > 0.0 { game.player.x + game.player.w } else { game.player.x - 40.0 };
        Some(HitRect { x: hx, y: game.player.ground_y - game.player.h, w: 40.0, h: game.player.h })
    } else if game.player.state == PlayerState::Special && game.player.anim_timer > 5 && game.player.anim_timer < 20 {
        Some(HitRect { x: game.player.x - 50.0, y: game.player.ground_y - 60.0, w: game.player.w + 100.0, h: 70.0 })
    } else if game.player.state == PlayerState::JumpKicking {
        let hx = if game.player.facing > 0.0 { game.player.x + game.player.w } else { game.player.x - 35.0 };
        Some(HitRect { x: hx, y: game.player.ground_y - game.player.z - game.player.h, w: 35.0, h: game.player.h })
    } else { None };

    if let Some(ref hb) = hitbox {
        let num = game.enemies.len();
        for ei in 0..num {
            if game.enemies[ei].dead || game.enemies[ei].hurt_timer > 0 || game.enemies[ei].state == EnemyState::Grabbed { continue; }
            if (game.player.ground_y - game.enemies[ei].ground_y).abs() > DEPTH_MATCH { continue; }
            let er = game.enemies[ei].body_rect();
            if !rects_overlap(hb, &er) { continue; }

            let mut dmg = game.player.attack_damage();
            if game.player.weapon.is_some() {
                dmg = game.player.attack_damage();
                game.player.weapon_durability -= 1;
                if game.player.weapon_durability <= 0 {
                    let px = game.player.x + game.player.w * 0.5;
                    let py = game.player.ground_y - 20.0;
                    game.spawn_particle(px, py, Color::new(0.6,0.6,0.6,1.0), 8, 4.0, 15);
                    game.player.weapon = None;
                }
            }

            game.enemies[ei].hp -= dmg;
            game.enemies[ei].hurt_timer = 12;
            game.enemies[ei].knockback_vx = game.player.facing * if game.player.state == PlayerState::JumpKicking { 6.0 } else { 3.0 };
            game.enemies[ei].knockback_vy = 0.0;
            game.enemies[ei].state = EnemyState::Stunned; // temp, overridden below

            let sx = game.enemies[ei].x + game.enemies[ei].w * 0.5;
            let sy = game.enemies[ei].ground_y - game.enemies[ei].h * 0.5;
            game.spawn_hit_spark(sx, sy);
            game.spawn_float_text(sx, sy, game.player.combo_count >= 3);

            if game.player.combo_count >= 3 || game.player.state == PlayerState::Special { game.flash_timer = 4; }

            // Armor system
            if game.enemies[ei].armor > 0 {
                game.enemies[ei].armor_hits += 1;
                if game.enemies[ei].armor_hits >= game.enemies[ei].armor {
                    game.enemies[ei].state = EnemyState::Stunned;
                    game.enemies[ei].stun_timer = 90;
                    game.enemies[ei].armor_hits = 0;
                } else {
                    game.enemies[ei].state = EnemyState::Idle; // not stunned yet
                }
            } else {
                if game.player.combo_count >= 3 || game.player.state == PlayerState::JumpKicking {
                    game.enemies[ei].state = EnemyState::Stunned;
                    game.enemies[ei].stun_timer = 60;
                    game.enemies[ei].knockback_vx = game.player.facing * 5.0;
                } else {
                    game.enemies[ei].state = EnemyState::Idle; // just hurt, not stunned
                }
            }

            if game.enemies[ei].hp <= 0.0 { kill_enemy_at(game, ei); }

            game.shake = ScreenShake { intensity: 2.0, frames: 4 };
            game.score += 10;

            if game.player.state != PlayerState::Special { break; }
        }
    }

    // Enemy attacks hitting player (handled in enemy update above)
}

fn check_wave_spawn(game: &mut Game) {
    let si = game.stage_index;
    if game.wave_index >= game.stages[si].waves.len() { return; }

    let wave_trigger = game.stages[si].waves[game.wave_index].trigger_x;
    if game.camera_x + SCREEN_W * 0.4 >= wave_trigger && !game.scroll_locked {
        // Boss wave check
        let is_boss_wave = game.stages[si].waves[game.wave_index].groups.iter().any(|g|
            matches!(g.kind, EnemyKind::BossBlade | EnemyKind::BossCrusher | EnemyKind::BossDragonKing));

        if is_boss_wave && !game.stages[si].waves[game.wave_index].boss_intro_shown {
            game.stages[si].waves[game.wave_index].boss_intro_shown = true;
            game.boss_intro_timer = 0;
            game.state = GameState::BossIntro;
            return;
        }

        // Spawn wave
        game.scroll_locked = true;
        let groups: Vec<(EnemyKind, usize, Option<&'static str>)> = game.stages[si].waves[game.wave_index].groups.iter()
            .map(|g| (g.kind, g.count, g.boss_name)).collect();
        game.stages[si].waves[game.wave_index].triggered = true;

        for (kind, count, _bname) in groups {
            for _ in 0..count {
                let side: f32 = if rand::gen_range(0.0_f32, 1.0) < 0.5 { -1.0 } else { 1.0 };
                let ex = if side < 0.0 { game.camera_x - 30.0 - rand::gen_range(0.0, 40.0) }
                         else { game.camera_x + SCREEN_W + 30.0 + rand::gen_range(0.0, 40.0) };
                let ey = GROUND_MIN + rand::gen_range(0.0, GROUND_MAX - GROUND_MIN);
                let mut e = Enemy::new(kind, ex, ey);
                e.facing = if side < 0.0 { 1.0 } else { -1.0 };
                game.enemies.push(e);
            }
        }
        game.wave_index += 1;

        // Mid-stage dialogue after wave 3
        let mid_key = si * 3 + game.wave_index;
        if game.wave_index == 3 && mid_key < 10 && !game.mid_stage_shown[mid_key] && si < STORY_MID_STAGE.len() {
            game.mid_stage_shown[mid_key] = true;
            game.mid_stage_dialogue = Some(STORY_MID_STAGE[si]);
            game.mid_stage_timer = 240;
        }
    }
}

fn update_camera(game: &mut Game) {
    if game.scroll_locked { return; }
    let target = game.player.x - SCREEN_W * 0.35;
    let max_cam = (game.stages[game.stage_index].length - SCREEN_W).max(0.0);
    game.camera_x += (target - game.camera_x) * 0.08;
    game.camera_x = game.camera_x.clamp(0.0, max_cam);
}

// ---------------------------------------------------------------------------
// Drawing
// ---------------------------------------------------------------------------
fn draw_game(game: &Game, sx: f32, sy: f32) {
    let cam = game.camera_x + sx;

    // Background
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, game.stages[game.stage_index].bg_color);

    // Far buildings
    let far_p = 0.15;
    for i in 0..20 {
        let bx = i as f32 * 160.0 - (cam * far_p) % 160.0;
        let bh = 80.0 + ((i * 37) % 60) as f32;
        draw_rectangle(bx, GROUND_MIN - 20.0 - bh + sy, 140.0, bh, Color::new(0.07,0.07,0.16,1.0));
        // Windows
        for wy in (0..(bh as i32 - 20)).step_by(18) {
            for wx in (10..130).step_by(25) {
                if (i as f32 * 3.0 + wx as f32 + wy as f32).sin() > 0.2 {
                    draw_rectangle(bx + wx as f32, GROUND_MIN - 20.0 - bh + 10.0 + wy as f32 + sy, 8.0, 8.0, Color::new(1.0,1.0,0.27,0.5));
                }
            }
        }
    }

    // Ground
    draw_rectangle(0.0, GROUND_MIN - 20.0 + sy, SCREEN_W, SCREEN_H - GROUND_MIN + 20.0, Color::new(0.13,0.13,0.13,1.0));
    for gy in (GROUND_MIN as i32..GROUND_MAX as i32 + 20).step_by(20) {
        draw_line(0.0, gy as f32 + sy, SCREEN_W, gy as f32 + sy, 1.0, Color::new(1.0,1.0,1.0,0.05));
    }

    // Collect drawables sorted by ground_y
    struct Drawable { gy: f32, kind: u8, idx: usize }
    let mut drawables: Vec<Drawable> = Vec::new();
    if game.player.state != PlayerState::Dead {
        drawables.push(Drawable { gy: game.player.ground_y, kind: 0, idx: 0 });
    }
    for (i, e) in game.enemies.iter().enumerate() {
        if !e.to_remove { drawables.push(Drawable { gy: e.ground_y, kind: 1, idx: i }); }
    }
    for (i, w) in game.ground_weapons.iter().enumerate() {
        if w.active { drawables.push(Drawable { gy: w.y, kind: 2, idx: i }); }
    }
    for (i, p) in game.pickups.iter().enumerate() {
        if p.active { drawables.push(Drawable { gy: p.y, kind: 3, idx: i }); }
    }
    drawables.sort_by(|a, b| a.gy.partial_cmp(&b.gy).unwrap());

    for d in &drawables {
        match d.kind {
            0 => draw_player_sprite(game, cam, sy),
            1 => draw_enemy_sprite(&game.enemies[d.idx], cam, sy, game.frame_count),
            2 => draw_weapon_ground(&game.ground_weapons[d.idx], cam, sy, game.frame_count),
            3 => draw_pickup_item(&game.pickups[d.idx], cam, sy, game.frame_count),
            _ => {}
        }
    }

    // Particles
    for p in &game.particles {
        if !p.active { continue; }
        let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
        draw_rectangle(p.x - cam, p.y + sy, p.size, p.size, Color::new(p.color.r, p.color.g, p.color.b, alpha));
    }

    // Float texts
    for ft in &game.float_texts {
        if !ft.active { continue; }
        let alpha = (ft.timer as f32 / FLOAT_TEXT_DURATION as f32).clamp(0.0, 1.0);
        let fsz = if ft.large { 20.0 } else { 14.0 };
        draw_text(ft.text, ft.x - cam + 1.0, ft.y + sy + 1.0, fsz, Color::new(0.0,0.0,0.0,alpha*0.6));
        draw_text(ft.text, ft.x - cam, ft.y + sy, fsz, Color::new(1.0,1.0,0.0,alpha));
    }

    draw_hud(game);

    // GO arrow
    if !game.scroll_locked && game.enemies.is_empty() && game.wave_index < game.stages[game.stage_index].waves.len() {
        if (game.frame_count / 20) % 2 == 0 {
            let txt = "GO >>>";
            let tw = measure_text(txt, None, 24, 1.0).width;
            draw_text(txt, SCREEN_W - tw - 20.0, SCREEN_H * 0.5, 24.0, NEON_YELLOW);
        }
    }
}

fn draw_player_sprite(game: &Game, cam: f32, sy: f32) {
    let p = &game.player;
    let x = p.x + p.w * 0.5 - cam;
    let y = p.ground_y + sy;

    // Shadow
    draw_ellipse(x, y + 2.0, p.w * 0.6, 6.0, 0.0, Color::new(0.0,0.0,0.0,0.35));

    // Inv blink
    if p.inv_frames > 0 && p.inv_frames % 4 < 2 { return; }
    if p.hurt_timer > 0 && game.frame_count % 4 < 2 { return; }

    let by = y - p.z; // body base
    let flip = p.facing < 0.0;
    let fx: f32 = if flip { -1.0 } else { 1.0 };

    // Legs
    let leg_off = if p.state == PlayerState::Walking { (p.anim_frame as f32 * 0.3).sin() * 4.0 } else { 0.0 };
    draw_rectangle(x - 8.0, by - 14.0 - leg_off, 7.0, 12.0 + leg_off, Color::new(0.2,0.2,0.4,1.0));
    draw_rectangle(x + 1.0, by - 14.0 + leg_off, 7.0, 12.0 - leg_off, Color::new(0.2,0.2,0.4,1.0));
    // Boots
    draw_rectangle(x - 9.0, by - 4.0, 9.0, 4.0, Color::new(0.27,0.2,0.13,1.0));
    draw_rectangle(x, by - 4.0, 9.0, 4.0, Color::new(0.27,0.2,0.13,1.0));

    // Body
    draw_rectangle(x - 10.0, by - 32.0, 20.0, 16.0, PLAYER_BLUE);
    // Belt
    draw_rectangle(x - 10.0, by - 18.0, 20.0, 3.0, Color::new(0.53,0.27,0.13,1.0));

    // Arms
    if p.state == PlayerState::Punching {
        let ext = if p.anim_frame < 2 { 8.0 } else if p.anim_frame < 4 { 14.0 } else { 18.0 };
        draw_rectangle(x + 10.0 * fx, by - 30.0, ext * fx, 6.0, SKIN_COLOR);
        draw_rectangle(x + (10.0 + ext - 4.0) * fx, by - 31.0, 6.0 * fx, 8.0, WHITE);
        draw_rectangle(x - 10.0 * fx, by - 30.0, 6.0, 10.0, SKIN_COLOR);
    } else if p.state == PlayerState::Special {
        draw_rectangle(x + 10.0, by - 32.0, 12.0, 6.0, SKIN_COLOR);
        draw_rectangle(x - 20.0, by - 32.0, 12.0, 6.0, SKIN_COLOR);
    } else {
        draw_rectangle(x + 8.0 * fx, by - 30.0, 6.0, 12.0, SKIN_COLOR);
        draw_rectangle(x - 12.0 * fx, by - 30.0, 6.0, 12.0, SKIN_COLOR);
        draw_rectangle(x + 8.0 * fx, by - 20.0, 6.0, 5.0, WHITE);
        draw_rectangle(x - 12.0 * fx, by - 20.0, 6.0, 5.0, WHITE);
    }

    // Head
    draw_rectangle(x - 8.0, by - 42.0, 16.0, 10.0, SKIN_COLOR);
    // Hair
    draw_rectangle(x - 10.0, by - 46.0, 20.0, 6.0, PLAYER_HAIR);
    // Eyes
    draw_rectangle(x - 5.0 + fx * 3.0, by - 39.0, 3.0, 3.0, BLACK);
    draw_rectangle(x + 2.0 + fx * 3.0, by - 39.0, 3.0, 3.0, BLACK);

    // Weapon
    if let Some(wk) = p.weapon {
        let wx = if flip { x - 12.0 } else { x + p.w * 0.5 };
        let wy = by - 30.0;
        match wk {
            WeaponKind::Pipe => draw_rectangle(wx, wy, if flip { -20.0 } else { 20.0 }, 4.0, GRAY),
            WeaponKind::Knife => draw_rectangle(wx, wy, if flip { -14.0 } else { 14.0 }, 3.0, Color::new(0.8,0.8,0.8,1.0)),
            WeaponKind::Bottle => draw_rectangle(wx, wy, 6.0, 10.0, Color::new(0.2,0.4,0.2,1.0)),
        }
    }
}

fn draw_enemy_sprite(e: &Enemy, cam: f32, sy: f32, fc: u64) {
    let x = e.x + e.w * 0.5 - cam;
    let y = e.ground_y + sy;

    // Shadow
    draw_ellipse(x, y + 2.0, e.w * 0.6, 6.0, 0.0, Color::new(0.0,0.0,0.0,0.35));

    // Death fade
    if e.dead {
        if e.death_timer > 30 { return; }
        let alpha = 1.0 - e.death_timer as f32 / 30.0;
        if fc % 4 < 2 {
            let bc = enemy_body_color(e.kind);
            draw_rectangle(x - e.w * 0.5, y - 8.0, e.w, 8.0, Color::new(bc.r, bc.g, bc.b, alpha));
        }
        return;
    }

    let flip = e.facing < 0.0;
    let fx: f32 = if flip { -1.0 } else { 1.0 };
    let is_big = matches!(e.kind, EnemyKind::Brawler | EnemyKind::BossBlade | EnemyKind::BossCrusher | EnemyKind::BossDragonKing);
    let sc = if is_big { 1.3 } else { 1.0 };
    let bc = enemy_body_color(e.kind);

    // Legs
    let lo = if e.state == EnemyState::Walking { (e.anim_frame as f32 * 0.25).sin() * 3.0 } else { 0.0 };
    draw_rectangle((x - 7.0) * 1.0, y - 14.0 * sc, 7.0 * sc, (10.0 + lo) * sc, Color::new(0.2,0.2,0.3,1.0));
    draw_rectangle((x + 1.0) * 1.0, y - 14.0 * sc, 7.0 * sc, (10.0 - lo) * sc, Color::new(0.2,0.2,0.3,1.0));

    // Body
    draw_rectangle(x - 11.0 * sc, y - 30.0 * sc, 22.0 * sc, 16.0 * sc, bc);

    // Arms
    if e.state == EnemyState::Attacking {
        draw_rectangle(x + 11.0 * sc * fx, y - 28.0 * sc, 14.0 * sc * fx, 6.0 * sc, SKIN_COLOR);
        draw_rectangle(x + (11.0 + 14.0 - 4.0) * sc * fx, y - 29.0 * sc, 6.0 * sc, 8.0 * sc, WHITE);
        draw_rectangle(x - 11.0 * sc * fx, y - 28.0 * sc, 6.0 * sc, 10.0 * sc, SKIN_COLOR);
    } else {
        draw_rectangle(x + 9.0 * sc * fx, y - 28.0 * sc, 6.0 * sc, 12.0 * sc, SKIN_COLOR);
        draw_rectangle(x - 11.0 * sc * fx, y - 28.0 * sc, 6.0 * sc, 12.0 * sc, SKIN_COLOR);
    }

    // Head
    draw_rectangle(x - 8.0 * sc, y - 44.0 * sc, 16.0 * sc, 12.0 * sc, SKIN_COLOR);
    // Hair/helmet
    let hair_color = match e.kind {
        EnemyKind::KnifeWielder => BLACK,
        EnemyKind::Brawler => Color::new(0.3,0.3,0.3,1.0),
        _ => Color::new(0.53,0.27,0.13,1.0),
    };
    draw_rectangle(x - 9.0 * sc, y - 48.0 * sc, 18.0 * sc, 5.0 * sc, hair_color);
    // Eyes
    draw_rectangle(x - 5.0 * sc + fx * 2.0 * sc, y - 40.0 * sc, 3.0 * sc, 3.0 * sc, BLACK);
    draw_rectangle(x + 2.0 * sc + fx * 2.0 * sc, y - 40.0 * sc, 3.0 * sc, 3.0 * sc, BLACK);

    // Boss scar
    if e.is_boss() {
        draw_rectangle(x - 8.0 * sc, y - 50.0 * sc, 16.0 * sc, 4.0 * sc, BOSS_GOLD);
    }

    // Knife for knife enemies
    if matches!(e.kind, EnemyKind::KnifeWielder | EnemyKind::BossBlade) && e.state == EnemyState::Attacking {
        draw_rectangle(x + 20.0 * sc * fx, y - 28.0 * sc, 10.0 * sc * fx, 3.0 * sc, Color::new(0.8,0.8,0.8,1.0));
    }

    // Stunned stars
    if e.state == EnemyState::Stunned {
        for s in 0..3 {
            let angle = e.anim_frame as f32 * 0.15 + s as f32 * 2.1;
            let star_x = x + angle.sin() * 14.0;
            let star_y = y - 50.0 * sc + angle.cos() * 5.0;
            draw_rectangle(star_x, star_y, 3.0, 3.0, NEON_YELLOW);
        }
    }
}

fn enemy_body_color(kind: EnemyKind) -> Color {
    match kind {
        EnemyKind::Thug => THUG_RED,
        EnemyKind::KnifeWielder => KNIFE_PURPLE,
        EnemyKind::Brawler => BRAWLER_GREEN,
        EnemyKind::BossBlade => Color::new(1.0,0.4,0.0,1.0),
        EnemyKind::BossCrusher => BRAWLER_GREEN,
        EnemyKind::BossDragonKing => Color::new(0.6,0.1,0.1,1.0),
    }
}

fn draw_weapon_ground(w: &GroundWeapon, cam: f32, sy: f32, fc: u64) {
    let x = w.x - cam;
    let y = w.y - 6.0 + sy;
    match w.kind {
        WeaponKind::Pipe => { draw_rectangle(x, y, 24.0, 5.0, GRAY); draw_rectangle(x, y, 4.0, 5.0, Color::new(0.47,0.47,0.47,1.0)); }
        WeaponKind::Knife => { draw_rectangle(x, y, 16.0, 3.0, Color::new(0.8,0.8,0.8,1.0)); draw_rectangle(x, y - 1.0, 5.0, 5.0, Color::new(0.53,0.27,0.13,1.0)); }
        WeaponKind::Bottle => { draw_rectangle(x, y, 8.0, 12.0, Color::new(0.2,0.4,0.2,0.8)); draw_rectangle(x + 2.0, y - 4.0, 4.0, 4.0, Color::new(0.2,0.4,0.2,0.8)); }
    }
    if fc % 60 < 10 { draw_rectangle(x + 8.0, y - 2.0, 3.0, 2.0, Color::new(1.0,1.0,1.0,0.6)); }
}

fn draw_pickup_item(p: &Pickup, cam: f32, sy: f32, fc: u64) {
    let x = p.x - cam;
    let y = p.y - 14.0 + sy;
    let bob = (fc as f32 * 0.08).sin() * 3.0;
    match p.kind {
        PickupKind::Chicken => {
            draw_rectangle(x + 2.0, y + bob, 12.0, 8.0, Color::new(0.8,0.53,0.27,1.0));
            draw_rectangle(x + 4.0, y + bob + 1.0, 8.0, 6.0, Color::new(1.0,0.67,0.4,1.0));
            draw_rectangle(x + 12.0, y + bob + 2.0, 5.0, 4.0, WHITE);
        }
        PickupKind::Pizza => {
            draw_triangle(vec2(x+8.0, y+bob), vec2(x+18.0, y+bob+12.0), vec2(x-2.0, y+bob+12.0), Color::new(1.0,0.8,0.2,1.0));
            draw_rectangle(x + 3.0, y + bob + 5.0, 3.0, 3.0, Color::new(0.8,0.2,0.0,1.0));
        }
        PickupKind::ExtraLife => {
            draw_circle(x + 6.0, y + bob + 6.0, 8.0, NEON_YELLOW);
            draw_text("1UP", x - 3.0, y + bob + 10.0, 14.0, BLACK);
        }
    }
    draw_circle(x + 8.0, y + bob + 6.0, 12.0, Color::new(1.0,1.0,0.4,0.1));
}

fn draw_hud(game: &Game) {
    let p = &game.player;
    draw_text("DRAGON", 15.0, 25.0, 18.0, WHITE);
    // HP bar
    draw_rectangle(15.0, 32.0, 150.0, 12.0, Color::new(0.2,0.2,0.2,1.0));
    let frac = (p.hp / p.max_hp).clamp(0.0, 1.0);
    let hpc = if frac > 0.5 { Color::new(0.0,1.0,0.0,1.0) } else if frac > 0.25 { NEON_YELLOW } else { RED };
    draw_rectangle(15.0, 32.0, 150.0 * frac, 12.0, hpc);
    draw_rectangle_lines(15.0, 32.0, 150.0, 12.0, 1.0, WHITE);

    // Score
    let stxt = format!("SCORE {:07}", game.score);
    let tw = measure_text(&stxt, None, 18, 1.0).width;
    draw_text(&stxt, SCREEN_W - tw - 15.0, 25.0, 18.0, NEON_CYAN);

    // Lives
    for i in 0..p.lives {
        draw_rectangle(15.0 + i as f32 * 18.0, 50.0, 12.0, 12.0, PLAYER_BLUE);
        draw_rectangle(18.0 + i as f32 * 18.0, 50.0, 6.0, 4.0, PLAYER_HAIR);
    }

    // Weapon
    if let Some(wk) = p.weapon {
        let name = match wk { WeaponKind::Pipe => "PIPE", WeaponKind::Knife => "KNIFE", WeaponKind::Bottle => "BOTTLE" };
        let txt = format!("{} x{}", name, p.weapon_durability);
        let tw2 = measure_text(&txt, None, 14, 1.0).width;
        draw_text(&txt, SCREEN_W - tw2 - 15.0, 50.0, 14.0, WHITE);
    }

    // Boss HP
    for e in &game.enemies {
        if e.boss_name.is_some() && !e.dead {
            let bname = e.boss_name.unwrap();
            let bw: f32 = 200.0;
            let bx = SCREEN_W * 0.5 - bw * 0.5;
            let bnw = measure_text(bname, None, 16, 1.0).width;
            draw_text(bname, (SCREEN_W - bnw) * 0.5, SCREEN_H - 50.0, 16.0, WHITE);
            draw_rectangle(bx, SCREEN_H - 42.0, bw, 10.0, Color::new(0.2,0.2,0.2,1.0));
            let bf = (e.hp / e.max_hp).clamp(0.0, 1.0);
            draw_rectangle(bx, SCREEN_H - 42.0, bw * bf, 10.0, NEON_PINK);
            draw_rectangle_lines(bx, SCREEN_H - 42.0, bw, 10.0, 1.0, WHITE);
            break;
        }
    }
}

fn draw_title(game: &Game) {
    draw_rectangle(0.0,0.0,SCREEN_W,SCREEN_H,Color::new(0.04,0.04,0.10,1.0));
    // Skyline
    for i in 0..20 { let bx = i as f32 * 45.0; let bh = 60.0 + ((i*37)%80) as f32; draw_rectangle(bx, SCREEN_H - 150.0 - bh, 38.0, bh + 150.0, Color::new(0.07,0.07,0.16,1.0)); }
    draw_rectangle(0.0, SCREEN_H - 150.0, SCREEN_W, 150.0, Color::new(0.1,0.1,0.18,1.0));

    let t1 = "DRAGON"; let t2 = "FURY";
    let t1w = measure_text(t1, None, 48, 1.0).width;
    let t2w = measure_text(t2, None, 48, 1.0).width;
    draw_text(t1, (SCREEN_W-t1w)*0.5, 175.0, 48.0, NEON_PINK);
    draw_text(t2, (SCREEN_W-t2w)*0.5, 230.0, 48.0, NEON_ORANGE);

    let sub = "STREETS OF VENGEANCE";
    let sw = measure_text(sub, None, 16, 1.0).width;
    draw_text(sub, (SCREEN_W-sw)*0.5, 262.0, 16.0, NEON_CYAN);
    let tag = "A Dragon Fist Story";
    let tagw = measure_text(tag, None, 14, 1.0).width;
    draw_text(tag, (SCREEN_W-tagw)*0.5, 285.0, 14.0, NEON_PINK);

    // Character preview
    draw_rectangle(SCREEN_W*0.5-10.0, 320.0, 20.0, 24.0, PLAYER_BLUE);
    draw_rectangle(SCREEN_W*0.5-6.0, 308.0, 12.0, 12.0, SKIN_COLOR);
    draw_rectangle(SCREEN_W*0.5-7.0, 304.0, 14.0, 6.0, PLAYER_HAIR);

    if (game.title_blink / 30) % 2 == 0 {
        let ps = "PRESS START";
        let pw = measure_text(ps, None, 22, 1.0).width;
        draw_text(ps, (SCREEN_W-pw)*0.5, 510.0, 22.0, WHITE);
    }
    let ctrl = "Arrows:Move  X:Punch  Space:Jump  Z:Special";
    let cw = measure_text(ctrl, None, 12, 1.0).width;
    draw_text(ctrl, (SCREEN_W-cw)*0.5, 545.0, 12.0, Color::new(0.5,0.5,0.5,1.0));
}

fn draw_story(game: &Game) {
    draw_rectangle(0.0,0.0,SCREEN_W,SCREEN_H,BLACK);
    let header = match game.story_phase {
        StoryPhase::Intro => "STREETS OF VENGEANCE".to_string(),
        StoryPhase::PreStage => format!("STAGE {} -- {}", game.stage_index+1, game.stages[game.stage_index].name),
        StoryPhase::PostStage => format!("STAGE {} CLEAR", game.stage_index+1),
        StoryPhase::VictoryStory => "EPILOGUE".to_string(),
    };
    let hw = measure_text(&header, None, 16, 1.0).width;
    draw_text(&header, (SCREEN_W-hw)*0.5, 40.0, 16.0, NEON_PINK);
    draw_line(SCREEN_W*0.15, 55.0, SCREEN_W*0.85, 55.0, 1.0, Color::new(1.0,0.0,0.4,0.4));

    let line_h = 24.0_f32;
    let start_y = 100.0_f32;
    let max_w = SCREEN_W - 120.0;
    let fsz = 14.0_f32;
    let mut y_off = 0.0_f32;

    for i in 0..=game.story_line_index {
        if i >= game.story_lines.len() { break; }
        let text = if i < game.story_line_index { game.story_lines[i].to_string() }
                   else { game.story_lines[i].chars().take(game.story_char_index).collect::<String>() };
        let color = if i == game.story_line_index { WHITE } else { Color::new(0.8,0.8,0.8,0.7) };
        let wrapped = wrap_text(&text, fsz, max_w);
        for wl in &wrapped { draw_text(wl, 50.0, start_y + y_off, fsz, color); y_off += line_h; }
        y_off += 12.0;
    }

    if game.story_line_index < game.story_lines.len() {
        let cc = game.story_lines[game.story_line_index].chars().count();
        if game.story_char_index >= cc && (game.frame_count / 20) % 2 == 0 {
            let h = "PRESS START TO CONTINUE";
            let hw2 = measure_text(h, None, 14, 1.0).width;
            draw_text(h, (SCREEN_W-hw2)*0.5, SCREEN_H - 30.0, 14.0, Color::new(0.5,0.5,0.5,1.0));
        }
    } else if (game.frame_count / 20) % 2 == 0 {
        let h = "PRESS START";
        let hw2 = measure_text(h, None, 14, 1.0).width;
        draw_text(h, (SCREEN_W-hw2)*0.5, SCREEN_H - 30.0, 14.0, Color::new(0.5,0.5,0.5,1.0));
    }
}

fn draw_boss_intro(game: &Game) {
    draw_rectangle(0.0,0.0,SCREEN_W,SCREEN_H,Color::new(0.0,0.0,0.0,0.85));
    let idx = game.stage_index.min(BOSS_INTROS.len()-1);
    let info = &BOSS_INTROS[idx];
    let t = (game.boss_intro_timer as f32 / 30.0).min(1.0);

    if game.boss_intro_timer < 60 && (game.boss_intro_timer / 4) % 2 == 0 {
        let w = "!! WARNING !!";
        let ww = measure_text(w, None, 16, 1.0).width;
        draw_text(w, (SCREEN_W-ww)*0.5, SCREEN_H*0.5 - 100.0, 16.0, Color::new(1.0,0.0,0.0,t));
    }
    let nw = measure_text(info.name, None, 28, 1.0).width;
    draw_text(info.name, (SCREEN_W-nw)*0.5, SCREEN_H*0.5 - 40.0, 28.0, Color::new(1.0,0.0,0.4,t));
    let tw = measure_text(info.title, None, 14, 1.0).width;
    draw_text(info.title, (SCREEN_W-tw)*0.5, SCREEN_H*0.5, 14.0, Color::new(1.0,1.0,0.0,t));
    draw_line(SCREEN_W*0.2, SCREEN_H*0.5+20.0, SCREEN_W*0.8, SCREEN_H*0.5+20.0, 2.0, Color::new(1.0,0.0,0.4,0.4*t));
    let quote = format!("\"{}\"", info.quote);
    let wrapped = wrap_text(&quote, 14.0, SCREEN_W * 0.7);
    let mut qy = SCREEN_H * 0.5 + 50.0;
    for line in &wrapped { draw_text(line, SCREEN_W*0.15, qy, 14.0, Color::new(1.0,1.0,1.0,t)); qy += 22.0; }

    if game.boss_intro_timer > 30 && (game.frame_count / 20) % 2 == 0 {
        let pr = "PRESS START TO FIGHT";
        let pw = measure_text(pr, None, 14, 1.0).width;
        draw_text(pr, (SCREEN_W-pw)*0.5, SCREEN_H - 40.0, 14.0, Color::new(0.5,0.5,0.5,1.0));
    }
}

fn draw_stage_intro(game: &Game) {
    draw_rectangle(0.0,0.0,SCREEN_W,SCREEN_H,BLACK);
    let alpha = if game.stage_intro_timer > 150 { (180 - game.stage_intro_timer) as f32 / 30.0 }
                else if game.stage_intro_timer < 30 { game.stage_intro_timer as f32 / 30.0 }
                else { 1.0 };
    let alpha = alpha.clamp(0.0, 1.0);
    let stxt = format!("STAGE {}", game.stage_index + 1);
    let sw = measure_text(&stxt, None, 20, 1.0).width;
    draw_text(&stxt, (SCREEN_W-sw)*0.5, SCREEN_H*0.5 - 30.0, 20.0, Color::new(1.0,0.0,0.4,alpha));
    let name = game.stages[game.stage_index].name;
    let nw = measure_text(name, None, 28, 1.0).width;
    draw_text(name, (SCREEN_W-nw)*0.5, SCREEN_H*0.5 + 10.0, 28.0, Color::new(1.0,1.0,1.0,alpha));
}

fn draw_game_over(game: &Game) {
    draw_rectangle(0.0,0.0,SCREEN_W,SCREEN_H,Color::new(0.0,0.0,0.0,0.7));
    let go = "GAME OVER";
    let gow = measure_text(go, None, 40, 1.0).width;
    draw_text(go, (SCREEN_W-gow)*0.5, SCREEN_H*0.5 - 40.0, 40.0, RED);
    let cont = "CONTINUE?";
    let cw = measure_text(cont, None, 20, 1.0).width;
    draw_text(cont, (SCREEN_W-cw)*0.5, SCREEN_H*0.5 + 10.0, 20.0, WHITE);
    let secs = (game.continue_timer / 60).max(0);
    let stxt = format!("{}", secs);
    let sw = measure_text(&stxt, None, 36, 1.0).width;
    draw_text(&stxt, (SCREEN_W-sw)*0.5, SCREEN_H*0.5 + 60.0, 36.0, NEON_YELLOW);
    let hint = "PRESS START TO CONTINUE";
    let hw = measure_text(hint, None, 14, 1.0).width;
    draw_text(hint, (SCREEN_W-hw)*0.5, SCREEN_H*0.5 + 100.0, 14.0, Color::new(0.5,0.5,0.5,1.0));
}

fn draw_victory(game: &Game) {
    draw_rectangle(0.0,0.0,SCREEN_W,SCREEN_H,Color::new(0.04,0.04,0.10,1.0));
    let v = "VICTORY";
    let vw = measure_text(v, None, 36, 1.0).width;
    draw_text(v, (SCREEN_W-vw)*0.5, 80.0, 36.0, NEON_YELLOW);
    let sub = "STREETS OF VENGEANCE";
    let sw = measure_text(sub, None, 14, 1.0).width;
    draw_text(sub, (SCREEN_W-sw)*0.5, 110.0, 14.0, NEON_CYAN);
    let stxt = format!("SCORE: {:07}", game.score);
    let stw = measure_text(&stxt, None, 20, 1.0).width;
    draw_text(&stxt, (SCREEN_W-stw)*0.5, 150.0, 20.0, NEON_CYAN);
    let endures = "THE DRAGON FIST ENDURES";
    let ew = measure_text(endures, None, 14, 1.0).width;
    draw_text(endures, (SCREEN_W-ew)*0.5, 185.0, 14.0, NEON_PINK);

    if (game.frame_count / 30) % 2 == 0 {
        let ps = "PRESS START";
        let pw = measure_text(ps, None, 18, 1.0).width;
        draw_text(ps, (SCREEN_W-pw)*0.5, 550.0, 18.0, WHITE);
    }
}

fn draw_pause() {
    draw_rectangle(0.0,0.0,SCREEN_W,SCREEN_H,Color::new(0.0,0.0,0.0,0.5));
    let t = "PAUSED";
    let tw = measure_text(t, None, 40, 1.0).width;
    draw_text(t, (SCREEN_W-tw)*0.5, SCREEN_H*0.45, 40.0, WHITE);
    let h = "Press Enter to resume";
    let hw = measure_text(h, None, 18, 1.0).width;
    draw_text(h, (SCREEN_W-hw)*0.5, SCREEN_H*0.55, 18.0, Color::new(0.7,0.7,0.7,1.0));
}

fn draw_mid_dialogue(game: &Game) {
    if let Some(text) = game.mid_stage_dialogue {
        let fade_in = ((240 - game.mid_stage_timer) as f32 / 15.0).min(1.0);
        let fade_out = (game.mid_stage_timer as f32 / 15.0).min(1.0);
        let alpha = fade_in.min(fade_out).max(0.0);
        draw_rectangle(20.0, SCREEN_H - 150.0, SCREEN_W - 40.0, 90.0, Color::new(0.0,0.0,0.0,0.8*alpha));
        draw_rectangle_lines(20.0, SCREEN_H - 150.0, SCREEN_W - 40.0, 90.0, 2.0, Color::new(1.0,0.0,0.4,0.5*alpha));
        let wrapped = wrap_text(text, 12.0, SCREEN_W - 80.0);
        let mut y = SCREEN_H - 132.0;
        for line in &wrapped { draw_text(line, 35.0, y, 12.0, Color::new(1.0,1.0,1.0,alpha)); y += 16.0; }
    }
}

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------
struct HitRect { x: f32, y: f32, w: f32, h: f32 }

fn rects_overlap(a: &HitRect, b: &HitRect) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

fn wrap_text(text: &str, font_size: f32, max_width: f32) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let test = if current.is_empty() { word.to_string() } else { format!("{} {}", current, word) };
        let w = measure_text(&test, None, font_size as u16, 1.0).width;
        if w > max_width && !current.is_empty() { lines.push(current); current = word.to_string(); }
        else { current = test; }
    }
    if !current.is_empty() { lines.push(current); }
    if lines.is_empty() { lines.push(String::new()); }
    lines
}
