use std::collections::HashMap;

use super::*;

#[test]
fn basic_test() {
    let text = r#"hello world"#;
    let mut scanner = PsonScanner::new(text.chars());
    scanner.scan().unwrap();
    let expr = scanner.get().unwrap();
    assert_eq!(expr, Expr::Array(vec![Expr::String("hello".to_string()), Expr::String("world".to_string())]));
}

#[test]
fn types_test() {
    let text = r#"N T F 1 1.0 "hello" [1 2 3] (a 1 b 2 c 3)"#;
    let mut scanner = PsonScanner::new(text.chars());
    scanner.scan().unwrap();
    let expr = scanner.get().unwrap();
    assert_eq!(expr, Expr::Array(vec![
        Expr::Null(),
        Expr::Boolean(true),
        Expr::Boolean(false),
        Expr::Number(1.0),
        Expr::Number(1.0),
        Expr::String("hello".to_string()),
        Expr::Array(vec![
            Expr::Number(1.0),
            Expr::Number(2.0),
            Expr::Number(3.0),
        ]),
        Expr::Map(vec![
            ("a".to_string(), Expr::Number(1.0)),
            ("b".to_string(), Expr::Number(2.0)),
            ("c".to_string(), Expr::Number(3.0)),
        ].into_iter().collect::<HashMap<String, Expr>>())
    ]));
}
