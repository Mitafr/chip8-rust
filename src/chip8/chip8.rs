use crate::chip8::memory::Memory;
use crate::driver::keypad::Keypad;
use crate::driver::gfx::Gfx;
extern crate rand;

use rand::Rng;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use std::thread;
use std::time::Duration;

pub struct Chip8 {
    delay_timer: u8,
    draw: bool,
    events: EventPump,
    gfx: Gfx,
    index_register: usize,
    keypad: Keypad,
    mem: Memory,
    pc: usize,
    registers: [u8; 16],
    rom: String,
    sound_timer: u8,
    stack: Vec<u16>,
}

impl Chip8 {
    pub fn new(rom: String) -> Chip8 {
        let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
        let events: EventPump = sdl_context.event_pump().unwrap();
        Chip8 {
            delay_timer: 0,
            draw: false,
            events: events,
            gfx: Gfx::new(&sdl_context, "Test"),
            index_register: 0,
            keypad: Keypad::new(),
            mem: Memory::new(),
            pc: 0x200,
            registers: [0; 16],
            rom: rom,
            sound_timer: 0,
            stack: Vec::new(),
        }
    }
    pub fn init(&mut self) -> Result<(), String> {
        self.mem.load_rom(&self.rom)?;
        println!("{}", self.mem);
        self.gfx.clear();
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        let sleep_duration = Duration::from_millis(1);
        'main: loop {
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit {..} => break 'main,
                    Event::KeyDown {keycode: Some(keycode), ..} => {
                        match keycode {
                            Keycode::Escape => break 'main,
                            _ => self.keypad.press(keycode, true),
                        }
                    }
                    Event::KeyUp {keycode: Some(keycode), ..} => self.keypad.press(keycode, false),
                    _ => {}
                }
            }
            if self.draw {
                self.gfx.draw_screen();
                self.draw = false;
            } else {
                let opcode = self.fetch_op();
                self.execute_op(opcode);
                while self.delay_timer > 0 {
                    self.delay_timer -= 1;
                }
                while self.sound_timer > 0 {
                    self.sound_timer -= 1;
                }
            }
            thread::sleep(sleep_duration);
        }
        Ok(())
    }
    fn wait_keypress(&mut self, x: usize) {
        for i in  0..15 {
            if self.keypad.is_pressed(i as usize) {
                self.registers[x] = i as u8;
                break;
            }
        }
    }
    pub fn fetch_op(&mut self) -> u16 {
        (self.mem.get_mem(self.pc) as u16) << 8 | (self.mem.get_mem(self.pc + 1) as u16)
    }
    pub fn execute_op(&mut self, opcode: u16) {
        let decoded: u16 = opcode & 0x0FFF;
        let kk: u8 = (opcode & 0x00FF) as u8;
        let n: u8 = (opcode & 0x000F) as u8;
        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
        let y: usize = ((opcode & 0x00F0) >> 4) as usize;
        match opcode & 0xf000 {
            0x1000 => self.pc = decoded as usize,
            0x2000 => {
                self.stack.push((self.pc) as u16);
                self.pc = decoded as usize;
            },
            0x3000 => self.pc += if self.registers[x] == kk { 4 } else { 2 },
            0x4000 => self.pc += if self.registers[x] != kk { 4 } else { 2 },
            0x5000 => self.pc += if self.registers[x] == self.registers[y] { 4 } else { 2 },
            0x6000 => {
                self.registers[x] = kk;
                self.pc += 2;
            },
            0x7000 => {
                self.registers[x] = self.registers[x].wrapping_add(kk);
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
                        let vx = self.registers[x] as u16;
                        let vy = self.registers[y] as u16;
                        let res = vx + vy;
                        self.registers[x] = self.registers[x].wrapping_add(self.registers[y]);
                        self.registers[0x0F] = if res > 0xFF { 1 } else { 0 };
                        self.pc += 2;
                    }
                    0x0005 => {
                        self.registers[0x0F] = if self.registers[x] > self.registers[y] { 1 } else { 0 };
                        self.registers[x] = self.registers[x].wrapping_sub(self.registers[y]);
                        self.pc += 2;
                    }
                    0x0006 => {
                        self.registers[0x0F] = self.registers[x] & 0x1;
                        self.registers[x] >>= 1;
                        self.pc += 2;
                    }
                    0x0007 => {
                        self.registers[0x0F] = if self.registers[x] > self.registers[y] { 0 } else { 1 };
                        self.registers[x] = self.registers[y].wrapping_sub(self.registers[x]);
                        self.pc += 2;
                    }
                    0x000E => {
                        self.registers[0xF] = self.registers[x] >> 7;
                        self.registers[x] <<= 1;
                        self.pc += 2;
                    }
                    _ => {
                        println!("Unrecognized opcode : {:x?}", opcode & 0x000f);
                    }
                }
            },
            0x9000 => self.pc += if self.registers[x] != self.registers[y] { 4 } else { 2 },
            0xA000 => {
                self.index_register = decoded as usize;
                self.pc += 2;
            },
            0xB000 => self.pc = (decoded + self.registers[0] as u16) as usize,
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
                    pixel = self.mem.get_mem(self.index_register + i as usize);
                    for j in 0..8 {
                        let y1 = self.registers[y as usize].wrapping_add(i) % 64;
                        if (pixel & (0x80 >> j)) != 0 {
                            let x1 = self.registers[x as usize].wrapping_add(j) % 64;
                            let color = (self.mem.get_mem(self.index_register + i as usize) >> (7 - j)) & 1;
                            self.registers[0xF] = if self.gfx.has_pixel(x1 as usize, y1 as usize) { 1 } else { 0 };
                            self.gfx.set_pixel(x1 as u32, y1 as u32, color);
                        }
                    }
                }
                self.draw = true;
                self.pc += 2;
            },
            0xE000 => {
                match opcode & 0x00FF {
                    0x009E => self.pc += if self.keypad.is_pressed(self.registers[x] as usize) { 4 } else { 2 },
                    0x00A1 => self.pc += if !self.keypad.is_pressed(self.registers[x] as usize) { 4 } else { 2 },
                    _ => {}
                }
            }
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => self.registers[x] = self.delay_timer,
                    0x000A => self.wait_keypress(x),
                    0x0015 => self.delay_timer = self.registers[x],
                    0x0018 => self.sound_timer = self.registers[x],
                    0x0029 => self.index_register = (self.registers[x] as usize) * 0x5,
                    0x0033 => {
                        self.mem.set_mem(self.index_register, self.registers[x] / 100);
                        self.mem.set_mem(self.index_register + 1, (self.registers[x] % 100) / 10);
                        self.mem.set_mem(self.index_register + 2, self.registers[x] % 10);
                    },
                    0x0055 => {
                        for i in 0..x + 1 {
                            self.mem.set_mem(self.index_register + i, self.registers[i]);
                        }
                    },
                    0x0065 => {
                        for i in 0..x + 1 {
                            self.registers[i] = self.mem.get_mem(self.index_register + i);
                        }
                    },
                    0x001e => {
                        self.index_register += self.registers[x] as usize;
                        self.registers[0x0F] = if self.index_register > 0x0F00 { 1 } else { 0 };
                    }  
                    _ => {
                        println!("Unrecognized opcode : {:x?}", opcode & 0xffff);
                    }
                }
                self.pc += 2;
            },
            0x0000 => {
                match opcode {
                    0x00EE => {
                        self.pc = self.stack.pop().unwrap() as usize;
                        self.pc += 2;
                    },
                    0x00E0 => {
                        self.gfx.clear();
                        self.draw = true;
                        self.pc += 2;
                    },
                    _ => {
                        println!("Unrecognized opcode : {:x?}", opcode & 0xf000);
                    }
                }
            }
            _ => {
                println!("Unrecognized opcode : {:x?}", opcode & 0xf000);
            }
        }
    }
}
