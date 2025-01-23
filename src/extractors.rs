use std::path::Path;

use crate::languages::{Language, LanguageConfig};
use crate::library::BoxedExtractor;
use daipendency_extractor::{LibraryMetadata, LibraryMetadataError};
use thiserror::Error;

pub fn get_extractor(language: Language) -> BoxedExtractor {
    let config = LanguageConfig::get_from_language(language);

    (config.extractor_initialiser)()
}

pub struct ExtractorDiscovery {
    pub language: Language,
    pub extractor: BoxedExtractor,
    pub library_metadata: LibraryMetadata,
}

impl std::fmt::Debug for ExtractorDiscovery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtractorDiscovery")
            .field("language", &self.language)
            .field("extractor", &"<dyn Extractor>")
            .field("library_metadata", &self.library_metadata)
            .finish()
    }
}

#[derive(Error, Debug)]
pub enum ExtractorDiscoveryError {
    #[error("Malformed manifest for {language:?}: {error}")]
    MalformedManifest { language: Language, error: String },
    #[error("No matching extractor found")]
    NotFound,
}

pub fn discover_extractor(path: &Path) -> Result<ExtractorDiscovery, ExtractorDiscoveryError> {
    for language in LanguageConfig::get_all().keys() {
        let extractor = get_extractor(*language);
        match extractor.get_library_metadata(path) {
            Ok(library_metadata) => {
                return Ok(ExtractorDiscovery {
                    language: *language,
                    extractor,
                    library_metadata,
                })
            }
            Err(LibraryMetadataError::MalformedManifest(error)) => {
                return Err(ExtractorDiscoveryError::MalformedManifest {
                    language: *language,
                    error,
                })
            }
            Err(LibraryMetadataError::MissingManifest(_)) => continue,
        }
    }
    Err(ExtractorDiscoveryError::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod extractor_getter {
        use super::*;
        use daipendency_extractor::Extractor;
        use daipendency_extractor_rust::RustExtractor;

        #[test]
        fn get_extractor_rust() {
            let extractor = get_extractor(Language::Rust);

            assert_eq!(
                format!("{:?}", extractor.get_parser_language()),
                format!("{:?}", RustExtractor::new().get_parser_language())
            );
        }
    }

    mod extractor_discovery {
        use super::*;
        use assertables::{assert_matches, assert_ok};
        use daipendency_extractor::Extractor;
        use daipendency_extractor_rust::RustExtractor;
        use std::fs;
        use tempfile::TempDir;

        #[test]
        fn no_matching_extractor() {
            let temp_dir = TempDir::new().unwrap();

            let result = discover_extractor(temp_dir.path());

            assert_matches!(result, Err(ExtractorDiscoveryError::NotFound));
        }

        #[test]
        fn malformed_manifest() {
            let temp_dir = TempDir::new().unwrap();
            fs::write(temp_dir.path().join("Cargo.toml"), "invalid toml").unwrap();

            let result = discover_extractor(temp_dir.path());

            assert_matches!(
                result,
                Err(ExtractorDiscoveryError::MalformedManifest {
                    language: Language::Rust,
                    error: _
                })
            );
        }

        #[test]
        fn successful_discovery() {
            let temp_dir = TempDir::new().unwrap();
            fs::write(
                temp_dir.path().join("Cargo.toml"),
                r#"[package]
name = "test-package"
version = "0.1.0"
"#,
            )
            .unwrap();

            let result = discover_extractor(temp_dir.path());
            let discovery = result.as_ref().unwrap();

            assert_ok!(&result);
            assert_eq!(discovery.language, Language::Rust);
            assert_eq!(discovery.library_metadata.name, "test-package");
            assert_eq!(
                format!("{:?}", discovery.extractor.get_parser_language()),
                format!("{:?}", RustExtractor::new().get_parser_language())
            );
        }
    }
}
