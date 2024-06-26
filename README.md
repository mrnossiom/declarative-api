# Declarative API

<p align="center"><strong>
A wannabe full toolchain for an OpenAPI replacement including a compiler with pluggable generators as output
</strong></p>

<p align="center">
  <a href="https://wakatime.com/badge/github/mrnossiom/declarative-api">
    <img alt="Time spent" src="https://wakatime.com/badge/github/mrnossiom/declarative-api.svg" />
  </a>
</p>

# Features

- Could be integrated with [oxidecomputer/progenitor](https://github.com/oxidecomputer/progenitor) which generated Rust clients based on an OpenAPI spec.

# Generation process

Files are processed this way:

1. They are first tokenized into `lexer::poor::Token`s that are deeply connected to source
2. These are next enriched (see `lexer::rich::Enricher`) to crate independent `lexer::rich::Token`s
3. The rich lexemes are then parsed into an AST with `parser::Parser` (the root node is `ast::Api`)
4. This AST is expanded by the `expand` create, notably to resolve external files (e.g. sub-scopes)
5. (WIP) AST is then lowered to reach an HIR, a easily machine readable state (AST lowering)
6. (TODO) Passes are done on the HIR to check validity, correctness and completeness
7. (TODO) Generators can output multiple format (`Markdown`, `Interactive page`, `OpenAPI`, `TypeScript`, `Rust`, etc.) from this HIR

# Development

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

---

Work is licensed under [`CECILL-2.1`](https://choosealicense.com/licenses/cecill-2.1/), a French OSS license that allows modification and distribution of the software while requiring the same license for derived works.

