pub trait Converter {
    fn to_base(&self, value: f64) -> f64;
    fn from_base(&self, value: f64) -> f64;
}