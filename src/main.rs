use std::env;
use std::path::PathBuf;

use daipendency::{generate_documentation, Language};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <language> <path>", args[0]);
        std::process::exit(1);
    }

    let language_str = &args[1];
    let language = match language_str.parse::<Language>() {
        Ok(lang) => lang,
        Err(error) => {
            eprintln!("Failed to get language: {}", error);
            std::process::exit(1);
        }
    };
    let path = PathBuf::from(&args[2]);

    match generate_documentation(&path, language) {
        Ok(output) => println!("{}", output),
        Err(error) => {
            eprintln!("Failed to generate documentation: {}", error);
            std::process::exit(1);
        }
    }
}
