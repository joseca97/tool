use crate::traits::Converter;
use std::str::FromStr;

#[derive(Clone)]
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

impl FromStr for TempScale {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "celsius" | "c" => Ok(TempScale::Celsius),
            "fahrenheit" | "f" => Ok(TempScale::Fahrenheit),
            "kelvin" | "k" => Ok(TempScale::Kelvin),
            _ => Err(format!("'{}' is not a valid temperature unit", s)),
        }
    }
}