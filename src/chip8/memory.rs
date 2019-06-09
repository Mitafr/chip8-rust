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
    pub fn load_rom(&mut self, filename: &str) {
        let mut f = File::open(filename).expect("file not found");
        if let Ok(bytes_read) = f.read(&mut self.mem) {
                bytes_read
            } else {
                0
            };
    }
}


impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for b in self.mem.iter() {
            write!(f, "{:x?}|", b);
        }
        Ok(())
    }
}