# Daipendency

_Daipendency_ extracts public API documentation from a library and outputs it in an LLM-friendly format.
**The endgame is to provide AI coding agents with all the context they need to use a particular dependency**,
but for now you can just use it manually on the CLI.

This project was inspired by [Aider's _repository map_](https://aider.chat/docs/repomap.html).

## Features

- Outputs public symbols (e.g. functions) only.
- Outputs function signatures and documentation, but not the implementation.
- Only supports Rust for now, but [any language supported by tree-sitter](https://github.com/tree-sitter/tree-sitter/wiki/List-of-parsers) can be supported.
- Reads the source code directly, so it doesn't process the HTML of the generated documentation, thus keeping the output clean.

## CLI Usage

To extract the documentation from a library, pass the name of the language and the path to the library. For example:

```sh
daipendency rust /path/to/library
```

## Library Usage

```rust
use daipendency::{generate_documentation, Language};
use std::path::Path;

let path = Path::new("/path/to/crate");
match generate_documentation(path, Language::Rust) {
    Ok(output) => println!("{}", output),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Development

### Adding Support for a New Language

To add support for a new language, you need to:

1. Implement the [`daipendency_extractor::Extractor` trait](https://docs.rs/daipendency-extractor/latest/daipendency_extractor/trait.Extractor.html) for the language. See [daipendency-extractor-rust](https://github.com/daipendency/daipendency-extractor-rust) for an example.
2. Release your crate. Note that only MIT- or Apache-2.0-licensed crates are eligible for inclusion in Daipendency.
3. Integrate your crate in `src/languages.rs`.
