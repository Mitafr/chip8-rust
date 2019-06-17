extern crate rand;

use sdl2;
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};

const FREQUENCY: i32 = 44100;
const CHANNEL: u8 = 1;


struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
pub struct Audio {
    device: AudioDevice<SquareWave>,
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
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.10
            }
        }).unwrap();

        Audio {
            device: device,
        }
    }
    pub fn beep_play(&mut self) {
        if self.device.status() != sdl2::audio::AudioStatus::Playing {
            self.device.resume();
        }
    }
    pub fn beep_stop(&mut self) {
        self.device.pause();
    }
}
