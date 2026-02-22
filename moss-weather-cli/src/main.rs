mod types;
mod weather;

use clap::{Parser, ValueEnum};
use types::TemperatureUnit;

#[derive(Parser)]
#[command(
    name = "moss-weather",
    about = "Query current weather for any city",
    version
)]
struct Args {
    /// City name to query (e.g. "Beijing", "New York")
    city: String,

    /// Temperature unit
    #[arg(short, long, value_enum, default_value_t = UnitArg::Celsius)]
    unit: UnitArg,
}

#[derive(Clone, ValueEnum)]
enum UnitArg {
    Celsius,
    Fahrenheit,
}

impl From<UnitArg> for TemperatureUnit {
    fn from(u: UnitArg) -> Self {
        match u {
            UnitArg::Celsius => TemperatureUnit::Celsius,
            UnitArg::Fahrenheit => TemperatureUnit::Fahrenheit,
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let unit: TemperatureUnit = args.unit.into();

    match weather::get_weather(&args.city, &unit).await {
        Ok(resp) => {
            println!("{}", weather::format_weather_info(&resp, &unit));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
