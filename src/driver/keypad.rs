use sdl2::keyboard::Keycode;

pub struct Keypad {
    keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: [false; 16],
        }
    }
    pub fn is_pressed(&mut self, x: usize) -> bool {
        self.keys[x]
    }
    pub fn press(&mut self, keycode: Keycode, state: bool) {
        match keycode {
            Keycode::A => self.keys[0] = state,
            Keycode::Z => self.keys[1] = state,
            Keycode::E => self.keys[2] = state,
            Keycode::R => self.keys[3] = state,
            Keycode::T => self.keys[4] = state,
            Keycode::Y => self.keys[5] = state,
            Keycode::U => self.keys[6] = state,
            Keycode::I => self.keys[7] = state,
            Keycode::O => self.keys[8] = state,
            Keycode::P => self.keys[9] = state,
            Keycode::Q => self.keys[10] = state,
            Keycode::S => self.keys[11] = state,
            Keycode::D => self.keys[12] = state,
            Keycode::F => self.keys[13] = state,
            Keycode::G => self.keys[14] = state,
            Keycode::H => self.keys[15] = state,
            _ => {}
        }
    }
}