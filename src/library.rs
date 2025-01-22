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

pub fn extract_public_api(path: &Path, language: Language) -> anyhow::Result<Library> {
    let extractor = get_extractor(language);
    let metadata = extractor.get_library_metadata(path)?;
    let mut parser = get_parser(&extractor.get_parser_language())?;
    let namespaces = extractor.extract_public_api(&metadata, &mut parser)?;

    Ok(Library {
        name: metadata.name,
        version: metadata.version,
        documentation: metadata.documentation,
        namespaces,
        language,
    })
}
