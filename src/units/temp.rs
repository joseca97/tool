use crate::traits::Converter;

pub enum TempScale { Celsius, Fahrenheit, Kelvin }

impl Converter for TempScale {
    fn to_base(&self, value: f64) -> f64 {
        match self {
            TempScale::Celsius => value + 273.15,
            TempScale::Fahrenheit => (value - 32.0) * 5.0 / 9.0 + 273.15,
            TempScale::Kelvin => value,
        }
    }

    fn from_base(&self, value: f64) -> f64 {
        match self {
            TempScale::Celsius => value - 273.15,
            TempScale::Fahrenheit => (value - 273.15) * 9.0 / 5.0 + 32.0,
            TempScale::Kelvin => value,
        }
    }
}