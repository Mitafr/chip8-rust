extern crate rand;

use sdl2;
use sdl2::AudioSubsystem;
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};
use std::time::Duration;

const FREQUENCY: i32 = 44100;
const CHANNEL: u8 = 1;


struct MyCallback {
    volume: f32
}
impl AudioCallback for MyCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        use self::rand::{Rng, thread_rng};
        let mut rng = thread_rng();

        // Generate white noise
        for x in out.iter_mut() {
            *x = (rng.gen_range(0.0, 2.0) - 1.0) * self.volume;
        }
    }
}

pub struct Audio {
    subsytem: AudioSubsystem,
    spec: AudioSpecDesired,
    device: AudioDevice<MyCallback>,
}

impl Audio {
    pub fn new(sdl_context: &sdl2::Sdl) -> Audio {
        let audio_subsys = sdl_context.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(FREQUENCY),
            channels: Some(CHANNEL),
            samples: Some(2048)
        };
        let device = audio_subsys.open_playback(None, &desired_spec, |spec| {
            println!("{:?}", spec);

            MyCallback { volume: 0.5 }
        }).unwrap();
        Audio {
            subsytem: audio_subsys,
            spec: desired_spec,
            device: device,
        }
    }
    pub fn beep(&mut self) {
        self.device.resume()
    }
}
