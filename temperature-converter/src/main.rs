use std::io;

fn main() {
    let mut choice = String::new();
    println!(
        "1. Celsius to Fahrenheit
2. Fahrenheit to Celsius"
    );

    io::stdin().read_line(&mut choice).unwrap();
    println!("choice = {}", choice.trim());

    let mut input = String::new();
    println!("Enter temperature:");
    io::stdin().read_line(&mut input).unwrap();

    let temp: f64 = input.trim().parse().unwrap();

    if choice.trim() == "1" {
        let fahrenheit = celsius_to_fahrenheit(temp);
        println!("{temp}C is {fahrenheit}F");
    } else if choice.trim() == "2" {
        let celsius = fahrenheit_to_celsius(temp);
        println!("{temp}F is {celsius}C");
    } else {
        println!("Invalid choice");
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
