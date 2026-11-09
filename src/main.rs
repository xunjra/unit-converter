use std::io::{self, Write};

// FUNCTIONS FOR LENGTH CONVERSION

fn km2mtr(km: f64) -> f64 {
    let m: f64 = km * 1000.00;
    return m;
}

fn mtr2km(m: f64) -> f64 {
    let km: f64 = m / 1000.00;
    return km;
}

fn mtr2mile(mtr: f64) -> f64 {
    let mile: f64 = mtr / 1609.344;
    return mile;
}
fn mile2mtr(mile: f64) -> f64 {
    let mtr: f64 = mile * 1609.344;
    return mtr;
}

// FUNCTION FOR TEMPERATURE CONVERSION

fn c2f(c: f64) -> f64 {
    let f: f64 = c * 9.0 / 5.0 + 32.0;
    return f;
}

fn f2c(f: f64) -> f64 {
    let c: f64 = (f - 32.0) * 5.0 / 9.0;
    return c;
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
                if tokens[0].to_lowercase() == "quit" {
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

                // Slices the from token and to token into character and checks if each value is a string or not !
                //  If not the code should display warning
                if !from.chars().all(|char| char.is_alphabetic())
                    || !to.chars().all(|char| char.is_alphabetic())
                {
                    println!("Units must contain letters only (e.g. km, mtr, c, f)");
                    continue;
                }

                match (from, to) {
                    ("km", "mtr") => {
                        if value >= 0.0 {
                            let result = km2mtr(value);
                            if result != 0.0 && result.abs() < 0.01 {
                                println!("{value} {from} = {result:.6} {to}");
                            } else {
                                println!("{value} {from} = {result:.2} {to}");
                            }
                        } else {
                            println!("Distance cannot be negative");
                            continue;
                        }
                    }

                    ("mtr", "km") => {
                        if value >= 0.0 {
                            let result = mtr2km(value);
                            if result != 0.0 && result.abs() < 0.01 {
                                println!("{value} {from} = {result:.6} {to}");
                            } else {
                                println!("{value} {from} = {result:.2} {to}");
                            }
                        } else {
                            println!("Distance cannot be negative");
                            continue;
                        }
                    }

                    ("mtr", "mile") => {
                        if value >= 0.0 {
                            let result = mtr2mile(value);
                            if result != 0.0 && result.abs() < 0.01 {
                                println!("{value} {from} = {result:.6} {to}");
                            } else {
                                println!("{value} {from} = {result:.2} {to}");
                            }
                        } else {
                            println!("Distance cannot be negative");
                            continue;
                        }
                    }

                    ("km", "mile") => {
                        if value >= 0.0 {
                            let mtr = km2mtr(value);
                            let result = mtr2mile(mtr);
                            if result != 0.0 && result.abs() < 0.01 {
                                println!("{value} {from} = {result:.6} {to}");
                            } else {
                                println!("{value} {from} = {result:.2} {to}");
                            }
                        } else {
                            println!("Distance cannot be negative");
                            continue;
                        }
                    }

                    ("mile", "km") => {
                        if value >= 0.0 {
                            let mtr = mile2mtr(value);
                            let result = mtr2km(mtr);
                            if result != 0.0 && result.abs() < 0.01 {
                                println!("{value} {from} = {result:.6} {to}");
                            } else {
                                println!("{value} {from} = {result:.2} {to}");
                            }
                        } else {
                            println!("Distance cannot be negative");
                            continue;
                        }
                    }

                    ("mile", "mtr") => {
                        if value >= 0.0 {
                            let result = mile2mtr(value);
                            if result != 0.0 && result.abs() < 0.01 {
                                println!("{value} {from} = {result:.6} {to}");
                            } else {
                                println!("{value} {from} = {result:.2} {to}");
                            }
                        } else {
                            println!("Distance cannot be negative");
                            continue;
                        }
                    }
                    ("cel", "fah") => {
                        if value >= -273.15 {
                            let result = c2f(value);
                            println!("{value} {from} = {result:.2} {to}");
                        } else {
                            println!("Temperature cannot go below absolute zero");
                        }
                    }

                    ("fah", "cel") => {
                        if value >= -459.67 {
                            let result = f2c(value);
                            println!("{value} {from} = {result:.2} {to}");
                        } else {
                            println!("Temperature cannot go below absolute zero");
                        }
                    }

                    ("cel", "kel") => {
                        if value >= -273.15 {
                            let result = value + 273.15;
                            println!("{value} {from} = {result:.2} {to}");
                        } else {
                            println!("Temperature cannot go below absolute zero");
                        }
                    }

                    ("kel", "cel") => {
                        if value >= 0.0 {
                            let result = value - 273.15;
                            println!("{value} {from} = {result:.2} {to}");
                        } else {
                            println!("Temperature cannot go below absolute zero");
                        }
                    }

                    ("fah", "kel") => {
                        if value >= -459.67 {
                            let cel = f2c(value);
                            let result = cel + 273.15;
                            println!("{value} {from} = {result:.2} {to}");
                        } else {
                            println!("Temperature cannot go below absolute zero");
                        }
                    }

                    ("kel", "fah") => {
                        if value >= 0.0 {
                            let cel = value - 273.15;
                            let result = c2f(cel);
                            println!("{value} {from} = {result:.2} {to}");
                        } else {
                            println!("Temperature cannot go below absolute zero");
                        }
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
