use std::fs;

use cpal::{traits::DeviceTrait, SampleFormat};

use crate::{sampleable::Sampleable, Amplitude, Seconds};

type Hz = f32;

pub struct Sampler<A: Sampleable> {
    source: A,
    sample_count: usize,
}

impl<A: Sampleable + 'static> Sampler<A> {
    pub fn new(source: A) -> Self {
        Self {
            source,
            sample_count: 0,
        }
    }

    pub fn get_next_sample(&mut self, sample_rate: Hz) -> Amplitude {
        let value = self
            .source
            .sample((self.sample_count as f32) / sample_rate)
            .clamp(0.0, 1.0);
        self.sample_count += 1;
        value
    }

    #[allow(dead_code)]
    pub fn record(&mut self, duration: Seconds, sample_rate: Hz) -> Recording {
        let num_samples = (sample_rate * duration) as usize;
        let samples = (0..=num_samples)
            .map(|_| self.get_next_sample(sample_rate))
            .collect();
        Recording {
            samples,
            sample_rate,
        }
    }

    pub fn cpal_stream(mut self) -> Result<cpal::Stream, cpal::BuildStreamError> {
        // Get host
        use cpal::traits::HostTrait;
        let host = cpal::default_host();

        // Get default device
        let device = host
            .default_output_device()
            .expect("no output device available");

        // Find compatible config
        let config = device
            .supported_output_configs()
            .unwrap()
            .find(|config| config.sample_format() == SampleFormat::F32)
            .expect("No supported config")
            .with_max_sample_rate()
            .config();

        // Create and return stream
        device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(config.channels as usize) {
                    let value = self.get_next_sample(config.sample_rate.0 as Hz);
                    for sample in frame.iter_mut() {
                        *sample = value * 0.1;
                    }
                }
            },
            |err| eprintln!("an error occurred on the output audio stream: {}", err),
            None,
        )
    }
}

#[allow(dead_code)]
pub struct Recording {
    samples: Vec<Amplitude>,
    #[allow(dead_code)]
    sample_rate: Hz,
}

#[allow(dead_code)]
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
