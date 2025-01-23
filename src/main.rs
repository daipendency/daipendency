use bpaf::*;
use daipendency::{generate_documentation, Language};
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum Command {
    /// Extract and document dependencies from a project
    Extract {
        /// Path to the project or file
        path: PathBuf,
        /// Programming language to use
        language: Option<Language>,
    },
}

fn make_path_arg() -> impl Parser<PathBuf> {
    positional("PATH")
        .help("Path to the project or file to generate documentation for")
        .map(|s: String| PathBuf::from(s))
}

fn make_language_option() -> impl Parser<Option<Language>> {
    long("language")
        .help("Programming language to use for documentation generation")
        .argument("LANG")
        .parse(|s: String| s.parse::<Language>())
        .optional()
}

fn extract() -> impl Parser<Command> {
    let language = make_language_option();
    let path = make_path_arg();

    construct!(Command::Extract { language, path })
}

fn options() -> OptionParser<Command> {
    let extract = extract()
        .to_options()
        .descr("Extract and document dependencies from a project")
        .command("extract");

    construct!([extract]).to_options()
}

fn main() -> Result<(), String> {
    let command = options().run();
    match command {
        Command::Extract { path, language } => {
            let output =
                generate_documentation(path.as_path(), language).map_err(|e| e.to_string())?;
            println!("{}", output);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_path() {
        let parser = options();

        let result = parser.run_inner(&["extract", "/some/path"]);

        assert!(result.is_ok());
        match result.unwrap() {
            Command::Extract { path, language } => {
                assert_eq!(path, PathBuf::from("/some/path"));
                assert!(language.is_none());
            }
        }
    }

    #[test]
    fn test_parse_with_language() {
        let parser = options();

        let result = parser.run_inner(&["extract", "/some/path", "--language", "rust"]);

        assert!(result.is_ok());
        match result.unwrap() {
            Command::Extract { path, language } => {
                assert_eq!(path, PathBuf::from("/some/path"));
                assert_eq!(language, Some(Language::Rust));
            }
        }
    }

    #[test]
    fn test_parse_with_invalid_language() {
        let parser = options();

        let result = parser.run_inner(&["extract", "/some/path", "--language", "invalid"]);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_no_args() {
        let parser = options();

        let result = parser.run_inner(&[]);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_multiple_paths() {
        let parser = options();

        let result = parser.run_inner(&["extract", "/some/path", "/another/path"]);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_without_extract_command() {
        let parser = options();

        let result = parser.run_inner(&["/some/path"]);

        assert!(result.is_err());
    }
}
