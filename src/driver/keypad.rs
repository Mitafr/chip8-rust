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
            Keycode::A => self.keys[0x0] = state,
            Keycode::Z => self.keys[0x1] = state,
            Keycode::E => self.keys[0x2] = state,
            Keycode::R => self.keys[0x3] = state,
            Keycode::Q => self.keys[0x4] = state,
            Keycode::Y => self.keys[0x5] = state,
            Keycode::D => self.keys[0x6] = state,
            Keycode::I => self.keys[0x7] = state,
            Keycode::O => self.keys[0x8] = state,
            Keycode::P => self.keys[0x9] = state,
            Keycode::T => self.keys[0xA] = state,
            Keycode::S => self.keys[0xB] = state,
            Keycode::B => self.keys[0xC] = state,
            Keycode::F => self.keys[0xD] = state,
            Keycode::G => self.keys[0xE] = state,
            Keycode::H => self.keys[0xF] = state,
            _ => {}
        }
    }
}