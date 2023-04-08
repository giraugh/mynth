use std::{thread::sleep, time::Duration};

use cpal::traits::StreamTrait;
use sampleable::MapSource;
use sampler::Sampler;
use waves::{SawToothWave, SineWave, TriangleWave};

mod sampleable;
mod sampler;
mod waves;

type Seconds = f32;
type Amplitude = f32;
type Hz = f32;

const PITCH_STANDARD_FREQ: Hz = 440.0;

fn main() {
    // Create wave to control amplitude
    let amp_wave = SineWave {
        amplitude: 0.3.into(),
        freq: 0.5.into(),
    }
    .map(|s| 0.5 + s);

    // Create wave to control frequency
    let freq_wave = SineWave {
        amplitude: 50.0.into(),
        freq: 0.5.into(),
    }
    .map(|sample| PITCH_STANDARD_FREQ + sample);

    // Create wave that we will hear
    let wave = SineWave {
        freq: Box::new(freq_wave),
        amplitude: Box::new(amp_wave),
    };

    // Create stream to sample the wave
    let sampler = Sampler::new(wave);
    let stream = sampler.cpal_stream().unwrap();

    // Stream it and keep playing...
    stream.play().unwrap();
    loop {
        sleep(Duration::from_secs(3))
    }
}
