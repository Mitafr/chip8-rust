use crate::chip8::memory::Memory;

pub struct Cpu {
    registers: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    pc: usize,
    stack: Vec<u16>,
    index_register: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            stack: Vec::new(),
            index_register: 0,
        }
    }
    pub fn emulate(&mut self, mem: &Memory) {
        let opcode: u16 = self.fetch_op(mem);
        println!("{:x?}", opcode);
        self.execute_op(opcode);
    }
    pub fn fetch_op(&mut self, mem: &Memory) -> u16 {
        println!("pc : {} ({:x?}, {:x?})", self.pc, mem.mem[self.pc], mem.mem[self.pc+1]);
        (mem.mem[self.pc] as u16) << 8 | (mem.mem[self.pc + 1] as u16)
    }
    pub fn decode_op(&mut self, opcode: u16) -> u16 {
        opcode & 0x0FFF
    }
    pub fn execute_op(&mut self, opcode: u16) {
        let decoded: u16 = self.decode_op(opcode);
        println!("execute: {:x?}", opcode);
        match opcode & 0xf000 {
            0xA000 => {
                self.index_register = decoded;
                self.pc += 2;
            },
            0x2000 => {
                self.stack.push(self.pc as u16);
                self.pc = decoded as usize;
            },
            0x6000 => {
                let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                self.registers[x] = (opcode & 0x00FF) as u8;
                self.pc += 2;
            },
            0x0000 => {
                match opcode {
                    0x00EE => {
                        self.pc = self.stack.pop().unwrap() as usize;
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}