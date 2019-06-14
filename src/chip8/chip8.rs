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
    index_register: usize,
    events: EventPump,
    key: [bool; 16],
    wait_for_key: bool,
    key_in_register: usize,
    draw: bool,
}

impl Chip8 {
    pub fn new(rom: String) -> Chip8 {
        let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
        let events: EventPump = sdl_context.event_pump().unwrap();
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
            events: events,
            key: [false; 16],
            wait_for_key: false,
            key_in_register: 0,
            draw: false,
        }
    }
    pub fn init(&mut self) -> Result<(), String> {
        self.mem.load_rom(&self.rom)?;
        println!("{}", self.mem);
        self.gfx.clear();
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
            if !self.wait_for_key {
                let opcode = self.fetch_op();
                self.execute_op(opcode);
                while self.delay_timer > 0 {
                    self.delay_timer -= 1;
                }
                while self.sound_timer > 0 {
                    self.sound_timer -= 1;
                }
            } else {
                for i in 0..self.key.len() {
                    if self.key[i] {
                        self.wait_for_key = false;
                        self.registers[self.key_in_register as usize] = i as u8;
                        break;
                    }
                }
            }
            if self.draw {
                let r = self.gfx.draw_screen();
                match r {
                    Ok(()) => {},
                    Err(err) => panic!("Erreur de rendu : {}", err)
                }
                self.draw = false;
            }
            thread::sleep(sleep_duration);
        }
        Ok(())
    }
    pub fn fetch_op(&mut self) -> u16 {
        (self.mem.mem[self.pc] as u16) << 8 | (self.mem.mem[self.pc + 1] as u16)
    }
    pub fn execute_op(&mut self, opcode: u16) {
        let decoded: u16 = opcode & 0x0FFF;
        println!("Current opcode: {:x?}", opcode);
        let n: u8 = (opcode & 0x000F) as u8;
        let kk: u8 = (opcode & 0x00FF) as u8;
        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
        let y: usize = ((opcode & 0x00F0) >> 4) as usize;
        match opcode & 0xf000 {
            0x1000 => {
                self.pc = decoded as usize;
            },
            0x2000 => {
                self.stack.push((self.pc + 2) as u16);
                self.pc = decoded as usize;
            },
            0x3000 => {
                self.pc += if self.registers[x] == kk { 4 } else { 2 };
            },
            0x4000 => {
                self.pc += if self.registers[x] != kk { 4 } else { 2 };
            },
            0x5000 => {
                self.pc += if self.registers[x] == self.registers[y] { 4 } else { 2 };
            },
            0x6000 => {
                self.registers[x] = kk;
                self.pc += 2;
            },
            0x7000 => {
                let vx = self.registers[x];
                self.registers[x] = (vx + kk) as u8;
                self.pc += 2;
            },
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        self.registers[x] = self.registers[y];
                        self.pc += 2;
                    }
                    0x0001 => {
                        self.registers[x] |= self.registers[y];
                        self.pc += 2;
                    }
                    0x0002 => {
                        self.registers[x] &= self.registers[y];
                        self.pc += 2;
                    }
                    0x0003 => {
                        self.registers[x] ^= self.registers[y];
                        self.pc += 2;
                    }
                    0x0004 => {
                        let res: u16 = (self.registers[x] + self.registers[y]) as u16;
                        self.registers[x] = res as u8;
                        self.registers[0x0F] = if res > 0xFF { 1 } else { 0 };
                        self.pc += 2;
                    }
                    0x0005 => {
                        self.registers[0x0F] = if self.registers[x] > self.registers[y] { 1 } else { 0 };
                        self.registers[x] -= self.registers[y];
                        self.pc += 2;
                    }
                    0x0006 => {
                        self.registers[0x0F] = self.registers[x] & 0x1;
                        self.registers[x] >>= 1;
                        self.pc += 2;
                    }
                    0x0007 => {
                        self.registers[0x0F] = if self.registers[x] > self.registers[y] { 0 } else { 1 };
                        self.registers[x] = self.registers[y] - self.registers[x];
                        self.pc += 2;
                    }
                    0x000E => {
                        self.registers[0xF] = self.registers[x] >> 7;
                        self.registers[x] <<= 1;
                        self.pc += 2;
                    }
                    _ => {
                        println!("unrecognized opcode : {:x?}", opcode & 0x000f);
                    }
                }
            },
            0x9000 => {
                self.pc += if self.registers[x] != self.registers[y] { 4 } else { 2 };
            }
            0xA000 => {
                self.index_register = (opcode & 0x0FFF) as usize;
                self.pc += 2;
            },
            0xB000 => {
                self.index_register = (decoded + self.registers[0] as u16) as usize;
            }
            0xC000 => {
                let mut rng = rand::thread_rng();
                let random: u8 = rng.gen_range(0, 255);
                self.registers[x] = kk & random;
                self.pc += 2;
            }
            0xD000 => {
                let mut pixel: u8;
                self.registers[0xF] = 0;
                for i in 0..n {
                    pixel = self.mem.mem[self.index_register + i as usize];
                    for j in 0..8 {
                        let y1 = (self.registers[y as usize] as usize + i as usize) % 64;
                        if (pixel & (0x80 >> j)) != 0 {
                            let x1 = (self.registers[x as usize] + j) % 64;
                            let color = (self.mem.mem[self.index_register + i as usize] >> (7 - j)) & 1;
                            self.registers[0xF] = if self.gfx.display[x1 as usize][y1 as usize] == 1 { 1 } else { 0 };
                            self.gfx.set_pixel(x1 as u32, y1 as u32, color);
                        }
                    }
                }
                self.draw = true;
                self.pc += 2;
            },
            0xE000 => {
                match opcode & 0x00FF {
                    0x009E => {
                        self.pc += 2;
                        if self.key[x] {
                            self.pc += 4;
                        }
                    }
                    0x00A1 => {
                        self.pc += 2;
                        if !self.key[x] {
                            self.pc += 2;
                        }
                    }
                    _ => {}
                }
            }
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => {
                        self.registers[x] = self.delay_timer;
                        self.pc += 2;
                    }
                    0x000A => {
                        self.wait_for_key = true;
                        self.key_in_register = x;
                        self.pc -= 2;
                    },
                    0x0015 => {
                        self.delay_timer = self.registers[x];
                        self.pc += 2;
                    }
                    0x0018 => {
                        self.sound_timer = self.registers[x];
                        self.pc += 2;
                    }
                    0x0029 => {
                        self.index_register = (self.registers[x] as usize) * 5;
                        self.pc += 2;
                    },
                    0x0033 => {
                        self.mem.mem[self.index_register] = self.registers[x] / 100;
                        self.mem.mem[self.index_register + 1] = (self.registers[x] % 100) / 10;
                        self.mem.mem[self.index_register + 2] = self.registers[x] %10;
                        self.pc += 2;
                    },
                    0x0055 => {
                        for i in 0..x + 1 {
                            self.mem.mem[self.index_register + i] = self.registers[i]
                        }
                        self.pc += 2;
                    },
                    0x0065 => {
                        for i in 0..x + 1 {
                            self.registers[i] = self.mem.mem[self.index_register + i];
                        }
                        self.pc += 2;
                    },
                    0x001e => {
                        self.index_register += self.registers[x] as usize;
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
                        self.draw = true;
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
