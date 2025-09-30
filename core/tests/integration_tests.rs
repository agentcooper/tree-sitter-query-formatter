use tree_sitter_query_formatter::format;

macro_rules! format_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            let input = include_str!(concat!("fixtures/input/", stringify!($name), ".txt"));
            let expected = include_str!(concat!("fixtures/expected/", stringify!($name), ".txt"));
            let result = format(input.trim(), 80).unwrap();
            assert_eq!(result.trim(), expected.trim());
        }
    };
}

format_test!(alternation);
format_test!(anchor);
format_test!(anchor_end);
format_test!(anchor_end_node);
format_test!(anonymous);
format_test!(capture);
format_test!(comment);
format_test!(directive);
format_test!(error);
format_test!(fields);
format_test!(grouping);
format_test!(grouping_long);
format_test!(grouping_quantification);
format_test!(missing);
format_test!(negated);
format_test!(nested);
format_test!(predicate);
format_test!(predicate_any_of);
format_test!(predicate_end);
format_test!(predicate_long);
format_test!(quantification);
format_test!(quantification_question_mark);
format_test!(simple);
format_test!(supertype);
format_test!(wildcard);
