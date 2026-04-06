// Nova Evader - Rust/Macroquad port for Miyoo Mini Plus
// Ported from the JS "Nova Evader - Bullet Hell"
//
// Mechanics:
//   - Pure dodge game, no shooting.
//   - 5 bullet-pattern types: rain, radial burst, aimed triple, side swipe, spiral.
//   - Score increases with survival time; level rises every 10 s (up to 10).
//   - Higher level = more bullets per volley + faster spawn interval.
//   - One hit = instant game over.
//   - Player: cyan triangle, radius-6 circular hitbox.
//   - Trail effect behind the player ship.
//   - Particle sparks on game-over.
//   - CRT scanlines + vignette overlay.
//
// Input (Miyoo Mini Plus):
//   D-Pad      → move player
//   Start / B  → confirm menus
//   A (X key)  → also confirm

use macroquad::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------
const GAME_W: f32 = 640.0;
const GAME_H: f32 = 480.0;

const PLAYER_RADIUS: f32 = 6.0;
const PLAYER_SPEED: f32 = 5.5; // slightly slower than web (web was 6.5 on a scaling canvas)

const MAX_BULLETS: usize = 500;
const BULLET_OOB: f32 = 80.0;

/// Frames between pattern spawns at level 1.  Reduces with level.
const SPAWN_INTERVAL_BASE: i32 = 108; // ≈1800 ms at 60 fps
const SPAWN_INTERVAL_MIN: i32 = 24;   // ≈400 ms

/// Frames between level-ups  (10 s × 60 fps = 600).
const LEVEL_UP_FRAMES: i32 = 600;
const MAX_LEVEL: i32 = 10;

const TRAIL_LEN: usize = 10;

const STAR_COUNT: usize = 120;

// Score: web ticked `score += FIXED_DT * 0.01` each logic step (≈0.167 per frame).
// We replicate with `score += 0.167` per frame.
const SCORE_PER_FRAME: f64 = 0.167;

// ---------------------------------------------------------------------------
// Game states
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Title,
    Playing,
    GameOver,
}

// ---------------------------------------------------------------------------
// Bullet
// ---------------------------------------------------------------------------
#[derive(Clone, Copy)]
struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
    color: Color,
    alive: bool,
}

impl Bullet {
    fn new(x: f32, y: f32, vx: f32, vy: f32, color: Color, radius: f32) -> Self {
        Self { x, y, vx, vy, radius, color, alive: true }
    }
}

// ---------------------------------------------------------------------------
// Particle
// ---------------------------------------------------------------------------
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

// ---------------------------------------------------------------------------
// Star
// ---------------------------------------------------------------------------
#[derive(Clone, Copy)]
struct Star {
    x: f32,
    y: f32,
    size: f32,
    speed: f32,
    brightness: f32,
}

impl Star {
    fn random(random_y: bool) -> Self {
        let size = rand::gen_range(1.0f32, 3.0f32);
        Self {
            x: rand::gen_range(0.0, GAME_W),
            y: if random_y { rand::gen_range(0.0, GAME_H) } else { -4.0 },
            size,
            speed: size * 0.6,
            brightness: rand::gen_range(0.2f32, 1.0f32),
        }
    }
}

// ---------------------------------------------------------------------------
// TrailPoint
// ---------------------------------------------------------------------------
#[derive(Clone, Copy)]
struct TrailPoint {
    x: f32,
    y: f32,
}

// ---------------------------------------------------------------------------
// Pattern queue entry (for delayed bullets — replaces JS setTimeout)
// ---------------------------------------------------------------------------
struct PendingBullet {
    delay: i32, // frames remaining before spawning
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
    radius: f32,
}

// ---------------------------------------------------------------------------
// Main game struct
// ---------------------------------------------------------------------------
struct Game {
    state: GameState,
    frame: i64,

    // Player
    px: f32,
    py: f32,
    trail: Vec<TrailPoint>,

    // Gameplay
    score: f64,
    level: i32,
    spawn_timer: i32,
    level_timer: i32,

