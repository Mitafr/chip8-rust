use crate::chip8::memory::Memory;
use crate::driver::gfx::Gfx;
extern crate rand;

use rand::Rng;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::thread;

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
    events: EventPump,
    key: [bool; 16],
}

impl Chip8 {
    pub fn new(rom: String) -> Chip8 {
        let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
        let mut events: EventPump = sdl_context.event_pump().unwrap();
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
            events: events,
            key: [false; 16],
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
        'main: loop {
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit {..} => break 'main,
                    Event::KeyDown {keycode: Some(keycode), ..} => {
                        match keycode {
                            Keycode::Escape => break 'main,
                            Keycode::A => self.key[0] = true,
                            Keycode::Z => self.key[1] = true,
                            Keycode::E => self.key[2] = true,
                            Keycode::R => self.key[3] = true,
                            Keycode::T => self.key[4] = true,
                            Keycode::Y => self.key[5] = true,
                            Keycode::U => self.key[6] = true,
                            Keycode::I => self.key[7] = true,
                            Keycode::O => self.key[8] = true,
                            Keycode::P => self.key[9] = true,
                            Keycode::Num0 => self.key[10] = true,
                            Keycode::Num1 => self.key[11] = true,
                            Keycode::Num2 => self.key[12] = true,
                            Keycode::Num3 => self.key[13] = true,
                            Keycode::Num4 => self.key[14] = true,
                            Keycode::Num5 => self.key[15] = true,
                            _ => {}
                        }
                    }
                    Event::KeyUp {keycode: Some(keycode), ..} => {
                        match keycode {
                            Keycode::A => self.key[0] = false,
                            Keycode::Z => self.key[1] = false,
                            Keycode::E => self.key[2] = false,
                            Keycode::R => self.key[3] = false,
                            Keycode::T => self.key[4] = false,
                            Keycode::Y => self.key[5] = false,
                            Keycode::U => self.key[6] = false,
                            Keycode::I => self.key[7] = false,
                            Keycode::O => self.key[8] = false,
                            Keycode::P => self.key[9] = false,
                            Keycode::Num0 => self.key[10] = false,
                            Keycode::Num1 => self.key[11] = false,
                            Keycode::Num2 => self.key[12] = false,
                            Keycode::Num3 => self.key[13] = false,
                            Keycode::Num4 => self.key[14] = false,
                            Keycode::Num5 => self.key[15] = false,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            self.emulate();
            while self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            while self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
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
        println!("{:x?}", opcode);
        match opcode & 0xf000 {
            0x1000 => {
                self.pc = decoded as usize;
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
                match opcode & 0x000F {
                    0x0000 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        let y: usize = ((opcode & 0x00F0) >> 4) as usize;
                        self.registers[x] = self.registers[y];
                        self.pc += 2;
                    }
                    0x0001 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        let y: usize = ((opcode & 0x00F0) >> 4) as usize;
                        self.registers[x] |= self.registers[y];
                        self.pc += 2;
                    }
                    0x0002 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        let y: usize = ((opcode & 0x00F0) >> 4) as usize;
                        self.registers[x] &= self.registers[y];
                        self.pc += 2;
                    }
                    0x0003 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        let y: usize = ((opcode & 0x00F0) >> 4) as usize;
                        self.registers[x] ^= self.registers[y];
                        self.pc += 2;
                    }
                    0x0004 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        let y: usize = ((opcode & 0x00F0) >> 4) as usize;
                        let r: u16 = self.registers[x] as u16 + self.registers[y] as u16;
                        if r > 255 {
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[0xF] = 0;
                        }
                        self.registers[x] = r as u8;
                        self.pc += 2;
                    }
                    0x0005 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        let y: usize = ((opcode & 0x00F0) >> 4) as usize;
                        if self.registers[x] > self.registers[y] {
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[0xF] = 0;
                        }
                        self.registers[x] -= self.registers[y];
                        self.pc += 2;
                    }
                    0x0006 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        if (self.registers[x] & 0x01) == 0x01 {
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[0xF] = 0;
                        }
                        self.registers[x] /= 2;
                        self.pc += 2;
                    }
                    0x0007 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        let y: usize = ((opcode & 0x00F0) >> 4) as usize;
                        if self.registers[y] > self.registers[x] {
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[0xF] = 0;
                        }
                        self.registers[x] -= self.registers[y];
                        self.pc += 2;
                    }
                    0x000E => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        if (self.registers[x] & 0x01) == 0x01 {
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[0xF] = 0;
                        }
                        self.registers[x] *= 2;
                        self.pc += 2;
                    }
                    _ => {
                        println!("unrecognized opcode : {:x?}", opcode & 0x000f);
                    }
                }
            },
            0x9000 => {
                let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                let y: usize = ((opcode & 0x00F0) >> 4) as usize;
                self.pc += match  self.registers[x] != self.registers[y] {
                    true => 4,
                    false => 2,
                }
            }
            0xA000 => {
                self.index_register = decoded;
                self.pc += 2;
            },
            0xB000 => {
                self.index_register = decoded;
                self.pc += 2;
            }
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
                let x = (opcode & 0x0F00) >> 8;
                let y = (opcode & 0x00F0) >> 4;
                let mut pixel: u8;
                self.registers[0xF] = 0;
                for i in 0..n {
                    pixel = self.mem.mem[self.index_register as usize + i as usize];
                    for j in 0..7 {
                        let coordx = self.registers[x as usize];
                        let coordy = self.registers[y as usize];
                        if (pixel & (0x80 >> j)) != 0 {
                            if self.gfx.display[(coordx + j) as usize][((coordy as u16 + i as u16)) as usize] == 1 {
                                self.registers[0xF] = 1;
                                println!("Collision");
                            }
                            self.gfx.set_pixel((coordx + j) as u32, ((coordy as u16 + i as u16)) as u32, pixel);
                        }
                    }
                }
                let r = self.gfx.draw_screen();
                match r {
                    Ok(()) => {},
                    Err(err) => panic!("Erreur de rendu : {}", err)
                }
                self.pc += 2;
            },
            0xE000 => {
                match opcode & 0x00FF {
                    0x009E => {
                        self.pc += 2;
                        let x = (opcode & 0x0F00) >> 8;
                        if self.key[x as usize] {
                            self.pc += 2;
                        }
                    }
                    0x00A1 => {
                        self.pc += 2;
                        let x = (opcode & 0x0F00) >> 8;
                        if !self.key[x as usize] {
                            self.pc += 2;
                        }
                    }
                    _ => {}
                }
            }
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        self.registers[x] = self.delay_timer;
                        self.pc += 2;
                    }
                    0x0015 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        self.delay_timer = self.registers[x];
                        self.pc += 2;
                    }
                    0x0018 => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        self.sound_timer = self.registers[x];
                        self.pc += 2;
                    }
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
                    0x001e => {
                        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                        self.index_register += self.registers[x] as u16;
                        self.pc += 2;
                    }  
                    _ => {
                        println!("unrecognized opcode : {:x?}", opcode & 0xffff);
                    }
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
                    _ => {
                        println!("unrecognized opcode : {:x?}", opcode & 0xf000);
                    }
                }
            }
            _ => {
                println!("unrecognized opcode : {:x?}", opcode & 0xf000);
            }
        }
    }
}
