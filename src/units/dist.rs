use crate::traits::Converter;

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