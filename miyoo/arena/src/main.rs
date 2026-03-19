use macroquad::prelude::*;

// ─── Constants ───────────────────────────────────────────────────────────────
const SCREEN_W: f32 = 640.0;
const SCREEN_H: f32 = 480.0;
const BORDER: f32 = 40.0;
const ARENA_X: f32 = BORDER;
const ARENA_Y: f32 = BORDER;
const ARENA_W: f32 = SCREEN_W - BORDER * 2.0;
const ARENA_H: f32 = SCREEN_H - BORDER * 2.0;

const PLAYER_SIZE: f32 = 16.0;
const PLAYER_HITBOX: f32 = 12.0;
const PLAYER_BASE_SPEED: f32 = 3.0;
const PLAYER_MAX_HP: f32 = 100.0;
const INVINCIBILITY_FRAMES: i32 = 60;

const MAX_WAVES: usize = 10;

const COMBO_TIMEOUT: f32 = 2.0;
const MAX_COMBO_MULT: f32 = 8.0;

// ─── Enums ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Title,
    Playing,
    WaveIntro,
    GameOver,
    Victory,
}

#[derive(Clone, Copy, PartialEq)]
enum WeaponType {
    Pistol,
    Shotgun,
    Laser,
    Rocket,
}

#[derive(Clone, Copy, PartialEq)]
enum EnemyType {
    Swarmer,
    Tank,
    Teleporter,
    Splitter,
    SplitterSmall,
    MegaTank,
    SwarmQueen,
}

#[derive(Clone, Copy, PartialEq)]
enum PickupKind {
    Shotgun,
    Laser,
    Rocket,
    SpeedBoost,
    Shield,
    DoubleDamage,
    Freeze,
}

// ─── Structs ─────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct Player {
    x: f32,
    y: f32,
    hp: f32,
    speed: f32,
    invincible: i32,
    current_weapon: WeaponType,
    ammo_shotgun: i32,
    ammo_laser: i32,
    ammo_rocket: i32,
    fire_cooldown: f32,
    aim_angle: f32,
    score: u32,
    combo_count: u32,
    combo_timer: f32,
    combo_multiplier: f32,
    // Power-up timers
    speed_boost_timer: f32,
    shield_hp: f32,
    double_damage_timer: f32,
    freeze_timer: f32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: SCREEN_W / 2.0,
            y: SCREEN_H / 2.0,
            hp: PLAYER_MAX_HP,
            speed: PLAYER_BASE_SPEED,
            invincible: 0,
            current_weapon: WeaponType::Pistol,
            ammo_shotgun: 0,
            ammo_laser: 0,
            ammo_rocket: 0,
            fire_cooldown: 0.0,
            aim_angle: 0.0,
            score: 0,
            combo_count: 0,
            combo_timer: 0.0,
            combo_multiplier: 1.0,
            speed_boost_timer: 0.0,
            shield_hp: 0.0,
            double_damage_timer: 0.0,
            freeze_timer: 0.0,
        }
    }

    fn effective_speed(&self) -> f32 {
        if self.speed_boost_timer > 0.0 {
            self.speed * 1.5
        } else {
            self.speed
        }
    }

    fn damage_multiplier(&self) -> f32 {
        if self.double_damage_timer > 0.0 {
            2.0
        } else {
            1.0
        }
    }
}

#[derive(Clone)]
struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    damage: f32,
    alive: bool,
    lifetime: f32,
    is_rocket: bool,
    traveled: f32,
}

#[derive(Clone)]
struct EnemyBullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    damage: f32,
    alive: bool,
    homing: bool,
}

#[derive(Clone)]
struct Enemy {
    x: f32,
    y: f32,
    hp: f32,
    max_hp: f32,
    speed: f32,
    size: f32,
    enemy_type: EnemyType,
    alive: bool,
    fire_timer: f32,
    teleport_timer: f32,
    teleporting: bool,
    teleport_cooldown: f32,
    spawn_timer: f32,
    flash_timer: f32,
    weave_offset: f32,
}

impl Enemy {
    fn new(etype: EnemyType, x: f32, y: f32) -> Self {
        let (hp, speed, size) = match etype {
            EnemyType::Swarmer => (15.0, 2.5, 10.0),
            EnemyType::Tank => (80.0, 1.0, 20.0),
            EnemyType::Teleporter => (30.0, 2.0, 14.0),
            EnemyType::Splitter => (50.0, 1.5, 18.0),
            EnemyType::SplitterSmall => (20.0, 3.0, 10.0),
            EnemyType::MegaTank => (300.0, 0.7, 32.0),
            EnemyType::SwarmQueen => (500.0, 1.5, 24.0),
        };
        Self {
            x,
            y,
            hp,
            max_hp: hp,
            speed,
            size,
            enemy_type: etype,
            alive: true,
            fire_timer: rand::gen_range(1.0, 3.0),
            teleport_timer: rand::gen_range(2.0, 4.0),
            teleporting: false,
            teleport_cooldown: 0.0,
            spawn_timer: 5.0,
            flash_timer: 0.0,
            weave_offset: rand::gen_range(0.0, std::f32::consts::TAU),
        }
    }

    fn color(&self) -> Color {
        match self.enemy_type {
            EnemyType::Swarmer => GREEN,
            EnemyType::Tank => Color::new(1.0, 0.4, 0.1, 1.0),
            EnemyType::Teleporter => MAGENTA,
            EnemyType::Splitter | EnemyType::SplitterSmall => Color::new(0.0, 0.8, 0.8, 1.0),
            EnemyType::MegaTank => Color::new(1.0, 0.2, 0.0, 1.0),
            EnemyType::SwarmQueen => Color::new(0.8, 0.0, 1.0, 1.0),
        }
    }
}

#[derive(Clone)]
struct Pickup {
    x: f32,
    y: f32,
    kind: PickupKind,
    timer: f32,
    alive: bool,
}

#[derive(Clone)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    lifetime: f32,
    max_lifetime: f32,
    color: Color,
    size: f32,
    alive: bool,
}

#[derive(Clone)]
struct ExplosionRing {
    x: f32,
    y: f32,
    max_radius: f32,
    frame: i32,
    max_frames: i32,
    alive: bool,
}

#[derive(Clone)]
struct SpawnTeleportEffect {
    x: f32,
    y: f32,
    frame: i32,
    max_frames: i32,
    alive: bool,
}

#[derive(Clone)]
struct KillStreakText {
    text: String,
    frame: i32,
    max_frames: i32,
    alive: bool,
}

// ─── Game ────────────────────────────────────────────────────────────────────

struct Game {
    state: GameState,
    player: Player,
    bullets: Vec<Bullet>,
    enemy_bullets: Vec<EnemyBullet>,
    enemies: Vec<Enemy>,
    pickups: Vec<Pickup>,
    particles: Vec<Particle>,
    wave: usize,
    enemies_to_spawn: i32,
    spawn_timer: f32,
    wave_intro_timer: f32,
    wave_enemies_killed: i32,
    wave_total_enemies: i32,
    screen_shake: f32,
    screen_shake_x: f32,
    screen_shake_y: f32,
    fence_pulse: f32,
    time: f32,
    laser_active: bool,
    laser_end_x: f32,
    laser_end_y: f32,
    blink_timer: f32,
    score_pop_timer: f32,
    score_pop_value: u32,
    explosion_rings: Vec<ExplosionRing>,
    spawn_effects: Vec<SpawnTeleportEffect>,
    kill_streak_texts: Vec<KillStreakText>,
    damage_border_frames: i32,
    fence_spark_timer: i32,
    last_streak_milestone: u32,
}

impl Game {
    fn new() -> Self {
        Self {
            state: GameState::Title,
            player: Player::new(),
            bullets: Vec::new(),
            enemy_bullets: Vec::new(),
            enemies: Vec::new(),
            pickups: Vec::new(),
            particles: Vec::new(),
            wave: 0,
            enemies_to_spawn: 0,
            spawn_timer: 0.0,
            wave_intro_timer: 0.0,
            wave_enemies_killed: 0,
            wave_total_enemies: 0,
            screen_shake: 0.0,
            screen_shake_x: 0.0,
            screen_shake_y: 0.0,
            fence_pulse: 0.0,
            time: 0.0,
            laser_active: false,
            laser_end_x: 0.0,
            laser_end_y: 0.0,
            blink_timer: 0.0,
            score_pop_timer: 0.0,
            score_pop_value: 0,
            explosion_rings: Vec::new(),
            spawn_effects: Vec::new(),
            kill_streak_texts: Vec::new(),
            damage_border_frames: 0,
            fence_spark_timer: 0,
            last_streak_milestone: 0,
        }
    }

    fn reset(&mut self) {
        self.player = Player::new();
        self.bullets.clear();
        self.enemy_bullets.clear();
        self.enemies.clear();
        self.pickups.clear();
        self.particles.clear();
        self.wave = 0;
        self.enemies_to_spawn = 0;
        self.spawn_timer = 0.0;
        self.wave_enemies_killed = 0;
        self.wave_total_enemies = 0;
        self.screen_shake = 0.0;
        self.laser_active = false;
        self.score_pop_timer = 0.0;
        self.score_pop_value = 0;
        self.explosion_rings.clear();
        self.spawn_effects.clear();
        self.kill_streak_texts.clear();
        self.damage_border_frames = 0;
        self.fence_spark_timer = 0;
        self.last_streak_milestone = 0;
    }

