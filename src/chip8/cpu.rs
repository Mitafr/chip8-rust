use std::num::Wrapping;
pub struct Cpu {
    pub registers: Vec<Wrapping<u8>>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: vec![],
            delay_timer: 0x00,
            sound_timer: 0x00,
            pc: 0x200,
        }
    }
}