mod dependencies;
mod extractors;
mod formatting;
mod languages;
mod library;

pub use dependencies::get_dependency;
pub use formatting::format_library_context;
use library::Library;
use std::path::Path;

pub use languages::Language;

/// Generate API documentation for a library.
///
/// # Arguments
///
/// * `path` - Path to the library's root directory
/// * `language` - Optional programming language of the library. If not provided, the language will be
///   automatically detected from the project structure (which can get slow as we add more languages).
///
/// # Returns
///
/// Returns a Result containing the generated documentation as a string, or an error if something went wrong.
pub fn generate_documentation(path: &Path, language: Option<Language>) -> anyhow::Result<String> {
    let library = Library::load(path, language)?;
    Ok(format_library_context(&library))
}