    fn start_wave(&mut self) {
        self.wave += 1;
        self.wave_enemies_killed = 0;
        self.state = GameState::WaveIntro;
        self.wave_intro_timer = 3.0;

        let (count, _types) = wave_config(self.wave);
        self.wave_total_enemies = count;
        self.enemies_to_spawn = count;
        self.spawn_timer = 0.5;
    }

    fn add_score(&mut self, base: u32) {
        self.player.combo_count += 1;
        self.player.combo_timer = COMBO_TIMEOUT;
        self.player.combo_multiplier =
            (1.0 + (self.player.combo_count as f32 - 1.0) * 0.5).min(MAX_COMBO_MULT);

        let points = (base as f32 * self.player.combo_multiplier) as u32;
        self.player.score += points;
        self.score_pop_timer = 0.5;
        self.score_pop_value = points;

        // Kill streak milestones
        let combo = self.player.combo_count;
        let milestone = if combo >= 20 { 20 } else if combo >= 15 { 15 } else if combo >= 10 { 10 } else if combo >= 5 { 5 } else { 0 };
        if milestone > 0 && milestone > self.last_streak_milestone {
            self.last_streak_milestone = milestone;
            let text = match milestone {
                5 => "KILLING SPREE!",
                10 => "UNSTOPPABLE!",
                15 => "GODLIKE!",
                _ => "LEGENDARY!",
            };
            self.kill_streak_texts.push(KillStreakText {
                text: text.to_string(),
                frame: 0,
                max_frames: 60,
                alive: true,
            });
        }
    }

    fn spawn_particles(&mut self, x: f32, y: f32, color: Color, count: usize) {
        for _ in 0..count {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let speed = rand::gen_range(1.0, 4.0);
            self.particles.push(Particle {
                x,
                y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                lifetime: rand::gen_range(0.3, 0.8),
                max_lifetime: rand::gen_range(0.3, 0.8),
                color,
                size: rand::gen_range(2.0, 5.0),
                alive: true,
            });
        }
    }

    fn add_screen_shake(&mut self, amount: f32) {
        self.screen_shake = self.screen_shake.max(amount);
    }

    fn spawn_enemy(&mut self, etype: EnemyType) {
        // Spawn at random edge
        let side = rand::gen_range(0, 4);
        let (x, y) = match side {
            0 => (rand::gen_range(ARENA_X + 10.0, ARENA_X + ARENA_W - 10.0), ARENA_Y + 5.0),
            1 => (rand::gen_range(ARENA_X + 10.0, ARENA_X + ARENA_W - 10.0), ARENA_Y + ARENA_H - 5.0),
            2 => (ARENA_X + 5.0, rand::gen_range(ARENA_Y + 10.0, ARENA_Y + ARENA_H - 10.0)),
            _ => (ARENA_X + ARENA_W - 5.0, rand::gen_range(ARENA_Y + 10.0, ARENA_Y + ARENA_H - 10.0)),
        };
        self.enemies.push(Enemy::new(etype, x, y));
        self.spawn_effects.push(SpawnTeleportEffect {
            x,
            y,
            frame: 0,
            max_frames: 15,
            alive: true,
        });
    }

    fn try_spawn_enemies(&mut self) {
        if self.enemies_to_spawn <= 0 {
            return;
        }
        self.spawn_timer -= 1.0 / 60.0;
        if self.spawn_timer <= 0.0 {
            let types = wave_enemy_types(self.wave);
            let etype = types[rand::gen_range(0, types.len())];

            // Boss spawns
            if self.wave == 5 && self.enemies_to_spawn == 1 {
                self.spawn_enemy(EnemyType::MegaTank);
            } else if self.wave == 10 && self.enemies_to_spawn == 1 {
                self.spawn_enemy(EnemyType::SwarmQueen);
            } else {
                // Spawn a small group of swarmers or single others
                let group = if etype == EnemyType::Swarmer {
                    rand::gen_range(2, 5).min(self.enemies_to_spawn)
                } else {
                    1
                };
                for _ in 0..group {
                    self.spawn_enemy(etype);
                    self.enemies_to_spawn -= 1;
                    if self.enemies_to_spawn <= 0 {
                        break;
                    }
                }
                // We already decremented, so adjust
                self.enemies_to_spawn += 1; // compensate the one below
            }
            self.enemies_to_spawn -= 1;
            self.spawn_timer = rand::gen_range(0.5, 1.5);
        }
    }

    fn player_take_damage(&mut self, damage: f32) {
        if self.player.invincible > 0 {
            return;
        }
        let actual = if self.player.shield_hp > 0.0 {
            let absorbed = damage.min(self.player.shield_hp);
            self.player.shield_hp -= absorbed;
            damage - absorbed
        } else {
            damage
        };
        self.player.hp -= actual;
        self.player.invincible = INVINCIBILITY_FRAMES;
        self.add_screen_shake(5.0);
        self.spawn_particles(self.player.x, self.player.y, RED, 6);
        self.damage_border_frames = 15;

        if self.player.hp <= 0.0 {
            self.player.hp = 0.0;
            self.state = GameState::GameOver;
        }
    }

    fn drop_pickup(&mut self, x: f32, y: f32) {
        // 15% chance power-up
        if rand::gen_range(0.0, 1.0) < 0.15 {
            let kind = match rand::gen_range(0, 4) {
                0 => PickupKind::SpeedBoost,
                1 => PickupKind::Shield,
                2 => PickupKind::DoubleDamage,
                _ => PickupKind::Freeze,
            };
            self.pickups.push(Pickup {
                x,
                y,
                kind,
                timer: 10.0,
                alive: true,
            });
        }
        // 10% chance weapon pickup
        if rand::gen_range(0.0, 1.0) < 0.10 {
            let kind = match rand::gen_range(0, 3) {
                0 => PickupKind::Shotgun,
                1 => PickupKind::Laser,
                _ => PickupKind::Rocket,
            };
            self.pickups.push(Pickup {
                x,
                y: y + 10.0,
                kind,
                timer: 10.0,
                alive: true,
            });
        }
    }

    fn fire_weapon(&mut self) {
        let dt_cooldown = 1.0 / 60.0;
        if self.player.fire_cooldown > 0.0 {
            return;
        }

        let angle = self.player.aim_angle;
        let px = self.player.x;
        let py = self.player.y;
        let dmg_mult = self.player.damage_multiplier();

        match self.player.current_weapon {
            WeaponType::Pistol => {
                let spread = rand::gen_range(-0.035, 0.035);
                let a = angle + spread;
                self.bullets.push(Bullet {
                    x: px,
                    y: py,
                    vx: a.cos() * 8.0,
                    vy: a.sin() * 8.0,
                    damage: 10.0 * dmg_mult,
                    alive: true,
                    lifetime: 2.0,
                    is_rocket: false,
                    traveled: 0.0,
                });
                self.player.fire_cooldown = 10.0 * dt_cooldown; // 6 rps
            }
            WeaponType::Shotgun => {
                if self.player.ammo_shotgun <= 0 {
                    self.player.current_weapon = WeaponType::Pistol;
                    return;
                }
                self.player.ammo_shotgun -= 1;
                let spread_total = 0.524; // ~30 degrees
                for i in 0..5 {
                    let a = angle - spread_total / 2.0
                        + spread_total * (i as f32 / 4.0)
                        + rand::gen_range(-0.03, 0.03);
                    self.bullets.push(Bullet {
                        x: px,
                        y: py,
                        vx: a.cos() * 7.0,
                        vy: a.sin() * 7.0,
                        damage: 8.0 * dmg_mult,
                        alive: true,
                        lifetime: 0.36, // ~150px range at 7px/frame
                        is_rocket: false,
                        traveled: 0.0,
                    });
                }
                self.player.fire_cooldown = 30.0 * dt_cooldown; // 2 rps
            }
            WeaponType::Laser => {
                if self.player.ammo_laser <= 0 {
                    self.player.current_weapon = WeaponType::Pistol;
                    return;
                }
                self.player.ammo_laser -= 1;
                // Raycast laser
                self.laser_active = true;
                let max_dist = 800.0;
                let step = 4.0;
                let mut dist = 0.0;
                let mut lx = px;
                let mut ly = py;
                let cos_a = angle.cos();
                let sin_a = angle.sin();

                while dist < max_dist {
                    lx += cos_a * step;
                    ly += sin_a * step;
                    dist += step;

                    // Out of arena
                    if lx < ARENA_X || lx > ARENA_X + ARENA_W || ly < ARENA_Y || ly > ARENA_Y + ARENA_H {
                        break;
                    }

                    // Hit enemies (pierce through)
                    for enemy in self.enemies.iter_mut() {
                        if !enemy.alive {
                            continue;
                        }
                        if enemy.teleporting {
                            continue;
                        }
                        let half = enemy.size / 2.0;
                        if lx > enemy.x - half
                            && lx < enemy.x + half
                            && ly > enemy.y - half
                            && ly < enemy.y + half
                        {
                            enemy.hp -= 3.0 * dmg_mult;
                            enemy.flash_timer = 0.05;
                        }
                    }
                }
                self.laser_end_x = lx;
                self.laser_end_y = ly;
                self.player.fire_cooldown = 1.0 * dt_cooldown; // continuous
            }
            WeaponType::Rocket => {
                if self.player.ammo_rocket <= 0 {
                    self.player.current_weapon = WeaponType::Pistol;
                    return;
                }
                self.player.ammo_rocket -= 1;
                self.bullets.push(Bullet {
                    x: px,
                    y: py,
                    vx: angle.cos() * 5.0,
                    vy: angle.sin() * 5.0,
                    damage: 30.0 * dmg_mult,
                    alive: true,
                    lifetime: 5.0,
                    is_rocket: true,
                    traveled: 0.0,
                });
                self.player.fire_cooldown = 60.0 * dt_cooldown; // 1 rps
            }
        }
    }

