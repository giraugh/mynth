use sampleable::MapSource;
use sampler::Sampler;
use waves::{SineWave, TriangleWave};

mod sampleable;
mod sampler;
mod waves;

type Seconds = f32;
type Amplitude = f32;
type Hz = f32;

const PITCH_STANDARD_FREQ: Hz = 440.0;
fn main() {
    // Create amplitude wave
    let amp_wave = TriangleWave {
        amplitude: 0.5.into(),
        freq: 3.0.into(),
    };

    // Create freq wave
    let freq_wave = SineWave {
        amplitude: 50.0.into(),
        freq: 5.0.into(),
    }
    .map(|sample| PITCH_STANDARD_FREQ + sample);

    // Create wave
    let wave = SineWave {
        freq: Box::new(freq_wave),
        amplitude: Box::new(amp_wave),
    };

    // Sample it
    let sampler: Sampler<_> = wave.into();
    let audio = sampler.record(1.0);

    // Output the audio
    audio.save("./test_audio.bin").unwrap()
}