    // Bullets
    bullets: Vec<Bullet>,
    pending: Vec<PendingBullet>,

    // Particles
    particles: Vec<Particle>,

    // Background
    stars: Vec<Star>,

    // Screen shake
    shake_mag: f32,
    shake_x: f32,
    shake_y: f32,

    // High score
    high_score: f64,
}

impl Game {
    fn new() -> Self {
        let mut stars = Vec::with_capacity(STAR_COUNT);
        for _ in 0..STAR_COUNT {
            stars.push(Star::random(true));
        }
        Self {
            state: GameState::Title,
            frame: 0,
            px: GAME_W / 2.0,
            py: GAME_H * 0.7,
            trail: Vec::with_capacity(TRAIL_LEN + 1),
            score: 0.0,
            level: 1,
            spawn_timer: 0,
            level_timer: 0,
            bullets: Vec::new(),
            pending: Vec::new(),
            particles: Vec::new(),
            stars,
            shake_mag: 0.0,
            shake_x: 0.0,
            shake_y: 0.0,
            high_score: 0.0,
        }
    }

    fn reset(&mut self) {
        self.frame = 0;
        self.px = GAME_W / 2.0;
        self.py = GAME_H * 0.7;
        self.trail.clear();
        self.score = 0.0;
        self.level = 1;
        self.spawn_timer = 0;
        self.level_timer = 0;
        self.bullets.clear();
        self.pending.clear();
        self.particles.clear();
        self.shake_mag = 0.0;
        self.shake_x = 0.0;
        self.shake_y = 0.0;
        self.state = GameState::Playing;
    }

    // ------------------------------------------------------------------
    // Helpers
    // ------------------------------------------------------------------
    fn trigger_shake(&mut self, mag: f32) {
        if mag > self.shake_mag {
            self.shake_mag = mag;
        }
    }

    fn spawn_particles(&mut self, x: f32, y: f32, count: usize, color: Color) {
        for _ in 0..count {
            let angle: f32 = rand::gen_range(0.0f32, std::f32::consts::TAU);
            let spd: f32 = rand::gen_range(1.0f32, 5.0f32);
            self.particles.push(Particle {
                x,
                y,
                vx: angle.cos() * spd,
                vy: angle.sin() * spd,
                life: 1.0,
                decay: rand::gen_range(0.02f32, 0.07f32),
                color,
                alive: true,
            });
        }
    }

    /// Spawn interval in frames, matching web formula:
    ///   max(1800 - level*150, 400) ms → converted to frames at 60 fps
    fn spawn_interval(&self) -> i32 {
        let ms = (1800 - self.level * 150).max(SPAWN_INTERVAL_MIN * 1000 / 60);
        // convert ms → frames
        (ms as f32 / 1000.0 * 60.0) as i32
    }

    // ------------------------------------------------------------------
    // Pattern spawners
    // ------------------------------------------------------------------

    /// Rain: bullets fall from the top, spread across width.
    fn spawn_rain(&mut self) {
        let count = (5 + self.level).min(20) as usize;
        for _ in 0..count {
            if self.bullets.len() >= MAX_BULLETS { break; }
            let bx: f32 = rand::gen_range(0.0f32, GAME_W);
            let vx: f32 = rand::gen_range(-1.0f32, 1.0f32);
            let vy: f32 = rand::gen_range(3.0f32, 6.0f32);
            self.bullets.push(Bullet::new(
                bx, -20.0, vx, vy,
                Color::new(1.0, 0.2, 0.4, 1.0), 3.0,
            ));
        }
    }

    /// Radial burst: ring of bullets expanding from a top point.
    fn spawn_radial(&mut self) {
        let cx: f32 = rand::gen_range(0.0f32, GAME_W);
        let count = (8 + self.level).min(24) as usize;
        for i in 0..count {
            if self.bullets.len() >= MAX_BULLETS { break; }
            let a: f32 = (i as f32 / count as f32) * std::f32::consts::TAU;
            self.bullets.push(Bullet::new(
                cx, -20.0,
                a.cos() * 4.0, a.sin() * 4.0 + 2.0,
                Color::new(1.0, 0.67, 0.0, 1.0), 3.0,
            ));
        }
    }

