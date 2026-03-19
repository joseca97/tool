use tool::traits::Converter;
use tool::units::temp::TempScale;
use tool::units::dist::DistScale;

// enum Unit {
//     Temperature(TempScale),
//     Distance(DistScale),
// }

fn perform_convertion<T: Converter>(value: f64, from: T, to: T) -> f64 {
    let base_value = from.to_base(value);
    to.from_base(base_value)
}

fn main() {
    println!("Result is {}", perform_convertion(30.0, DistScale::Meters, DistScale::Miles));
}
