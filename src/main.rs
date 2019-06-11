mod chip8;
mod driver;

use chip8::Chip8;

use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    println!("{}", args.len());
    let mut chip8: Chip8;
    if args.len() > 1 {
        chip8 = Chip8::new(args[1].clone());
    } else {
        chip8 = Chip8::new(String::from("roms/INVADERS.ch8"));
    }
    chip8.init()?;
    chip8.run()?;
    Ok(())
}