    /// Aimed triple: 3 delayed shots aimed at the player's current position.
    fn spawn_aimed(&mut self) {
        let tx: f32 = rand::gen_range(0.0f32, GAME_W);
        // angle toward player from spawn point
        let dx = self.px - tx;
        let dy = self.py - (-20.0_f32);
        let mag = (dx * dx + dy * dy).sqrt();
        if mag < 0.001 { return; }
        let cos_a = dx / mag;
        let sin_a = dy / mag;
        // 3 shots, 12 frames apart (≈200 ms)
        for i in 0..3i32 {
            self.pending.push(PendingBullet {
                delay: i * 12,
                x: tx,
                y: -20.0,
                vx: cos_a * 7.0,
                vy: sin_a * 7.0,
                color: Color::new(0.0, 1.0, 0.67, 1.0),
                radius: 3.0,
            });
        }
    }

    /// Side swipe: bullets enter from left and right walls.
    fn spawn_side_swipe(&mut self) {
        let ly: f32 = rand::gen_range(0.0f32, GAME_H);
        let ry: f32 = rand::gen_range(0.0f32, GAME_H);
        let magenta = Color::new(1.0, 0.0, 1.0, 1.0);
        if self.bullets.len() < MAX_BULLETS {
            self.bullets.push(Bullet::new(-20.0, ly, 5.0, 0.0, magenta, 4.0));
        }
        if self.bullets.len() < MAX_BULLETS {
            self.bullets.push(Bullet::new(GAME_W + 20.0, ry, -5.0, 0.0, magenta, 4.0));
        }
    }

    /// Spiral: 15 bullets fired in sequence from top-center with rotating angle.
    fn spawn_spiral(&mut self) {
        let sx = GAME_W / 2.0;
        for i in 0..15i32 {
            let a: f32 = i as f32 * 0.4;
            self.pending.push(PendingBullet {
                delay: i * 6, // 6 frames ≈ 100 ms
                x: sx,
                y: -20.0,
                vx: a.cos() * 6.0,
                vy: a.sin() * 2.0 + 4.0,
                color: WHITE,
                radius: 3.0,
            });
        }
    }

    fn spawn_pattern(&mut self) {
        // Pattern type depends on level — same logic as web
        let available = self.level.min(5);
        let kind = rand::gen_range(0i32, available);
        match kind {
            0 => self.spawn_rain(),
            1 => self.spawn_radial(),
            2 => self.spawn_aimed(),
            3 => self.spawn_side_swipe(),
            _ => self.spawn_spiral(),
        }
    }

    // ------------------------------------------------------------------
    // Collision: bullet circle vs player circle
    // ------------------------------------------------------------------
    fn circle_hit(bx: f32, by: f32, br: f32, px: f32, py: f32) -> bool {
        let dx = bx - px;
        let dy = by - py;
        let sum_r = br + PLAYER_RADIUS;
        dx * dx + dy * dy < sum_r * sum_r
    }

    // ------------------------------------------------------------------
    // Update
    // ------------------------------------------------------------------
    fn update(&mut self) {
        self.frame += 1;

        // --- Star scroll ---
        let speed_mult = if self.state == GameState::Playing { 1.5 } else { 0.5 };
        for s in self.stars.iter_mut() {
            s.y += s.speed * speed_mult;
            if s.y > GAME_H + 4.0 {
                *s = Star::random(false);
            }
        }

        // --- Particles ---
        for p in self.particles.iter_mut() {
            p.x += p.vx;
            p.y += p.vy;
            p.vy += 0.05; // slight gravity for game-over sparks
            p.life -= p.decay;
            if p.life <= 0.0 { p.alive = false; }
        }
        // Retain without borrow issues
        let mut i = self.particles.len();
        while i > 0 {
            i -= 1;
            if !self.particles[i].alive {
                self.particles.remove(i);
            }
        }

        // --- Screen shake decay ---
        if self.shake_mag > 0.1 {
            self.shake_x = rand::gen_range(-1.0f32, 1.0f32) * self.shake_mag;
            self.shake_y = rand::gen_range(-1.0f32, 1.0f32) * self.shake_mag;
            self.shake_mag *= 0.85;
        } else {
            self.shake_x = 0.0;
            self.shake_y = 0.0;
            self.shake_mag = 0.0;
        }

        // --- Input ---
        let confirm = is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::X);

