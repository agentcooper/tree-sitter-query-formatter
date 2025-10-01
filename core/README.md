Formatter for [Tree-sitter queries](https://tree-sitter.github.io/tree-sitter/using-parsers/queries/1-syntax.html).

## CLI

```bash
# Format from a file
tree-sitter-query-formatter query.scm

# Format from stdin
echo '(call_expression function: (identifier) @f arguments: (arguments) @args)' | tree-sitter-query-formatter
```

## Code

```rust
use tree_sitter_query_formatter::format;

let query = "(call_expression function: (identifier) @f arguments: (arguments) @args)";
let formatted = format(query, 80).unwrap();
```

## Web

https://agentcooper.github.io/tree-sitter-query-formatter/
