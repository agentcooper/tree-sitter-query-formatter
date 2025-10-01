Formatter for [Tree-sitter queries](https://tree-sitter.github.io/tree-sitter/using-parsers/queries/1-syntax.html).

Available [on the web](https://agentcooper.github.io/tree-sitter-query-formatter/) and as a [Rust crate](https://crates.io/crates/tree-sitter-query-formatter).

## Development

Use `cargo test --test integration_tests` to run tests.

Use `cargo run <FILE>` to print output for a file or `echo '...' | cargo run` for a short snippet. Use `cargo run <FILE> --tree` to also print a parse tree.

Use `make dev` to run the browser playground locally.

## Useful links

- [Tree-sitter query grammar](https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-query/refs/heads/master/grammar.js)
