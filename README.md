# Declarative API

## Process

Files are process this way

1. They are first tokenized into [`lexer::poor::Token`]s that are deeply connected to source
2. These are next enriched (see [`lexer::rich::Enricher`]) to crate independent [`lexer::rich::Token`]s
3. (WIP) The rich lexemes are then parsed into an AST (the root node is [`ast::ast::Api`]) with [`parser::Parser`]
4. (TODO) Multiples passes are done on the ast to check validity, correctness and completeness
5. (TODO) The AST is then expanded to reach a easily machine readable state
6. (TODO) AST is morphed to an IR that generators can use
7. (TODO) Generators can output multiple format (`Markdown`, `Interactive page`, `OpenAPI`, `TypeScript`, `Rust`, etc.)
