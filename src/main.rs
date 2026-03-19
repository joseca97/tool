use std::str::FromStr;
use clap::Parser;
use tool::{traits::Converter, units::{dist::DistScale, temp::TempScale}};
//use tool::units::{dist::DistScale, temp::TempScale};

#[derive(Parser)]
#[command(name = "tool")]
#[command(about = "A versatile unit converter built in Rust", long_about = None)]
struct Cli {
    // The unit to convert from
    #[arg(short, long)]
    from: UnitType,

    // The uniit to convert to
    #[arg(short, long)]
    to: UnitType,

    // The value to convert
    value: f64,
}

#[derive(Clone)]
enum UnitType {
    Temp(TempScale),
    Dist(DistScale),
}

impl UnitType {
    fn convert_to(&self, target: &UnitType, value: f64) -> Result<f64, String> {
        match (self, target) {
            (UnitType::Temp(f), UnitType::Temp(t)) => {
                let base  = f.to_base(value);
                Ok(t.from_base(base))
            },
            (UnitType::Dist(f), UnitType::Dist(t)) => {
                let base = f.to_base(value);
                Ok(t.from_base(base))
            },
            _ => Err("Error: Units must be of the same type".to_string()),
        }
    }
}

impl FromStr for UnitType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(t) = TempScale::from_str(s) {
            return Ok(UnitType::Temp(t));
        }

        if let Ok(t) = DistScale::from_str(s) {
            return Ok(UnitType::Dist(t));
        }

        Err(format!("'{}' is not recognized unit", s))
    }
}

fn main() {
    let args = Cli::parse();

    match args.from.convert_to(&args.to, args.value) {
        Ok(result) => println!("{:.2} is {:.2}", args.value, result),
        Err(e) => eprintln!("{}", e),
    }
}