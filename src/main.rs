use ::fppc::*;
use std::io::{self, Write};

fn main() {
    println!("=== FPPC Parser Interactive Console ===");
    println!("Commands:");
    println!("  label <input>      - Parse as LabelType");
    println!("  simple <input>     - Parse as SimpleType");
    println!("  property <input>   - Parse as PropertyType");
    println!("  descriptor_type <input> - Parse as DescriptorType");
    println!("  descriptor <input> - Parse as Descriptor");
    println!("  filler <input>     - Parse as ElementPatternFiller");
    println!("  node <input>       - Parse as NodePattern");
    println!("  quit               - Exit");
    println!();

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
            println!("Goodbye!");
            break;
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        if parts.len() < 2 {
            eprintln!("Error: Please provide a command and input. Example: node (p: Person)");
            continue;
        }

        let command = parts[0];
        let parse_input = parts[1];

        match command {
            "label" => {
                match LabelTypeParser::new().parse(parse_input) {
                    Ok(result) => println!("✓ Valid: {:?}", result),
                    Err(e) => eprintln!("✗ Parse error: {}", e),
                }
            }
            "simple" => {
                match SimpleTypeParser::new().parse(parse_input) {
                    Ok(result) => println!("✓ Valid: {:?}", result),
                    Err(e) => eprintln!("✗ Parse error: {}", e),
                }
            }
            "property" => {
                match PropertyTypeParser::new().parse(parse_input) {
                    Ok(result) => println!("✓ Valid: {:?}", result),
                    Err(e) => eprintln!("✗ Parse error: {}", e),
                }
            }
            "descriptor_type" => {
                match DescriptorTypeParser::new().parse(parse_input) {
                    Ok(result) => println!("✓ Valid: {:?}", result),
                    Err(e) => eprintln!("✗ Parse error: {}", e),
                }
            }
            "descriptor" => {
                match DescriptorParser::new().parse(parse_input) {
                    Ok(result) => println!("✓ Valid: {:?}", result),
                    Err(e) => eprintln!("✗ Parse error: {}", e),
                }
            }
            "filler" => {
                match ElementPatternFillerParser::new().parse(parse_input) {
                    Ok(result) => println!("✓ Valid: {:?}", result),
                    Err(e) => eprintln!("✗ Parse error: {}", e),
                }
            }
            "node" => {
                match NodePatternParser::new().parse(parse_input) {
                    Ok(result) => println!("✓ Valid: {:?}", result),
                    Err(e) => eprintln!("✗ Parse error: {}", e),
                }
            }
            _ => {
                eprintln!("Unknown command: {}. Use: label, simple, property, descriptor_type, descriptor, filler, or node", command);
            }
        }
    }
}

