use crate::extractors::get_extractor;
use crate::languages::Language;
use daipendency_extractor::{get_parser, Namespace};
use std::path::Path;

pub struct Library {
    pub name: String,
    pub version: Option<String>,
    pub documentation: String,
    pub namespaces: Vec<Namespace>,
    pub language: Language,
}

impl Library {
    pub fn load(path: &Path, language: Language) -> anyhow::Result<Self> {
        let extractor = get_extractor(language);
        let metadata = extractor.get_library_metadata(path)?;
        let mut parser = get_parser(&extractor.get_parser_language())?;
        let namespaces = extractor.extract_public_api(&metadata, &mut parser)?;

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

            let library = Library::load(&library_path, Language::Rust).unwrap();

            assert_eq!(library.name, STUB_NAME);
        }

        #[test]
        fn version_present() {
            let library_path = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, Language::Rust).unwrap();

            assert_eq!(library.version, Some(STUB_VERSION.to_string()));
        }

        #[test]
        fn version_absent() {
            let library_path = create_temp_library(None);

            let library = Library::load(&library_path, Language::Rust).unwrap();

            assert_eq!(library.version, None);
        }

        #[test]
        fn documentation() {
            let library_path = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, Language::Rust).unwrap();

            assert_eq!(library.documentation, STUB_DOCUMENTATION);
        }

        #[test]
        fn namespaces() {
            let library_path = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, Language::Rust).unwrap();

            assert_eq!(library.namespaces.len(), 1);
            let namespace = &library.namespaces[0];
            assert_eq!(namespace.name, STUB_NAME);
            assert_eq!(namespace.symbols.len(), 1);
            assert_eq!(namespace.symbols[0].name, "TestEnum");
        }

        #[test]
        fn language() {
            let library_path = create_temp_library(Some(STUB_VERSION.to_string()));

            let library = Library::load(&library_path, Language::Rust).unwrap();

            assert_eq!(library.language, Language::Rust);
        }
    }
}
