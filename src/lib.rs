mod extractors;
mod formatting;
mod languages;
mod library;

use formatting::format_library_context;
use library::extract_public_api;
use std::path::Path;

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
    let library = extract_public_api(path, language)?;
    Ok(format_library_context(&library, language))
}
