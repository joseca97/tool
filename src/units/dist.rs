use std::str::FromStr;
use crate::traits::Converter;

#[derive(Clone)]
pub enum DistScale { Meters, Kilometers, Miles }

impl Converter for DistScale {
    fn to_base(&self, value: f64) -> f64 {
        match self {
            DistScale::Kilometers => value / 1000.0,
            DistScale::Miles => value * 1609.34,
            DistScale::Meters => value,
        }
    }

    fn from_base(&self, value: f64) -> f64 {
        match self {
            DistScale::Kilometers => value * 1000.0,
            DistScale::Miles => value / 1609.34,
            DistScale::Meters => value,
        }
    }
}

impl FromStr for DistScale {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kilometers" | "km" => Ok(DistScale::Kilometers),
            "miles" | "ms" => Ok(DistScale::Miles),
            "meters" | "m" => Ok(DistScale::Meters),
            _ => Err(format!("'{}' is not a valid distance unit", s)),
        }
    }
}