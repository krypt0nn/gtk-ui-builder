use crate::parser::prelude::*;

#[test]
fn check_allowed_chars_tokenizing() {
    assert_eq!(Tokenizer::parse("hi;"), Ok(vec![
        Token::Other { begin: 0, end: 2, value: String::from("hi;") }
    ]));

    if let Err(TokenizeError::IncorrectChar { .. }) = Tokenizer::parse(";hi") {
        assert!(true);
    }
    
    else {
        assert!(false);
    }
}

#[test]
fn check_other_tokenizing() {
    assert_eq!(Tokenizer::parse("Hello World"), Ok(vec![
        Token::Other { begin: 0, end: 4, value: String::from("Hello") },
        Token::Other { begin: 6, end: 10, value: String::from("World") }
    ]));

    assert_eq!(Tokenizer::parse("   "), Ok(vec![]));
    assert_eq!(Tokenizer::parse("\n\r\t "), Ok(vec![]));

    assert_eq!(Tokenizer::parse("  Hi\r\t"), Ok(vec![
        Token::Other { begin: 2, end: 3, value: String::from("Hi") }
    ]));
}

#[test]
fn check_string_tokenizing() {
    assert_eq!(Tokenizer::parse("  Hi\r\t\n\"Hello, World!\"\n"), Ok(vec![
        Token::Other { begin: 2, end: 3, value: String::from("Hi") },
        Token::Text { begin: 7, end: 21, value: String::from("Hello, World!") }
    ]));
}

#[test]
fn check_brackets_tokenizing() {
    assert_eq!(Tokenizer::parse("()"), Ok(vec![
        Token::Parentheses { begin: 0, end: 1, tokens: vec![] }
    ]));

    assert_eq!(Tokenizer::parse("[]"), Ok(vec![
        Token::SquareBrackets { begin: 0, end: 1, tokens: vec![] }
    ]));

    assert_eq!(Tokenizer::parse("{}"), Ok(vec![
        Token::CurlyBrackets { begin: 0, end: 1, tokens: vec![] }
    ]));

    assert_eq!(Tokenizer::parse(" ( )"), Ok(vec![
        Token::Parentheses { begin: 1, end: 3, tokens: vec![] }
    ]));

    assert_eq!(Tokenizer::parse("[\t]"), Ok(vec![
        Token::SquareBrackets { begin: 0, end: 2, tokens: vec![] }
    ]));

    assert_eq!(Tokenizer::parse("{\n\t\n}\n"), Ok(vec![
        Token::CurlyBrackets { begin: 0, end: 4, tokens: vec![] }
    ]));

    assert_eq!(Tokenizer::parse("(a \"b\" c)"), Ok(vec![
        Token::Parentheses { begin: 0, end: 8, tokens: vec![
            Token::Other { begin: 1, end: 1, value: String::from("a") },
            Token::Text { begin: 3, end: 5, value: String::from("b") },
            Token::Other { begin: 7, end: 7, value: String::from("c") }
        ] }
    ]));

    assert_eq!(Tokenizer::parse(" ( []{a }) "), Ok(vec![
        Token::Parentheses { begin: 1, end: 9, tokens: vec![
            Token::SquareBrackets { begin: 3, end: 4, tokens: vec![] },
            Token::CurlyBrackets { begin: 5, end: 8, tokens: vec![
                Token::Other { begin: 6, end: 6, value: String::from("a") }
            ] }
        ] }
    ]));

    assert_eq!(Tokenizer::parse("({ [] })"), Ok(vec![
        Token::Parentheses { begin: 0, end: 7, tokens: vec![
            Token::CurlyBrackets { begin: 1, end: 6, tokens: vec![
                Token::SquareBrackets { begin: 3, end: 4, tokens: vec![] }
            ] }
        ] }
    ]));
}

#[test]
fn check_incorrect_char_error_tokenizing() {
    if let Err(TokenizeError::IncorrectChar { .. }) = Tokenizer::parse("Incorrect\"character") { assert!(true); } else { assert!(false); }
    if let Err(TokenizeError::IncorrectChar { .. }) = Tokenizer::parse("Hi!")                  { assert!(true); } else { assert!(false); }
    if let Err(TokenizeError::IncorrectChar { .. }) = Tokenizer::parse("Incorrect[")           { assert!(true); } else { assert!(false); }
    if let Err(TokenizeError::IncorrectChar { .. }) = Tokenizer::parse(")")                    { assert!(true); } else { assert!(false); }
}

#[test]
fn check_incorrect_string_error_tokenizing() {
    if let Err(TokenizeError::IncorrectString { .. }) = Tokenizer::parse("Hi \"Example string") { assert!(true); } else { assert!(false); }
    if let Err(TokenizeError::IncorrectString { .. }) = Tokenizer::parse("\"")                  { assert!(true); } else { assert!(false); }
}

#[test]
fn check_incorrect_brackets_error_tokenizing() {
    if let Err(TokenizeError::IncorrectBrackets { .. }) = Tokenizer::parse("Hi\r[(){]") { assert!(true); } else { assert!(false); }
    if let Err(TokenizeError::IncorrectBrackets { .. }) = Tokenizer::parse("[)")        { assert!(true); } else { assert!(false); }
}

#[test]
fn check_complex_tokenizing() {
    assert_eq!(Tokenizer::parse("use Gtk 4.0\nuse Adw 1\n\nAdw.ApplicationWindow window {\n\tdefault-width: 900\n\tdefault-height: 600\n}\n"), Ok(vec![
        Token::Other { begin: 0, end: 2, value: String::from("use") },
        Token::Other { begin: 4, end: 6, value: String::from("Gtk") },
        Token::Other { begin: 8, end: 10, value: String::from("4.0") },

        Token::Other { begin: 12, end: 14, value: String::from("use") },
        Token::Other { begin: 16, end: 18, value: String::from("Adw") },
        Token::Other { begin: 20, end: 20, value: String::from("1") },

        Token::Other { begin: 23, end: 43, value: String::from("Adw.ApplicationWindow") },
        Token::Other { begin: 45, end: 50, value: String::from("window") },
        Token::CurlyBrackets { begin: 52, end: 95, tokens: vec![
            Token::Other { begin: 55, end: 68, value: String::from("default-width:") },
            Token::Other { begin: 70, end: 72, value: String::from("900") },
            Token::Other { begin: 75, end: 89, value: String::from("default-height:") },
            Token::Other { begin: 91, end: 93, value: String::from("600") },
        ] },
    ]));
}
