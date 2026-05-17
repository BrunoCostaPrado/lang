use bcp_lexer::{tokenize, TokenType};

#[test]
fn test_basic_tokens() {
    let tokens = tokenize("let x i32 = 10").unwrap();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].value, "let");
    assert_eq!(tokens[1].value, "x");
    assert_eq!(tokens[2].value, "i32");
    assert_eq!(tokens[3].value, "=");
    assert_eq!(tokens[4].value, "10");
}

#[test]
fn test_newline_separator() {
    let tokens = tokenize("a\nb").unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[1].kind, TokenType::Newline);
    assert_eq!(tokens[3].kind, TokenType::Eof);
}
