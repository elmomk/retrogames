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
    text: &'static str,
    target_kills: u32, // u32::MAX = infinity wave
    spawn_rate: u32,
    enemy_types: &'static [u8],
}

const WAVES: &[WaveInfo] = &[
    WaveInfo {
        text: "SYSTEM BOOT: YEAR 2084.\n\nTHE NEON SYNDICATE HAS BREACHED\nTHE OUTER MAINFRAME.\n\nYOU ARE THE LAST DEFENDER PROGRAM.\n\nPURGE THE BASIC INFECTIONS.",
        target_kills: 15,
        spawn_rate: 60,
        enemy_types: &[0],
    },
    WaveInfo {
        text: "WARNING: VIRUS MUTATION DETECTED.\n\nFAST-ATTACK VECTORS COMPROMISING\nPROXY SERVERS.\n\nADAPT AND DESTROY.",
        target_kills: 35,
        spawn_rate: 45,
        enemy_types: &[0, 1],
    },
    WaveInfo {
        text: "CRITICAL ALERT: FIREWALL BREACHED.\n\nARMED LOGIC BOMBS DEPLOYED.\nEVASIVE MANEUVERS REQUIRED.\n\nDEFEND THE CORE.",
        target_kills: 60,
        spawn_rate: 40,
        enemy_types: &[0, 1, 2],
    },
    WaveInfo {
        text: "SYSTEM OVERLOAD IMMINENT.\n\nTHE SYNDICATE IS POURING IN.\n\nSURVIVE AT ALL COSTS.",
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
    story_char_idx: usize,
    story_displayed: String,
    story_wait: i32,

    // shoot held
    shoot_held: bool,
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
            story_char_idx: 0,
            story_displayed: String::new(),
            story_wait: 0,
            shoot_held: false,
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
    fn start_story(&mut self) {
        if self.state == GameState::Title || self.state == GameState::GameOver || self.state == GameState::Victory {
            self.score = 0;
            self.lives = 3;
            self.kills = 0;
            self.wave_idx = 0;
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
                self.start_story();
            }
            return;
        }

        // ----- Story -----
        if self.state == GameState::Story {
            let wave = self.current_wave();
            let txt = wave.text;
            if self.story_char_idx < txt.len() {
                if self.frame % 2 == 0 {
                    // All story text is ASCII so byte index is fine
                    self.story_displayed.push(txt.as_bytes()[self.story_char_idx] as char);
                    self.story_char_idx += 1;
                }
                // Allow skipping with Enter/X
                if enter || is_key_pressed(KeyCode::X) {
                    self.story_displayed = txt.to_string();
                    self.story_char_idx = txt.len();
                }
            } else {
                if self.story_wait == 0 {
                    self.story_wait = 150;
                }
                self.story_wait -= 1;
                if self.story_wait <= 0 || enter || is_key_pressed(KeyCode::X) {
                    self.start_wave();
                }
            }
            self.frame += 1;
            return;
        }

        // ----- Game Over / Victory -----
        if self.state == GameState::GameOver || self.state == GameState::Victory {
            if enter || is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Space) {
                self.start_story();
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

        // Check wave cleared
        let target = self.current_wave().target_kills;
        if self.kills >= target {
            if !self.wave_cleared {
                self.wave_cleared = true;
                self.slowmo_timer = 30;
            }
            if self.slowmo_timer > 0 {
                self.slowmo_timer -= 1;
                if self.frame % 2 == 0 {
                    return;
                }
            }
            if self.slowmo_timer <= 0 {
                self.wave_idx += 1;
                if self.wave_idx >= WAVES.len() {
                    self.state = GameState::Victory;
                    return;
                }
                self.kills = 0;
                self.start_story();
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

        // Stars
        for s in &self.stars {
            let c = Color::new(1.0, 1.0, 1.0, s.brightness);
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
        let wave_txt = format!("WAVE: {}", self.wave_idx + 1);
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

        let sub = "SYSTEM REBOOT";
        let sw = measure_text(sub, None, 22, 1.0).width;
        draw_text(sub, GAME_W / 2.0 - sw / 2.0, GAME_H * 0.3 + 35.0, 22.0, SKYBLUE);

        let hint1 = "Collect drops for Shields, Speed & Weapons";
        let h1w = measure_text(hint1, None, 12, 1.0).width;
        draw_text(hint1, GAME_W / 2.0 - h1w / 2.0, GAME_H * 0.55, 12.0, GRAY);

        let hint2 = "D-Pad: Move   A(X): Shoot   Enter: Start";
        let h2w = measure_text(hint2, None, 12, 1.0).width;
        draw_text(hint2, GAME_W / 2.0 - h2w / 2.0, GAME_H * 0.55 + 20.0, 12.0, GRAY);

        // Blinking prompt
        if (self.frame / 30) % 2 == 0 {
            let prompt = "PRESS ENTER TO INITIALIZE";
            let pw = measure_text(prompt, None, 16, 1.0).width;
            draw_text(prompt, GAME_W / 2.0 - pw / 2.0, GAME_H * 0.72, 16.0, SKYBLUE);
        }
    }

    fn draw_story(&self) {
        let overlay = Color::new(0.0, 0.0, 0.0, 0.75);
        draw_rectangle(0.0, 0.0, GAME_W, GAME_H, overlay);

        // Draw story text line by line
        let margin = 60.0;
        let mut y = GAME_H * 0.2;
        for line in self.story_displayed.lines() {
            draw_text(line, margin, y, 16.0, SKYBLUE);
            y += 24.0;
        }

        // Skip hint
        let wave = self.current_wave();
        if self.story_char_idx >= wave.text.len() {
            if (self.frame / 20) % 2 == 0 {
                let skip = "Press ENTER to begin...";
                let sw = measure_text(skip, None, 14, 1.0).width;
                draw_text(skip, GAME_W / 2.0 - sw / 2.0, GAME_H * 0.85, 14.0, MAGENTA);
            }
        }
    }

    fn draw_game_over(&self) {
        let overlay = Color::new(0.0, 0.0, 0.0, 0.75);
        draw_rectangle(0.0, 0.0, GAME_W, GAME_H, overlay);

        let title = "FATAL ERROR";
        let tw = measure_text(title, None, 32, 1.0).width;
        draw_text(title, GAME_W / 2.0 - tw / 2.0, GAME_H * 0.3, 32.0, RED);

        let sc = format!("FINAL SCORE: {}", self.score);
        let sw = measure_text(&sc, None, 18, 1.0).width;
        draw_text(&sc, GAME_W / 2.0 - sw / 2.0, GAME_H * 0.45, 18.0, WHITE);

        let wv = format!("WAVES CLEARED: {}", self.wave_idx);
        let ww = measure_text(&wv, None, 16, 1.0).width;
        draw_text(&wv, GAME_W / 2.0 - ww / 2.0, GAME_H * 0.45 + 26.0, 16.0, SKYBLUE);

        if (self.frame / 30) % 2 == 0 {
            let prompt = "PRESS ENTER TO REBOOT";
            let pw = measure_text(prompt, None, 16, 1.0).width;
            draw_text(prompt, GAME_W / 2.0 - pw / 2.0, GAME_H * 0.65, 16.0, MAGENTA);
        }
    }

    fn draw_victory(&self) {
        let overlay = Color::new(0.0, 0.0, 0.0, 0.75);
        draw_rectangle(0.0, 0.0, GAME_W, GAME_H, overlay);

        let title = "SYSTEM SECURED";
        let tw = measure_text(title, None, 32, 1.0).width;
        draw_text(title, GAME_W / 2.0 - tw / 2.0, GAME_H * 0.25, 32.0, GREEN);

        let sub = "THE NEON SYNDICATE HAS BEEN PURGED";
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
        window_title: "Neon Defender".to_owned(),
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
