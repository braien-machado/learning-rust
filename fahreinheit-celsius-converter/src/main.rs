use std::io;

fn main() {
    println!("Welcome to Fahrenheit and Celsius Converter!");

    loop {
        println!("Type '1' to convert Fahreinheit to Celsius or '2' to do the opposite.");

        let mut option: String = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: usize = option
            .trim()
            .parse()
            .expect("Option should be an unsigned integer value.");

        if option == 1 {
            println!("Type the Fahreinheit temperature to be converted.");

            let temp: f64 = wait_for_float_from_input();
            let result: f64 = convert_fahrenheit_to_celsius(temp);

            println!("{temp}ºF = {result}ºC");
            break;
        }

        if option == 2 {
            println!("Type the Celsius temperature to be converted.");

            let temp: f64 = wait_for_float_from_input();
            let result: f64 = convert_celsius_to_fahrenheit(temp);

            println!("{temp}ºC = {result}ºF");
            break;
        }

        println!("Option should be '1' or '2'. Try again.")
    }
}

fn wait_for_float_from_input() -> f64 {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
        .trim()
        .parse()
        .expect("Option should be a float value")
}

fn convert_fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) / (9.0 / 5.0)
}

fn convert_celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * (9.0 / 5.0)) + 32.0
}
