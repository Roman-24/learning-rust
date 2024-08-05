use std::io;

fn convert_celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 1.8 + 32.0
}

fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

enum ConversionChoice {
    CelsiusToFahrenheit,
    FahrenheitToCelsius,
}

fn main() {
    println!("Hello, welcome to the Temperature Converter!");

    println!("Choose the conversion type:");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");

    let mut user_choice = String::new();
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read line");

    let user_choice: ConversionChoice = match user_choice.trim() {
        "1" => ConversionChoice::CelsiusToFahrenheit,
        "2" => ConversionChoice::FahrenheitToCelsius,
        _ => {
            println!("Invalid choice. Please restart the program and choose 1 or 2.");
            return;
        }
    };

    println!("Input your temperature value:");

    let mut user_value = String::new();
    io::stdin()
        .read_line(&mut user_value)
        .expect("Failed to read line");

    let user_value: f32 = match user_value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please restart the program and enter a valid number.");
            return;
        }
    };

    let result = match user_choice {
        ConversionChoice::CelsiusToFahrenheit => convert_celsius_to_fahrenheit(user_value),
        ConversionChoice::FahrenheitToCelsius => convert_fahrenheit_to_celsius(user_value),
    };

    println!("The converted temperature is: {}", result);
}
