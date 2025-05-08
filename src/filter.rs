use core::f32;

use num::complex::Complex64;

#[derive(Debug)]
pub enum FilterError {
    FrequencyOverNyqist,
    FrequencyNegative,
    QNegative,
}

#[allow(unused)]
#[derive(Default, Copy, Clone, PartialEq)]
pub enum FilterType {
    #[default]
    Lowpass,
    Highpass,
    Bandpass,
    Notch,
    Peak,
    Allpass,
    Bell,
    Lowshelf,
    Highshelf,
}

#[derive(Copy, Clone, PartialEq)]
pub struct FilterParams {
    pub frequency: f32,
    pub quality: f32,
    pub gain: f32,
}

impl Default for FilterParams {
    fn default() -> Self {
        Self {
            frequency: 440.0,
            quality: 0.71,
            gain: 0.0,
        }
    }
}

#[derive(Default, Clone)]
pub struct Coefficients {
    a1: f32,
    a2: f32,
    a3: f32,
    m0: f32,
    m1: f32,
    m2: f32,
}

impl Coefficients {
    pub fn new(
        filter_type: FilterType,
        sample_rate: f32,
        params: FilterParams,
    ) -> Result<Self, FilterError> {
        if params.frequency > sample_rate / 2.0 {
            return Err(FilterError::FrequencyOverNyqist);
        }
        if params.frequency.is_sign_negative() {
            return Err(FilterError::FrequencyNegative);
        }
        if params.quality.is_sign_negative() {
            return Err(FilterError::QNegative);
        }

        let mut coeffs = Coefficients::default();
        match filter_type {
            FilterType::Lowpass => {
                let g = (std::f32::consts::PI * params.frequency / sample_rate).tan();
                let k = 1.0 / params.quality;
                coeffs.a1 = 1.0 / (1.0 + g * (g + k));
                coeffs.a2 = g * coeffs.a1;
                coeffs.a3 = g * coeffs.a2;
                coeffs.m0 = 0.0;
                coeffs.m1 = 0.0;
                coeffs.m2 = 1.0;
            }
            _ => todo!(),
        }
        Ok(coeffs)
    }
}

#[derive(Default, Clone)]
pub struct Filter {
    params: FilterParams,
    filter_type: FilterType,
    sample_rate: f32,
    coeffs: Coefficients,
    ic1eq: f32,
    ic2eq: f32,
}

impl Filter {
    pub fn new(filter_type: FilterType) -> Self {
        Self {
            filter_type,
            sample_rate: 48000.0,
            params: FilterParams::default(),
            coeffs: Coefficients::new(filter_type, 48000.0, FilterParams::default())
                .expect("Those settings always work"),
            ic1eq: 0.0,
            ic2eq: 0.0,
        }
    }

    #[allow(unused)]
    pub fn set_filter_type(&mut self, filter_type: FilterType) -> Result<(), FilterError> {
        if self.filter_type != filter_type {
            self.filter_type = filter_type;
            self.update_coefficients()?;
        }
        Ok(())
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) -> Result<(), FilterError> {
        if self.sample_rate != sample_rate {
            self.sample_rate = sample_rate;
            self.update_coefficients()?;
        }
        Ok(())
    }

    pub fn set_params(&mut self, params: FilterParams) -> Result<(), FilterError> {
        if self.params != params {
            self.params = params;
            self.update_coefficients()?;
        }
        Ok(())
    }

    #[inline]
    pub fn tick(&mut self, input: f32) -> f32 {
        let v0 = input;
        let v3 = v0 - self.ic2eq;
        let v1 = self.coeffs.a1 * self.ic1eq + self.coeffs.a2 * v3;
        let v2 = self.ic2eq + self.coeffs.a2 * self.ic1eq + self.coeffs.a3 * v3;
        self.ic1eq = 2.0 * v1 - self.ic1eq;
        self.ic2eq = 2.0 * v2 - self.ic2eq;

        self.coeffs.m0 * v0 + self.coeffs.m1 * v1 + self.coeffs.m2 * v2
    }

    pub fn reset(&mut self) {
        self.ic1eq = 0.0;
        self.ic2eq = 0.0;
    }

    fn update_coefficients(&mut self) -> Result<(), FilterError> {
        self.coeffs = Coefficients::new(self.filter_type, self.sample_rate, self.params)?;
        Ok(())
    }
}

pub fn filter_response(
    eval_frequency: f64,
    filter_type: FilterType,
    sample_rate: f64,
    params: FilterParams,
) -> Complex64 {
    match filter_type {
        FilterType::Lowpass => {
            let g = std::f64::consts::PI * params.frequency as f64 / sample_rate;
            let k = 1.0 / params.quality as f64;
            let f = eval_frequency * std::f64::consts::TAU / sample_rate;
            let z = Complex64::from_polar(1.0, f);
            (g * g * (1.0 + z) * (1.0 + z))
                / ((z - 1.0) * (z - 1.0) + g * g * (1.0 + z) * (1.0 + z) + g * k * (z * z - 1.0))
        }
        _ => todo!(),
    }
}
