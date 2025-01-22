#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Rust,
}

impl std::str::FromStr for Language {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rust" => Ok(Language::Rust),
            _ => anyhow::bail!("Unknown language '{}'", s),
        }
    }
}
