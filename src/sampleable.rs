use crate::{Amplitude, Seconds};

pub trait Sampleable: Send {
    fn sample(&self, at: Seconds) -> Amplitude;
}

pub type Source = Box<dyn Sampleable>;

pub struct Constant(f32);

impl From<f32> for Box<dyn Sampleable> {
    fn from(value: f32) -> Self {
        Box::new(Constant(value))
    }
}

impl Sampleable for Constant {
    fn sample(&self, _at: Seconds) -> Amplitude {
        self.0
    }
}

pub struct MappedSource<F: Fn(f32) -> f32> {
    pub source: Source,
    pub map_fn: F,
}

impl<F: Fn(f32) -> f32 + Send> Sampleable for MappedSource<F> {
    fn sample(&self, at: Seconds) -> Amplitude {
        let inner_sample = self.source.sample(at);
        (self.map_fn)(inner_sample)
    }
}

pub trait MapSource {
    fn map<F: Fn(f32) -> f32>(self, map_fn: F) -> MappedSource<F>;
}

impl<A: Sampleable + 'static> MapSource for A {
    fn map<F: Fn(f32) -> f32>(self, map_fn: F) -> MappedSource<F> {
        MappedSource {
            map_fn,
            source: Box::new(self),
        }
    }
}