    fn rocket_explode(&mut self, x: f32, y: f32, dmg_mult: f32) {
        self.add_screen_shake(8.0);
        self.spawn_particles(x, y, ORANGE, 16);
        self.spawn_particles(x, y, YELLOW, 8);

        self.explosion_rings.push(ExplosionRing {
            x,
            y,
            max_radius: 40.0,
            frame: 0,
            max_frames: 10,
            alive: true,
        });

        let splash_radius = 40.0;
        for enemy in self.enemies.iter_mut() {
            if !enemy.alive {
                continue;
            }
            let dx = enemy.x - x;
            let dy = enemy.y - y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < splash_radius + enemy.size / 2.0 {
                enemy.hp -= 20.0 * dmg_mult;
                enemy.flash_timer = 0.1;
            }
        }
    }

    fn update(&mut self) {
        let dt = 1.0 / 60.0;
        self.time += dt;
        self.fence_pulse = (self.time * 4.0).sin() * 0.3 + 0.7;
        self.blink_timer += dt;

        match self.state {
            GameState::Title => {
                if is_key_pressed(KeyCode::Enter) {
                    self.reset();
                    self.start_wave();
                }
            }
            GameState::WaveIntro => {
                self.wave_intro_timer -= dt;
                if self.wave_intro_timer <= 0.0 {
                    self.state = GameState::Playing;
                }
            }
            GameState::Playing => {
                self.update_playing(dt);
            }
            GameState::GameOver | GameState::Victory => {
                if is_key_pressed(KeyCode::Enter) {
                    self.state = GameState::Title;
                }
            }
        }
    }

    fn update_playing(&mut self, dt: f32) {
        // ── Player movement ──
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            dy -= 1.0;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            dy += 1.0;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            dx -= 1.0;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            dx += 1.0;
        }

        // Normalize diagonal
        let len = (dx * dx + dy * dy).sqrt();
        if len > 0.0 {
            dx /= len;
            dy /= len;
        }

        let spd = self.player.effective_speed();
        self.player.x += dx * spd;
        self.player.y += dy * spd;

        // Clamp to arena
        let half = PLAYER_HITBOX / 2.0;
        self.player.x = self.player.x.clamp(ARENA_X + half, ARENA_X + ARENA_W - half);
        self.player.y = self.player.y.clamp(ARENA_Y + half, ARENA_Y + ARENA_H - half);

        // Electric fence damage
        let px = self.player.x;
        let py = self.player.y;
        if px - half < ARENA_X + 3.0
            || px + half > ARENA_X + ARENA_W - 3.0
            || py - half < ARENA_Y + 3.0
            || py + half > ARENA_Y + ARENA_H - 3.0
        {
            self.player_take_damage(0.5);
        }

        // ── Auto-aim at nearest enemy ──
        let mut nearest_dist = f32::MAX;
        let mut nearest_angle = self.player.aim_angle;
        for enemy in &self.enemies {
            if !enemy.alive || enemy.teleporting {
                continue;
            }
            let edx = enemy.x - self.player.x;
            let edy = enemy.y - self.player.y;
            let dist = (edx * edx + edy * edy).sqrt();
            if dist < nearest_dist {
                nearest_dist = dist;
                nearest_angle = edy.atan2(edx);
            }
        }
        self.player.aim_angle = nearest_angle;

        // ── Invincibility ──
        if self.player.invincible > 0 {
            self.player.invincible -= 1;
        }

        // ── Fire cooldown ──
        if self.player.fire_cooldown > 0.0 {
            self.player.fire_cooldown -= dt;
        }

        // ── Laser off by default ──
        self.laser_active = false;

        // ── Shoot (A button = KeyCode::X) ──
        let shooting = if self.player.current_weapon == WeaponType::Laser {
            is_key_down(KeyCode::X)
        } else {
            is_key_pressed(KeyCode::X)
        };
        if shooting && !self.enemies.is_empty() {
            self.fire_weapon();
        }

        // ── Check for laser kills (enemies reduced to 0 HP by laser) ──
        {
            let mut laser_kills: Vec<(f32, f32, EnemyType, u32)> = Vec::new();
            for enemy in self.enemies.iter_mut() {
                if enemy.alive && enemy.hp <= 0.0 {
                    enemy.alive = false;
                    let score = match enemy.enemy_type {
                        EnemyType::Swarmer => 10,
                        EnemyType::Tank => 50,
                        EnemyType::Teleporter => 30,
                        EnemyType::Splitter => 40,
                        EnemyType::SplitterSmall => 15,
                        EnemyType::MegaTank => 200,
                        EnemyType::SwarmQueen => 500,
                    };
                    laser_kills.push((enemy.x, enemy.y, enemy.enemy_type, score));
                }
            }
            for (kx, ky, etype, score) in &laser_kills {
                self.add_score(*score);
                self.spawn_particles(*kx, *ky, WHITE, 8);
                self.drop_pickup(*kx, *ky);
                self.wave_enemies_killed += 1;
                if *etype == EnemyType::Splitter {
                    let count = rand::gen_range(2, 4) as usize;
                    for _ in 0..count {
                        let ox = rand::gen_range(-15.0, 15.0);
                        let oy = rand::gen_range(-15.0, 15.0);
                        self.enemies.push(Enemy::new(EnemyType::SplitterSmall, kx + ox, ky + oy));
                    }
                }
            }
        }

        // ── Switch weapon (B button = Space) ──
        if is_key_pressed(KeyCode::Space) {
            self.player.current_weapon = match self.player.current_weapon {
                WeaponType::Pistol => {
                    if self.player.ammo_shotgun > 0 {
                        WeaponType::Shotgun
                    } else if self.player.ammo_laser > 0 {
                        WeaponType::Laser
                    } else if self.player.ammo_rocket > 0 {
                        WeaponType::Rocket
                    } else {
                        WeaponType::Pistol
                    }
                }
                WeaponType::Shotgun => {
                    if self.player.ammo_laser > 0 {
                        WeaponType::Laser
                    } else if self.player.ammo_rocket > 0 {
                        WeaponType::Rocket
                    } else {
                        WeaponType::Pistol
                    }
                }
                WeaponType::Laser => {
                    if self.player.ammo_rocket > 0 {
                        WeaponType::Rocket
                    } else {
                        WeaponType::Pistol
                    }
                }
                WeaponType::Rocket => WeaponType::Pistol,
            };
        }

        // ── Combo timer ──
        if self.player.combo_timer > 0.0 {
            self.player.combo_timer -= dt;
            if self.player.combo_timer <= 0.0 {
                self.player.combo_count = 0;
                self.player.combo_multiplier = 1.0;
                self.last_streak_milestone = 0;
            }
        }

        // ── Power-up timers ──
        if self.player.speed_boost_timer > 0.0 {
            self.player.speed_boost_timer -= dt;
        }
        if self.player.double_damage_timer > 0.0 {
            self.player.double_damage_timer -= dt;
        }
        if self.player.freeze_timer > 0.0 {
            self.player.freeze_timer -= dt;
        }

        // ── Score pop timer ──
        if self.score_pop_timer > 0.0 {
            self.score_pop_timer -= dt;
        }

        // ── Update bullets ──
        let dmg_mult = self.player.damage_multiplier();
        for bullet in self.bullets.iter_mut() {
            if !bullet.alive {
                continue;
            }
            bullet.x += bullet.vx;
            bullet.y += bullet.vy;
            bullet.lifetime -= dt;
            bullet.traveled += (bullet.vx * bullet.vx + bullet.vy * bullet.vy).sqrt();

            // Out of arena or expired
            if bullet.x < ARENA_X
                || bullet.x > ARENA_X + ARENA_W
                || bullet.y < ARENA_Y
                || bullet.y > ARENA_Y + ARENA_H
                || bullet.lifetime <= 0.0
            {
                if bullet.is_rocket {
                    // Explode at max range
                    let bx = bullet.x.clamp(ARENA_X, ARENA_X + ARENA_W);
                    let by = bullet.y.clamp(ARENA_Y, ARENA_Y + ARENA_H);
                    // We'll handle explosion after this loop
                    bullet.x = bx;
                    bullet.y = by;
                }
                bullet.alive = false;
                continue;
            }

            // Rocket auto-explode at 300px
            if bullet.is_rocket && bullet.traveled >= 300.0 {
                bullet.alive = false;
                continue;
            }
        }

        // Handle rocket explosions for dead rockets
        let rocket_explosions: Vec<(f32, f32)> = self
            .bullets
            .iter()
            .filter(|b| !b.alive && b.is_rocket)
            .map(|b| (b.x, b.y))
            .collect();
        for (rx, ry) in &rocket_explosions {
            self.rocket_explode(*rx, *ry, dmg_mult);
        }

