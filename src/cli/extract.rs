use super::{make_language_option, Command};
use bpaf::{parsers::ParseCommand, *};
use std::path::PathBuf;

fn make_path_arg() -> impl Parser<PathBuf> {
    positional("PATH").help("Path to the project or file to generate documentation for")
}

pub fn make_extract_subcommand() -> ParseCommand<Command> {
    let language = make_language_option();
    let path = make_path_arg();

    construct!(Command::Extract { language, path })
        .to_options()
        .descr("Extract and document dependencies from a project")
        .command("extract")
}

#[cfg(test)]
mod tests {
    use daipendency::Language;

    use super::*;

    #[test]
    fn test_parse_valid_path() {
        let parser = make_extract_subcommand().to_options();

        let result = parser.run_inner(&["extract", "/some/path"]);

        assert!(result.is_ok());
        match result.unwrap() {
            Command::Extract { path, language } => {
                assert_eq!(path, PathBuf::from("/some/path"));
                assert!(language.is_none());
            }
            _ => panic!("Expected Extract command"),
        }
    }

    #[test]
    fn test_parse_with_language() {
        let parser = make_extract_subcommand().to_options();

        let result = parser.run_inner(&["extract", "/some/path", "--language", "rust"]);

        assert!(result.is_ok());
        match result.unwrap() {
            Command::Extract { path, language } => {
                assert_eq!(path, PathBuf::from("/some/path"));
                assert_eq!(language, Some(Language::Rust));
            }
            _ => panic!("Expected Extract command"),
        }
    }

    #[test]
    fn test_parse_with_invalid_language() {
        let parser = make_extract_subcommand().to_options();

        let result = parser.run_inner(&["extract", "/some/path", "--language", "invalid"]);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_without_path() {
        let parser = make_extract_subcommand().to_options();

        let result = parser.run_inner(&["extract"]);

        assert!(result.is_err());
    }
}
