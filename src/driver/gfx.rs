use sdl2;
use sdl2::render::Canvas;
use sdl2::pixels;
use sdl2::video::Window;
use sdl2::rect::{Point, Rect};

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const SCALE: u32 = 10;

pub struct Gfx {
    renderer: Canvas<Window>,
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
        }
    }
    pub fn update(&mut self) {
        self.renderer.present();
    }
    pub fn draw(&mut self, x: i32, y: i32) -> Result<(), String> {
        self.renderer.set_draw_color(color(false));
        self.renderer.draw_point(Point::new(x, y))
    }
    pub fn draw_rect(&mut self, x: u32, y: u32, h: u32) -> Result<(), String> {
        self.renderer.set_draw_color(color(false));
        self.renderer.fill_rect(Rect::new((x * SCALE) as i32, (y * SCALE) as i32, 8 * SCALE, h * SCALE))
    }
    pub fn clear(&mut self) {
        self.renderer.set_draw_color(color(true));
        self.renderer.clear();
        self.renderer.present();
    }
}


fn color(value: bool) -> pixels::Color {
    if value == true {
        return pixels::Color::RGB(0, 0, 0);
    }
    pixels::Color::RGB(255, 255, 255)
}