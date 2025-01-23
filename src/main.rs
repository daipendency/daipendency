use daipendency::generate_documentation;
mod cli;
use cli::{make_command_parser, Command};

fn main() -> Result<(), String> {
    let command = make_command_parser().run();
    match command {
        Command::Extract { path, language } => {
            let output =
                generate_documentation(path.as_path(), language).map_err(|e| e.to_string())?;
            println!("{}", output);
        }
        Command::ExtractDep {
            dependency,
            dependant,
        } => {
            println!("hello {}, from {}", dependency, dependant.display());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_command_execution() {
        let parser = make_command_parser();

        let result = parser.run_inner(&["extract", "/some/path"]);

        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_dep_command_execution() {
        let parser = make_command_parser();

        let result = parser.run_inner(&["extract-dep", "my-dep"]);

        assert!(result.is_ok());
    }
}
