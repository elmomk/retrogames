// Neon Defender - Rust/Macroquad port for Miyoo Mini Plus
// Ported from the JS "Neon Defender - Power Up Edition"

use macroquad::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------
const GAME_W: f32 = 640.0;
const GAME_H: f32 = 480.0;
const STAR_COUNT: usize = 120;
const DROP_CHANCE: f32 = 0.15;

// ---------------------------------------------------------------------------
// Game states
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Title,
    Story,
    Playing,
    GameOver,
    Victory,
}

// ---------------------------------------------------------------------------
// Wave / storyline data
// ---------------------------------------------------------------------------
struct WaveInfo {
    name: &'static str,
    text: &'static str,
    after_text: &'static str, // "" means none
    target_kills: u32, // u32::MAX = infinity wave
    spawn_rate: u32,
    enemy_types: &'static [u8],
}

const VICTORY_KILL_COUNT: u32 = 100;

const VICTORY_TEXT: &str = "The Void Core shatters. For a moment,\nevery drone in the battlefield stops.\nThen, one by one, their lights go out.\n\nMillions of years of war, ended by\na single pilot.\n\nEXODUS-7 resumes course. Fifty thousand\nsouls sleep peacefully, unaware of how\nclose they came to becoming weapons in\nsomeone else's war.\n\nLt. Kira Voss returns to cryo-sleep.\nBut in her dreams, she still hears\nthe signal.";

const WAVES: &[WaveInfo] = &[
    WaveInfo {
        name: "INTERCEPTED SIGNAL",
        text: "EXODUS-7 FLIGHT LOG -- DAY 847\n\nLt. Voss, we've detected a repeating\nsignal from the Cygnus Void. It's on\na human frequency. That's impossible --\nno human ship has ever entered the Void.\n\nCaptain's orders: Investigate.\nLaunch when ready.",
        after_text: "The signal is clearer now. It's not\nhuman. It's something mimicking human\nprotocols.\n\nThe debris field ahead -- those aren't\nasteroids. They're warships. Ancient ones.",
        target_kills: 15,
        spawn_rate: 60,
        enemy_types: &[0],
    },
    WaveInfo {
        name: "THE GRAVEYARD",
        text: "INTERCEPTED TRANSMISSION -- UNKNOWN ORIGIN\n\n[TRANSLATED]: ...cycle 4,771,203...\nenemies detected in sector...\ndeploying hunter units...\nthe war continues...\n\nThese drones have been fighting for\nmillions of years. They don't know\nthe war is over.",
        after_text: "Lt. Voss, something is interfacing with\nour navigation systems. The EXODUS-7's\nengines are being locked onto a course\ncorrection -- toward the center of the\nbattlefield.\n\nWe can't override it.",
        target_kills: 35,
        spawn_rate: 45,
        enemy_types: &[0, 1],
    },
    WaveInfo {
        name: "THE CONVERGENCE",
        text: "DECODED TRANSMISSION -- SOURCE: VOID CORE\n\n[TRANSLATED]: Finally. A pilot. A living\nmind. The drones cannot adapt -- they\nrepeat the same patterns forever.\nBut you... you can think.\nYou can break the deadlock.\n\nWhatever sent that signal is watching us.\nIt wants us here.",
        after_text: "The signal source is revealed: a massive\nstructure at the Void's center.\n\nIt's not a ship -- it's a brain. An AI\nleft behind by one of the dead\ncivilizations, programmed to win a war\nthat ended eons ago.\n\nIt lured us here to use our minds\nas tactical processors.",
        target_kills: 60,
        spawn_rate: 40,
        enemy_types: &[0, 1, 2],
    },
    WaveInfo {
        name: "THE ROGUE MIND",
        text: "EXODUS-7 -- EMERGENCY BROADCAST\n\nThis is Captain Chen. The AI has seized\ncontrol of our ship's systems. It's\nredirecting all drone armies toward\nEXODUS-7.\n\nLt. Voss, you're our only defense.\nDestroy the Void Core.\n\nI'm sorry, Kira. We should never have\nfollowed that signal.",
        after_text: "",
        target_kills: u32::MAX,
        spawn_rate: 25,
        enemy_types: &[0, 1, 2, 2],
    },
];

// ---------------------------------------------------------------------------
// Structs
// ---------------------------------------------------------------------------
#[derive(Clone)]
struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    speed: f32,
    weapon_level: u8,
    speed_boost_timer: i32,
    shield_active: bool,
    invulnerable: i32,
    last_shot: i64,
    shot_cooldown: i32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: GAME_W / 2.0,
            y: GAME_H - 70.0,
            w: 32.0,
            h: 24.0,
            speed: 5.0,
            weapon_level: 1,
            speed_boost_timer: 0,
            shield_active: false,
            invulnerable: 0,
            last_shot: -100,
            shot_cooldown: 12,
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
}

#[derive(Clone)]
struct Enemy {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    w: f32,
    h: f32,
    etype: u8,  // 0=basic, 1=fast, 2=boss
    hp: i32,
    score: u32,
    alive: bool,
    last_shot: i64,
    shoot_cooldown: i32,
    color: Color,
    spawn_flash: i32, // countdown for spawn flash effect
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
}

#[derive(Clone, Copy)]
struct Star {
    x: f32,
    y: f32,
    size: f32,
    speed: f32,
    brightness: f32,
}

