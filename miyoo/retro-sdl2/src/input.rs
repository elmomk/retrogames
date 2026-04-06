use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::EventPump;
use std::collections::HashSet;

/// Logical key codes matching macroquad's KeyCode subset used in Miyoo ports.
/// D-Pad: Left/Right/Up/Down
/// Face buttons: X = A, Space = B
/// Menu: Return = Start, Escape = quit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Left,
    Right,
    Up,
    Down,
    Space,
    Z,
    X,
    Return,
    Escape,
}

impl KeyCode {
    fn to_scancode(self) -> Scancode {
        match self {
            KeyCode::Left => Scancode::Left,
            KeyCode::Right => Scancode::Right,
            KeyCode::Up => Scancode::Up,
            KeyCode::Down => Scancode::Down,
            KeyCode::Space => Scancode::Space,
            KeyCode::Z => Scancode::Z,
            KeyCode::X => Scancode::X,
            KeyCode::Return => Scancode::Return,
            KeyCode::Escape => Scancode::Escape,
        }
    }
}

/// Keyboard + quit-event input handler.
pub struct Input {
    event_pump: EventPump,
    /// Keys held this frame.
    current: HashSet<Scancode>,
    /// Keys held the previous frame.
    previous: HashSet<Scancode>,
    /// Set to true when the user closes the window or presses Escape.
    quit: bool,
}

impl Input {
    /// Create an Input handler from an SDL EventPump.
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            event_pump,
            current: HashSet::new(),
            previous: HashSet::new(),
            quit: false,
        }
    }

    /// Poll all pending SDL events and update key state.
    /// Call once per frame at the start of your game loop.
    pub fn poll(&mut self) {
        // Rotate current → previous.
        self.previous = self.current.clone();
        self.current.clear();

        // Process all queued events.
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.quit = true,
                Event::KeyDown {
                    scancode: Some(sc), ..
                } => {
                    if sc == Scancode::Escape {
                        self.quit = true;
                    }
                    self.current.insert(sc);
                }
                Event::KeyUp {
                    scancode: Some(sc), ..
                } => {
                    // The key is no longer held; do not insert into current.
                    let _ = sc;
                }
                _ => {}
            }
        }

        // Merge continuous press state from the keyboard snapshot
        // (handles keys held across frames even if no new KeyDown events fire).
        let kb = self.event_pump.keyboard_state();
        for sc in [
            Scancode::Left,
            Scancode::Right,
            Scancode::Up,
            Scancode::Down,
            Scancode::Space,
            Scancode::Z,
            Scancode::X,
            Scancode::Return,
            Scancode::Escape,
        ] {
            if kb.is_scancode_pressed(sc) {
                self.current.insert(sc);
            }
        }
    }

    /// Returns `true` while `key` is held down this frame.
    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.current.contains(&key.to_scancode())
    }

    /// Returns `true` only on the frame the key was first pressed (edge detection).
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        let sc = key.to_scancode();
        self.current.contains(&sc) && !self.previous.contains(&sc)
    }

    /// Returns `true` when the window-close event was received or Escape was pressed.
    pub fn should_quit(&self) -> bool {
        self.quit
    }
}
