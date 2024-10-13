use std::{io, process::exit};

fn main() {
    loop {
        println!("\nInsert desired operation:
                \n0 - Celsius to Fahrenheit
                \n1 - Fahrenheit to Celsius
                \n2 - Exit application");

        let mut operation = String::new();

        io::stdin()
            .read_line(&mut operation)
            .expect("Couldn't read input");

        let operation : u8 = match operation.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        if operation >= 2 {
            exit(0);
        }

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Couldn't read input");

        let temperature : f64 = match temperature.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let answer = if operation == 0 {
            celsius_to_fahrenheit(temperature)
        } else {
            fahrenheit_to_celsius(temperature)
        };

        println!("{answer}");}
}

fn celsius_to_fahrenheit(temperature : f64) -> f64 {
    temperature * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(temperature : f64) -> f64 {
    (temperature - 32.0) / 9.0 * 5.0
}