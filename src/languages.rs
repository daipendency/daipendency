use daipendency_extractor::Extractor;
use daipendency_extractor_rust::RustExtractor;
use std::collections::HashMap;
use std::sync::OnceLock;

type ExtractorInitialiser = fn() -> Box<dyn Extractor + Send + Sync>;

/// The languages supported by daipendency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
}

pub struct LanguageConfig {
    pub name: &'static str,
    pub extractor_initialiser: ExtractorInitialiser,
}

static LANGUAGE_CONFIGS: OnceLock<HashMap<Language, LanguageConfig>> = OnceLock::new();

fn initialise_config() -> HashMap<Language, LanguageConfig> {
    let mut configs = HashMap::new();
    configs.insert(
        Language::Rust,
        LanguageConfig {
            name: "rust",
            extractor_initialiser: || Box::new(RustExtractor::new()),
        },
    );
    configs
}

impl LanguageConfig {
    pub fn get_from_language(language: Language) -> &'static LanguageConfig {
        Self::get_all()
            .get(&language)
            .unwrap_or_else(|| panic!("No config found for language {:?}", language))
    }

    pub fn get_all() -> &'static HashMap<Language, LanguageConfig> {
        LANGUAGE_CONFIGS.get_or_init(initialise_config)
    }
}

impl std::str::FromStr for Language {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LanguageConfig::get_all()
            .iter()
            .find(|(_, config)| config.name == s)
            .map(|(lang, _)| *lang)
            .ok_or_else(|| anyhow::anyhow!("Unknown language '{}'", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod language_config {
        use super::*;

        #[test]
        fn get_from_language() {
            let config = LanguageConfig::get_from_language(Language::Rust);

            assert_eq!(config.name, "rust");
        }

        #[test]
        fn get_all() {
            let configs = LanguageConfig::get_all();

            assert_eq!(configs.len(), 1);
            let rust_config = configs.get(&Language::Rust).unwrap();
            assert_eq!(rust_config.name, "rust");
        }
    }

    mod language {
        use super::*;

        #[test]
        fn from_str() {
            let input = "rust";

            let result = input.parse::<Language>().unwrap();

            assert_eq!(result, Language::Rust);
        }

        #[test]
        fn unsupported_language() {
            let input = "python";

            let result = input.parse::<Language>();

            assert!(result.is_err());
        }
    }
}
