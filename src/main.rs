use std::io::{self, Write};

fn km2m(km: f64) -> f64 {
    let m: f64 = km * 1000.00;
    return m;
}

fn m2km(m: f64) -> f64 {
    let km: f64 = m / 1000.00;
    return km;
}

fn main() {
    println!("Welcome to Unit Converter");
    loop {
        print!(">> ");
        if let Err(e) = io::stdout().flush() {
            eprint!("Error flusing stdout: {e}");
            break;
        }
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                let tokens: Vec<&str> = input.split_whitespace().collect();
                if tokens.is_empty() {
                    continue;
                }
                if tokens[0] == "quit" {
                    println!("Goodbye !");
                    break;
                }
                if tokens.len() != 3 {
                    println!("Invalid input format");
                    continue; // (Error code E0308 via return approach) Wasted 20 min of time !
                }
                // Input format : initial_unit, value, conversion_unit
                let value = match tokens[1].parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("'{}' is not a valid number", tokens[1]);
                        continue;
                    }
                };

                let from: &str = &tokens[0].to_lowercase();
                let to: &str = &tokens[2].to_lowercase();

                // Slices the from token and to token into character and checks if each value is a string or not , If not the code should display warning
                if !from.chars().all(|char| char.is_alphabetic())
                    || !to.chars().all(|char| char.is_alphabetic())
                {
                    println!("Units must contain letters only (e.g. km, m, c, f)");
                    continue;
                }

                match (from, to) {
                    ("km", "m") => {
                        let result = km2m(value);
                        println!("{value} km = {result:.2} m");
                    }
                    ("m", "km") => {
                        let result = m2km(value);
                        println!("{value} m = {result:.2} km");
                    }
                    _ => println!("Conversion not supported yet"),
                }

                // if tokens[0].to_lowercase() == "km" && tokens[2].to_lowercase() == "m" {
                //     let result = km2m(value);
                //     println!("{value} km = {result:.2} m"); // (Error code E0308 while printing the value) Wasted another 15 min of time trust I have recorded my suffering!
                // }
                // if tokens[0].to_lowercase() == "m" && tokens[2].to_lowercase() == "km" {
                //     let result = m2km(value);
                //     println!("{value} m = {result:.2} km");
                // }
            }
            Err(e) => {
                println!("Error reading input : {e}");
                break;
            }
        }
    }
}
