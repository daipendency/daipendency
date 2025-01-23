use std::env;
use std::path::PathBuf;

use daipendency::{generate_documentation, Language};

fn main() -> Result<(), String> {
    run_with_args(env::args().collect(), |path, lang| {
        generate_documentation(path.as_path(), lang).map_err(|e| e.to_string())
    })
}

fn run_with_args(
    args: Vec<String>,
    doc_generator: impl Fn(&PathBuf, Option<Language>) -> Result<String, String>,
) -> Result<(), String> {
    let (path, language) = parse_args(&args)?;
    let output = doc_generator(&path, language)?;
    println!("{}", output);
    Ok(())
}

fn parse_args(args: &[String]) -> Result<(PathBuf, Option<Language>), String> {
    if args.len() < 2 {
        return Err(format!("Usage: {} <path> [--language=<language>]", args[0]));
    }

    let mut path = None;
    let mut language = None;

    for arg in &args[1..] {
        if arg.starts_with("--language=") {
            let lang_str = arg.strip_prefix("--language=").unwrap();
            language = Some(
                lang_str
                    .parse::<Language>()
                    .map_err(|error| format!("Failed to get language: {}", error))?,
            );
        } else if path.is_none() {
            path = Some(PathBuf::from(arg));
        } else {
            return Err(format!("Usage: {} <path> [--language=<language>]", args[0]));
        }
    }

    let path = path.ok_or_else(|| format!("Usage: {} <path> [--language=<language>]", args[0]))?;
    Ok((path, language))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_generate_docs(_path: &PathBuf, language: Option<Language>) -> Result<String, String> {
        match language {
            Some(_) => Ok("Rust docs".to_string()),
            None => Ok("Default docs".to_string()),
        }
    }

    #[test]
    fn test_run_with_valid_path() {
        let args = vec!["program".to_string(), "/some/path".to_string()];

        let result = run_with_args(args, mock_generate_docs);

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_language() {
        let args = vec![
            "program".to_string(),
            "/some/path".to_string(),
            "--language=rust".to_string(),
        ];

        let result = run_with_args(args, mock_generate_docs);

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_invalid_language() {
        let args = vec![
            "program".to_string(),
            "/some/path".to_string(),
            "--language=invalid".to_string(),
        ];

        let result = run_with_args(args, mock_generate_docs);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to get language"));
    }

    #[test]
    fn test_run_with_no_args() {
        let args = vec!["program".to_string()];

        let result = run_with_args(args, mock_generate_docs);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Usage:"));
    }

    #[test]
    fn test_run_with_multiple_paths() {
        let args = vec![
            "program".to_string(),
            "/some/path".to_string(),
            "/another/path".to_string(),
        ];

        let result = run_with_args(args, mock_generate_docs);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Usage:"));
    }
}
