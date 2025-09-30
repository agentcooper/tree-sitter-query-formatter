use tree_sitter_query_formatter::format;

wit_bindgen::generate!({
    world: "host",
});

struct MyHost;

impl Guest for MyHost {
    fn format(query: String) -> Option<String> {
        format(&query, 80)
    }
}

export!(MyHost);
