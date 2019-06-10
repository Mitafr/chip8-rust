extern crate sdl2;
mod driver;
mod chip8;

use chip8::Cpu;
use chip8::Memory;
use driver::Gfx;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;


use std::time::Duration;
use std::thread;
use std::env;
use std::fs;
use std::io;
use std::io::Write;


struct Chip8 {
    chip: Cpu,
    mem: Memory,
    rom: String,
    gfx: Gfx,
    context: sdl2::Sdl,
}

impl Chip8 {
    fn new(rom: String) -> Chip8 {
        let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
        Chip8 {
            chip: Cpu::new(),
            mem: Memory::new(),
            rom: rom,
            gfx: Gfx::new(&sdl_context, "Test"),
            context: sdl_context,
        }
    }
    fn init(&mut self) -> Result<(), String> {
        self.mem.load_rom(&self.rom)?;
        println!("{}", self.mem);
        self.gfx.clear();
        self.gfx.update();
        Ok(())
    }
    fn run(&mut self) -> Result<(), String> {
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
            self.chip.emulate(&self.mem);
            self.gfx.draw_rect(32, 16, 1)?;
            self.gfx.update();
            thread::sleep(sleep_duration);
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    println!("{}", args.len());
    let mut chip8: Chip8;
    if args.len() > 1 {
        chip8 = Chip8::new(args[1].clone());
    } else {
        chip8 = Chip8::new(String::from("roms/MAZE.ch8"));
    }
    chip8.init()?;
    chip8.run()?;
    Ok(())
}
