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
}

#[cfg(test)]
mod tests {
    use super::*;

    mod load {
        use super::*;
        use std::fs;
        use tempfile::TempDir;

        const STUB_NAME: &str = "test_crate";
        const STUB_VERSION: &str = "1.0.0";
        const STUB_DOCUMENTATION: &str = "Test documentation";

        fn create_temp_library(version: Option<String>) -> std::path::PathBuf {
            let dir = TempDir::new().unwrap();
            let src_dir = dir.path().join("src");
            fs::create_dir_all(&src_dir).unwrap();

            let cargo_toml = format!(
                r#"[package]
    name = "{}"
    {}
    "#,
                STUB_NAME,
                version.map_or(String::new(), |v| format!("version = \"{}\"", v))
            );
            fs::write(dir.path().join("Cargo.toml"), cargo_toml).unwrap();
            fs::write(dir.path().join("README.md"), STUB_DOCUMENTATION).unwrap();

            let lib_rs = r#"pub enum TestEnum {
        A,
    }
    "#;
            fs::write(src_dir.join("lib.rs"), lib_rs).unwrap();

            dir.into_path()
        }

        #[test]
        fn name() {
            let library_path = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, Some(Language::Rust)).unwrap();

            assert_eq!(library.name, STUB_NAME);
        }

        #[test]
        fn version_present() {
            let library_path = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, Some(Language::Rust)).unwrap();

            assert_eq!(library.version, Some(STUB_VERSION.to_string()));
        }

        #[test]
        fn version_absent() {
            let library_path = create_temp_library(None);

            let library = Library::load(&library_path, Some(Language::Rust)).unwrap();

            assert_eq!(library.version, None);
        }

        #[test]
        fn documentation() {
            let library_path = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, Some(Language::Rust)).unwrap();

            assert_eq!(library.documentation, STUB_DOCUMENTATION);
        }

        #[test]
        fn namespaces() {
            let library_path = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, Some(Language::Rust)).unwrap();

            assert_eq!(library.namespaces.len(), 1);
            let namespace = &library.namespaces[0];
            assert_eq!(namespace.name, STUB_NAME);
            assert_eq!(namespace.symbols.len(), 1);
            assert_eq!(namespace.symbols[0].name, "TestEnum");
        }

        #[test]
        fn language_present() {
            let library_path = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, Some(Language::Rust)).unwrap();

            assert_eq!(library.language, Language::Rust);
        }

        #[test]
        fn language_absent_but_discovered() {
            let library_path = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, None).unwrap();

            assert_eq!(library.language, Language::Rust);
        }

        #[test]
        fn language_absent_and_not_discovered() {
            let dir = TempDir::new().unwrap();

            let result = Library::load(dir.path(), None);

            assert!(result.is_err());
        }
    }
}
