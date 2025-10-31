use std::{
    fs,
    io::{Read, Write},
};

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let grammar = grammar_to_string();

    // Write the concatenated grammar to OUT_DIR
    let grammar_path = format!("{}/grammar.lalrpop", out_dir);
    let mut outfile = fs::File::create(&grammar_path).unwrap();
    write!(outfile, "{}", grammar).unwrap();
    
    // Process the grammar file
    lalrpop::Configuration::new()
        .process_file(&grammar_path)
        .unwrap();
}

fn grammar_to_string() -> String {
    let mut file =
        fs::File::open("src/grammar/mod.lalrpop").expect("expected root module `mod.lalrpop`");
    let mut grammar = String::new();
    file.read_to_string(&mut grammar).unwrap();

    let grammar_dir = fs::read_dir("src/grammar").unwrap();
    for dir_entry in grammar_dir.into_iter() {
        let dir_entry = dir_entry.unwrap();
        if dir_entry.file_name() != "mod.lalrpop" {
            let mut file = fs::File::open(dir_entry.path()).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            grammar.push_str(&contents);
        }
        println!(
            "cargo::rerun-if-changed={}",
            dir_entry.path().to_string_lossy()
        );
    }
    grammar
}