        match self.state {
            GameState::Title => {
                if confirm {
                    self.reset();
                }
                return;
            }
            GameState::GameOver => {
                if confirm {
                    self.reset();
                }
                return;
            }
            GameState::Playing => {}
        }

        // ================================================================
        //  PLAYING
        // ================================================================

        // --- Player movement ---
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;
        if is_key_down(KeyCode::Left)  { dx -= 1.0; }
        if is_key_down(KeyCode::Right) { dx += 1.0; }
        if is_key_down(KeyCode::Up)    { dy -= 1.0; }
        if is_key_down(KeyCode::Down)  { dy += 1.0; }

        let mag = (dx * dx + dy * dy).sqrt();
        if mag > 0.001 {
            self.px += (dx / mag) * PLAYER_SPEED;
            self.py += (dy / mag) * PLAYER_SPEED;
        }
        self.px = self.px.clamp(PLAYER_RADIUS, GAME_W - PLAYER_RADIUS);
        self.py = self.py.clamp(PLAYER_RADIUS, GAME_H - PLAYER_RADIUS);

        // Trail
        self.trail.insert(0, TrailPoint { x: self.px, y: self.py });
        if self.trail.len() > TRAIL_LEN {
            self.trail.pop();
        }

        // --- Scoring ---
        self.score += SCORE_PER_FRAME;

        // --- Level-up ---
        self.level_timer += 1;
        if self.level_timer >= LEVEL_UP_FRAMES && self.level < MAX_LEVEL {
            self.level += 1;
            self.level_timer = 0;
        }

        // --- Spawn pattern ---
        self.spawn_timer += 1;
        if self.spawn_timer >= self.spawn_interval() {
            self.spawn_timer = 0;
            self.spawn_pattern();
        }

        // --- Tick pending bullets ---
        let mut new_bullets: Vec<Bullet> = Vec::new();
        let mut i = self.pending.len();
        while i > 0 {
            i -= 1;
            self.pending[i].delay -= 1;
            if self.pending[i].delay <= 0 {
                let p = &self.pending[i];
                if new_bullets.len() + self.bullets.len() < MAX_BULLETS {
                    new_bullets.push(Bullet::new(p.x, p.y, p.vx, p.vy, p.color, p.radius));
                }
                self.pending.remove(i);
            }
        }
        self.bullets.extend(new_bullets);

        // --- Update bullets + collision ---
        let mut hit = false;
        for b in self.bullets.iter_mut() {
            b.x += b.vx;
            b.y += b.vy;
            // Out-of-bounds
            if b.x < -BULLET_OOB || b.x > GAME_W + BULLET_OOB
               || b.y < -BULLET_OOB || b.y > GAME_H + BULLET_OOB
            {
                b.alive = false;
                continue;
            }
            // Collision with player
            if Self::circle_hit(b.x, b.y, b.radius, self.px, self.py) {
                b.alive = false;
                hit = true;
            }
        }

        // Cleanup dead bullets (index-based reverse remove)
        let mut j = self.bullets.len();
        while j > 0 {
            j -= 1;
            if !self.bullets[j].alive {
                self.bullets.remove(j);
            }
        }

