use fppc::*;
use std::io::{self, Write};

fn main() {
    println!("=== FPPC Parser Interactive Console ===");
    println!("Commands:");
    println!("  expr <input>       - Parse as Expr");
    println!("  descriptor <input> - Parse as Descriptor");
    println!("  path <input>       - Parse as PathPattern");
    println!("  pretty             - Toggle pretty printing");
    println!("  quit               - Exit");
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

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        if parts.len() < 2 {
            eprintln!("Error: Please provide a command and input. Example: path (p: Person)");
            continue;
        }

        let command = parts[0];
        let parse_input = parts[1];

        macro_rules! print_result {
            ($result:expr) => {
                if pretty {
                    println!("✓ Valid: {:#?}", $result)
                } else {
                    println!("✓ Valid: {:?}", $result)
                }
            };
        }

        match command {
            "expr" => match ExprParser::new().parse(parse_input) {
                Ok(result) => print_result!(result),
                Err(e) => eprintln!("✗ Parse error: {}", e),
            },
            "descriptor" => match DescriptorParser::new().parse(parse_input) {
                Ok(result) => print_result!(result),
                Err(e) => eprintln!("✗ Parse error: {}", e),
            },
            "path" => match PathPatternParser::new().parse(parse_input) {
                Ok(result) => print_result!(result),
                Err(e) => eprintln!("✗ Parse error: {}", e),
            },
            _ => {
                eprintln!(
                    "Unknown command: {}. Use: expr, descriptor, or path",
                    command
                );
            }
        }
    }
}
