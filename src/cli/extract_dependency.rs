use super::Command;
use bpaf::{parsers::ParseCommand, *};
use std::env::current_dir;
use std::path::PathBuf;

pub fn make_extract_dep_subcommand() -> ParseCommand<Command> {
    let dependant = long("dependant")
        .help("Path to the dependant project")
        .argument("PATH")
        .map(|s: String| PathBuf::from(s))
        .fallback(current_dir().unwrap());
    let dependency = positional("DEPENDENCY").help("Name of the dependency to extract");

    construct!(Command::ExtractDep {
        dependant,
        dependency
    })
    .to_options()
    .descr("Extract a specific dependency")
    .command("extract-dep")
}

#[cfg(test)]
mod tests {
    use assertables::assert_matches;

    use super::*;

    #[test]
    fn test_extract_dep_with_dependency() {
        let parser = make_extract_dep_subcommand().to_options();

        let result = parser.run_inner(&["extract-dep", "my-dep"]);

        assert!(result.is_ok());
        assert_matches!(result.unwrap(),
            Command::ExtractDep {
                dependency,
                dependant,
            } if dependency == "my-dep" && dependant == current_dir().unwrap()
        );
    }

    #[test]
    fn test_extract_dep_with_dependant() {
        let parser = make_extract_dep_subcommand().to_options();

        let result = parser.run_inner(&["extract-dep", "my-dep", "--dependant", "/some/path"]);

        assert!(result.is_ok());
        assert_matches!(result.unwrap(),
            Command::ExtractDep {
                dependency,
                dependant,
            } if dependency == "my-dep" && dependant == PathBuf::from("/some/path")
        );
    }

    #[test]
    fn test_extract_dep_without_dependency() {
        let parser = make_extract_dep_subcommand().to_options();

        let result = parser.run_inner(&["extract-dep"]);

        assert!(result.is_err());
    }
}
