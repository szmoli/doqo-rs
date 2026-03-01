use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("Error loading Rust grammar");

    let rust_source_code = "// this is a line comment\nfn test() {} // this is a line comment after test fn\n/* this\nis\na\nblock\ncomment.\n*/\n/// this is a doc comment.\n//! this is also a doc comment.";

    let tree = parser.parse(rust_source_code, None).unwrap();

    let query_str = "
        [
            (line_comment)
            (block_comment)
        ] @comment
    ";
    let query =
        Query::new(&tree_sitter_rust::LANGUAGE.into(), query_str).expect("Error compiling query.");
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), rust_source_code.as_bytes());

    while let Some(m) = matches.next() {
        println!("{:?}", m);

        for capture in m.captures {
            println!("{:?}", capture);
            let start = capture.node.start_byte();
            let end = capture.node.end_byte();
            let text = &rust_source_code[start..end];
            println!("{}", text);
        }

        println!();
    }
}
