use crate::extractors::{discover_extractor, get_extractor};
use crate::languages::Language;
use daipendency_extractor::{get_parser, Extractor, Namespace};
use std::path::Path;

pub type BoxedExtractor = Box<dyn Extractor + Send + Sync>;

pub struct Library {
    pub name: String,
    pub version: Option<String>,
    pub documentation: String,
    pub namespaces: Vec<Namespace>,
    pub language: Language,
}

impl Library {
    /// Load a library from a `path`.
    ///
    /// Omitting the `language` argument will attempt to discover the language of the library by iterating over the supported languages, which can get very slow as we add more languages.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the loaded library, or an error if something went wrong.
    pub fn load(path: &Path, language: Option<Language>) -> anyhow::Result<Self> {
        let (extractor, metadata, language) = if let Some(lang) = language {
            let extractor = get_extractor(lang);
            let metadata = extractor.get_library_metadata(path)?;
            (extractor, metadata, lang)
        } else {
            let discovery = discover_extractor(path).map_err(|e| anyhow::anyhow!(e))?;
            (
                discovery.extractor,
                discovery.library_metadata,
                discovery.language,
            )
        };

        let mut parser = get_parser(&extractor.get_parser_language())?;
        let namespaces = match extractor.extract_public_api(&metadata, &mut parser) {
            Ok(namespaces) => namespaces,
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to extract public API: {}", e));
            }
        };

        Ok(Self {
            name: metadata.name,
            version: metadata.version,
            documentation: metadata.documentation,
            namespaces,
            language,
        })
    }

    /// Load a dependency of a crate.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the loaded dependency, or an error if something went wrong.
    pub fn load_dependency(
        name: &str,
        dependant_path: &Path,
        language: Option<Language>,
    ) -> anyhow::Result<Self> {
        let (extractor, language) = if let Some(lang) = language {
            let extractor = get_extractor(lang);
            (extractor, lang)
        } else {
            let discovery = discover_extractor(dependant_path).map_err(|e| anyhow::anyhow!(e))?;
            (discovery.extractor, discovery.language)
        };
        let dependency_path = extractor
            .resolve_dependency_path(name, dependant_path)
            .map_err(|e| anyhow::anyhow!(e))?;
        Self::load(&dependency_path, Some(language))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod load {
        use super::*;
        use daipendency_testing::tempdir::TempDir;

        const STUB_NAME: &str = "test_crate";
        const STUB_VERSION: &str = "1.0.0";
        const STUB_DOCUMENTATION: &str = "Test documentation";

        fn create_temp_library(version: Option<String>) -> (std::path::PathBuf, TempDir) {
            let temp_dir = TempDir::new();
            let cargo_toml = format!(
                r#"[package]
    name = "{}"
    {}
    "#,
                STUB_NAME,
                version.map_or(String::new(), |v| format!("version = \"{}\"", v))
            );
            temp_dir.create_file("Cargo.toml", &cargo_toml).unwrap();
            temp_dir
                .create_file("README.md", STUB_DOCUMENTATION)
                .unwrap();
            temp_dir
                .create_file(
                    "src/lib.rs",
                    r#"pub enum TestEnum {
        A,
    }
    "#,
                )
                .unwrap();
            (temp_dir.path.to_path_buf(), temp_dir)
        }

        #[test]
        fn name() {
            let (library_path, _temp_dir) = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, Some(Language::Rust)).unwrap();

            assert_eq!(library.name, STUB_NAME);
        }

        #[test]
        fn version_present() {
            let (library_path, _temp_dir) = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, Some(Language::Rust)).unwrap();

            assert_eq!(library.version, Some(STUB_VERSION.to_string()));
        }

        #[test]
        fn version_absent() {
            let (library_path, _temp_dir) = create_temp_library(None);

            let library = Library::load(&library_path, Some(Language::Rust)).unwrap();

            assert_eq!(library.version, None);
        }

        #[test]
        fn documentation() {
            let (library_path, _temp_dir) = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, Some(Language::Rust)).unwrap();

            assert_eq!(library.documentation, STUB_DOCUMENTATION);
        }

        #[test]
        fn namespaces() {
            let (library_path, _temp_dir) = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, Some(Language::Rust)).unwrap();

            assert_eq!(library.namespaces.len(), 1);
            let namespace = &library.namespaces[0];
            assert_eq!(namespace.name, STUB_NAME);
            assert_eq!(namespace.symbols.len(), 1);
            assert_eq!(namespace.symbols[0].name, "TestEnum");
        }

        #[test]
        fn language_present() {
            let (library_path, _temp_dir) = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, Some(Language::Rust)).unwrap();

            assert_eq!(library.language, Language::Rust);
        }

        #[test]
        fn language_absent_but_discovered() {
            let (library_path, _temp_dir) = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, None).unwrap();

            assert_eq!(library.language, Language::Rust);
        }

        #[test]
        fn language_absent_and_not_discovered() {
            let temp_dir = TempDir::new();

            let result = Library::load(&temp_dir.path, None);

            assert!(result.is_err());
        }
    }
}