        // ── Bullet-enemy collision ──
        let mut kills: Vec<(f32, f32, EnemyType, u32)> = Vec::new();
        let mut rocket_hits: Vec<(f32, f32)> = Vec::new();
        for bullet in self.bullets.iter_mut() {
            if !bullet.alive {
                continue;
            }
            for enemy in self.enemies.iter_mut() {
                if !enemy.alive || enemy.teleporting {
                    continue;
                }
                let half_e = enemy.size / 2.0;
                if bullet.x > enemy.x - half_e
                    && bullet.x < enemy.x + half_e
                    && bullet.y > enemy.y - half_e
                    && bullet.y < enemy.y + half_e
                {
                    enemy.hp -= bullet.damage;
                    enemy.flash_timer = 0.1;
                    // Bullet impact sparks
                    let spark_count = rand::gen_range(4, 7) as usize;
                    for _ in 0..spark_count {
                        let angle = rand::gen_range(0.0, std::f32::consts::TAU);
                        let speed = rand::gen_range(2.0, 6.0);
                        let colors = [WHITE, YELLOW, Color::new(1.0, 1.0, 0.6, 1.0)];
                        let c = colors[rand::gen_range(0, colors.len())];
                        self.particles.push(Particle {
                            x: bullet.x,
                            y: bullet.y,
                            vx: angle.cos() * speed,
                            vy: angle.sin() * speed,
                            lifetime: rand::gen_range(8.0, 12.0) / 60.0,
                            max_lifetime: rand::gen_range(8.0, 12.0) / 60.0,
                            color: c,
                            size: rand::gen_range(1.5, 3.0),
                            alive: true,
                        });
                    }
                    if bullet.is_rocket {
                        rocket_hits.push((bullet.x, bullet.y));
                    }
                    bullet.alive = false;

                    if enemy.hp <= 0.0 {
                        enemy.alive = false;
                        let score = match enemy.enemy_type {
                            EnemyType::Swarmer => 10,
                            EnemyType::Tank => 50,
                            EnemyType::Teleporter => 30,
                            EnemyType::Splitter => 40,
                            EnemyType::SplitterSmall => 15,
                            EnemyType::MegaTank => 200,
                            EnemyType::SwarmQueen => 500,
                        };
                        kills.push((enemy.x, enemy.y, enemy.enemy_type, score));
                    }
                    break;
                }
            }
        }

        // Process rocket direct-hit explosions
        for (rx, ry) in &rocket_hits {
            self.rocket_explode(*rx, *ry, dmg_mult);
        }

        // Process kills
        for (kx, ky, etype, score) in &kills {
            self.add_score(*score);
            self.spawn_particles(*kx, *ky, WHITE, 8);
            self.drop_pickup(*kx, *ky);
            self.wave_enemies_killed += 1;

            // Splitter splits
            if *etype == EnemyType::Splitter {
                let count = rand::gen_range(2, 4) as usize;
                for _ in 0..count {
                    let ox = rand::gen_range(-15.0, 15.0);
                    let oy = rand::gen_range(-15.0, 15.0);
                    self.enemies.push(Enemy::new(
                        EnemyType::SplitterSmall,
                        kx + ox,
                        ky + oy,
                    ));
                }
            }
        }

        // Remove dead bullets
        self.bullets.retain(|b| b.alive);

        // ── Update enemies ──
        let freeze_mult = if self.player.freeze_timer > 0.0 {
            0.3
        } else {
            1.0
        };
        let player_x = self.player.x;
        let player_y = self.player.y;

        // Collect new enemy bullets and spawns
        let mut new_enemy_bullets: Vec<EnemyBullet> = Vec::new();
        let mut new_spawns: Vec<(EnemyType, f32, f32)> = Vec::new();

        for enemy in self.enemies.iter_mut() {
            if !enemy.alive {
                continue;
            }
            enemy.flash_timer -= dt;

            match enemy.enemy_type {
                EnemyType::Swarmer => {
                    let edx = player_x - enemy.x;
                    let edy = player_y - enemy.y;
                    let _dist = (edx * edx + edy * edy).sqrt().max(1.0);
                    let weave = (self.time * 3.0 + enemy.weave_offset).sin() * 0.3;
                    let angle = edy.atan2(edx) + weave;
                    enemy.x += angle.cos() * enemy.speed * freeze_mult;
                    enemy.y += angle.sin() * enemy.speed * freeze_mult;
                }
                EnemyType::Tank => {
                    let edx = player_x - enemy.x;
                    let edy = player_y - enemy.y;
                    let dist = (edx * edx + edy * edy).sqrt().max(1.0);
                    enemy.x += (edx / dist) * enemy.speed * freeze_mult;
                    enemy.y += (edy / dist) * enemy.speed * freeze_mult;

                    enemy.fire_timer -= dt * freeze_mult;
                    if enemy.fire_timer <= 0.0 {
                        enemy.fire_timer = 3.0;
                        let angle = edy.atan2(edx);
                        new_enemy_bullets.push(EnemyBullet {
                            x: enemy.x,
                            y: enemy.y,
                            vx: angle.cos() * 3.0,
                            vy: angle.sin() * 3.0,
                            damage: 15.0,
                            alive: true,
                            homing: false,
                        });
                    }
                }
                EnemyType::Teleporter => {
                    if enemy.teleporting {
                        enemy.teleport_cooldown -= dt;
                        if enemy.teleport_cooldown <= 0.0 {
                            enemy.teleporting = false;
                            enemy.x = rand::gen_range(ARENA_X + 20.0, ARENA_X + ARENA_W - 20.0);
                            enemy.y = rand::gen_range(ARENA_Y + 20.0, ARENA_Y + ARENA_H - 20.0);
                        }
                    } else {
                        let edx = player_x - enemy.x;
                        let edy = player_y - enemy.y;
                        let dist = (edx * edx + edy * edy).sqrt().max(1.0);
                        enemy.x += (edx / dist) * enemy.speed * freeze_mult;
                        enemy.y += (edy / dist) * enemy.speed * freeze_mult;

                        enemy.teleport_timer -= dt * freeze_mult;
                        if enemy.teleport_timer <= 0.0 {
                            enemy.teleporting = true;
                            enemy.teleport_cooldown = 0.3;
                            enemy.teleport_timer = rand::gen_range(2.0, 4.0);
                        }
                    }
                }
                EnemyType::Splitter | EnemyType::SplitterSmall => {
                    let edx = player_x - enemy.x;
                    let edy = player_y - enemy.y;
                    let dist = (edx * edx + edy * edy).sqrt().max(1.0);
                    enemy.x += (edx / dist) * enemy.speed * freeze_mult;
                    enemy.y += (edy / dist) * enemy.speed * freeze_mult;
                }
                EnemyType::MegaTank => {
                    let edx = player_x - enemy.x;
                    let edy = player_y - enemy.y;
                    let dist = (edx * edx + edy * edy).sqrt().max(1.0);
                    enemy.x += (edx / dist) * enemy.speed * freeze_mult;
                    enemy.y += (edy / dist) * enemy.speed * freeze_mult;

                    // 3-way spread shot
                    enemy.fire_timer -= dt * freeze_mult;
                    if enemy.fire_timer <= 0.0 {
                        enemy.fire_timer = 2.0;
                        let base_angle = edy.atan2(edx);
                        for i in -1..=1 {
                            let a = base_angle + i as f32 * 0.3;
                            new_enemy_bullets.push(EnemyBullet {
                                x: enemy.x,
                                y: enemy.y,
                                vx: a.cos() * 3.0,
                                vy: a.sin() * 3.0,
                                damage: 15.0,
                                alive: true,
                                homing: false,
                            });
                        }
                    }

                    // Spawn swarmers
                    enemy.spawn_timer -= dt * freeze_mult;
                    if enemy.spawn_timer <= 0.0 {
                        enemy.spawn_timer = 5.0;
                        for _ in 0..2 {
                            let ox = rand::gen_range(-20.0, 20.0);
                            let oy = rand::gen_range(-20.0, 20.0);
                            new_spawns.push((EnemyType::Swarmer, enemy.x + ox, enemy.y + oy));
                        }
                    }
                }
                EnemyType::SwarmQueen => {
                    let edx = player_x - enemy.x;
                    let edy = player_y - enemy.y;
                    let dist = (edx * edx + edy * edy).sqrt().max(1.0);
                    enemy.x += (edx / dist) * enemy.speed * freeze_mult;
                    enemy.y += (edy / dist) * enemy.speed * freeze_mult;

                    // Teleport frequently
                    enemy.teleport_timer -= dt * freeze_mult;
                    if enemy.teleport_timer <= 0.0 {
                        enemy.x = rand::gen_range(ARENA_X + 30.0, ARENA_X + ARENA_W - 30.0);
                        enemy.y = rand::gen_range(ARENA_Y + 30.0, ARENA_Y + ARENA_H - 30.0);
                        enemy.teleport_timer = rand::gen_range(2.0, 3.5);
                    }

                    // Homing projectiles
                    enemy.fire_timer -= dt * freeze_mult;
                    if enemy.fire_timer <= 0.0 {
                        enemy.fire_timer = 1.5;
                        let angle = edy.atan2(edx);
                        new_enemy_bullets.push(EnemyBullet {
                            x: enemy.x,
                            y: enemy.y,
                            vx: angle.cos() * 2.5,
                            vy: angle.sin() * 2.5,
                            damage: 20.0,
                            alive: true,
                            homing: true,
                        });
                    }

                    // Spawn all types
                    enemy.spawn_timer -= dt * freeze_mult;
                    if enemy.spawn_timer <= 0.0 {
                        enemy.spawn_timer = 4.0;
                        let types = [
                            EnemyType::Swarmer,
                            EnemyType::Tank,
                            EnemyType::Teleporter,
                            EnemyType::Splitter,
                        ];
                        let etype = types[rand::gen_range(0, types.len())];
                        let ox = rand::gen_range(-20.0, 20.0);
                        let oy = rand::gen_range(-20.0, 20.0);
                        new_spawns.push((etype, enemy.x + ox, enemy.y + oy));
                    }
                }
            }

            // Clamp enemy to arena
            let half_e = enemy.size / 2.0;
            enemy.x = enemy.x.clamp(ARENA_X + half_e, ARENA_X + ARENA_W - half_e);
            enemy.y = enemy.y.clamp(ARENA_Y + half_e, ARENA_Y + ARENA_H - half_e);
        }

