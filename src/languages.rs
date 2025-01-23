#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Rust,
}

impl std::str::FromStr for Language {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "rust" => Ok(Language::Rust),
            _ => anyhow::bail!("Unknown language '{}'", string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_str {
        use super::*;

        #[test]
        fn rust() {
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
