use clap::{Arg, Command};
use std::fs;
use std::io::{self, Read};
use tree_sitter::{Node, Parser};
use tree_sitter_query_formatter::format;

fn print_tree_recursive(node: Node, source: &str, depth: usize) -> String {
    let indent = "  ".repeat(depth);
    let node_text = if node.child_count() == 0 {
        let text = &source[node.start_byte()..node.end_byte()];
        format!(" \"{}\"", text.replace('\n', "\\n"))
    } else {
        String::new()
    };
    
    let mut result = format!("{}{}{}\n", indent, node.kind(), node_text);
    
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            result.push_str(&print_tree_recursive(child, source, depth + 1));
        }
    }
    
    result
}

fn print_tree(input: &str) -> Option<String> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_tsquery::LANGUAGE.into())
        .expect("Error loading tree-sitter query grammar");

    if let Some(tree) = parser.parse(input, None) {
        let root_node = tree.root_node();
        return Some(print_tree_recursive(root_node, input, 0));
    }

    None
}

fn main() {
    let matches = Command::new("tree-sitter-query-formatter")
        .version("0.1.0")
        .about("Format tree-sitter queries")
        .arg(
            Arg::new("input")
                .help("Input file or query string")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("tree")
                .long("tree")
                .help("Print the parse tree")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("width")
                .long("width")
                .short('w')
                .help("Output width")
                .value_parser(clap::value_parser!(usize))
                .default_value("80"),
        )
        .get_matches();

    let width = *matches.get_one::<usize>("width").unwrap();
    let show_tree = matches.get_flag("tree");

    let input = if let Some(input_arg) = matches.get_one::<String>("input") {
        if std::path::Path::new(input_arg).exists() {
            fs::read_to_string(input_arg).unwrap_or_else(|e| {
                eprintln!("Error reading file {}: {}", input_arg, e);
                std::process::exit(1);
            })
        } else {
            input_arg.clone()
        }
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap_or_else(|e| {
            eprintln!("Error reading from stdin: {}", e);
            std::process::exit(1);
        });
        buffer
    };

    if show_tree {
        if let Some(tree_output) = print_tree(&input) {
            println!("Parse tree:");
            println!("{}", tree_output);
            println!();
        }
    }

    if let Some(formatted) = format(&input, width) {
        println!("{}", formatted);
    } else {
        eprintln!("Failed to parse input");
        std::process::exit(1);
    }
}