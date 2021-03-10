//  Rash
//Rusty bASH
// Really Awesome SHell
use std::io::Write;
use std::io::{stdin, stdout};

use toml::Value;

const VALUE: &str = "
cursor='> '
";

const VERSION: &str = "0.1.0";
fn main() {
    let value = VALUE.parse::<Value>().unwrap();
    let motd = "frick";
    let terminal_cursor = value["cursor"].as_str();

    println!("{}", value);

    clear_term();
    println!("{}", motd);

    loop {
        print!("{}", terminal_cursor.unwrap());
        let _err = stdout().flush(); // Excplicitly flush to ensure > gets printed
                                     // handle the _err
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap(); // read_line leaves a trailing newline, which trim removes
        let command = input.trim();
        match command {
            "rush" => {
                println!("Version: {}", VERSION);
            } // Maybe parse via clap
            "clear()" | "clean()" => {
                clear_term();
            }
            "random()" => {
                println!("1"); // this has been decided as the cryptographically secure random
            }
            "script()" => {
                let mut script = String::new();
                loop {
                    let mut line = String::new();
                    stdin().read_line(&mut line).unwrap(); // read_line leaves a trailing
                    line = line.trim().to_string();
                    if line == "end()" {
                        println!("{}", script);
                        break;
                    } else {
                        // append line to script
                        script = format!("{}\n{}", script, line);
                    }
                }
            }
            "ls" => {
                println!("bonk");
            }
            _ => {
                println!("Command {} not found", command);
            }
        }
    }
}

fn clear_term() {
    print!("\x1B[2J\x1B[1;1H");
}
