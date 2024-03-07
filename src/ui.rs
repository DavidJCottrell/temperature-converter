slint::include_modules!();
use slint::SharedString;

use temperature::{Temperature, TemperatureUnit};

use crate::temperature;

pub fn show_ui() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    fn get_unit(unit_str: &str) -> Option<TemperatureUnit> {
        match unit_str {
            "Celsius" => Some(TemperatureUnit::Celsius),
            "Fahrenheit" => Some(TemperatureUnit::Fahrenheit),
            "Kelvin" => Some(TemperatureUnit::Kelvin),
            _ => None,
        }
    }

    ui.on_calculate(
        move |input_unit: slint::SharedString,
              input_temp: slint::SharedString,
              desiered_unit: slint::SharedString| {
            let ui: AppWindow = ui_handle.unwrap();

            let input_unit = get_unit(input_unit.as_str()).unwrap();

            let input_temp = match input_temp.parse::<f32>() {
                Ok(temp) => temp,
                Err(_) => {
                    println!("Invalid input. Please enter a valid number.");
                    return;
                }
            };

            let desiered_unit = get_unit(desiered_unit.as_str()).unwrap();

            let mut temperature = Temperature::new(input_temp, input_unit);
            let new_temperature = temperature.convert_to(desiered_unit);

            let result_string = format!("{}{}", new_temperature.value, new_temperature.si_unit);

            ui.set_output(SharedString::from(result_string));
        },
    );

    ui.run()
}
