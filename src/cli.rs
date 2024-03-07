use crate::temperature::TemperatureUnit;

pub fn get_scale() -> TemperatureUnit {
    use std::io::{stdin, stdout, Write};

    fn get_scale_input(scale_id: u8, scale_str: String) {
        println!("{} ({}): ", scale_id, scale_str);
    }

    get_scale_input(
        TemperatureUnit::Celsius as u8,
        TemperatureUnit::Celsius.to_string(),
    );
    get_scale_input(
        TemperatureUnit::Fahrenheit as u8,
        TemperatureUnit::Fahrenheit.to_string(),
    );
    get_scale_input(
        TemperatureUnit::Kelvin as u8,
        TemperatureUnit::Kelvin.to_string(),
    );

    print!("> ");

    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input = input.trim().to_string();

    match input.parse::<TemperatureUnit>() {
        Ok(scale) => scale,
        _ => {
            println!("Invalid input. Please enter a valid option.");
            get_scale()
        }
    }
}

pub fn get_user_temp() -> f32 {
    use std::io::{stdin, stdout, Write};

    print!("> ");

    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input = input.trim().to_string();

    match input.parse::<f32>() {
        Ok(temp) => temp,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            get_user_temp()
        }
    }
}
