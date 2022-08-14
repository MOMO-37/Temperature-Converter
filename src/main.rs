use std::io;

fn c_to_f(&c: &f32) -> f32 {
    c * 1.8 + 32.
}
fn f_to_c(&f: &f32) -> f32 {
    (f - 32.) / 1.8
}

fn deg_str(c: &bool) -> String {
    match c {
        true => String::from("°C"),
        false => String::from("°F"),
    }
}

fn main() {
    loop {
        let deg_c: bool;    
        loop {
            println!("\nWhat conversion do you want to do?\n1. °C -> °F\n2.°F -> °C");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Readline Error.");
            
            deg_c = match input.trim().parse() {
                Ok(num) => {
                    match num {
                        1 => true,
                        2 => false,
                        _ => {
                            println!("Not a valid option!");
                            continue;
                        }
                    }
                },
                Err(_) => {
                    println!("Not a number!");
                    continue;
                },
            };
            break;
        }

        let temp: f32;
        loop {
            println!(
                "\nEnter a temperature in {}.",
                deg_str(&deg_c)
            );
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Readline error.");

            temp = match input.trim().parse() {
                Ok(num) => {
                    num
                },
                Err(_) => {
                    println!("Not a number!");
                    continue;
                },
            };
            break;
        }

        let temp_new: f32 = match deg_c {
            true => c_to_f(&temp),
            false => f_to_c(&temp),
        };

        println!(
            "\n{}{} = {}{}",
            temp,
            deg_str(&deg_c),
            temp_new,
            deg_str(&!deg_c)
        );

        println!("\nWould you like to perform another conversion?\n1. Yes\n2. No");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Readline error."); 
        match input.trim().parse() {
            Ok(num) => {
                match num {
                    1 => continue,
                    2 => {
                        println!("\nGoodbye!");
                        break
                    },
                    _ => {
                        println!("Option not recognized; exiting.");
                        break;
                    }
                }
            },
            Err(_) => {
                println!("Readline error; exiting.");
                break;
            }
        }
    }
}
