use std::str::FromStr;

pub struct Temperature {
    pub value: f32,
    pub unit: TemperatureUnit,
    pub si_unit: char,
}

pub enum TemperatureUnit {
    Celsius = 1,
    Fahrenheit,
    Kelvin,
}

impl Temperature {
    fn get_si_unit(unit: &TemperatureUnit) -> char {
        match unit {
            TemperatureUnit::Celsius => 'C',
            TemperatureUnit::Fahrenheit => 'F',
            TemperatureUnit::Kelvin => 'K',
        }
    }

    pub fn new(value: f32, unit: TemperatureUnit) -> Temperature {
        let si_unit = Self::get_si_unit(&unit);
        Temperature {
            value,
            unit,
            si_unit,
        }
    }

    pub fn convert_to(&mut self, new_unit: TemperatureUnit) -> &mut Temperature {
        self.si_unit = Self::get_si_unit(&new_unit);
        match (&self.unit, &new_unit) {
            // Celsius -> ...
            (&TemperatureUnit::Celsius, &TemperatureUnit::Fahrenheit) => {
                self.value = self.value * (9.0 / 5.0) + 32.0;
                self
            }
            (&TemperatureUnit::Celsius, &TemperatureUnit::Kelvin) => {
                self.value = self.value + 273.15;
                self
            }

            // Fahrenheit -> ...
            (&TemperatureUnit::Fahrenheit, &TemperatureUnit::Celsius) => {
                self.value = (self.value - 32.0) * (5.0 / 9.0);
                self
            }
            (&TemperatureUnit::Fahrenheit, &TemperatureUnit::Kelvin) => {
                self.value = (self.value - 32.0) * (5.0 / 9.0) + 273.15;
                self
            }

            // Kelvin -> ...
            (&TemperatureUnit::Kelvin, &TemperatureUnit::Celsius) => {
                self.value = self.value - 273.15;
                self
            }
            (&TemperatureUnit::Kelvin, &TemperatureUnit::Fahrenheit) => {
                self.value = (self.value - 273.15) * (9.0 / 5.0) + 32.0;
                self
            }

            // Same unit
            _ => self,
        }
    }
}

impl FromStr for TemperatureUnit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse::<i32>() {
            Ok(1) => Ok(TemperatureUnit::Celsius),
            Ok(2) => Ok(TemperatureUnit::Fahrenheit),
            Ok(3) => Ok(TemperatureUnit::Kelvin),
            _ => Err(()),
        }
    }
}

impl ToString for TemperatureUnit {
    fn to_string(&self) -> String {
        match self {
            TemperatureUnit::Celsius => "Celsius",
            TemperatureUnit::Fahrenheit => "Fahrenheit",
            TemperatureUnit::Kelvin => "Kelvin",
        }
        .to_string()
    }
}
