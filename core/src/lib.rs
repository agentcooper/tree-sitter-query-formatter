use pretty::RcDoc;
use tree_sitter::{Node, Parser};

fn map_named_node_without_captures<'a>(node: Node<'a>, source: &'a str) -> RcDoc<'a, ()> {
    let mut docs = Vec::new();

    let mut field_docs = Vec::new();
    let mut has_fields = false;

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            match child.kind() {
                "(" => docs.push(RcDoc::text("(")),
                ")" => {}
                "field_definition" => {
                    field_docs.push(map(child, source));
                    has_fields = true;
                }
                "negated_field" => {
                    field_docs.push(map(child, source));
                    has_fields = true;
                }
                "identifier" => docs.push(map(child, source)),
                "_" => docs.push(map(child, source)),
                "named_node" => {
                    // When a named_node (which has no fields) contains nested named_nodes,
                    // try to fit on one line, but break if too long
                    let nested_content = RcDoc::group(RcDoc::concat(vec![RcDoc::nest(
                        RcDoc::concat(vec![RcDoc::line(), map(child, source)]),
                        2,
                    )]));
                    docs.push(nested_content);
                }
                "capture" => {}
                _ => docs.push(map(child, source)),
            }
        }
    }

    if has_fields {
        let nested_fields = RcDoc::nest(
            RcDoc::concat(vec![
                RcDoc::hardline(),
                RcDoc::intersperse(field_docs, RcDoc::hardline()),
            ]),
            2,
        );
        docs.push(nested_fields);
        docs.push(RcDoc::text(")"));
        RcDoc::concat(docs)
    } else {
        docs.push(RcDoc::text(")"));
        RcDoc::concat(docs)
    }
}

