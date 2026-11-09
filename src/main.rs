use std::io::{self, Write};

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
                println!("{:?}", tokens);
            }
            Err(e) => {
                println!("Error reading input : {e}");
                break;
            }
        }
    }
}
