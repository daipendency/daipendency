# Daipendency

_Daipendency_ extracts public API documentation from a library and outputs it in an LLM-friendly format.
**This project allows you to use the functionality programmatically or via the CLI**,
so if you just want to integrate it into your favourite AI coding agent,
check out [daipendency-mcp](https://github.com/daipendency/daipendency-mcp).

## Features

- Outputs public symbols (e.g. functions) only.
- Outputs function signatures and documentation, but not the implementation.
- Only supports Rust for now, but [any language supported by tree-sitter](https://github.com/tree-sitter/tree-sitter/wiki/List-of-parsers) can be supported.
- Reads the source code directly, so it doesn't process the HTML of the generated documentation, thus keeping the output clean.

## CLI Usage

### `daipendency extract-dep`: Extract the documentation of a dependency

To extract the documentation of a dependency of **the project in the current directory**, pass the name of the dependency. For example:

```sh
daipendency extract-dep thiserror
```

Alternatively, you can specify the path to the project that contains the dependency with the `--dependant` option. For example:

```sh
daipendency extract-dep --dependant=/path/to/your/crate thiserror
```

**This command will honour the version of the dependency specified in the manifest file**,
like `Cargo.toml` in the case of a Rust crate.

### `daipendency extract`: Extract the documentation of a library

To extract the documentation from a library, pass the path to it. For example:

```sh
daipendency extract /path/to/library
```

## Library Usage

You can use the [`daipendency`](https://crates.io/crates/daipendency) crate in your own Rust project.

Firstly, you need to load the library from which you want to extract the documentation using `Library::load_dependency` or `Library::load`. For example:

```rust
use daipendency::{Library, Language};
use std::path::Path;

let library = Library::load_dependency(
    "thiserror",
    Path::new("/path/to/crate"),
    Some(Language::Rust),
)?;
```

[`Library`](https://docs.rs/daipendency/latest/daipendency/struct.Library.html) instances contain all the [_symbols_](https://docs.rs/daipendency-extractor/latest/daipendency_extractor/struct.Symbol.html) (e.g. functions) in the library, grouped into [_namespaces_](https://docs.rs/daipendency-extractor/latest/daipendency_extractor/struct.Namespace.html) (e.g. Rust _modules_, Java _packages_).
You can extract the namespaces and symbols in which you're interested and process them however you want,
or you can use the `generate_markdown_documentation` function to generate a Markdown file as follows:

```rust
use daipendency::generate_markdown_documentation;

let documentation = generate_markdown_documentation(&library);
```

## Automatic Language Detection

Daipendency can automatically detect the language of a library if you don't specify it in the CLI with the `--language` option or in the `Library` function.
However, you should try to specify the language explicitly, since auto-detection can get slow as we add more languages.

## Development

### Adding Support for a New Language

To add support for a new language, you need to:

1. Implement the [`daipendency_extractor::Extractor` trait](https://docs.rs/daipendency-extractor/latest/daipendency_extractor/trait.Extractor.html) for the language. See [daipendency-extractor-rust](https://github.com/daipendency/daipendency-extractor-rust) for an example.
2. Release your crate. Note that only MIT- or Apache-2.0-licensed crates are eligible for inclusion in Daipendency.
3. Integrate your crate in `src/languages.rs`.
