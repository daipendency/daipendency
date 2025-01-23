use crate::languages::{Language, LanguageConfig};
use daipendency_extractor::Extractor;

pub fn get_extractor(language: Language) -> Box<dyn Extractor + Send + Sync> {
    let config = LanguageConfig::get_from_language(language);

    (config.extractor_initialiser)()
}

#[cfg(test)]
mod tests {
    use super::*;
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
