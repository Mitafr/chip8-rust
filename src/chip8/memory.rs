use crate::chip8::fonts::FONTS;

use std::fs::File;
use std::fmt;
use std::io::prelude::*;

pub struct Memory {
    pub mem: [u8; 4096],
    size: usize,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; 4096],
            size: 0,
        }
    }
    pub fn get_mem(&mut self, i: usize) -> u8 {
        self.mem[i]
    }
    pub fn set_mem(&mut self, i: usize, value: u8) {
        self.mem[i] = value;
    }
    pub fn load_rom(&mut self, filename: &str) -> Result<(), String> {
        println!("Loading : {}", filename);
        for i in 0..FONTS.len() {
            self.mem[i] = FONTS[i];
        }
        
        let mut f = File::open(filename).expect(&format!("file not found: {}", filename));
        let mut buffer = [0u8; 3584];
        if let Ok(bytes_read) = f.read(&mut buffer) {
            bytes_read
        } else {
            0
        };
        for (i, byte) in buffer.bytes().enumerate() {
            let bit: u8 = byte.unwrap();
            if bit != 0 {
                self.size += 1;
                self.mem[i + 512] = bit;
            }
        }
        Ok(())
    }
}


impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Size: {}", self.size)?;
        for (i, b) in self.mem.iter().enumerate() {
            if i >= 0x200 && i <= 0x200 + self.size {
                write!(f, "{:x?} ", b)?;
            }
        }
        Ok(())
    }
}
