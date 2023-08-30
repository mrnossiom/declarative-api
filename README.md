# Declarative API

## Generation process

Files are process this way:

1. They are first tokenized into [`lexer::poor::Token`]s that are deeply connected to source
2. These are next enriched (see [`lexer::rich::Enricher`]) to crate independent [`lexer::rich::Token`]s
3. (WIP) The rich lexemes are then parsed into an AST with [`parser::Parser`] (the root node is [`ast::types::Api`])
4. (TODO) Multiples passes are done on the ast to check validity, correctness and completeness
5. (TODO) The AST is then expanded to reach a easily machine readable state
6. (TODO) AST is morphed to an IR that generators can use
7. (TODO) Generators can output multiple format (`Markdown`, `Interactive page`, `OpenAPI`, `TypeScript`, `Rust`, etc.)

## Development

You test manually certain components of this project with the `driver` cli.

running `cargo run -- dev --help` shows you the available commands:

-   `lex`: creates a cursor of the given source and displays every token (passing `--rich` uses the `Enricher` to stream rich tokens)
-   `parse`: creates a parser of the given source and parses the AST emitting errors along the way

You can observe the components and different function calls with the `RUST_LOG` env:

```sh
# something like (see `tracing_subscriber::filter::EnvFilter` struct for more information)
RUST_LOG=[<crate-name>=]<level>
# e.g.
RUST_LOG=parser=tracer
RUST_LOG=error,lexer=debug
```
