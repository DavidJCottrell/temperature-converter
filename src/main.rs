mod cli;
mod temperature;

use cli::{get_scale, get_user_temp};
use temperature::{Temperature, TemperatureUnit};

fn main() {
    println!("Please choose the input temperature unit: ");
    let input_scale: TemperatureUnit = get_scale();

    println!("Please enter your temperature: ");
    let input_temperature: f32 = get_user_temp();

    println!("Please choose the temperature unit to convert to: ");
    let conversion_unit: TemperatureUnit = get_scale();

    let mut temperature = Temperature::new(input_temperature, input_scale);
    let new_temperature = temperature.convert_to(conversion_unit);

    println!("{}{}", new_temperature.value, new_temperature.si_unit)
}
