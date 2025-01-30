use std::path::PathBuf;

use crate::{extractors::get_extractor, languages::Language, library::Library};

pub fn get_dependency(name: &str, _dependant_path: &PathBuf) -> anyhow::Result<Library> {
    let dependency_path = get_dependency_path(name, _dependant_path)?;
    Ok(Library::load(&dependency_path, Some(Language::Rust))?)
}

fn get_dependency_path(name: &str, dependant_path: &PathBuf) -> anyhow::Result<PathBuf> {
    let extractor = get_extractor(Language::Rust);
    extractor.resolve_dependency_path(name, dependant_path).map_err(|e| anyhow::anyhow!(e))
}
