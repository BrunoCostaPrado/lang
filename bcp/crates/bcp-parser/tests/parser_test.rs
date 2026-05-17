use bcp_lexer::tokenize;
use bcp_parser::{Parser, Expr, Stmt, VarDecl, BinaryOp};

#[test]
fn test_parse_var_decl() {
    let source = "let x i32 = 10";
    let tokens = tokenize(source).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    assert_eq!(program.stmts.len(), 1);
    match &program.stmts[0] {
        Stmt::VarDeclaration(VarDecl { name, value, .. }) => {
            assert_eq!(name, "x");
            assert!(value.is_some());
            match value.as_ref().unwrap() {
                Expr::Int(v) => assert_eq!(*v, 10),
                _ => panic!("expected int literal"),
            }
        }
        _ => panic!("expected var declaration"),
    }
}

#[test]
fn test_parse_binary_expr() {
    let source = "1 + 2 * 3";
    let tokens = tokenize(source).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    match &program.stmts[0] {
        Stmt::Expression(Expr::Binary { left, operator, right }) => {
            assert_eq!(*operator, BinaryOp::Add);
            match left.as_ref() {
                Expr::Int(v) => assert_eq!(*v, 1),
                _ => panic!("expected int"),
            }
            match right.as_ref() {
                Expr::Binary { left: _, operator: op, right: _ } => {
                    assert_eq!(*op, BinaryOp::Mul);
                }
                _ => panic!("expected binary"),
            }
        }
        _ => panic!("expected expression"),
    }
}
