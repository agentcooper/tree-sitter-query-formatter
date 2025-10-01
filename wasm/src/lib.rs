use tree_sitter_query_formatter::format;

wit_bindgen::generate!({
    world: "host",
});

struct MyHost;

impl Guest for MyHost {
    fn format(query: String) -> Result<String, String> {
        format(&query, 80).map_err(|e| e.to_string())
    }
}

export!(MyHost);
