use std::fs;

use crate::{sampleable::Sampleable, Amplitude, Seconds};

const DEFAULT_SAMPLE_RATE: Hz = 48_000.0;

type Hz = f32;

pub struct Sampler<A: Sampleable> {
    sample_rate: Hz,
    source: A,
}

impl<A: Sampleable> From<A> for Sampler<A> {
    fn from(source: A) -> Self {
        Self {
            source,
            sample_rate: DEFAULT_SAMPLE_RATE,
        }
    }
}

impl<A: Sampleable> Sampler<A> {
    pub fn record(&self, duration: Seconds) -> Recording {
        let num_samples = (self.sample_rate * duration) as usize;
        let samples = (0..=num_samples)
            .map(|sample| self.source.sample((sample as f32) / self.sample_rate))
            .collect();
        Recording {
            samples,
            sample_rate: self.sample_rate,
        }
    }
}

pub struct Recording {
    samples: Vec<Amplitude>,
    #[allow(dead_code)]
    sample_rate: Hz,
}

impl Recording {
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let bytes: Vec<u8> = self
            .samples
            .iter()
            .flat_map(|sample| sample.to_le_bytes())
            .collect();
        fs::write(path, bytes)?;
        Ok(())
    }
}