        // Add new enemy bullets and spawns
        self.enemy_bullets.extend(new_enemy_bullets);
        for (etype, x, y) in new_spawns {
            self.enemies.push(Enemy::new(etype, x, y));
        }

        // ── Enemy-player collision ──
        for enemy in &self.enemies {
            if !enemy.alive || enemy.teleporting {
                continue;
            }
            let half_e = enemy.size / 2.0;
            let half_p = PLAYER_HITBOX / 2.0;
            if (self.player.x - enemy.x).abs() < half_e + half_p
                && (self.player.y - enemy.y).abs() < half_e + half_p
            {
                self.player_take_damage(10.0);
            }
        }

        // ── Update enemy bullets ──
        for eb in self.enemy_bullets.iter_mut() {
            if !eb.alive {
                continue;
            }

            // Homing adjustment
            if eb.homing {
                let edx = player_x - eb.x;
                let edy = player_y - eb.y;
                let target_angle = edy.atan2(edx);
                let current_angle = eb.vy.atan2(eb.vx);
                let speed = (eb.vx * eb.vx + eb.vy * eb.vy).sqrt();
                let mut diff = target_angle - current_angle;
                while diff > std::f32::consts::PI {
                    diff -= std::f32::consts::TAU;
                }
                while diff < -std::f32::consts::PI {
                    diff += std::f32::consts::TAU;
                }
                let new_angle = current_angle + diff.clamp(-0.03, 0.03);
                eb.vx = new_angle.cos() * speed;
                eb.vy = new_angle.sin() * speed;
            }

            eb.x += eb.vx;
            eb.y += eb.vy;

            // Out of arena
            if eb.x < ARENA_X || eb.x > ARENA_X + ARENA_W || eb.y < ARENA_Y || eb.y > ARENA_Y + ARENA_H
            {
                eb.alive = false;
                continue;
            }

            // Hit player
            let half_p = PLAYER_HITBOX / 2.0;
            if (eb.x - player_x).abs() < half_p + 4.0
                && (eb.y - player_y).abs() < half_p + 4.0
            {
                eb.alive = false;
                self.player_take_damage(eb.damage);
            }
        }
        self.enemy_bullets.retain(|b| b.alive);

        // ── Update pickups ──
        for pickup in self.pickups.iter_mut() {
            if !pickup.alive {
                continue;
            }
            pickup.timer -= dt;
            if pickup.timer <= 0.0 {
                pickup.alive = false;
                continue;
            }

            // Check player collision
            let half_p = PLAYER_HITBOX / 2.0;
            if (pickup.x - self.player.x).abs() < half_p + 8.0
                && (pickup.y - self.player.y).abs() < half_p + 8.0
            {
                pickup.alive = false;
                match pickup.kind {
                    PickupKind::Shotgun => {
                        self.player.ammo_shotgun += 20;
                        if self.player.current_weapon == WeaponType::Pistol {
                            self.player.current_weapon = WeaponType::Shotgun;
                        }
                    }
                    PickupKind::Laser => {
                        self.player.ammo_laser += 50;
                        if self.player.current_weapon == WeaponType::Pistol {
                            self.player.current_weapon = WeaponType::Laser;
                        }
                    }
                    PickupKind::Rocket => {
                        self.player.ammo_rocket += 8;
                        if self.player.current_weapon == WeaponType::Pistol {
                            self.player.current_weapon = WeaponType::Rocket;
                        }
                    }
                    PickupKind::SpeedBoost => {
                        self.player.speed_boost_timer = 8.0;
                    }
                    PickupKind::Shield => {
                        self.player.shield_hp = 50.0;
                    }
                    PickupKind::DoubleDamage => {
                        self.player.double_damage_timer = 8.0;
                    }
                    PickupKind::Freeze => {
                        self.player.freeze_timer = 5.0;
                    }
                }
                self.spawn_particles(pickup.x, pickup.y, YELLOW, 6);
            }
        }
        self.pickups.retain(|p| p.alive);

        // ── Update particles ──
        for p in self.particles.iter_mut() {
            if !p.alive {
                continue;
            }
            p.x += p.vx;
            p.y += p.vy;
            p.lifetime -= dt;
            p.vx *= 0.96;
            p.vy *= 0.96;
            if p.lifetime <= 0.0 {
                p.alive = false;
            }
        }
        self.particles.retain(|p| p.alive);

        // ── Update explosion rings ──
        for ring in self.explosion_rings.iter_mut() {
            if !ring.alive { continue; }
            ring.frame += 1;
            if ring.frame >= ring.max_frames { ring.alive = false; }
        }
        self.explosion_rings.retain(|r| r.alive);

        // ── Update spawn teleport effects ──
        for eff in self.spawn_effects.iter_mut() {
            if !eff.alive { continue; }
            eff.frame += 1;
            if eff.frame >= eff.max_frames { eff.alive = false; }
        }
        self.spawn_effects.retain(|e| e.alive);

        // ── Update kill streak texts ──
        for ks in self.kill_streak_texts.iter_mut() {
            if !ks.alive { continue; }
            ks.frame += 1;
            if ks.frame >= ks.max_frames { ks.alive = false; }
        }
        self.kill_streak_texts.retain(|k| k.alive);

        // ── Update damage border ──
        if self.damage_border_frames > 0 {
            self.damage_border_frames -= 1;
        }

        // ── Electric fence spark particles ──
        self.fence_spark_timer += 1;
        if self.fence_spark_timer >= 10 {
            self.fence_spark_timer = 0;
            let spark_count = rand::gen_range(1, 4);
            for _ in 0..spark_count {
                let side = rand::gen_range(0, 4);
                let (sx, sy) = match side {
                    0 => (rand::gen_range(ARENA_X, ARENA_X + ARENA_W), ARENA_Y),
                    1 => (rand::gen_range(ARENA_X, ARENA_X + ARENA_W), ARENA_Y + ARENA_H),
                    2 => (ARENA_X, rand::gen_range(ARENA_Y, ARENA_Y + ARENA_H)),
                    _ => (ARENA_X + ARENA_W, rand::gen_range(ARENA_Y, ARENA_Y + ARENA_H)),
                };
                // Scatter inward
                let inward_x = (SCREEN_W / 2.0 - sx).signum();
                let inward_y = (SCREEN_H / 2.0 - sy).signum();
                let angle = rand::gen_range(-0.5, 0.5);
                let speed = rand::gen_range(1.5, 4.0);
                self.particles.push(Particle {
                    x: sx,
                    y: sy,
                    vx: inward_x * speed * angle.cos() + rand::gen_range(-0.5, 0.5),
                    vy: inward_y * speed * angle.sin() + rand::gen_range(-0.5, 0.5),
                    lifetime: rand::gen_range(0.2, 0.5),
                    max_lifetime: rand::gen_range(0.2, 0.5),
                    color: Color::new(0.0, 1.0, 1.0, 1.0),
                    size: rand::gen_range(1.5, 3.0),
                    alive: true,
                });
            }
        }

        // ── Screen shake ──
        if self.screen_shake > 0.0 {
            self.screen_shake_x = rand::gen_range(-self.screen_shake, self.screen_shake);
            self.screen_shake_y = rand::gen_range(-self.screen_shake, self.screen_shake);
            self.screen_shake *= 0.9;
            if self.screen_shake < 0.5 {
                self.screen_shake = 0.0;
                self.screen_shake_x = 0.0;
                self.screen_shake_y = 0.0;
            }
        }

        // ── Spawning ──
        self.try_spawn_enemies();

        // ── Remove dead enemies ──
        self.enemies.retain(|e| e.alive);

