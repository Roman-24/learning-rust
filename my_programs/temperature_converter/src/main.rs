use std::io;

fn convert_from_celsius_to_fahrenheit(input: f32) -> f32 {
    return input * 1.8 + 32.0
}

fn convert_from_fahrenheit_to_celsius(input: f32) -> f32 {
    return (input - 32.0) / 1.8
}

fn main() {
    println!("Hello, weolcome to Temperature converter!");

    println!("Choose what you want to convert");
    println!("For convert from Celsius to Fahrenheit input 1");
    println!("For convert from Fahrenheit to Celsius input 2");

    let mut user_choose = String::new();

    io::stdin()
        .read_line(&mut user_choose)
        .expect("Fail to read line");

    user_choose = user_choose.trim().to_string() ;

    println!("Input your value: ");

    let mut user_value = String::new();

    io::stdin()
        .read_line(&mut user_value)
        .expect("Fail to read line");

    let user_value: f32 = match user_value.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Can not parse yoour value!"),
    };

    if user_choose == "1" {
        println!("Your result is: {}", convert_from_celsius_to_fahrenheit(user_value));
    } else if user_choose == "2" {
        println!("Your result is: {}", convert_from_fahrenheit_to_celsius(user_value));
    } else {
        println!("Your choose does not exist :(");
    }

}
