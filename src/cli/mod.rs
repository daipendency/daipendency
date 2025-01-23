use bpaf::*;
use daipendency::Language;
use std::path::PathBuf;

mod extract;
mod extract_dependency;

use extract::make_extract_subcommand;
use extract_dependency::make_extract_dep_subcommand;

#[derive(Debug, Clone)]
pub enum Command {
    /// Extract and document dependencies from a project
    Extract {
        /// Path to the project or file
        path: PathBuf,
        /// Programming language to use
        language: Option<Language>,
    },
    /// Extract a specific dependency
    ExtractDep {
        /// Name of the dependency to extract
        dependency: String,
        /// Path to the dependant project
        dependant: PathBuf,
    },
}

pub fn make_command_parser() -> OptionParser<Command> {
    let extract = make_extract_subcommand();

    let extract_dep = make_extract_dep_subcommand();

    construct!([extract, extract_dep])
        .to_options()
        .descr("A tool for extracting and documenting dependencies")
        .header("daipendency")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_command_registered() {
        let parser = make_command_parser();

        let result = parser.run_inner(&["extract", "/some/path"]);

        assert!(matches!(
            result.unwrap(),
            Command::Extract {
                path: _,
                language: _
            }
        ));
    }

    #[test]
    fn test_extract_dep_command_registered() {
        let parser = make_command_parser();

        let result = parser.run_inner(&["extract-dep", "my-dep"]);

        assert!(matches!(
            result.unwrap(),
            Command::ExtractDep {
                dependency: _,
                dependant: _
            }
        ));
    }
}
