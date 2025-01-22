use crate::languages::Language;
use daipendency_extractor::Extractor;
use daipendency_extractor_rust::RustExtractor;

pub fn get_extractor(language: Language) -> Box<dyn Extractor> {
    match language {
        Language::Rust => Box::new(RustExtractor::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_extractor_rust() {
        let extractor = get_extractor(Language::Rust);

        assert_eq!(
            format!("{:?}", extractor.get_parser_language()),
            format!("{:?}", RustExtractor::new().get_parser_language())
        );
    }
}
