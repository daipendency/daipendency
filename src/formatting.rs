use crate::languages::Language;
use crate::library::Library;
use daipendency_extractor::Namespace;

pub fn format_library_context(library: &Library, language: Language) -> String {
    let api_content = format_namespaces_content(
        &library.namespaces,
        &format!("{:?}", language).to_lowercase(),
    );

    format!(
        r#"---
library_name: {name}
library_version: {version}
---

{documentation}

# API

{api_content}"#,
        name = library.name,
        version = library.version.as_deref().unwrap_or("null"),
        documentation = library.documentation.trim(),
        api_content = api_content
    )
}

fn format_namespaces_content(namespaces: &[Namespace], language: &str) -> String {
    namespaces
        .iter()
        .filter(|n| !n.symbols.is_empty())
        .map(|n| format_namespace_content(n, language))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_namespace_content(namespace: &Namespace, language: &str) -> String {
    let mut content = format!("## {}\n\n", namespace.name);

    if !namespace.symbols.is_empty() {
        let mut code_block = String::new();
        if let Some(doc) = &namespace.doc_comment {
            code_block.push_str(doc);
            code_block.push('\n');
        }
        code_block.push_str(
            &namespace
                .symbols
                .iter()
                .map(|s| s.source_code.as_str())
                .collect::<Vec<_>>()
                .join("\n\n"),
        );
        content.push_str(&format!("```{}\n{}\n```\n", language, code_block));
    }

    content
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertables::assert_contains;
    use daipendency_extractor::Symbol;

    const STUB_LIBRARY_NAME: &str = "test-lib";
    const STUB_LIBRARY_VERSION: &str = "1.0.0";
    const STUB_DOCUMENTATION: &str = "Test documentation";
    const STUB_LANGUAGE: Language = Language::Rust;
    const STUB_LANGUAGE_STR: &str = "rust";
    const STUB_SOURCE_CODE: &str = "SOURCE_CODE";
    const STUB_MULTI_LINE_SOURCE_CODE: &str = "MULTI_LINE\nSOURCE_CODE";
    const STUB_DOC_COMMENT: &str = "This is a doc comment";

    fn create_library(namespaces: Vec<Namespace>) -> Library {
        Library {
            name: STUB_LIBRARY_NAME.to_string(),
            version: Some(STUB_LIBRARY_VERSION.to_string()),
            documentation: STUB_DOCUMENTATION.to_string(),
            namespaces,
        }
    }

    mod metadata {
        use super::*;

        fn get_frontmatter_lines(documentation: String) -> Option<Vec<String>> {
            let mut lines = documentation.lines();
            (lines.next() == Some("---")).then(|| {
                lines
                    .take_while(|&line| line != "---")
                    .map(String::from)
                    .collect()
            })
        }

        #[test]
        fn library_name() {
            let library = create_library(vec![]);
            let documentation = format_library_context(&library, STUB_LANGUAGE);
            let frontmatter_lines = get_frontmatter_lines(documentation).unwrap();

            assert_contains!(
                frontmatter_lines,
                &format!("library_name: {STUB_LIBRARY_NAME}")
            );
        }

        #[test]
        fn with_library_version() {
            let library = create_library(vec![]);
            let documentation = format_library_context(&library, STUB_LANGUAGE);
            let frontmatter_lines = get_frontmatter_lines(documentation).unwrap();

            assert_contains!(
                frontmatter_lines,
                &format!("library_version: {STUB_LIBRARY_VERSION}")
            );
        }

        #[test]
        fn without_library_version() {
            let mut library = create_library(vec![]);
            library.version = None;
            let documentation = format_library_context(&library, STUB_LANGUAGE);
            let frontmatter_lines = get_frontmatter_lines(documentation).unwrap();

            assert_contains!(frontmatter_lines, &"library_version: null".to_string());
        }

        #[test]
        fn library_documentation() {
            let library = create_library(vec![]);
            let documentation = format_library_context(&library, STUB_LANGUAGE);

            assert_contains!(
                documentation,
                &format!("\n---\n\n{STUB_DOCUMENTATION}\n\n# API")
            );
        }
    }

    mod api {
        use super::*;

        fn create_namespace(
            name: &str,
            symbols: Vec<Symbol>,
            doc_comment: Option<&str>,
        ) -> Namespace {
            Namespace {
                name: name.to_string(),
                symbols,
                doc_comment: doc_comment.map(String::from),
            }
        }

        fn create_symbol(name: &str, source_code: &str) -> Symbol {
            Symbol {
                name: name.to_string(),
                source_code: source_code.to_string(),
            }
        }

        fn assert_api_is_empty(documentation: &str) {
            let api_content = documentation.split("\n# API\n").nth(1).unwrap_or("").trim();
            assert!(
                api_content.is_empty(),
                "Expected empty API content, got: {api_content}"
            );
        }

        #[test]
        fn no_namespaces() {
            let library = create_library(vec![]);
            let documentation = format_library_context(&library, STUB_LANGUAGE);
            assert_api_is_empty(&documentation);
        }

        #[test]
        fn single_namespace() {
            let namespace = create_namespace(
                "test",
                vec![create_symbol("symbol", STUB_SOURCE_CODE)],
                None,
            );
            let namespace_name = namespace.name.clone();
            let library = create_library(vec![namespace]);
            let documentation = format_library_context(&library, STUB_LANGUAGE);

            assert_contains!(
                documentation,
                &format!(
                    "## {}\n\n```{}\n{}\n```",
                    namespace_name, STUB_LANGUAGE_STR, STUB_SOURCE_CODE
                )
            );
        }

        #[test]
        fn multiple_namespaces() {
            let namespace1 = create_namespace(
                "test1",
                vec![create_symbol("symbol1", STUB_SOURCE_CODE)],
                None,
            );
            let namespace2 = create_namespace(
                "test2",
                vec![create_symbol("symbol2", STUB_SOURCE_CODE)],
                None,
            );
            let namespace1_name = namespace1.name.clone();
            let namespace2_name = namespace2.name.clone();
            let library = create_library(vec![namespace1, namespace2]);
            let documentation = format_library_context(&library, STUB_LANGUAGE);

            assert_contains!(
                documentation,
                &format!("## {}\n\n```{}\n", namespace1_name, STUB_LANGUAGE_STR)
            );
            assert_contains!(
                documentation,
                &format!("## {}\n\n```{}\n", namespace2_name, STUB_LANGUAGE_STR)
            );
        }

        mod symbols {
            use super::*;

            #[test]
            fn namespace_without_symbols() {
                let namespace = create_namespace("test", vec![], None);
                let library = create_library(vec![namespace]);
                let documentation = format_library_context(&library, STUB_LANGUAGE);
                assert_api_is_empty(&documentation);
            }

            #[test]
            fn single_symbol() {
                let namespace = create_namespace(
                    "test",
                    vec![create_symbol("symbol", STUB_SOURCE_CODE)],
                    None,
                );
                let library = create_library(vec![namespace]);
                let documentation = format_library_context(&library, STUB_LANGUAGE);

                assert_contains!(documentation, &format!("```{}\n", STUB_LANGUAGE_STR));
                assert_contains!(documentation, STUB_SOURCE_CODE);
                assert_contains!(documentation, "\n```");
            }

            #[test]
            fn multiple_symbols() {
                let namespace = create_namespace(
                    "test",
                    vec![
                        create_symbol("symbol1", "FIRST"),
                        create_symbol("symbol2", "SECOND"),
                    ],
                    None,
                );
                let library = create_library(vec![namespace]);
                let documentation = format_library_context(&library, STUB_LANGUAGE);

                assert_contains!(documentation, &format!("```{}\n", STUB_LANGUAGE_STR));
                assert_contains!(documentation, "FIRST\n\nSECOND\n");
                assert_contains!(documentation, "```\n");
            }

            #[test]
            fn single_line_symbol() {
                let namespace = create_namespace(
                    "test",
                    vec![create_symbol("symbol", STUB_SOURCE_CODE)],
                    None,
                );
                let library = create_library(vec![namespace]);
                let documentation = format_library_context(&library, STUB_LANGUAGE);

                assert_contains!(documentation, &format!("```{}\n", STUB_LANGUAGE_STR));
                assert_contains!(documentation, STUB_SOURCE_CODE);
                assert_contains!(documentation, "\n```");
            }

            #[test]
            fn multi_line_symbol() {
                let namespace = create_namespace(
                    "test",
                    vec![create_symbol("symbol", STUB_MULTI_LINE_SOURCE_CODE)],
                    None,
                );
                let library = create_library(vec![namespace]);
                let documentation = format_library_context(&library, STUB_LANGUAGE);

                assert_contains!(documentation, &format!("```{}\n", STUB_LANGUAGE_STR));
                assert_contains!(documentation, STUB_MULTI_LINE_SOURCE_CODE);
                assert_contains!(documentation, "\n```");
            }

            #[test]
            fn namespace_without_doc_comment() {
                let namespace = create_namespace(
                    "test",
                    vec![create_symbol("symbol", STUB_SOURCE_CODE)],
                    None,
                );
                let library = create_library(vec![namespace]);
                let documentation = format_library_context(&library, STUB_LANGUAGE);

                assert_contains!(
                    documentation,
                    &format!("```{}\n{}\n```", STUB_LANGUAGE_STR, STUB_SOURCE_CODE)
                );
            }

            #[test]
            fn namespace_with_doc_comment() {
                let namespace = create_namespace(
                    "test",
                    vec![create_symbol("symbol", STUB_SOURCE_CODE)],
                    Some(STUB_DOC_COMMENT),
                );
                let library = create_library(vec![namespace]);
                let documentation = format_library_context(&library, STUB_LANGUAGE);

                assert_contains!(
                    documentation,
                    &format!(
                        "```{}\n{}\n{}\n```",
                        STUB_LANGUAGE_STR, STUB_DOC_COMMENT, STUB_SOURCE_CODE
                    )
                );
            }
        }
    }
}
