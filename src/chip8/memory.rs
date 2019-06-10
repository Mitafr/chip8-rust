use std::fs::File;
use std::io::prelude::*;
use std::fmt;

pub struct Memory {
    pub mem: [u8; 4096],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; 4096],
        }
    }
    pub fn load_rom(&mut self, filename: &str) -> Result<(), String> {
        let f = File::open(filename).expect(&format!("file not found: {}", filename));
        for (i, byte) in f.bytes().enumerate() {
            self.mem[i + 512] = byte.unwrap();
        }
        Ok(())
    }
}


impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for b in self.mem.iter() {
            write!(f, "{:x?}|", b)?;
        }
        Ok(())
    }
}