use std::collections::HashMap;

use super::*;

#[test]
fn general_test() {
    let text = r#"
        N
        T
        F
        1
        1.0
        "hello"
        [1 2 3]
        (a 1 b 2 c 3)
        [1 [2 [3]]]
        (a (b (c N)))
    "#;
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
        ].into_iter().collect::<HashMap<String, Expr>>()),
        Expr::Array(vec![
            Expr::Number(1.0),
            Expr::Array(vec![
                Expr::Number(2.0),
                Expr::Array(vec![
                    Expr::Number(3.0),
                ]),
            ]),
        ]),
        Expr::Map(vec![
            ("a".to_string(), Expr::Map(vec![
                ("b".to_string(), Expr::Map(vec![
                    ("c".to_string(), Expr::Null()),
                ].into_iter().collect::<HashMap<String, Expr>>())),
            ].into_iter().collect::<HashMap<String, Expr>>())),
        ].into_iter().collect::<HashMap<String, Expr>>()),
    ]));
}

#[test]
fn long_string_test(){
    let text: String = (0..100000).map(|_| "a").collect();
    let mut scanner = PsonScanner::new(text.chars());
    scanner.scan().unwrap();
    let expr = scanner.get().unwrap();
    assert_eq!(expr, Expr::Array(vec![Expr::String(text)]));
}

#[test]
fn long_array_test(){
    let text: String = (0..100000).map(|_| "1 ").collect();
    let mut scanner = PsonScanner::new(text.chars());
    scanner.scan().unwrap();
    let expr = scanner.get().unwrap();
    assert_eq!(expr, Expr::Array(vec![Expr::Number(1.0); 100000]));
}
