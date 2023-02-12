use std::collections::HashMap;

use super::*;
use pson_schema::pson_schemas;

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
    let mut scanner = PsonParser::new(text.chars());
    scanner.parse().unwrap();
    let expr = scanner.get().unwrap();
    assert_eq!(expr, Expr::Array(vec![
        Expr::Null(),
        Expr::Boolean(true),
        Expr::Boolean(false),
        Expr::Integer(1),
        Expr::Float(1.0),
        Expr::String("hello".to_string()),
        Expr::Array(vec![
            Expr::Integer(1),
            Expr::Integer(2),
            Expr::Integer(3),
        ]),
        Expr::Map(vec![
            ("a".to_string(), Expr::Integer(1)),
            ("b".to_string(), Expr::Integer(2)),
            ("c".to_string(), Expr::Integer(3)),
        ].into_iter().collect::<HashMap<String, Expr>>()),
        Expr::Array(vec![
            Expr::Integer(1),
            Expr::Array(vec![
                Expr::Integer(2),
                Expr::Array(vec![
                    Expr::Integer(3),
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
    let mut scanner = PsonParser::new(text.chars());
    scanner.parse().unwrap();
    let expr = scanner.get().unwrap();
    assert_eq!(expr, Expr::Array(vec![Expr::String(text)]));
}

#[test]
fn long_array_test(){
    let text: String = (0..100000).map(|_| "1 ").collect();
    let mut scanner = PsonParser::new(text.chars());
    scanner.parse().unwrap();
    let expr = scanner.get().unwrap();
    assert_eq!(expr, Expr::Array(vec![Expr::Integer(1); 100000]));
}

#[test]
fn long_map_test(){
    let mut text = String::with_capacity(111111 * 4);
    text.push('(');
    text.push_str((0..100000).map(|i| format!("a{} 1 ", i)).collect::<String>().as_str());
    text.push(')');
    let mut scanner = PsonParser::new(text.chars());
    scanner.parse().unwrap();
    let expr = scanner.get().unwrap();
    let mut map = HashMap::new();
    for i in 0..100000 {
        map.insert(format!("a{}", i), Expr::Integer(1));
    }
    assert_eq!(expr, Expr::Array(vec![Expr::Map(map)]));
}

#[test]
fn schema_test(){
    pson_schemas!{
        SizeDto [map (
            name string
            price float
        )]
        PizzaDto [map (
            name string
            sizes [array _SizeDto]
        )]
        PizzaPairDto [tuple [
            _PizzaDto
            _PizzaDto
        ]]
    }
    let _size = SizeDto{
        name: "S".to_string(),
        price: 6.99
    };
}
