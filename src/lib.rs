mod analysers;
mod formatting;

use std::path::Path;

use analysers::get_analyser;
use daipendency_extractor::{get_parser, LaibraryError};
use formatting::format_library_context;

/// Generate API documentation for a library in the specified language.
///
/// # Arguments
///
/// * `language` - The programming language of the library
/// * `path` - Path to the library's root directory
///
/// # Returns
///
/// Returns a Result containing the generated documentation as a string, or an error if something went wrong.
pub fn generate_documentation(language: &str, path: &Path) -> Result<String, LaibraryError> {
    let analyser = get_analyser(language)?;

    let metadata = analyser.get_package_metadata(path)?;
    let mut parser = get_parser(&analyser.get_parser_language())?;
    let namespaces = analyser.extract_public_api(&metadata, &mut parser)?;

    format_library_context(&metadata, &namespaces, language)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn unsupported_language() {
        let result = generate_documentation("unsupported", &PathBuf::new());
        assert!(matches!(result, Err(LaibraryError::UnsupportedLanguage(_))));
    }
}
