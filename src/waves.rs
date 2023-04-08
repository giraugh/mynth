use std::f32::consts::TAU;

use crate::{
    sampleable::{Sampleable, Source},
    Amplitude, Seconds,
};

pub struct SineWave {
    pub freq: Source,
    pub amplitude: Source,
}

impl Sampleable for SineWave {
    fn sample(&self, at: Seconds) -> Amplitude {
        let freq = self.freq.sample(at);
        let amp = self.amplitude.sample(at).max(0.0);
        (at * freq * TAU).sin() * amp
    }
}

pub struct TriangleWave {
    pub freq: Source,
    pub amplitude: Source,
}

impl Sampleable for TriangleWave {
    fn sample(&self, at: Seconds) -> Amplitude {
        let freq = self.freq.sample(at);
        let amp = self.amplitude.sample(at).max(0.0);
        let period = freq.recip();
        (4.0 * amp / period) * (((at - period / 4.0) % period) - period / 2.0).abs() - amp
    }
}

pub struct SquareWave {
    pub freq: Source,
    pub amplitude: Source,
}

impl Sampleable for SquareWave {
    fn sample(&self, at: Seconds) -> Amplitude {
        let freq = self.freq.sample(at);
        let amp = self.amplitude.sample(at).max(0.0);
        ((at * freq * TAU).sin()).signum() * amp
    }
}

pub struct SawToothWave {
    pub freq: Source,
    pub amplitude: Source,
}

impl Sampleable for SawToothWave {
    fn sample(&self, at: Seconds) -> Amplitude {
        let freq = self.freq.sample(at);
        let period = freq.recip();
        let amp = self.amplitude.sample(at).max(0.0);
        ((at / period) - ((0.5 + at / period).floor())) * 2.0 * amp
    }
}
