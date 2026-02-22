use fppc::parse;
use std::io::{self, Write};

fn main() {
    println!("=== FPPC Parser Interactive Console ===");
    println!("Enter a path pattern to parse, or 'quit' to exit.");
    println!("Toggle pretty printing with 'pretty'.");
    println!();

    let mut pretty = false;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input == "quit" || input == "exit" {
            break;
        }

        if input == "pretty" {
            pretty = !pretty;
            println!("Pretty printing: {}", if pretty { "on" } else { "off" });
            continue;
        }

        match parse(input) {
            Ok(result) => {
                if pretty {
                    println!("✓ {:#?}", result)
                } else {
                    println!("✓ {:?}", result)
                }
            }
            Err(e) => eprintln!("✗ Parse error: {}", e),
        }
    }
}
