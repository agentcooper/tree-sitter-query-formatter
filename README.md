Pretty printer for [Tree-sitter queries](https://tree-sitter.github.io/tree-sitter/using-parsers/queries/1-syntax.html).

## Useful links

- [Tree-sitter query grammar](https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-query/refs/heads/master/grammar.js)

## Development

Use `cargo test --test integration_tests` to run tests.

Use `cargo run <INPUT>` to print output for a file or query string. Use `cargo run <INPUT> --tree` to also print a parse tree.