        // ── Check wave completion ──
        if self.enemies_to_spawn <= 0 && self.enemies.is_empty() {
            if self.wave >= MAX_WAVES {
                self.state = GameState::Victory;
            } else {
                self.start_wave();
            }
        }
    }

    fn draw(&self) {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        let shake_x = self.screen_shake_x;
        let shake_y = self.screen_shake_y;

        match self.state {
            GameState::Title => self.draw_title(),
            GameState::Playing | GameState::WaveIntro => {
                self.draw_arena(shake_x, shake_y);
                self.draw_entities(shake_x, shake_y);
                self.draw_hud();
                if self.state == GameState::WaveIntro {
                    self.draw_wave_intro();
                }
            }
            GameState::GameOver => {
                self.draw_arena(shake_x, shake_y);
                self.draw_entities(shake_x, shake_y);
                self.draw_hud();
                self.draw_game_over();
            }
            GameState::Victory => {
                self.draw_arena(shake_x, shake_y);
                self.draw_entities(shake_x, shake_y);
                self.draw_hud();
                self.draw_victory();
            }
        }

        // ── Kill streak text (drawn above HUD) ──
        for ks in &self.kill_streak_texts {
            if !ks.alive { continue; }
            let t = ks.frame as f32 / ks.max_frames as f32;
            let scale = 1.0 + t * 0.5;
            let alpha = 1.0 - t;
            let size = 40.0 * scale;
            let y_off = -t * 30.0;
            draw_text_centered(
                &ks.text,
                SCREEN_W / 2.0,
                SCREEN_H / 2.0 - 60.0 + y_off,
                size,
                Color::new(1.0, 0.9, 0.0, alpha),
            );
        }

        // ── Player damage red border ──
        if self.damage_border_frames > 0 {
            let alpha = self.damage_border_frames as f32 / 15.0 * 0.5;
            let thickness = 12.0;
            let c = Color::new(1.0, 0.0, 0.0, alpha);
            // Top
            draw_rectangle(0.0, 0.0, SCREEN_W, thickness, c);
            // Bottom
            draw_rectangle(0.0, SCREEN_H - thickness, SCREEN_W, thickness, c);
            // Left
            draw_rectangle(0.0, 0.0, thickness, SCREEN_H, c);
            // Right
            draw_rectangle(SCREEN_W - thickness, 0.0, thickness, SCREEN_H, c);
        }

        // ── Vignette ──
        {
            let edge = 50.0;
            let steps = 10;
            for i in 0..steps {
                let t = i as f32 / steps as f32;
                let alpha = 0.3 * (1.0 - t);
                let c = Color::new(0.0, 0.0, 0.0, alpha);
                let offset = t * edge;
                // Top
                draw_rectangle(0.0, offset, SCREEN_W, edge / steps as f32, c);
                // Bottom
                draw_rectangle(0.0, SCREEN_H - offset - edge / steps as f32, SCREEN_W, edge / steps as f32, c);
                // Left
                draw_rectangle(offset, 0.0, edge / steps as f32, SCREEN_H, c);
                // Right
                draw_rectangle(SCREEN_W - offset - edge / steps as f32, 0.0, edge / steps as f32, SCREEN_H, c);
            }
        }

        // ── CRT scanline overlay ──
        {
            let mut y = 0.0;
            while y < SCREEN_H {
                draw_rectangle(0.0, y, SCREEN_W, 1.0, Color::new(0.0, 0.0, 0.0, 0.12));
                y += 4.0;
            }
        }
    }

    fn draw_title(&self) {
        let cx = SCREEN_W / 2.0;
        let cy = SCREEN_H / 2.0;

        draw_text_centered("ARENA BLITZ", cx, cy - 60.0, 48.0, Color::new(1.0, 0.3, 0.1, 1.0));
        draw_text_centered("TWIN-STICK ARENA SHOOTER", cx, cy - 20.0, 16.0, GRAY);

        let blink = ((self.blink_timer * 3.0).sin() + 1.0) / 2.0;
        let alpha = 0.3 + blink * 0.7;
        draw_text_centered(
            "PRESS START",
            cx,
            cy + 40.0,
            24.0,
            Color::new(1.0, 1.0, 1.0, alpha),
        );

        draw_text_centered("D-Pad: Move    A(X): Shoot    B(Space): Switch Weapon", cx, cy + 90.0, 12.0, GRAY);
        draw_text_centered("Auto-aim at nearest enemy", cx, cy + 110.0, 12.0, DARKGRAY);
    }

    fn draw_arena(&self, sx: f32, sy: f32) {
        // Dark floor with grid
        draw_rectangle(
            ARENA_X + sx,
            ARENA_Y + sy,
            ARENA_W,
            ARENA_H,
            Color::new(0.08, 0.08, 0.12, 1.0),
        );

        // Grid lines
        let grid_color = Color::new(0.12, 0.12, 0.18, 1.0);
        let step = 40.0;
        let mut gx = ARENA_X + step;
        while gx < ARENA_X + ARENA_W {
            draw_line(gx + sx, ARENA_Y + sy, gx + sx, ARENA_Y + ARENA_H + sy, 1.0, grid_color);
            gx += step;
        }
        let mut gy = ARENA_Y + step;
        while gy < ARENA_Y + ARENA_H {
            draw_line(ARENA_X + sx, gy + sy, ARENA_X + ARENA_W + sx, gy + sy, 1.0, grid_color);
            gy += step;
        }

        // Electric fence border
        let pulse = self.fence_pulse;
        let fence_color = Color::new(0.2 + pulse * 0.5, 0.6 + pulse * 0.4, 1.0, 0.8 + pulse * 0.2);

        // Top
        draw_rectangle(ARENA_X + sx - 3.0, ARENA_Y + sy - 3.0, ARENA_W + 6.0, 3.0, fence_color);
        // Bottom
        draw_rectangle(ARENA_X + sx - 3.0, ARENA_Y + ARENA_H + sy, ARENA_W + 6.0, 3.0, fence_color);
        // Left
        draw_rectangle(ARENA_X + sx - 3.0, ARENA_Y + sy, 3.0, ARENA_H, fence_color);
        // Right
        draw_rectangle(ARENA_X + ARENA_W + sx, ARENA_Y + sy, 3.0, ARENA_H, fence_color);

        // Fence spark particles
        if rand::gen_range(0.0, 1.0) < 0.3 {
            let side = rand::gen_range(0, 4);
            let (spark_x, spark_y) = match side {
                0 => (rand::gen_range(ARENA_X, ARENA_X + ARENA_W), ARENA_Y),
                1 => (rand::gen_range(ARENA_X, ARENA_X + ARENA_W), ARENA_Y + ARENA_H),
                2 => (ARENA_X, rand::gen_range(ARENA_Y, ARENA_Y + ARENA_H)),
                _ => (ARENA_X + ARENA_W, rand::gen_range(ARENA_Y, ARENA_Y + ARENA_H)),
            };
            draw_circle(spark_x + sx, spark_y + sy, 2.0, WHITE);
        }
    }

    fn draw_entities(&self, sx: f32, sy: f32) {
        // ── Draw pickups ──
        for pickup in &self.pickups {
            if !pickup.alive {
                continue;
            }
            let _rot = self.time * 3.0;
            let bob = (self.time * 4.0).sin() * 2.0;
            let (color, label) = match pickup.kind {
                PickupKind::Shotgun => (ORANGE, "SG"),
                PickupKind::Laser => (SKYBLUE, "LS"),
                PickupKind::Rocket => (RED, "RK"),
                PickupKind::SpeedBoost => (BLUE, "SP"),
                PickupKind::Shield => (WHITE, "SH"),
                PickupKind::DoubleDamage => (RED, "2X"),
                PickupKind::Freeze => (SKYBLUE, "FR"),
            };
            draw_rectangle(
                pickup.x - 6.0 + sx,
                pickup.y - 6.0 + bob + sy,
                12.0,
                12.0,
                color,
            );
            draw_text(
                label,
                pickup.x - 5.0 + sx,
                pickup.y + 3.0 + bob + sy,
                10.0,
                BLACK,
            );

            // Blinking when about to expire
            if pickup.timer < 3.0 && (self.time * 8.0).sin() > 0.0 {
                draw_rectangle_lines(
                    pickup.x - 8.0 + sx,
                    pickup.y - 8.0 + bob + sy,
                    16.0,
                    16.0,
                    1.0,
                    WHITE,
                );
            }
        }

        // ── Draw enemies ──
        for enemy in &self.enemies {
            if !enemy.alive {
                continue;
            }
            if enemy.teleporting {
                // Flash effect
                let alpha = ((self.time * 20.0).sin() + 1.0) / 2.0 * 0.5;
                let mut c = enemy.color();
                c.a = alpha;
                draw_rectangle(
                    enemy.x - enemy.size / 2.0 + sx,
                    enemy.y - enemy.size / 2.0 + sy,
                    enemy.size,
                    enemy.size,
                    c,
                );
                continue;
            }

            let color = if enemy.flash_timer > 0.0 {
                WHITE
            } else if self.player.freeze_timer > 0.0 {
                Color::new(0.5, 0.5, 1.0, 1.0)
            } else {
                enemy.color()
            };

            let half = enemy.size / 2.0;
            match enemy.enemy_type {
                EnemyType::Swarmer => {
                    // Small diamond shape
                    let cx = enemy.x + sx;
                    let cy = enemy.y + sy;
                    draw_triangle(
                        Vec2::new(cx, cy - half),
                        Vec2::new(cx - half, cy + half),
                        Vec2::new(cx + half, cy + half),
                        color,
                    );
                }
                EnemyType::Tank | EnemyType::MegaTank => {
                    draw_rectangle(
                        enemy.x - half + sx,
                        enemy.y - half + sy,
                        enemy.size,
                        enemy.size,
                        color,
                    );
                    // Turret line
                    let angle = (self.player.y - enemy.y).atan2(self.player.x - enemy.x);
                    draw_line(
                        enemy.x + sx,
                        enemy.y + sy,
                        enemy.x + angle.cos() * enemy.size * 0.7 + sx,
                        enemy.y + angle.sin() * enemy.size * 0.7 + sy,
                        2.0,
                        WHITE,
                    );
                    // HP bar for tanks/bosses
                    if enemy.max_hp >= 50.0 {
                        let bar_w = enemy.size;
                        let hp_frac = enemy.hp / enemy.max_hp;
                        draw_rectangle(
                            enemy.x - half + sx,
                            enemy.y - half - 6.0 + sy,
                            bar_w,
                            3.0,
                            DARKGRAY,
                        );
                        draw_rectangle(
                            enemy.x - half + sx,
                            enemy.y - half - 6.0 + sy,
                            bar_w * hp_frac,
                            3.0,
                            RED,
                        );
                    }
                }
                EnemyType::Teleporter => {
                    draw_circle(enemy.x + sx, enemy.y + sy, half, color);
                    // Inner glow
                    draw_circle(enemy.x + sx, enemy.y + sy, half * 0.5, Color::new(1.0, 0.5, 1.0, 0.5));
                }
                EnemyType::Splitter | EnemyType::SplitterSmall => {
                    // Hexagon-ish shape
                    draw_circle(enemy.x + sx, enemy.y + sy, half, color);
                    draw_circle(
                        enemy.x + sx,
                        enemy.y + sy,
                        half * 0.6,
                        Color::new(0.0, 0.4, 0.4, 1.0),
                    );
                }
                EnemyType::SwarmQueen => {
                    draw_circle(enemy.x + sx, enemy.y + sy, half, color);
                    // Crown-like top
                    for i in 0..3 {
                        let cx = enemy.x + sx + (i as f32 - 1.0) * 6.0;
                        let cy = enemy.y + sy - half - 3.0;
                        draw_triangle(
                            Vec2::new(cx, cy - 4.0),
                            Vec2::new(cx - 3.0, cy + 2.0),
                            Vec2::new(cx + 3.0, cy + 2.0),
                            YELLOW,
                        );
                    }
                    // HP bar
                    let bar_w = enemy.size * 1.5;
                    let hp_frac = enemy.hp / enemy.max_hp;
                    draw_rectangle(
                        enemy.x - bar_w / 2.0 + sx,
                        enemy.y - half - 12.0 + sy,
                        bar_w,
                        3.0,
                        DARKGRAY,
                    );
                    draw_rectangle(
                        enemy.x - bar_w / 2.0 + sx,
                        enemy.y - half - 12.0 + sy,
                        bar_w * hp_frac,
                        3.0,
                        RED,
                    );
                }
            }
        }

        // ── Draw enemy bullets ──
        for eb in &self.enemy_bullets {
            if !eb.alive {
                continue;
            }
            let color = if eb.homing {
                Color::new(1.0, 0.0, 1.0, 1.0)
            } else {
                Color::new(1.0, 0.3, 0.3, 1.0)
            };
            draw_circle(eb.x + sx, eb.y + sy, 3.0, color);
            // Trail
            draw_circle(eb.x - eb.vx * 0.5 + sx, eb.y - eb.vy * 0.5 + sy, 2.0, Color::new(color.r, color.g, color.b, 0.4));
        }

        // ── Draw player bullets ──
        for bullet in &self.bullets {
            if !bullet.alive {
                continue;
            }
            let color = if bullet.is_rocket {
                ORANGE
            } else {
                YELLOW
            };
            let size = if bullet.is_rocket { 4.0 } else { 2.5 };
            draw_circle(bullet.x + sx, bullet.y + sy, size, color);
            // Trail
            draw_circle(
                bullet.x - bullet.vx * 0.3 + sx,
                bullet.y - bullet.vy * 0.3 + sy,
                size * 0.6,
                Color::new(color.r, color.g, color.b, 0.4),
            );
        }

        // ── Draw laser ──
        if self.laser_active {
            let px = self.player.x + sx;
            let py = self.player.y + sy;
            let lx = self.laser_end_x + sx;
            let ly = self.laser_end_y + sy;
            // Glow
            draw_line(px, py, lx, ly, 6.0, Color::new(0.0, 1.0, 1.0, 0.2));
            draw_line(px, py, lx, ly, 3.0, Color::new(0.0, 1.0, 1.0, 0.5));
            draw_line(px, py, lx, ly, 1.5, Color::new(0.8, 1.0, 1.0, 1.0));
        }

        // ── Draw player ──
        let player_visible = self.player.invincible == 0
            || (self.player.invincible % 6) < 3;
        if player_visible {
            let px = self.player.x + sx;
            let py = self.player.y + sy;

            // Body
            let body_color = if self.player.double_damage_timer > 0.0 {
                Color::new(1.0, 0.3, 0.3, 1.0)
            } else {
                Color::new(0.2, 0.8, 1.0, 1.0)
            };
            draw_rectangle(
                px - PLAYER_SIZE / 2.0,
                py - PLAYER_SIZE / 2.0,
                PLAYER_SIZE,
                PLAYER_SIZE,
                body_color,
            );

            // Gun direction indicator
            let aim = self.player.aim_angle;
            draw_line(
                px,
                py,
                px + aim.cos() * 12.0,
                py + aim.sin() * 12.0,
                2.0,
                WHITE,
            );

            // Shield visual
            if self.player.shield_hp > 0.0 {
                draw_circle_lines(px, py, 14.0, 1.5, Color::new(1.0, 1.0, 1.0, 0.5));
            }

            // Speed boost trail
            if self.player.speed_boost_timer > 0.0 {
                draw_circle(
                    px - aim.cos() * 6.0,
                    py - aim.sin() * 6.0,
                    3.0,
                    Color::new(0.2, 0.3, 1.0, 0.4),
                );
            }
        }

        // ── Draw particles ──
        for p in &self.particles {
            if !p.alive {
                continue;
            }
            let alpha = (p.lifetime / p.max_lifetime).clamp(0.0, 1.0);
            let color = Color::new(p.color.r, p.color.g, p.color.b, alpha);
            let size = p.size * alpha;
            draw_circle(p.x + sx, p.y + sy, size, color);
        }

        // ── Draw explosion rings ──
        for ring in &self.explosion_rings {
            if !ring.alive { continue; }
            let t = ring.frame as f32 / ring.max_frames as f32;
            let radius = 5.0 + (ring.max_radius - 5.0) * t;
            let alpha = 1.0 - t;
            let r = 1.0;
            let g = 0.5 * (1.0 - t);
            let b = 0.0;
            draw_circle_lines(ring.x + sx, ring.y + sy, radius, 2.0, Color::new(r, g, b, alpha));
            // Inner glow
            if t < 0.5 {
                draw_circle_lines(ring.x + sx, ring.y + sy, radius * 0.7, 1.0, Color::new(1.0, 0.8, 0.0, alpha * 0.5));
            }
        }

        // ── Draw spawn teleport effects ──
        for eff in &self.spawn_effects {
            if !eff.alive { continue; }
            let t = eff.frame as f32 / eff.max_frames as f32;
            let alpha = 1.0 - t;
            for i in 0..4 {
                let delay = i as f32 * 0.15;
                let local_t = (t - delay).clamp(0.0, 1.0);
                if local_t <= 0.0 { continue; }
                let radius = 5.0 + local_t * 25.0;
                let a = alpha * (1.0 - local_t);
                draw_circle_lines(eff.x + sx, eff.y + sy, radius, 1.5, Color::new(0.5, 0.8, 1.0, a));
            }
        }

        // ── Crosshair at aim point ──
        if !self.enemies.is_empty() && self.state == GameState::Playing {
            let aim = self.player.aim_angle;
            let cx = self.player.x + aim.cos() * 30.0 + sx;
            let cy = self.player.y + aim.sin() * 30.0 + sy;
            draw_circle_lines(cx, cy, 6.0, 1.0, Color::new(1.0, 0.3, 0.3, 0.7));
            draw_line(cx - 8.0, cy, cx + 8.0, cy, 1.0, Color::new(1.0, 0.3, 0.3, 0.5));
            draw_line(cx, cy - 8.0, cx, cy + 8.0, 1.0, Color::new(1.0, 0.3, 0.3, 0.5));
        }
    }

    fn draw_hud(&self) {
        // ── HP bar (top-left) ──
        let bar_x = 10.0;
        let bar_y = 8.0;
        let bar_w = 120.0;
        let bar_h = 12.0;
        draw_rectangle(bar_x, bar_y, bar_w, bar_h, DARKGRAY);
        let hp_frac = (self.player.hp / PLAYER_MAX_HP).clamp(0.0, 1.0);
        let hp_color = Color::new(1.0 - hp_frac, hp_frac, 0.0, 1.0);
        draw_rectangle(bar_x, bar_y, bar_w * hp_frac, bar_h, hp_color);
        draw_rectangle_lines(bar_x, bar_y, bar_w, bar_h, 1.0, WHITE);
        draw_text(
            &format!("{:.0}", self.player.hp),
            bar_x + bar_w + 5.0,
            bar_y + bar_h - 1.0,
            16.0,
            WHITE,
        );

        // Shield bar
        if self.player.shield_hp > 0.0 {
            let shield_frac = self.player.shield_hp / 50.0;
            draw_rectangle(bar_x, bar_y + bar_h + 2.0, bar_w * shield_frac, 4.0, Color::new(0.5, 0.5, 1.0, 0.8));
        }

        // ── Wave (top-center) ──
        draw_text_centered(
            &format!("WAVE {}/{}", self.wave, MAX_WAVES),
            SCREEN_W / 2.0,
            18.0,
            20.0,
            WHITE,
        );

        // ── Score (top-right) ──
        let score_text = format!("{}", self.player.score);
        let score_size = if self.score_pop_timer > 0.0 {
            20.0 + self.score_pop_timer * 10.0
        } else {
            20.0
        };
        draw_text(&score_text, SCREEN_W - 120.0, 18.0, score_size, YELLOW);

        // Combo multiplier
        if self.player.combo_multiplier > 1.0 {
            draw_text(
                &format!("x{:.1}", self.player.combo_multiplier),
                SCREEN_W - 60.0,
                34.0,
                14.0,
                Color::new(1.0, 0.8, 0.0, 1.0),
            );
        }

        // ── Weapon + Ammo (bottom-left) ──
        let weapon_name = match self.player.current_weapon {
            WeaponType::Pistol => "PISTOL",
            WeaponType::Shotgun => "SHOTGUN",
            WeaponType::Laser => "LASER",
            WeaponType::Rocket => "ROCKET",
        };
        let ammo_text = match self.player.current_weapon {
            WeaponType::Pistol => "INF".to_string(),
            WeaponType::Shotgun => format!("{}", self.player.ammo_shotgun),
            WeaponType::Laser => format!("{}", self.player.ammo_laser),
            WeaponType::Rocket => format!("{}", self.player.ammo_rocket),
        };
        let weapon_color = match self.player.current_weapon {
            WeaponType::Pistol => YELLOW,
            WeaponType::Shotgun => ORANGE,
            WeaponType::Laser => SKYBLUE,
            WeaponType::Rocket => RED,
        };
        draw_text(weapon_name, 10.0, SCREEN_H - 10.0, 16.0, weapon_color);
        draw_text(&ammo_text, 90.0, SCREEN_H - 10.0, 16.0, WHITE);

        // ── Weapon slots (bottom-center) ──
        let slot_x = SCREEN_W / 2.0 - 60.0;
        let slot_y = SCREEN_H - 22.0;
        let weapons = [
            (WeaponType::Pistol, "P", YELLOW),
            (WeaponType::Shotgun, "S", ORANGE),
            (WeaponType::Laser, "L", SKYBLUE),
            (WeaponType::Rocket, "R", RED),
        ];
        for (i, (wtype, label, color)) in weapons.iter().enumerate() {
            let x = slot_x + i as f32 * 32.0;
            let is_active = self.player.current_weapon == *wtype;
            let bg = if is_active {
                Color::new(0.3, 0.3, 0.3, 1.0)
            } else {
                Color::new(0.1, 0.1, 0.1, 0.8)
            };
            draw_rectangle(x, slot_y, 28.0, 18.0, bg);
            draw_rectangle_lines(x, slot_y, 28.0, 18.0, 1.0, if is_active { WHITE } else { GRAY });
            draw_text(label, x + 9.0, slot_y + 14.0, 14.0, *color);
        }

        // ── Active power-ups (bottom-right) ──
        let mut pup_x = SCREEN_W - 150.0;
        let pup_y = SCREEN_H - 18.0;
        if self.player.speed_boost_timer > 0.0 {
            draw_text(
                &format!("SPD {:.0}", self.player.speed_boost_timer),
                pup_x,
                pup_y,
                12.0,
                BLUE,
            );
            pup_x += 50.0;
        }
        if self.player.shield_hp > 0.0 {
            draw_text(
                &format!("SHD {:.0}", self.player.shield_hp),
                pup_x,
                pup_y,
                12.0,
                WHITE,
            );
            pup_x += 50.0;
        }
        if self.player.double_damage_timer > 0.0 {
            draw_text(
                &format!("DMG {:.0}", self.player.double_damage_timer),
                pup_x,
                pup_y,
                12.0,
                RED,
            );
            pup_x += 50.0;
        }
        if self.player.freeze_timer > 0.0 {
            draw_text(
                &format!("FRZ {:.0}", self.player.freeze_timer),
                pup_x,
                pup_y,
                12.0,
                SKYBLUE,
            );
        }

        // ── Enemies remaining ──
        let remaining = self.enemies.len() as i32 + self.enemies_to_spawn;
        draw_text(
            &format!("ENEMIES: {}", remaining),
            SCREEN_W / 2.0 - 40.0,
            34.0,
            12.0,
            GRAY,
        );
    }

    fn draw_wave_intro(&self) {
        let cx = SCREEN_W / 2.0;
        let cy = SCREEN_H / 2.0;
        let progress = 1.0 - (self.wave_intro_timer / 3.0);
        let scale = 1.0 + progress * 0.3;
        let alpha = if progress < 0.3 {
            progress / 0.3
        } else if progress > 0.7 {
            (1.0 - progress) / 0.3
        } else {
            1.0
        };

        let text = if self.wave == 5 {
            format!("WAVE {} - BOSS!", self.wave)
        } else if self.wave == 10 {
            format!("WAVE {} - FINAL BOSS!", self.wave)
        } else {
            format!("WAVE {}", self.wave)
        };

        let size = 36.0 * scale;
        let color = Color::new(1.0, 0.8, 0.0, alpha.clamp(0.0, 1.0));
        draw_text_centered(&text, cx, cy, size, color);
    }

    fn draw_game_over(&self) {
        let cx = SCREEN_W / 2.0;
        let cy = SCREEN_H / 2.0;

        // Darken overlay
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.0, 0.0, 0.0, 0.6));

        draw_text_centered("GAME OVER", cx, cy - 30.0, 48.0, RED);
        draw_text_centered(
            &format!("SCORE: {}", self.player.score),
            cx,
            cy + 10.0,
            24.0,
            WHITE,
        );
        draw_text_centered(
            &format!("WAVE: {}/{}", self.wave, MAX_WAVES),
            cx,
            cy + 40.0,
            20.0,
            GRAY,
        );

        let blink = ((self.blink_timer * 3.0).sin() + 1.0) / 2.0;
        draw_text_centered(
            "PRESS START",
            cx,
            cy + 80.0,
            20.0,
            Color::new(1.0, 1.0, 1.0, 0.3 + blink * 0.7),
        );
    }

    fn draw_victory(&self) {
        let cx = SCREEN_W / 2.0;
        let cy = SCREEN_H / 2.0;

        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::new(0.0, 0.0, 0.0, 0.6));

        draw_text_centered("VICTORY!", cx, cy - 40.0, 48.0, GOLD);
        draw_text_centered("ALL WAVES CLEARED!", cx, cy, 20.0, WHITE);
        draw_text_centered(
            &format!("FINAL SCORE: {}", self.player.score),
            cx,
            cy + 30.0,
            24.0,
            YELLOW,
        );

        let blink = ((self.blink_timer * 3.0).sin() + 1.0) / 2.0;
        draw_text_centered(
            "PRESS START",
            cx,
            cy + 80.0,
            20.0,
            Color::new(1.0, 1.0, 1.0, 0.3 + blink * 0.7),
        );
    }
}

