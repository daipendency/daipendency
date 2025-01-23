use bpaf::*;
use daipendency::{generate_documentation, Language};
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct OptionSet {
    path: PathBuf,
    language: Option<Language>,
}

impl OptionSet {
    fn new(path: PathBuf, language: Option<Language>) -> Self {
        Self { path, language }
    }
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

fn make_option_set_parser() -> OptionParser<OptionSet> {
    construct!(OptionSet::new(make_path_arg(), make_language_option()))
        .to_options()
        .descr("Generate documentation for dependencies")
}

fn main() -> Result<(), String> {
    let options = make_option_set_parser().run();
    let output = generate_documentation(options.path.as_path(), options.language)
        .map_err(|e| e.to_string())?;
    println!("{}", output);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_path() {
        let parser = make_option_set_parser();

        let result = parser.run_inner(&["/some/path"]);

        assert!(result.is_ok());
        let opts = result.unwrap();
        assert_eq!(opts.path, PathBuf::from("/some/path"));
        assert!(opts.language.is_none());
    }

    #[test]
    fn test_parse_with_language() {
        let parser = make_option_set_parser();

        let result = parser.run_inner(&["/some/path", "--language", "rust"]);

        assert!(result.is_ok());
        let opts = result.unwrap();
        assert_eq!(opts.path, PathBuf::from("/some/path"));
        assert_eq!(opts.language, Some(Language::Rust));
    }

    #[test]
    fn test_parse_with_invalid_language() {
        let parser = make_option_set_parser();

        let result = parser.run_inner(&["/some/path", "--language", "invalid"]);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_no_args() {
        let parser = make_option_set_parser();

        let result = parser.run_inner(&[]);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_multiple_paths() {
        let parser = make_option_set_parser();

        let result = parser.run_inner(&["/some/path", "/another/path"]);

        assert!(result.is_err());
    }
}
