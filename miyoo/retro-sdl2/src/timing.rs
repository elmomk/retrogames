use std::time::{Duration, Instant};

/// Fixed-timestep game clock.
///
/// Typical usage:
/// ```ignore
/// let mut clock = GameClock::new(60.0);
/// loop {
///     let dt = clock.tick();
///     update(dt);
///     draw();
/// }
/// ```
pub struct GameClock {
    target_delta: Duration,
    last_tick: Instant,
    start: Instant,
    /// Accumulated time not yet consumed by update steps.
    accumulator: f64,
}

impl GameClock {
    /// Create a clock targeting `target_fps` updates per second.
    pub fn new(target_fps: f64) -> Self {
        let now = Instant::now();
        Self {
            target_delta: Duration::from_secs_f64(1.0 / target_fps),
            last_tick: now,
            start: now,
            accumulator: 0.0,
        }
    }

    /// Return the fixed delta time in seconds (1.0 / target_fps).
    pub fn fixed_dt(&self) -> f64 {
        self.target_delta.as_secs_f64()
    }

    /// Advance the clock by the real elapsed time.
    ///
    /// Returns the fixed delta time; callers should call this in a loop until
    /// [`should_update`] returns false for a pure fixed-timestep loop, or simply
    /// use the returned value as dt for a semi-fixed update.
    pub fn tick(&mut self) -> f64 {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);
        self.last_tick = now;

        // Cap at 250 ms to prevent the "death spiral" on lag spikes.
        let capped = elapsed.min(Duration::from_millis(250));
        self.accumulator += capped.as_secs_f64();

        self.target_delta.as_secs_f64()
    }

    /// Returns true and consumes one fixed-step slice when the accumulator has
    /// enough time. Use in a `while clock.should_update() { update(clock.fixed_dt()); }` loop.
    pub fn should_update(&mut self) -> bool {
        if self.accumulator >= self.target_delta.as_secs_f64() {
            self.accumulator -= self.target_delta.as_secs_f64();
            true
        } else {
            false
        }
    }

    /// Fractional interpolation factor in [0, 1) for rendering between frames.
    pub fn alpha(&self) -> f64 {
        self.accumulator / self.target_delta.as_secs_f64()
    }

    /// Wall-clock seconds since the clock was created.
    pub fn time(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    /// Sleep the calling thread until the next frame is due.
    /// Call after `present()` to pace the loop without burning CPU.
    pub fn wait_for_next_frame(&self) {
        let now = Instant::now();
        let next = self.last_tick + self.target_delta;
        if next > now {
            std::thread::sleep(next - now);
        }
    }
}
