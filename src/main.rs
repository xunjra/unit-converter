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
                print!("Goodbye!");
                break;
            }
            Ok(_) => {
                let trimmed = input.trim();
                if trimmed == "quit" {
                    break;
                }
                if trimmed.is_empty() {
                    continue;
                }
                match evaulate(trimmed) {
                    Ok(result) => println!("{result}"),
                    Err(error) => println!("{error}"),
                }
            }
            Err(e) => {
                println!("Error reading input : {e}");
                break;
            }
        }
    }
}

fn evaulate(input: &str) -> Result<String, String> {
    if input == "ping" {
        Ok("pong".to_string())
    } else {
        Err(format!("Unknown Command: `{input}`"))
    }
}