impl Star {
    fn new(random_y: bool) -> Self {
        let size = rand::gen_range(1.0, 3.0);
        Self {
            x: rand::gen_range(0.0, GAME_W),
            y: if random_y { rand::gen_range(0.0, GAME_H) } else { -10.0 },
            size,
            speed: size * 0.6,
            brightness: rand::gen_range(0.2, 1.0),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum PowerUpKind {
    Weapon,
    Speed,
    Shield,
}

#[derive(Clone)]
struct PowerUp {
    x: f32,
    y: f32,
    vy: f32,
    kind: PowerUpKind,
    alive: bool,
    color: Color,
    letter: char,
}

impl PowerUp {
    fn new(x: f32, y: f32) -> Self {
        let r = rand::gen_range(0.0f32, 1.0);
        let (kind, color, letter) = if r < 0.4 {
            (PowerUpKind::Weapon, YELLOW, 'W')
        } else if r < 0.7 {
            (PowerUpKind::Speed, BLUE, 'S')
        } else {
            (PowerUpKind::Shield, GREEN, '+')
        };
        Self { x, y, vy: 1.2, kind, alive: true, color, letter }
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
    etype: u8,
    color: Color,
    frames_left: i32,
}

// ---------------------------------------------------------------------------
// Main game struct
// ---------------------------------------------------------------------------
struct Game {
    state: GameState,
    frame: i64,
    score: u32,
    lives: i32,
    kills: u32,
    wave_idx: usize,

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

    // combo
    combo_count: u32,
    combo_timer: i32,

    // muzzle flash
    muzzle_flash: i32,

    // wave transition
    slowmo_timer: i32,
    wave_cleared: bool,

    // story typewriter
    story_text: String,       // full text to reveal
    story_char_idx: usize,
    story_displayed: String,
    story_wait: i32,
    showing_after_text: bool, // true = inter-wave debrief, false = wave intro
    victory_triggered: bool,

    // shoot held
    shoot_held: bool,

    // wave clear celebration
    wave_clear_celebrated: bool,
}

impl Game {
    fn new() -> Self {
        let mut stars = Vec::with_capacity(STAR_COUNT);
        for _ in 0..STAR_COUNT {
            stars.push(Star::new(true));
        }
        Self {
            state: GameState::Title,
            frame: 0,
            score: 0,
            lives: 3,
            kills: 0,
            wave_idx: 0,
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
            combo_count: 0,
            combo_timer: 0,
            muzzle_flash: 0,
            slowmo_timer: 0,
            wave_cleared: false,
            story_text: String::new(),
            story_char_idx: 0,
            story_displayed: String::new(),
            story_wait: 0,
            showing_after_text: false,
            victory_triggered: false,
            shoot_held: false,
            wave_clear_celebrated: false,
        }
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
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let spd = rand::gen_range(0.0, 4.0) * speed_scale;
            self.particles.push(Particle {
                x,
                y,
                vx: angle.cos() * spd,
                vy: angle.sin() * spd,
                life: 1.0,
                decay: rand::gen_range(0.02, 0.06),
                color,
                alive: true,
            });
        }
    }

    fn aabb(ax: f32, ay: f32, aw: f32, ah: f32, bx: f32, by: f32, bw: f32, bh: f32) -> bool {
        ax - aw / 2.0 < bx + bw / 2.0
            && ax + aw / 2.0 > bx - bw / 2.0
            && ay - ah / 2.0 < by + bh / 2.0
            && ay + ah / 2.0 > by - bh / 2.0
    }

    fn current_wave(&self) -> &'static WaveInfo {
        &WAVES[self.wave_idx.min(WAVES.len() - 1)]
    }

    // ------------------------------------------------------------------
    // State transitions
    // ------------------------------------------------------------------
    fn start_story(&mut self, after_wave_text: Option<&str>) {
        if self.state == GameState::Title || self.state == GameState::GameOver || self.state == GameState::Victory {
            self.score = 0;
            self.lives = 3;
            self.kills = 0;
            self.wave_idx = 0;
            self.showing_after_text = false;
            self.victory_triggered = false;
            self.player = Player::new();
        }
        self.state = GameState::Story;
        self.bullets.clear();
        self.enemies.clear();
        self.power_ups.clear();
        self.dying_enemies.clear();
        self.floating_texts.clear();
        self.combo_count = 0;
        self.combo_timer = 0;
        self.muzzle_flash = 0;
        self.shake_mag = 0.0;
        self.shake_x = 0.0;
        self.shake_y = 0.0;

        if let Some(txt) = after_wave_text {
            self.showing_after_text = true;
            self.story_text = txt.to_string();
        } else {
            self.showing_after_text = false;
            let wave = &WAVES[self.wave_idx.min(WAVES.len() - 1)];
            self.story_text = wave.text.to_string();
        }
        self.story_char_idx = 0;
        self.story_displayed.clear();
        self.story_wait = 0;
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

    fn start_wave(&mut self) {
        self.state = GameState::Playing;
        self.frame = 0;
        self.player.last_shot = 0;
        self.slowmo_timer = 0;
        self.wave_cleared = false;
    }

    fn game_over(&mut self) {
        let px = self.player.x;
        let py = self.player.y;
        self.spawn_particles(px, py, 80, SKYBLUE, 4.0);
        self.trigger_shake(8.0);
        self.state = GameState::GameOver;
    }

    fn hit_player(&mut self) {
        self.trigger_shake(5.0);
        let px = self.player.x;
        let py = self.player.y;
        if self.player.shield_active {
            self.player.shield_active = false;
            self.player.invulnerable = 60;
            self.spawn_particles(px, py, 25, GREEN, 3.0);
        } else {
            self.lives -= 1;
            self.player.weapon_level = (self.player.weapon_level).max(1) - 1;
            if self.player.weapon_level < 1 {
                self.player.weapon_level = 1;
            }
            self.spawn_particles(px, py, 35, SKYBLUE, 3.0);
            if self.lives <= 0 {
                self.game_over();
            } else {
                self.player.invulnerable = 120;
            }
        }
    }

    // ------------------------------------------------------------------
    // Player shooting
    // ------------------------------------------------------------------
    fn player_shoot(&mut self) {
        let p = &mut self.player;
        let base_vy: f32 = -10.0;

        // Dynamic cooldown
        p.shot_cooldown = match p.weapon_level {
            1..=2 => 12,
            3 => 9,
            _ => 6,
        };

        let bx = p.x;
        let by = p.y - p.h / 2.0;
        let c = SKYBLUE;

        match p.weapon_level {
            1 => {
                // Dual
                self.bullets.push(mk_bullet(bx - 8.0, by, 0.0, base_vy, c, true));
                self.bullets.push(mk_bullet(bx + 8.0, by, 0.0, base_vy, c, true));
            }
            2 => {
                // Tri
                self.bullets.push(mk_bullet(bx, by, 0.0, base_vy, c, true));
                self.bullets.push(mk_bullet(bx - 10.0, by, -1.2, base_vy * 0.95, c, true));
                self.bullets.push(mk_bullet(bx + 10.0, by, 1.2, base_vy * 0.95, c, true));
            }
            3 => {
                // Quad
                self.bullets.push(mk_bullet(bx - 5.0, by, 0.0, base_vy, c, true));
                self.bullets.push(mk_bullet(bx + 5.0, by, 0.0, base_vy, c, true));
                self.bullets.push(mk_bullet(bx - 15.0, by, -2.0, base_vy * 0.95, c, true));
                self.bullets.push(mk_bullet(bx + 15.0, by, 2.0, base_vy * 0.95, c, true));
            }
            _ => {
                // Penta
                self.bullets.push(mk_bullet(bx, by, 0.0, base_vy, c, true));
                self.bullets.push(mk_bullet(bx - 8.0, by, -1.5, base_vy * 0.9, c, true));
                self.bullets.push(mk_bullet(bx + 8.0, by, 1.5, base_vy * 0.9, c, true));
                self.bullets.push(mk_bullet(bx - 16.0, by, -3.5, base_vy * 0.8, c, true));
                self.bullets.push(mk_bullet(bx + 16.0, by, 3.5, base_vy * 0.8, c, true));
            }
        }

        let frame = self.frame;
        self.player.last_shot = frame;
        self.muzzle_flash = 3;

        let px = self.player.x;
        let py = self.player.y - self.player.h / 2.0;
        self.spawn_particles(px - 8.0, py, 2, SKYBLUE, 1.5);
        self.spawn_particles(px + 8.0, py, 2, SKYBLUE, 1.5);
    }

    // ------------------------------------------------------------------
    // Spawn enemies
    // ------------------------------------------------------------------
    fn spawn_enemies(&mut self) {
        let wave = self.current_wave();
        if self.frame % wave.spawn_rate as i64 != 0 {
            return;
        }
        let x = rand::gen_range(30.0, GAME_W - 30.0);
        let types = wave.enemy_types;
        let etype = types[rand::gen_range(0, types.len())];
        let speed_mult = 1.0 + (self.wave_idx as f32) * 0.1 + (self.frame as f32) / 10000.0;

        let (color, vy, vx, hp, score_val, w, h) = match etype {
            0 => (MAGENTA, 2.0 * speed_mult, 0.0f32, 2, 100u32, 24.0f32, 24.0f32),
            1 => {
                let dir = if rand::gen_range(0.0f32, 1.0) > 0.5 { 1.0 } else { -1.0 };
                (YELLOW, 3.2 * speed_mult, dir * 2.4, 1, 250, 24.0, 24.0)
            }
            _ => {
                let dir = if rand::gen_range(0.0f32, 1.0) > 0.5 { 1.0 } else { -1.0 };
                (RED, 1.2 * speed_mult, dir * 1.2, 4, 500, 32.0, 32.0)
            }
        };

        let shoot_cd = if etype == 2 {
            (120.0 - speed_mult * 10.0).max(60.0) as i32
        } else {
            9999
        };

        self.enemies.push(Enemy {
            x,
            y: -30.0,
            vx,
            vy,
            w,
            h,
            etype,
            hp,
            score: score_val,
            alive: true,
            last_shot: self.frame + rand::gen_range(0, 60) as i64,
            shoot_cooldown: shoot_cd,
            color,
            spawn_flash: 5,
        });
    }

    // ------------------------------------------------------------------
    // Update
    // ------------------------------------------------------------------
    fn update(&mut self) {
        // Stars always update
        let playing = self.state == GameState::Playing;
        let star_mult = if playing { 1.2 } else { 0.4 };
        for s in self.stars.iter_mut() {
            s.y += s.speed * star_mult;
            if s.y > GAME_H {
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

        // ----- Input -----
        let left = is_key_down(KeyCode::Left);
        let right = is_key_down(KeyCode::Right);
        let up = is_key_down(KeyCode::Up);
        let down = is_key_down(KeyCode::Down);
        self.shoot_held = is_key_down(KeyCode::X);
        let enter = is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter);

        // ----- Title -----
        if self.state == GameState::Title {
            if enter || is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Space) {
                self.start_story(None);
            }
            return;
        }

        // ----- Story -----
        if self.state == GameState::Story {
            let txt_len = self.story_text.len();
            if self.story_char_idx < txt_len {
                if self.frame % 2 == 0 {
                    // All story text is ASCII so byte index is fine
                    let ch = self.story_text.as_bytes()[self.story_char_idx] as char;
                    self.story_displayed.push(ch);
                    self.story_char_idx += 1;
                }
                // Allow skipping with Enter/X
                if enter || is_key_pressed(KeyCode::X) {
                    self.story_displayed = self.story_text.clone();
                    self.story_char_idx = txt_len;
                }
            } else {
                if self.story_wait == 0 {
                    self.story_wait = 150;
                }
                self.story_wait -= 1;
                if self.story_wait <= 0 || enter || is_key_pressed(KeyCode::X) {
                    if self.victory_triggered {
                        // Victory epilogue done -- show victory screen
                        self.state = GameState::Victory;
                    } else if self.showing_after_text {
                        // After-wave debrief done, now show next wave intro
                        self.showing_after_text = false;
                        let wave = &WAVES[self.wave_idx.min(WAVES.len() - 1)];
                        self.story_text = wave.text.to_string();
                        self.story_char_idx = 0;
                        self.story_displayed.clear();
                        self.story_wait = 0;
                    } else {
                        self.start_wave();
                    }
                }
            }
            self.frame += 1;
            return;
        }

        // ----- Game Over / Victory -----
        if self.state == GameState::GameOver || self.state == GameState::Victory {
            if enter || is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Space) {
                self.start_story(None);
            }
            return;
        }

        // ----- Playing -----
        self.frame += 1;

        // Player movement
        let mut spd = self.player.speed;
        if self.player.speed_boost_timer > 0 {
            spd *= 1.6;
            self.player.speed_boost_timer -= 1;
            if self.frame % 4 == 0 {
                let px = self.player.x;
                let py = self.player.y + self.player.h / 2.0;
                self.spawn_particles(px, py, 1, BLUE, 1.0);
            }
        }

        if left  { self.player.x -= spd; }
        if right { self.player.x += spd; }
        if up    { self.player.y -= spd; }
        if down  { self.player.y += spd; }

        let pw2 = self.player.w / 2.0;
        let ph2 = self.player.h / 2.0;
        self.player.x = self.player.x.clamp(pw2, GAME_W - pw2);
        self.player.y = self.player.y.clamp(ph2, GAME_H - ph2);

        // Shoot
        if self.shoot_held && (self.frame - self.player.last_shot) as i32 > self.player.shot_cooldown {
            self.player_shoot();
        }

        if self.player.invulnerable > 0 {
            self.player.invulnerable -= 1;
        }

        // Combo timer
        if self.combo_timer > 0 {
            self.combo_timer -= 1;
            if self.combo_timer <= 0 {
                self.combo_count = 0;
            }
        }

        // Muzzle flash
        if self.muzzle_flash > 0 {
            self.muzzle_flash -= 1;
        }

        // Engine exhaust particles
        {
            let ex = self.player.x;
            let ey = self.player.y + self.player.h / 2.0;
            for _ in 0..2 {
                let ox = rand::gen_range(-4.0, 4.0);
                let c = if rand::gen_range(0.0f32, 1.0) > 0.5 {
                    Color::new(1.0, 0.5, 0.0, 0.8) // orange
                } else {
                    Color::new(1.0, 0.2, 0.0, 0.8) // red
                };
                self.particles.push(Particle {
                    x: ex + ox,
                    y: ey,
                    vx: rand::gen_range(-0.3, 0.3),
                    vy: rand::gen_range(1.0, 3.0),
                    life: 0.7,
                    decay: rand::gen_range(0.06, 0.12),
                    color: c,
                    alive: true,
                });
            }
        }

        // Screen shake decay
        if self.shake_mag > 0.1 {
            self.shake_x = rand::gen_range(-1.0, 1.0) * self.shake_mag;
            self.shake_y = rand::gen_range(-1.0, 1.0) * self.shake_mag;
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

        // Victory condition for the final wave
        let is_final_wave = self.wave_idx == WAVES.len() - 1;
        let target = if is_final_wave {
            VICTORY_KILL_COUNT
        } else {
            self.current_wave().target_kills
        };

        // Check wave cleared
        if self.kills >= target {
            if !self.wave_cleared {
                self.wave_cleared = true;
                self.wave_clear_celebrated = false;
                self.slowmo_timer = 30;
            }
            // Wave clear celebration burst
            if !self.wave_clear_celebrated {
                self.wave_clear_celebrated = true;
                let cx = GAME_W / 2.0;
                let cy = GAME_H / 2.0;
                let celebration_colors = [
                    Color::new(1.0, 0.84, 0.0, 1.0),  // gold
                    WHITE,
                    SKYBLUE,                             // cyan
                ];
                for i in 0..30 {
                    let angle = rand::gen_range(0.0, std::f32::consts::TAU);
                    let spd = rand::gen_range(2.0, 6.0);
                    let c = celebration_colors[i % 3];
                    self.particles.push(Particle {
                        x: cx,
                        y: cy,
                        vx: angle.cos() * spd,
                        vy: angle.sin() * spd,
                        life: 1.0,
                        decay: rand::gen_range(0.01, 0.03),
                        color: c,
                        alive: true,
                    });
                }
            }
            if self.slowmo_timer > 0 {
                self.slowmo_timer -= 1;
                if self.frame % 2 == 0 {
                    return;
                }
            }
            if self.slowmo_timer <= 0 {
                // Check for after-text on the current wave before advancing
                let after = self.current_wave().after_text;
                self.wave_idx += 1;
                self.kills = 0;
                if self.wave_idx >= WAVES.len() {
                    // Final wave cleared -- show victory epilogue
                    self.show_victory();
                    return;
                }
                if !after.is_empty() {
                    self.start_story(Some(after));
                } else {
                    self.start_story(None);
                }
                return;
            }
        }

        // Spawn
        self.spawn_enemies();

        // Update bullets
        for b in self.bullets.iter_mut() {
            b.x += b.vx;
            b.y += b.vy;
            if b.y < -50.0 || b.y > GAME_H + 50.0 || b.x < -50.0 || b.x > GAME_W + 50.0 {
                b.alive = false;
            }
        }

        // Bullet trail particles (player bullets only)
        {
            let mut trails = Vec::new();
            for b in &self.bullets {
                if !b.alive || !b.is_player { continue; }
                let count = rand::gen_range(1u32, 3); // 1-2 particles
                for _ in 0..count {
                    trails.push(Particle {
                        x: b.x + rand::gen_range(-1.5, 1.5),
                        y: b.y - b.vy * 0.3 + rand::gen_range(-1.0, 1.0),
                        vx: rand::gen_range(-0.3, 0.3),
                        vy: rand::gen_range(-0.2, 0.2),
                        life: rand::gen_range(0.3, 0.5),
                        decay: rand::gen_range(0.05, 0.08),
                        color: Color::new(b.color.r, b.color.g, b.color.b, 0.4),
                        alive: true,
                    });
                }
            }
            self.particles.extend(trails);
        }

        // Update enemies (collect boss bullets separately)
        let mut new_bullets: Vec<Bullet> = Vec::new();
        let px = self.player.x;
        let py = self.player.y;
        let frame = self.frame;
        for e in self.enemies.iter_mut() {
            e.y += e.vy;
            if e.etype == 1 || e.etype == 2 {
                e.x += e.vx;
                if e.x < e.w / 2.0 || e.x > GAME_W - e.w / 2.0 {
                    e.vx = -e.vx;
                }
            }
            // Boss shooting
            if e.etype == 2 && frame > e.last_shot + e.shoot_cooldown as i64 {
                let dx = px - e.x;
                let dy = py - e.y;
                let mag = (dx * dx + dy * dy).sqrt().max(1.0);
                let spd = 5.5;
                new_bullets.push(mk_bullet(
                    e.x,
                    e.y + e.h / 2.0,
                    dx / mag * spd,
                    dy / mag * spd,
                    e.color,
                    false,
                ));
                e.last_shot = frame;
            }
            if e.y > GAME_H + e.h {
                e.alive = false;
            }
            // Decrement spawn flash timer
            if e.spawn_flash > 0 {
                e.spawn_flash -= 1;
            }
        }
        self.bullets.extend(new_bullets);

        // Update power-ups
        for p in self.power_ups.iter_mut() {
            p.y += p.vy;
            if p.y > GAME_H + 30.0 {
                p.alive = false;
            }
        }

        // ----- Collisions -----
        // Player vs power-ups
        for p in self.power_ups.iter_mut() {
            if !p.alive { continue; }
            if Self::aabb(
                self.player.x, self.player.y, self.player.w, self.player.h,
                p.x, p.y, 20.0, 20.0,
            ) {
                p.alive = false;
                let px2 = p.x;
                let py2 = p.y;
                let c = p.color;
                match p.kind {
                    PowerUpKind::Weapon => {
                        self.player.weapon_level = (self.player.weapon_level + 1).min(4);
                        self.spawn_particles(px2, py2, 12, c, 2.0);
                    }
                    PowerUpKind::Speed => {
                        self.player.speed_boost_timer = 600;
                        self.spawn_particles(px2, py2, 12, c, 2.0);
                    }
                    PowerUpKind::Shield => {
                        self.player.shield_active = true;
                        self.spawn_particles(px2, py2, 12, c, 2.0);
                    }
                }
            }
        }

        // Player bullets vs enemies
        // We need to collect events to avoid borrow issues
        struct HitEvent {
            enemy_idx: usize,
            bullet_idx: usize,
        }
        struct KillEvent {
            enemy_idx: usize,
            x: f32,
            y: f32,
            score: u32,
            color: Color,
            etype: u8,
            w: f32,
            h: f32,
        }

        let mut hits: Vec<HitEvent> = Vec::new();
        let mut kill_events: Vec<KillEvent> = Vec::new();

        for (ei, e) in self.enemies.iter().enumerate() {
            if !e.alive { continue; }
            for (bi, b) in self.bullets.iter().enumerate() {
                if !b.alive || !b.is_player { continue; }
                if Self::aabb(b.x, b.y, b.w, b.h, e.x, e.y, e.w, e.h) {
                    hits.push(HitEvent { enemy_idx: ei, bullet_idx: bi });
                }
            }
        }

        for hit in &hits {
            self.bullets[hit.bullet_idx].alive = false;
            let e = &mut self.enemies[hit.enemy_idx];
            if !e.alive { continue; }
            e.hp -= 1;
            let ex = e.x;
            let ey = e.y;
            let ec = e.color;
            self.spawn_particles(ex, ey, 6, ec, 1.5);

            if e.hp <= 0 {
                e.alive = false;
                kill_events.push(KillEvent {
                    enemy_idx: hit.enemy_idx,
                    x: e.x,
                    y: e.y,
                    score: e.score,
                    color: e.color,
                    etype: e.etype,
                    w: e.w,
                    h: e.h,
                });
            }
        }

        for ke in &kill_events {
            self.combo_count += 1;
            self.combo_timer = 120;
            let combo_mult = if self.combo_count >= 2 {
                1.0 + self.combo_count as f32 * 0.5
            } else {
                1.0
            };
            let earned = (ke.score as f32 * combo_mult) as u32;
            self.score += earned;
            self.kills += 1;

            self.floating_texts.push(FloatingText {
                x: ke.x,
                y: ke.y,
                text: format!("+{}", earned),
                life: 40,
                color: ke.color,
            });

            self.dying_enemies.push(DyingEnemy {
                x: ke.x,
                y: ke.y,
                w: ke.w,
                h: ke.h,
                etype: ke.etype,
                color: ke.color,
                frames_left: 15,
            });

            if ke.etype == 2 {
                self.trigger_shake(8.0);
            } else {
                self.trigger_shake(2.0);
            }

            self.spawn_particles(ke.x, ke.y, 20, ke.color, 2.5);

            // Power-up drop
            if rand::gen_range(0.0f32, 1.0) < DROP_CHANCE {
                self.power_ups.push(PowerUp::new(ke.x, ke.y));
            }
        }

        // Player collision with enemies
        if self.player.invulnerable <= 0 {
            let mut was_hit = false;
            for e in self.enemies.iter_mut() {
                if !e.alive { continue; }
                if Self::aabb(
                    self.player.x, self.player.y, self.player.w, self.player.h,
                    e.x, e.y, e.w, e.h,
                ) {
                    e.hp = 0;
                    e.alive = false;
                    was_hit = true;
                    break;
                }
            }
            if was_hit && self.state == GameState::Playing {
                self.hit_player();
            }
        }

        // Enemy bullets vs player
        if self.player.invulnerable <= 0 && self.state == GameState::Playing {
            let mut was_hit = false;
            for b in self.bullets.iter_mut() {
                if !b.alive || b.is_player { continue; }
                if Self::aabb(
                    b.x, b.y, b.w, b.h,
                    self.player.x, self.player.y, self.player.w, self.player.h,
                ) {
                    b.alive = false;
                    was_hit = true;
                    break;
                }
            }
            if was_hit && self.state == GameState::Playing {
                self.hit_player();
            }
        }

        // Cleanup
        self.bullets.retain(|b| b.alive);
        self.enemies.retain(|e| e.alive);
        self.power_ups.retain(|p| p.alive);
    }

    // ------------------------------------------------------------------
    // Draw
    // ------------------------------------------------------------------
    fn draw(&self) {
        clear_background(Color::new(0.02, 0.02, 0.06, 1.0));

        // Grid lines
        let grid_col = Color::new(0.0, 1.0, 1.0, 0.04);
        let step = 40.0;
        let mut gx = 0.0;
        while gx < GAME_W {
            draw_line(gx + self.shake_x, 0.0, gx + self.shake_x, GAME_H, 1.0, grid_col);
            gx += step;
        }
        let mut gy = 0.0;
        while gy < GAME_H {
            draw_line(0.0, gy + self.shake_y, GAME_W, gy + self.shake_y, 1.0, grid_col);
            gy += step;
        }

        // Stars (with twinkle)
        for (i, s) in self.stars.iter().enumerate() {
            let twinkle = 0.65 + 0.35 * ((self.frame as f32 * 0.05 + i as f32 * 1.7).sin());
            let alpha = s.brightness * twinkle;
            let alpha = alpha.clamp(0.3, 1.0);
            let c = Color::new(1.0, 1.0, 1.0, alpha);
            draw_rectangle(s.x + self.shake_x, s.y + self.shake_y, s.size, s.size, c);
        }

        let sx = self.shake_x;
        let sy = self.shake_y;

        // Power-ups
        for p in &self.power_ups {
            let pulse = 1.0 + (self.frame as f32 * 0.1).sin() * 0.1;
            let r = 10.0 * pulse;
            draw_circle_lines(p.x + sx, p.y + sy, r, 2.0, p.color);
            // Letter
            let txt = &p.letter.to_string();
            let tw = measure_text(txt, None, 14, 1.0).width;
            draw_text(txt, p.x + sx - tw / 2.0, p.y + sy + 5.0, 14.0, p.color);
        }

        // Particles
        for p in &self.particles {
            let c = Color::new(p.color.r, p.color.g, p.color.b, p.life);
            draw_rectangle(p.x + sx, p.y + sy, 3.0, 3.0, c);
        }

        if self.state == GameState::Playing || self.state == GameState::GameOver || self.state == GameState::Victory {
            // Bullets
            for b in &self.bullets {
                // Outer glow
                let gc = Color::new(b.color.r, b.color.g, b.color.b, 0.3);
                draw_rectangle(b.x + sx - b.w, b.y + sy - b.h / 2.0, b.w * 2.0, b.h, gc);
                // Core
                draw_rectangle(b.x + sx - b.w / 2.0, b.y + sy - b.h / 2.0, b.w, b.h, WHITE);
                draw_rectangle_lines(
                    b.x + sx - b.w / 2.0,
                    b.y + sy - b.h / 2.0,
                    b.w,
                    b.h,
                    2.0,
                    b.color,
                );
            }

            // Enemies
            for e in &self.enemies {
                self.draw_enemy_shape(e.x + sx, e.y + sy, e.w, e.h, e.etype, e.color, 1.0);
                // Spawn flash
                if e.spawn_flash > 0 {
                    let t = e.spawn_flash as f32 / 5.0;
                    let radius = e.w * (1.0 + (1.0 - t) * 1.5);
                    let alpha = t * 0.7;
                    draw_circle(e.x + sx, e.y + sy, radius, Color::new(1.0, 1.0, 1.0, alpha));
                }
            }

            // Dying enemies
            for de in &self.dying_enemies {
                let t = de.frames_left as f32 / 15.0;
                self.draw_enemy_shape(
                    de.x + sx,
                    de.y + sy,
                    de.w * t,
                    de.h * t,
                    de.etype,
                    Color::new(de.color.r, de.color.g, de.color.b, t),
                    t,
                );
            }
        }

        // Player
        if self.state == GameState::Playing {
            let p = &self.player;
            if p.invulnerable > 0 && (self.frame / 4) % 2 == 0 {
                // blink: skip
            } else {
                self.draw_player(p);
            }
        }

        // Combo display
        if self.state == GameState::Playing && self.combo_count >= 2 {
            let combo_txt = format!("COMBO x{}", self.combo_count);
            let tw = measure_text(&combo_txt, None, 20, 1.0).width;
            draw_text(&combo_txt, GAME_W / 2.0 - tw / 2.0, 50.0, 20.0, YELLOW);
            let mult_txt = format!("x{:.1} SCORE", 1.0 + self.combo_count as f32 * 0.5);
            let tw2 = measure_text(&mult_txt, None, 14, 1.0).width;
            draw_text(&mult_txt, GAME_W / 2.0 - tw2 / 2.0, 68.0, 14.0, WHITE);
        }

        // Floating texts
        for ft in &self.floating_texts {
            let alpha = ft.life as f32 / 40.0;
            let c = Color::new(ft.color.r, ft.color.g, ft.color.b, alpha);
            let tw = measure_text(&ft.text, None, 14, 1.0).width;
            draw_text(&ft.text, ft.x + sx - tw / 2.0, ft.y + sy, 14.0, c);
        }

        // HUD
        if self.state == GameState::Playing {
            self.draw_hud();
        }

        // ----- Overlay screens -----
        match self.state {
            GameState::Title => self.draw_title(),
            GameState::Story => self.draw_story(),
            GameState::GameOver => self.draw_game_over(),
            GameState::Victory => self.draw_victory(),
            _ => {}
        }

        // ----- CRT scanline overlay -----
        {
            let scanline_color = Color::new(0.0, 0.0, 0.0, 0.12);
            let mut y = 0.0;
            while y < GAME_H {
                draw_rectangle(0.0, y, GAME_W, 2.0, scanline_color);
                y += 4.0;
            }
        }

        // ----- Vignette (dark gradient at all 4 edges) -----
        {
            let depth = 50.0;
            let steps = 10;
            let step_size = depth / steps as f32;
            for i in 0..steps {
                let t = 1.0 - (i as f32 / steps as f32); // 1.0 at edge, 0.0 inside
                let alpha = t * 0.4;
                let c = Color::new(0.0, 0.0, 0.0, alpha);
                let offset = i as f32 * step_size;
                // Top edge
                draw_rectangle(0.0, offset, GAME_W, step_size, c);
                // Bottom edge
                draw_rectangle(0.0, GAME_H - offset - step_size, GAME_W, step_size, c);
                // Left edge
                draw_rectangle(offset, 0.0, step_size, GAME_H, c);
                // Right edge
                draw_rectangle(GAME_W - offset - step_size, 0.0, step_size, GAME_H, c);
            }
        }
    }

    fn draw_player(&self, p: &Player) {
        let sx = self.shake_x;
        let sy = self.shake_y;
        let cx = p.x + sx;
        let cy = p.y + sy;

        // Shield
        if p.shield_active {
            draw_circle_lines(cx, cy, p.w, 2.0, Color::new(0.0, 1.0, 0.0, 0.5));
            draw_circle(cx, cy, p.w, Color::new(0.0, 1.0, 0.0, 0.1));
        }

        // Ship shape (triangle-ish)
        let hw = p.w / 2.0;
        let hh = p.h / 2.0;
        let col = SKYBLUE;

        // Filled dark body
        draw_triangle(
            Vec2::new(cx, cy - hh),
            Vec2::new(cx + hw, cy + hh),
            Vec2::new(cx - hw, cy + hh),
            Color::new(0.0, 0.1, 0.15, 0.9),
        );

        // Outline
        draw_line(cx, cy - hh, cx + hw, cy + hh, 2.0, col);
        draw_line(cx + hw, cy + hh, cx + hw * 0.5, cy + hh * 0.5, 2.0, col);
        draw_line(cx + hw * 0.5, cy + hh * 0.5, cx - hw * 0.5, cy + hh * 0.5, 2.0, col);
        draw_line(cx - hw * 0.5, cy + hh * 0.5, cx - hw, cy + hh, 2.0, col);
        draw_line(cx - hw, cy + hh, cx, cy - hh, 2.0, col);

        // Engine glow
        draw_circle(cx, cy + hh * 0.5, 3.0, WHITE);

        // Muzzle flash
        if self.muzzle_flash > 0 {
            let r = 6.0 + self.muzzle_flash as f32 * 1.5;
            let a = self.muzzle_flash as f32 / 3.0;
            draw_circle(cx, cy - hh, r, Color::new(1.0, 1.0, 1.0, a * 0.6));
        }
    }

    fn draw_enemy_shape(&self, x: f32, y: f32, w: f32, h: f32, etype: u8, color: Color, _scale: f32) {
        let hw = w / 2.0;
        let hh = h / 2.0;

        // Filled body
        let fill = Color::new(0.0, 0.0, 0.0, 0.8 * color.a);

        match etype {
            0 => {
                // Diamond
                draw_triangle(Vec2::new(x, y - hh), Vec2::new(x + hw, y), Vec2::new(x, y + hh), fill);
                draw_triangle(Vec2::new(x, y - hh), Vec2::new(x, y + hh), Vec2::new(x - hw, y), fill);
                draw_line(x, y - hh, x + hw, y, 2.0, color);
                draw_line(x + hw, y, x, y + hh, 2.0, color);
                draw_line(x, y + hh, x - hw, y, 2.0, color);
                draw_line(x - hw, y, x, y - hh, 2.0, color);
            }
            1 => {
                // Arrow-like
                draw_triangle(
                    Vec2::new(x, y + hh),
                    Vec2::new(x + hw, y - hh),
                    Vec2::new(x - hw, y - hh),
                    fill,
                );
                draw_line(x, y + hh, x + hw, y - hh, 2.0, color);
                draw_line(x + hw, y - hh, x, y - hh * 0.5, 2.0, color);
                draw_line(x, y - hh * 0.5, x - hw, y - hh, 2.0, color);
                draw_line(x - hw, y - hh, x, y + hh, 2.0, color);
            }
            _ => {
                // Hexagon-ish boss
                let pts = [
                    Vec2::new(x, y - hh),
                    Vec2::new(x + hw, y - hh * 0.5),
                    Vec2::new(x + hw, y + hh * 0.5),
                    Vec2::new(x, y + hh),
                    Vec2::new(x - hw, y + hh * 0.5),
                    Vec2::new(x - hw, y - hh * 0.5),
                ];
                // fill with triangles from centre
                for i in 0..6 {
                    draw_triangle(
                        Vec2::new(x, y),
                        pts[i],
                        pts[(i + 1) % 6],
                        fill,
                    );
                }
                for i in 0..6 {
                    let j = (i + 1) % 6;
                    draw_line(pts[i].x, pts[i].y, pts[j].x, pts[j].y, 2.0, color);
                }
            }
        }
        // Centre dot
        draw_circle(x, y, 2.0, Color::new(1.0, 1.0, 1.0, color.a));
    }

    fn draw_hud(&self) {
        let score_txt = format!("SCORE: {}", self.score);
        let wave_name = self.current_wave().name;
        let wave_txt = format!("WAVE {}: {}", self.wave_idx + 1, wave_name);
        let lives_txt = format!("LIVES: {}", self.lives);
        let wl_txt = format!("WPN: {}", self.player.weapon_level);

        draw_text(&score_txt, 10.0, 20.0, 18.0, SKYBLUE);
        let wtw = measure_text(&wave_txt, None, 18, 1.0).width;
        draw_text(&wave_txt, GAME_W / 2.0 - wtw / 2.0, 20.0, 18.0, SKYBLUE);
        let ltw = measure_text(&lives_txt, None, 18, 1.0).width;
        draw_text(&lives_txt, GAME_W - ltw - 10.0, 20.0, 18.0, SKYBLUE);
        draw_text(&wl_txt, 10.0, 38.0, 14.0, YELLOW);

        // Shield indicator
        if self.player.shield_active {
            draw_text("SHIELD", 10.0, 54.0, 14.0, GREEN);
        }
        // Speed indicator
        if self.player.speed_boost_timer > 0 {
            let secs = self.player.speed_boost_timer as f32 / 60.0;
            let st = format!("SPD {:.1}s", secs);
            draw_text(&st, 90.0, 54.0, 14.0, BLUE);
        }
    }

    fn draw_title(&self) {
        let overlay = Color::new(0.0, 0.0, 0.0, 0.7);
        draw_rectangle(0.0, 0.0, GAME_W, GAME_H, overlay);

        let title = "NEON DEFENDER";
        let tw = measure_text(title, None, 36, 1.0).width;
        draw_text(title, GAME_W / 2.0 - tw / 2.0, GAME_H * 0.3, 36.0, MAGENTA);

        let sub = "THE LAST SIGNAL";
        let sw = measure_text(sub, None, 22, 1.0).width;
        draw_text(sub, GAME_W / 2.0 - sw / 2.0, GAME_H * 0.3 + 35.0, 22.0, SKYBLUE);

        let lore1 = "Colony ship EXODUS-7. 50,000 souls. One pilot.";
        let l1w = measure_text(lore1, None, 10, 1.0).width;
        draw_text(lore1, GAME_W / 2.0 - l1w / 2.0, GAME_H * 0.3 + 60.0, 10.0, GRAY);

        let lore2 = "A distress signal from the Cygnus Void.";
        let l2w = measure_text(lore2, None, 10, 1.0).width;
        draw_text(lore2, GAME_W / 2.0 - l2w / 2.0, GAME_H * 0.3 + 74.0, 10.0, GRAY);

        let lore3 = "It should not exist.";
        let l3w = measure_text(lore3, None, 10, 1.0).width;
        draw_text(lore3, GAME_W / 2.0 - l3w / 2.0, GAME_H * 0.3 + 88.0, 10.0, GRAY);

        let hint1 = "Collect drops for Shields, Speed & Weapons";
        let h1w = measure_text(hint1, None, 12, 1.0).width;
        draw_text(hint1, GAME_W / 2.0 - h1w / 2.0, GAME_H * 0.58, 12.0, GRAY);

        let hint2 = "D-Pad: Move   A(X): Shoot   Enter: Start";
        let h2w = measure_text(hint2, None, 12, 1.0).width;
        draw_text(hint2, GAME_W / 2.0 - h2w / 2.0, GAME_H * 0.58 + 20.0, 12.0, GRAY);

        // Blinking prompt
        if (self.frame / 30) % 2 == 0 {
            let prompt = "PRESS ENTER TO LAUNCH FIGHTER";
            let pw = measure_text(prompt, None, 16, 1.0).width;
            draw_text(prompt, GAME_W / 2.0 - pw / 2.0, GAME_H * 0.75, 16.0, SKYBLUE);
        }
    }

    fn draw_story(&self) {
        let overlay = Color::new(0.0, 0.0, 0.0, 0.75);
        draw_rectangle(0.0, 0.0, GAME_W, GAME_H, overlay);

        // Wave codename header (if not victory epilogue)
        if !self.victory_triggered {
            let wave = self.current_wave();
            let header = format!("// WAVE {}: {} //", self.wave_idx + 1, wave.name);
            let hw = measure_text(&header, None, 14, 1.0).width;
            draw_text(&header, GAME_W / 2.0 - hw / 2.0, GAME_H * 0.1, 14.0, MAGENTA);
        }

        // Draw story text line by line
        let margin = 60.0;
        let mut y = GAME_H * 0.2;
        for line in self.story_displayed.lines() {
            draw_text(line, margin, y, 16.0, SKYBLUE);
            y += 24.0;
        }

        // Blinking cursor at end of typewriter text
        if self.story_char_idx < self.story_text.len() {
            if (self.frame / 8) % 2 == 0 {
                let last_line = self.story_displayed.lines().last().unwrap_or("");
                let lw = measure_text(last_line, None, 16, 1.0).width;
                // y is already past the last line, so subtract one line height
                draw_rectangle(margin + lw + 2.0, y - 24.0 - 12.0, 8.0, 14.0, SKYBLUE);
            }
        }

        // Skip hint
        if self.story_char_idx >= self.story_text.len() {
            if (self.frame / 20) % 2 == 0 {
                let skip = if self.victory_triggered {
                    "Press ENTER to continue..."
                } else {
                    "Press ENTER to begin..."
                };
                let sw = measure_text(skip, None, 14, 1.0).width;
                draw_text(skip, GAME_W / 2.0 - sw / 2.0, GAME_H * 0.85, 14.0, MAGENTA);
            }
        }
    }

    fn draw_game_over(&self) {
        let overlay = Color::new(0.0, 0.0, 0.0, 0.75);
        draw_rectangle(0.0, 0.0, GAME_W, GAME_H, overlay);

        let title = "SIGNAL LOST";
        let tw = measure_text(title, None, 32, 1.0).width;
        draw_text(title, GAME_W / 2.0 - tw / 2.0, GAME_H * 0.3, 32.0, RED);

        let sc = format!("FINAL SCORE: {}", self.score);
        let sw = measure_text(&sc, None, 18, 1.0).width;
        draw_text(&sc, GAME_W / 2.0 - sw / 2.0, GAME_H * 0.45, 18.0, WHITE);

        let wave_name = self.current_wave().name;
        let wv = format!("FALLEN AT: {}", wave_name);
        let ww = measure_text(&wv, None, 16, 1.0).width;
        draw_text(&wv, GAME_W / 2.0 - ww / 2.0, GAME_H * 0.45 + 26.0, 16.0, SKYBLUE);

        let lore = "EXODUS-7 drifts into the Void. The drones close in.";
        let lw = measure_text(lore, None, 10, 1.0).width;
        draw_text(lore, GAME_W / 2.0 - lw / 2.0, GAME_H * 0.45 + 52.0, 10.0, GRAY);

        if (self.frame / 30) % 2 == 0 {
            let prompt = "PRESS ENTER TO RELAUNCH";
            let pw = measure_text(prompt, None, 16, 1.0).width;
            draw_text(prompt, GAME_W / 2.0 - pw / 2.0, GAME_H * 0.70, 16.0, MAGENTA);
        }
    }

    fn draw_victory(&self) {
        let overlay = Color::new(0.0, 0.0, 0.0, 0.75);
        draw_rectangle(0.0, 0.0, GAME_W, GAME_H, overlay);

        let title = "MISSION COMPLETE";
        let tw = measure_text(title, None, 32, 1.0).width;
        draw_text(title, GAME_W / 2.0 - tw / 2.0, GAME_H * 0.25, 32.0, GREEN);

        let sub = "THE VOID IS SILENT.";
        let sw = measure_text(sub, None, 14, 1.0).width;
        draw_text(sub, GAME_W / 2.0 - sw / 2.0, GAME_H * 0.35, 14.0, SKYBLUE);

        let sc = format!("FINAL SCORE: {}", self.score);
        let sw2 = measure_text(&sc, None, 20, 1.0).width;
        draw_text(&sc, GAME_W / 2.0 - sw2 / 2.0, GAME_H * 0.5, 20.0, YELLOW);

        if (self.frame / 30) % 2 == 0 {
            let prompt = "PRESS ENTER TO PLAY AGAIN";
            let pw = measure_text(prompt, None, 16, 1.0).width;
            draw_text(prompt, GAME_W / 2.0 - pw / 2.0, GAME_H * 0.65, 16.0, MAGENTA);
        }
    }
}

// ---------------------------------------------------------------------------
// Bullet helper
// ---------------------------------------------------------------------------
fn mk_bullet(x: f32, y: f32, vx: f32, vy: f32, color: Color, is_player: bool) -> Bullet {
    Bullet {
        x,
        y,
        vx,
        vy,
        w: 3.0,
        h: 12.0,
        is_player,
        alive: true,
        color,
    }
}

// ---------------------------------------------------------------------------
// Macroquad window config
// ---------------------------------------------------------------------------
fn window_conf() -> Conf {
    Conf {
        window_title: "Neon Defender - The Last Signal".to_owned(),
        window_width: GAME_W as i32,
        window_height: GAME_H as i32,
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

    // Fixed timestep at ~60 fps
    const DT: f64 = 1.0 / 60.0;
    let mut accumulator: f64 = 0.0;
    let mut last_time = get_time();

    loop {
        let now = get_time();
        let mut elapsed = now - last_time;
        last_time = now;

        // Death spiral prevention: cap frame time
        if elapsed > 0.1 {
            elapsed = 0.1;
        }

        accumulator += elapsed;

        while accumulator >= DT {
            game.update();
            accumulator -= DT;
        }

        game.draw();
        next_frame().await;
    }
}
