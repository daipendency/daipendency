use daipendency::{generate_markdown_documentation, Library};
mod cli;
use cli::{make_command_parser, Command};

fn main() -> Result<(), String> {
    let command = make_command_parser().run();
    match command {
        Command::Extract { path, language } => {
            let library = Library::load(path.as_path(), language).map_err(|e| e.to_string())?;
            println!("{}", generate_markdown_documentation(&library));
        }
        Command::ExtractDep {
            dependency,
            dependant,
            language,
        } => {
            let dependency = Library::load_dependency(&dependency, &dependant, language)
                .map_err(|e| e.to_string())?;
            println!("{}", generate_markdown_documentation(&dependency));
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
