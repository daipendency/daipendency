mod extractors;
mod formatting;
mod languages;
use std::path::Path;

use daipendency_extractor::get_parser;
use extractors::get_extractor;
use formatting::format_library_context;

pub use languages::Language;

/// Generate API documentation for a library in the specified language.
///
/// # Arguments
///
/// * `path` - Path to the library's root directory
/// * `language` - The programming language of the library
///
/// # Returns
///
/// Returns a Result containing the generated documentation as a string, or an error if something went wrong.
pub fn generate_documentation(path: &Path, language: Language) -> anyhow::Result<String> {
    let extractor = get_extractor(language);
    let metadata = extractor.get_library_metadata(path)?;
    let mut parser = get_parser(&extractor.get_parser_language())?;
    let namespaces = extractor.extract_public_api(&metadata, &mut parser)?;

    let documentation = format_library_context(&metadata, &namespaces, language);

    Ok(documentation)
}
