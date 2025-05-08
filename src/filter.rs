use std::f32::consts::{PI, TAU};

use num::complex::Complex32;

#[derive(Debug)]
pub enum FilterError {
    FrequencyOverNyqist,
    FrequencyNegative,
    QualityNegative,
}

pub struct Lowpass {
    sample_rate: f32,
    frequency: f32,
    quality: f32,
}

impl Lowpass {
    pub fn new(sample_rate: f32, frequency: f32, quality: f32) -> Result<Self, FilterError> {
        if quality.is_sign_negative() {
            return Err(FilterError::QualityNegative);
        }
        if frequency > sample_rate / 2.0 {
            return Err(FilterError::FrequencyOverNyqist);
        }
        if frequency.is_sign_negative() {
            return Err(FilterError::FrequencyNegative);
        }
        Ok(Self {
            sample_rate,
            frequency,
            quality,
        })
    }

    pub fn response(&self, frequency: f32) -> Complex32 {
        let g = PI * self.frequency / self.sample_rate;
        let k = 1.0 / self.quality;
        let f = frequency * TAU / self.sample_rate;
        let z = Complex32::from_polar(1.0, f);
        (g * g * (1.0 + z) * (1.0 + z))
            / ((z - 1.0) * (z - 1.0) + g * g * (1.0 + z) * (1.0 + z) + g * k * (z * z - 1.0))
    }
}