fn map<'a>(node: Node<'a>, source: &'a str) -> RcDoc<'a, ()> {
    match node.kind() {
        "program" => {
            let mut docs = Vec::new();
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    docs.push(map(child, source));
                }
            }
            RcDoc::intersperse(docs, RcDoc::line())
        }
        "named_node" => {
            let mut docs = Vec::new();

            let mut field_docs = Vec::new();
            let mut predicate_docs = Vec::new();
            let mut has_fields = false;
            let mut has_capture_after_paren = false;

            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    if child.kind() == ")" {
                        if i + 1 < node.child_count() {
                            has_capture_after_paren = true;
                        }
                    } else if child.kind() == "field_definition" || child.kind() == "negated_field"
                    {
                        has_fields = true;
                    }
                }
            }

            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    match child.kind() {
                        "(" => docs.push(RcDoc::text("(")),
                        ")" => {
                            if has_capture_after_paren {
                                docs.push(RcDoc::text(")"));
                            } else {
                            }
                        }
                        "field_definition" => {
                            field_docs.push(map(child, source));
                        }
                        "negated_field" => {
                            field_docs.push(map(child, source));
                        }
                        "identifier" => docs.push(map(child, source)),
                        "_" => docs.push(map(child, source)),
                        "named_node" => {
                            docs.push(RcDoc::text(" "));
                            docs.push(map(child, source));
                        }
                        "capture" => docs.push(map(child, source)),
                        "quantifier" => docs.push(map(child, source)),
                        "predicate" => {
                            if has_fields {
                                predicate_docs.push(map(child, source));
                            } else {
                                docs.push(RcDoc::text(" "));
                                docs.push(map(child, source));
                            }
                        }
                        _ => docs.push(map(child, source)),
                    }
                }
            }

            if has_fields {
                let nested_fields = RcDoc::nest(
                    RcDoc::concat(vec![
                        RcDoc::hardline(),
                        RcDoc::intersperse(field_docs, RcDoc::hardline()),
                        if !predicate_docs.is_empty() {
                            RcDoc::concat(vec![
                                RcDoc::hardline(),
                                RcDoc::intersperse(predicate_docs, RcDoc::hardline()),
                            ])
                        } else {
                            RcDoc::nil()
                        },
                    ]),
                    2,
                );
                docs.push(nested_fields);
                docs.push(RcDoc::text(")"));
                RcDoc::concat(docs)
            } else {
                for predicate_doc in predicate_docs {
                    docs.push(RcDoc::text(" "));
                    docs.push(predicate_doc);
                }
                if !has_capture_after_paren {
                    docs.push(RcDoc::text(")"));
                }
                RcDoc::concat(docs)
            }
        }
        "field_definition" => {
            let mut docs = Vec::new();

            if let Some(name_child) = node.child_by_field_name("name") {
                docs.push(map(name_child, source));
                docs.push(RcDoc::text(": "));
            }

            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    if child.kind() == "named_node" {
                        docs.push(map_named_node_without_captures(child, source));
                        for j in 0..child.child_count() {
                            if let Some(capture_child) = child.child(j) {
                                if capture_child.kind() == "capture" {
                                    docs.push(map(capture_child, source));
                                }
                            }
                        }
                    } else if child.kind() != "identifier" && child.kind() != ":" {
                        docs.push(map(child, source));
                    }
                }
            }

            RcDoc::concat(docs)
        }
        "identifier" => {
            let text = &source[node.start_byte()..node.end_byte()];
            RcDoc::text(text)
        }
        "capture" => {
            let text = &source[node.start_byte()..node.end_byte()];
            RcDoc::text(format!(" {}", text))
        }
        "anonymous_node" => {
            let text = &source[node.start_byte()..node.end_byte()];
            RcDoc::text(text)
        }
        "missing_node" => {
            let mut docs = Vec::new();

            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    match child.kind() {
                        "(" => docs.push(RcDoc::text("(")),
                        "MISSING" => docs.push(RcDoc::text("MISSING")),
                        ")" => docs.push(RcDoc::text(")")),
                        "capture" => docs.push(map(child, source)),
                        _ => docs.push(map(child, source)),
                    }
                }
            }

            RcDoc::concat(docs)
        }
        "quantifier" => {
            let text = &source[node.start_byte()..node.end_byte()];
            RcDoc::text(text)
        }
        "grouping" => {
            let mut docs = Vec::new();
            let mut child_docs = Vec::new();
            let mut quantifier_docs = Vec::new();
            let mut capture_docs = Vec::new();

            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    match child.kind() {
                        "(" => docs.push(RcDoc::text("(")),
                        ")" => {}
                        "named_node" => {
                            child_docs.push(map(child, source));
                        }
                        "anonymous_node" => {
                            child_docs.push(map(child, source));
                        }
                        "predicate" => {
                            child_docs.push(map(child, source));
                        }
                        "." => {
                            child_docs.push(RcDoc::text("."));
                        }
                        "quantifier" => {
                            quantifier_docs.push(map(child, source));
                        }
                        "capture" => {
                            capture_docs.push(map(child, source));
                        }
                        _ => docs.push(map(child, source)),
                    }
                }
            }

            if child_docs.len() > 1 {
                let content = RcDoc::group(RcDoc::nest(
                    RcDoc::concat(vec![
                        RcDoc::line_(),
                        RcDoc::intersperse(child_docs, RcDoc::line()),
                        RcDoc::line_(),
                    ]),
                    2,
                ));
                docs.push(content);
            } else if child_docs.len() == 1 {
                docs.push(child_docs.into_iter().next().unwrap());
            }

            docs.push(RcDoc::text(")"));

            for quantifier_doc in quantifier_docs {
                docs.push(quantifier_doc);
            }

            for capture_doc in capture_docs {
                docs.push(capture_doc);
            }

            RcDoc::concat(docs)
        }
        "list" => {
            let mut docs = Vec::new();
            let mut child_docs = Vec::new();
            let mut captures = Vec::new();

            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    match child.kind() {
                        "[" => docs.push(RcDoc::text("[")),
                        "]" => {}
                        "capture" => captures.push(map(child, source)),
                        "anonymous_node" => {
                            child_docs.push(map(child, source));
                        }
                        _ => {
                            child_docs.push(map(child, source));
                        }
                    }
                }
            }

            if !child_docs.is_empty() {
                let content = RcDoc::nest(
                    RcDoc::concat(vec![
                        RcDoc::hardline(),
                        RcDoc::intersperse(child_docs, RcDoc::hardline()),
                        RcDoc::hardline(),
                    ]),
                    2,
                );
                docs.push(content);
            }

            docs.push(RcDoc::text("]"));

            for capture in captures {
                docs.push(capture);
            }

            RcDoc::concat(docs)
        }
        "_" => {
            let text = &source[node.start_byte()..node.end_byte()];
            RcDoc::text(text)
        }
        "predicate" => {
            let mut docs = Vec::new();

            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    docs.push(map(child, source));
                }
            }

            RcDoc::concat(docs)
        }
        "predicate_type" => {
            let text = &source[node.start_byte()..node.end_byte()];
            RcDoc::text(text)
        }
        "parameters" => {
            let mut docs = Vec::new();
            let mut string_docs = Vec::new();

            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    match child.kind() {
                        "identifier" => {
                            docs.push(RcDoc::space());
                            docs.push(map(child, source))
                        }
                        "capture" => docs.push(map(child, source)),
                        "string" => {
                            string_docs.push(map(child, source));
                        }
                        _ => docs.push(map(child, source)),
                    }
                }
            }

            if string_docs.len() > 1 {
                let formatted_strings = RcDoc::nest(
                    RcDoc::concat(vec![
                        RcDoc::hardline(),
                        RcDoc::intersperse(string_docs, RcDoc::hardline()),
                    ]),
                    2,
                );
                docs.push(formatted_strings);
            } else if string_docs.len() == 1 {
                docs.push(RcDoc::space());
                docs.push(string_docs.into_iter().next().unwrap());
            }

            RcDoc::concat(docs)
        }
        "string" => {
            let text = &source[node.start_byte()..node.end_byte()];
            RcDoc::text(text)
        }
        "string_content" => {
            let text = &source[node.start_byte()..node.end_byte()];
            RcDoc::text(text)
        }
        "negated_field" => {
            let text = &source[node.start_byte()..node.end_byte()];
            RcDoc::text(text)
        }
        "#" => {
            let text = &source[node.start_byte()..node.end_byte()];
            RcDoc::text(text)
        }
        "(" => RcDoc::text("("),
        ")" => RcDoc::text(")"),
        "." => RcDoc::text(" ."),
        "comment" => {
            let text = &source[node.start_byte()..node.end_byte()];
            RcDoc::text(text)
        }
        _ => {
            println!("Did not handle {}", node.kind());
            RcDoc::nil()
        }
    }
}

/// Formats a Tree-sitter query string with proper indentation and line breaks.
///
/// Takes a Tree-sitter query as input and formats it according to the grammar rules,
/// applying consistent indentation and line breaking to improve readability.
///
/// # Arguments
///
/// * `input` - The Tree-sitter query string to format
/// * `width` - The target line width for formatting
///
/// # Returns
///
/// Returns a `Result` containing the formatted query string on success, or an error
/// if parsing or formatting fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The Tree-sitter grammar cannot be loaded
/// - The input query cannot be parsed
/// - The formatted output cannot be rendered
///
/// # Example
///
/// ```
/// use tree_sitter_query_formatter::format;
///
/// let query = "(function_definition name: (identifier) @func)";
/// let formatted = format(query, 80).unwrap();
/// ```
pub fn format(input: &str, width: usize) -> Result<String, Box<dyn std::error::Error>> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_tsquery::LANGUAGE.into())
        .map_err(|e| format!("Error loading grammar: {:?}", e))?;

    let tree = parser.parse(input, None).ok_or("Failed to parse input")?;

    let root_node = tree.root_node();
    let doc = map(root_node, input);

    let mut w = Vec::new();
    doc.render(width, &mut w)?;
    let output = String::from_utf8(w)?;

    Ok(output)
}
