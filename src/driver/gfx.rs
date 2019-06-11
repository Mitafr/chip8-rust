use sdl2;
use sdl2::render::Canvas;
use sdl2::pixels;
use sdl2::video::Window;
use sdl2::rect::{Rect};

use std::fmt;

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const SCALE: u32 = 5;

pub struct Gfx {
    renderer: Canvas<Window>,
    pub display: [[u8; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],
}

impl Gfx {
    pub fn new(sdl_context: &sdl2::Sdl, name: &str) -> Gfx {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys.window(name, SCREEN_WIDTH * SCALE, SCREEN_HEIGHT * SCALE)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();
        Gfx {
            renderer: canvas,
            display: [[0; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize]
        }
    }
    pub fn update(&mut self) {
        self.renderer.present();
    }
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: u8) {
        self.display[x as usize][y as usize] = pixel;
    }
    pub fn draw_screen(&mut self) -> Result<(), String> {
        self.renderer.set_draw_color(color(false));
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if self.display[x as usize][y as usize] != 0 {
                    self.renderer.fill_rect(Rect::new(((x as u32) * SCALE) as i32, ((y as u32) * SCALE) as i32, SCALE, SCALE))?;
                }
            }
        }
        self.renderer.present();
        Ok(())
    }
    pub fn clear(&mut self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.display[x as usize][y as usize] = 0;
            }
        }
    }
}


fn color(value: bool) -> pixels::Color {
    if value == true {
        return pixels::Color::RGB(0, 0, 0);
    }
    pixels::Color::RGB(255, 255, 255)
}


impl fmt::Display for Gfx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for b in 0..SCREEN_WIDTH {
            for c in 0..SCREEN_HEIGHT {
                writeln!(f, "({}, {}) => {:x?}", b, c, self.display[c as usize][b as usize])?;
            }
        }
        Ok(())
    }
}