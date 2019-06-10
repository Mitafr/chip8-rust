use crate::chip8::memory::Memory;
use crate::driver::gfx::Gfx;
extern crate rand;

use rand::Rng;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::thread;
use std::fs;
use std::io;
use std::io::Write;

pub struct Chip8 {
    registers: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    pc: usize,
    stack: Vec<u16>,
    mem: Memory,
    rom: String,
    gfx: Gfx,
    context: sdl2::Sdl,
    index_register: u16,
}

impl Chip8 {
    pub fn new(rom: String) -> Chip8 {
        let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
        Chip8 {
            registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            stack: Vec::new(),
            index_register: 0,
            mem: Memory::new(),
            rom: rom,
            gfx: Gfx::new(&sdl_context, "Test"),
            context: sdl_context,
        }
    }
    pub fn init(&mut self) -> Result<(), String> {
        self.mem.load_rom(&self.rom)?;
        self.gfx.clear();
        self.gfx.update();
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        let sleep_duration = Duration::from_millis(16);
        let mut events = self.context.event_pump()?;
        'main: loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit {..} => break 'main,
                    Event::KeyDown {keycode: Some(keycode), ..} => {
                        if keycode == Keycode::Escape {
                            break 'main
                        }
                    }
                    _ => {}
                }
            }
            self.emulate();
            thread::sleep(sleep_duration);
        }
        Ok(())
    }
    pub fn emulate(&mut self) {
        let opcode: u16 = self.fetch_op();
        self.execute_op(opcode);
    }
    pub fn fetch_op(&mut self) -> u16 {
        (self.mem.mem[self.pc] as u16) << 8 | (self.mem.mem[self.pc + 1] as u16)
    }
    pub fn decode_op(&mut self, opcode: u16) -> u16 {
        opcode & 0x0FFF
    }
    pub fn execute_op(&mut self, opcode: u16) {
        let decoded: u16 = self.decode_op(opcode);
        match opcode & 0xf000 {
            0xA000 => {
                self.index_register = decoded;
                self.pc += 2;
            },
            0x1000 => {
                self.pc = decoded as usize;
                //println!("{}", self.pc);
            },
            0x2000 => {
                self.stack.push(self.pc as u16);
                self.pc = decoded as usize;
            },
            0x3000 => {
                let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                let kk: u8 = (opcode & 0x00FF) as u8;
                self.pc += match self.registers[x] == kk {
                    true => 4,
                    false => 2,
                }
            },
            0x4000 => {
                let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                let kk: u8 = (opcode & 0x00FF) as u8;
                self.pc += match self.registers[x] != kk {
                    true => 4,
                    false => 2,
                }
            },
            0x6000 => {
                let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                self.registers[x] = (opcode & 0x00FF) as u8;
                self.pc += 2;
            },
            0x7000 => {
                //Adds the value kk to the value of register Vx, then stores the result in Vx. 
                let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                let kk: u8 = (opcode & 0x00FF) as u8;
                let vx = self.registers[x] as u16;
                let val = kk as u16;
                let result = vx + val;
                self.registers[x] = result as u8;
                self.pc += 2;
            },
            0x8000 => {
                let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                let y: usize = ((opcode & 0x00F0) >> 4) as usize;
                self.registers[x] = self.registers[y];
                self.pc += 2;
            },
            0xC000 => {
                let mut rng = rand::thread_rng();
                let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                let kk: u8 = (opcode & 0x00FF) as u8;
                let random: u8 = rng.gen_range(0, 255);
                self.registers[x] = kk & random;
                self.pc += 2;
            }
            0xD000 => {
                let n: u8 = (opcode & 0x000F) as u8;
                let y = (opcode & 0x00F0) >> 4;
                let x = (opcode & 0x0F00) >> 8;
                let mut byte: u8;
                let mut i: usize = 0;
                while i < n as usize {
                    byte = self.mem.mem[self.index_register as usize  + i];
                    self.gfx.set_pixel(x as u32, y as u32, byte);
                    self.gfx.draw_screen();
                    i += 1;
                }
                self.pc += 2;
            },
            0xF000 => {
                match opcode & 0x00FF {
                    0x0029 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        self.index_register = (self.registers[x] as u16) * 5;
                        self.pc += 2;
                    },
                    0x0033 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        self.mem.mem[self.index_register as usize] = self.registers[x] / 100;
                        self.mem.mem[self.index_register as usize + 1] = (self.registers[x] % 100) / 10;
                        self.mem.mem[self.index_register as usize + 2] = self.registers[x] % 10;
                        self.pc += 2;
                    },
                    0x0055 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        for i in 0..x {
                            self.mem.mem[i + 1] = self.registers[i];
                        }
                        self.pc += 2;
                    },
                    0x0065 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        for i in 0..x {
                            self.registers[i] = self.mem.mem[i + 1];
                        }
                        self.pc += 2;
                    },
                    _ => {}
                }
            },
            0x0000 => {
                match opcode {
                    0x00EE => {
                        self.pc = self.stack.pop().unwrap() as usize;
                    },
                    0x00E0 => {
                        self.gfx.clear();
                        self.pc += 2;
                    },
                    _ => {}
                }
            }
            _ => {
                println!("unrecognized opcode : {:x?}", opcode & 0xf000);
            }
        }
    }
}