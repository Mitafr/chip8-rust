mod chip8;
mod driver;

use chip8::Chip8;

use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let mut chip8: Chip8;
    if args.len() < 2 {
        println!("Usage: ./chip8 /filename where filename is the path of your rom");
        panic!("No rom");
    }
    chip8 = Chip8::new(args[1].clone());
    chip8.init()?;
    chip8.run()?;
    Ok(())
}