// ─── Helper functions ────────────────────────────────────────────────────────

fn draw_text_centered(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    draw_text(text, x - dims.width / 2.0, y + dims.height / 2.0, font_size, color);
}

fn wave_config(wave: usize) -> (i32, &'static str) {
    match wave {
        1 => (8, "Swarmers"),
        2 => (12, "Swarmers+Tanks"),
        3 => (16, "Swarmers+Tanks+Teleporters"),
        4 => (20, "All"),
        5 => (16, "All+Boss"), // 15 + boss
        6 => (25, "All"),
        7 => (30, "All"),
        8 => (35, "All"),
        9 => (40, "All"),
        10 => (31, "All+Boss"), // 30 + boss
        _ => (8, "Swarmers"),
    }
}

fn wave_enemy_types(wave: usize) -> Vec<EnemyType> {
    match wave {
        1 => vec![EnemyType::Swarmer],
        2 => vec![EnemyType::Swarmer, EnemyType::Swarmer, EnemyType::Tank],
        3 => vec![
            EnemyType::Swarmer,
            EnemyType::Swarmer,
            EnemyType::Tank,
            EnemyType::Teleporter,
        ],
        _ => vec![
            EnemyType::Swarmer,
            EnemyType::Swarmer,
            EnemyType::Tank,
            EnemyType::Teleporter,
            EnemyType::Splitter,
        ],
    }
}

// ─── Window config ───────────────────────────────────────────────────────────

fn window_conf() -> Conf {
    Conf {
        window_title: "Arena Blitz".to_owned(),
        window_width: SCREEN_W as i32,
        window_height: SCREEN_H as i32,
        window_resizable: false,
        ..Default::default()
    }
}

// ─── Main ────────────────────────────────────────────────────────────────────

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