        // --- Game over ---
        if hit {
            if self.score > self.high_score {
                self.high_score = self.score;
            }
            // Explosion particles
            self.spawn_particles(self.px, self.py, 60, Color::new(0.0, 0.95, 1.0, 1.0));
            self.spawn_particles(self.px, self.py, 20, WHITE);
            self.trigger_shake(10.0);
            self.state = GameState::GameOver;
        }
    }

    // ------------------------------------------------------------------
    // Draw
    // ------------------------------------------------------------------
    fn draw(&self) {
        clear_background(Color::new(0.02, 0.02, 0.047, 1.0));

        let sx = self.shake_x;
        let sy = self.shake_y;

        // --- Stars ---
        for (idx, s) in self.stars.iter().enumerate() {
            let twinkle = 0.65 + 0.35 * ((self.frame as f32 * 0.05 + idx as f32 * 1.7).sin());
            let alpha = (s.brightness * twinkle).clamp(0.3, 1.0);
            draw_rectangle(
                s.x + sx, s.y + sy, s.size, s.size,
                Color::new(1.0, 1.0, 1.0, alpha),
            );
        }

        // --- Grid lines ---
        {
            let gc = Color::new(0.0, 1.0, 1.0, 0.04);
            let step = 50.0;
            let mut gx = 0.0f32;
            while gx <= GAME_W {
                draw_line(gx + sx, 0.0, gx + sx, GAME_H, 1.0, gc);
                gx += step;
            }
            let mut gy = 0.0f32;
            while gy <= GAME_H {
                draw_line(0.0, gy + sy, GAME_W, gy + sy, 1.0, gc);
                gy += step;
            }
        }

        // --- Particles ---
        for p in &self.particles {
            let c = Color::new(p.color.r, p.color.g, p.color.b, p.life.clamp(0.0, 1.0));
            draw_circle(p.x + sx, p.y + sy, 2.0, c);
        }

        // --- Bullets ---
        for b in &self.bullets {
            // Glow ring
            let gc = Color::new(b.color.r, b.color.g, b.color.b, 0.25);
            draw_circle(b.x + sx, b.y + sy, b.radius + 3.0, gc);
            // Core
            draw_circle(b.x + sx, b.y + sy, b.radius, b.color);
            // Bright centre highlight
            draw_circle(b.x + sx - 1.0, b.y + sy - 1.0, b.radius * 0.4, WHITE);
        }

        // --- Player trail ---
        if self.state == GameState::Playing && self.trail.len() > 1 {
            for k in 1..self.trail.len() {
                let alpha = 1.0 - k as f32 / TRAIL_LEN as f32;
                let tc = Color::new(0.0, 0.95, 1.0, alpha * 0.18);
                let t0 = &self.trail[k - 1];
                let t1 = &self.trail[k];
                draw_line(t0.x + sx, t0.y + sy, t1.x + sx, t1.y + sy, 2.0, tc);
            }
        }

        // --- Player ---
        if self.state == GameState::Playing {
            self.draw_player(sx, sy);
        }

        // --- HUD ---
        if self.state == GameState::Playing {
            self.draw_hud();
        }

        // --- Overlay screens ---
        match self.state {
            GameState::Title    => self.draw_title(),
            GameState::GameOver => self.draw_game_over(),
            GameState::Playing  => {}
        }

        // --- CRT scanlines ---
        {
            let sc = Color::new(0.0, 0.0, 0.0, 0.12);
            let mut y = 0.0f32;
            while y < GAME_H {
                draw_rectangle(0.0, y, GAME_W, 2.0, sc);
                y += 4.0;
            }
        }

        // --- Vignette ---
        {
            let depth = 60.0f32;
            let steps = 12i32;
            let step_size = depth / steps as f32;
            for i in 0..steps {
                let t = 1.0 - i as f32 / steps as f32;
                let alpha = t * 0.4;
                let c = Color::new(0.0, 0.0, 0.0, alpha);
                let offset = i as f32 * step_size;
                draw_rectangle(0.0, offset, GAME_W, step_size, c);
                draw_rectangle(0.0, GAME_H - offset - step_size, GAME_W, step_size, c);
                draw_rectangle(offset, 0.0, step_size, GAME_H, c);
                draw_rectangle(GAME_W - offset - step_size, 0.0, step_size, GAME_H, c);
            }
        }
    }

    fn draw_player(&self, sx: f32, sy: f32) {
        let cx = self.px + sx;
        let cy = self.py + sy;

        // Glow circle behind ship
        draw_circle(cx, cy, PLAYER_RADIUS + 4.0, Color::new(0.0, 0.95, 1.0, 0.12));

        // Triangle ship: tip up
        let tip = Vec2::new(cx, cy - 9.0);
        let bl  = Vec2::new(cx - 7.0, cy + 6.0);
        let br  = Vec2::new(cx + 7.0, cy + 6.0);

        // Dark fill
        draw_triangle(tip, br, bl, Color::new(0.0, 0.1, 0.15, 0.9));

        // Cyan outline
        let col = Color::new(0.0, 0.95, 1.0, 1.0);
        draw_line(tip.x, tip.y, br.x, br.y, 2.0, col);
        draw_line(br.x, br.y, bl.x, bl.y, 2.0, col);
        draw_line(bl.x, bl.y, tip.x, tip.y, 2.0, col);

        // Engine glow dot
        draw_circle(cx, cy + 4.0, 2.5, col);
    }

    fn draw_hud(&self) {
        let score_txt = format!("SCORE: {}", self.score as u64);
        let level_txt = format!("LVL: {}", self.level);
        let hi_txt    = format!("BEST: {}", self.high_score as u64);

        draw_text(&score_txt, 10.0, 22.0, 20.0, Color::new(0.0, 0.95, 1.0, 1.0));

        let ltw = measure_text(&level_txt, None, 20, 1.0).width;
        draw_text(&level_txt, GAME_W / 2.0 - ltw / 2.0, 22.0, 20.0, Color::new(0.0, 0.95, 1.0, 1.0));

        let htw = measure_text(&hi_txt, None, 16, 1.0).width;
        draw_text(&hi_txt, GAME_W - htw - 10.0, 22.0, 16.0, Color::new(0.0, 0.95, 1.0, 0.6));

        // Level bar at the bottom showing progress to next level
        if self.level < MAX_LEVEL {
            let bar_w = GAME_W * 0.5;
            let bar_x = (GAME_W - bar_w) / 2.0;
            let bar_y = GAME_H - 16.0;
            let progress = self.level_timer as f32 / LEVEL_UP_FRAMES as f32;

            draw_rectangle(bar_x, bar_y, bar_w, 6.0, Color::new(0.0, 0.3, 0.4, 0.5));
            draw_rectangle(bar_x, bar_y, bar_w * progress, 6.0, Color::new(0.0, 0.95, 1.0, 0.7));
            draw_rectangle_lines(bar_x, bar_y, bar_w, 6.0, 1.0, Color::new(0.0, 0.95, 1.0, 0.3));
        }
    }

    fn draw_title(&self) {
        // Semi-transparent overlay
        draw_rectangle(0.0, 0.0, GAME_W, GAME_H, Color::new(0.0, 0.0, 0.05, 0.82));

        let title = "NOVA EVADER";
        let tw = measure_text(title, None, 48, 1.0).width;
        draw_text(title, GAME_W / 2.0 - tw / 2.0, GAME_H * 0.28, 48.0, Color::new(0.0, 0.95, 1.0, 1.0));

        let sub = "BULLET HELL";
        let sw = measure_text(sub, None, 22, 1.0).width;
        draw_text(sub, GAME_W / 2.0 - sw / 2.0, GAME_H * 0.28 + 36.0, 22.0, Color::new(1.0, 0.2, 0.4, 1.0));

        // Flavour
        let lore = "Floating Control System Online.";
        let lw = measure_text(lore, None, 14, 1.0).width;
        draw_text(lore, GAME_W / 2.0 - lw / 2.0, GAME_H * 0.47, 14.0, Color::new(0.0, 0.95, 1.0, 0.7));

        // Instructions
        let lines: &[&str] = &[
            "D-Pad  — move ship",
            "Dodge bullet patterns to survive",
            "Level rises every 10 seconds",
        ];
        for (idx, line) in lines.iter().enumerate() {
            let lnw = measure_text(line, None, 14, 1.0).width;
            let y = GAME_H * 0.56 + idx as f32 * 22.0;
            draw_text(line, GAME_W / 2.0 - lnw / 2.0, y, 14.0, Color::new(0.0, 0.95, 1.0, 0.8));
        }

        // Prompt
        let prompt = "START / A / B  - Launch Mission";
        let pw = measure_text(prompt, None, 16, 1.0).width;
        // Blink
        let blink = ((self.frame / 30) % 2 == 0) as i32;
        if blink == 1 {
            draw_text(prompt, GAME_W / 2.0 - pw / 2.0, GAME_H * 0.82, 16.0, WHITE);
        }

        if self.high_score > 0.0 {
            let hi = format!("BEST SCORE: {}", self.high_score as u64);
            let hiw = measure_text(&hi, None, 14, 1.0).width;
            draw_text(&hi, GAME_W / 2.0 - hiw / 2.0, GAME_H * 0.90, 14.0, Color::new(1.0, 0.84, 0.0, 1.0));
        }
    }

    fn draw_game_over(&self) {
        // Dim overlay
        draw_rectangle(0.0, 0.0, GAME_W, GAME_H, Color::new(0.05, 0.0, 0.02, 0.75));

        let title = "SYSTEM CRITICAL";
        let tw = measure_text(title, None, 36, 1.0).width;
        draw_text(title, GAME_W / 2.0 - tw / 2.0, GAME_H * 0.32, 36.0, Color::new(1.0, 0.2, 0.4, 1.0));

        let score_txt = format!("FINAL SCORE: {}", self.score as u64);
        let stw = measure_text(&score_txt, None, 24, 1.0).width;
        draw_text(&score_txt, GAME_W / 2.0 - stw / 2.0, GAME_H * 0.46, 24.0, Color::new(0.0, 0.95, 1.0, 1.0));

        if self.score >= self.high_score && self.high_score > 0.0 {
            let new_hi = "NEW BEST!";
            let nhw = measure_text(new_hi, None, 20, 1.0).width;
            draw_text(new_hi, GAME_W / 2.0 - nhw / 2.0, GAME_H * 0.54, 20.0, Color::new(1.0, 0.84, 0.0, 1.0));
        }

        let lvl_txt = format!("REACHED LEVEL {}", self.level);
        let ltw = measure_text(&lvl_txt, None, 18, 1.0).width;
        draw_text(&lvl_txt, GAME_W / 2.0 - ltw / 2.0, GAME_H * 0.63, 18.0, Color::new(0.0, 0.95, 1.0, 0.8));

        // Prompt
        let prompt = "START / A / B  -  Reboot Ship";
        let pw = measure_text(prompt, None, 16, 1.0).width;
        let blink = ((self.frame / 30) % 2 == 0) as i32;
        if blink == 1 {
            draw_text(prompt, GAME_W / 2.0 - pw / 2.0, GAME_H * 0.78, 16.0, WHITE);
        }
    }
}

// ---------------------------------------------------------------------------
// Window config
// ---------------------------------------------------------------------------
fn window_conf() -> Conf {
    Conf {
        window_title: "Nova Evader".to_string(),
        window_width: GAME_W as i32,
        window_height: GAME_H as i32,
        fullscreen: false,
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------
#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);

    let mut game = Game::new();

    // Fixed-timestep accumulator (death-spiral prevention)
    let fixed_dt = 1.0 / 60.0;
    let mut accumulator = 0.0f64;
    let mut last_time = get_time();

    loop {
        let now = get_time();
        let mut frame_dt = now - last_time;
        last_time = now;

        // Cap to 100 ms to prevent death spiral
        if frame_dt > 0.1 { frame_dt = 0.1; }

        accumulator += frame_dt;

        // Fixed-step updates
        while accumulator >= fixed_dt {
            accumulator -= fixed_dt;
            game.update();
        }

        // Render
        game.draw();

        next_frame().await;
    }
}
