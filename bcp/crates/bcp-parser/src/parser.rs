use bcp_lexer::{Token, TokenType};
use crate::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut stmts = Vec::new();
        while self.peek().kind != TokenType::Eof {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
            while self.peek().kind == TokenType::Newline {
                self.advance();
            }
        }
        Ok(Program { stmts })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.peek().kind {
            TokenType::Let | TokenType::Const | TokenType::Mut => self.parse_var_decl(),
            TokenType::Fn => self.parse_fn_decl(),
            TokenType::Return => self.parse_return(),
            TokenType::While => self.parse_while(),
            TokenType::For => self.parse_for(),
            TokenType::OpenBrace => {
                self.advance();
                let body = self.parse_block()?;
                Ok(Stmt::Block(body))
            }
            _ => {
                let expr = self.parse_expr(0)?;
                Ok(Stmt::Expression(expr))
            }
        }
    }

    fn parse_var_decl(&mut self) -> Result<Stmt, String> {
        let constant = self.peek().kind == TokenType::Const;
        let mutable = self.peek().kind == TokenType::Mut;
        self.advance();
        let name = self.expect(TokenType::Identifier, "variable name")?.value.clone();
        let type_annotation = if self.peek().kind == TokenType::Identifier {
            Some(self.parse_type()?)
        } else {
            None
        };
        let value = if self.peek().kind == TokenType::Equal {
            self.advance();
            Some(self.parse_expr(0)?)
        } else {
            None
        };
        Ok(Stmt::VarDeclaration(VarDecl {
            name,
            type_annotation,
            value,
            constant,
            mutable,
        }))
    }

    fn parse_fn_decl(&mut self) -> Result<Stmt, String> {
        self.advance(); // fn
        let name = self.expect(TokenType::Identifier, "function name")?.value.clone();
        self.expect(TokenType::OpenParen, "'(' after function name")?;
        let mut params = Vec::new();
        if self.peek().kind != TokenType::CloseParen {
            loop {
                let param_name = self.expect(TokenType::Identifier, "parameter name")?.value.clone();
                let param_type = self.parse_type()?;
                params.push((param_name, param_type));
                if self.peek().kind == TokenType::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(TokenType::CloseParen, "')' after parameters")?;
        let return_type = if self.peek().kind == TokenType::Arrow {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = if self.peek().kind == TokenType::Equal {
            self.advance();
            let expr = self.parse_expr(0)?;
            ExprOrBlock::Expr(expr)
        } else {
            self.expect(TokenType::OpenBrace, "'{' for function body")?;
            let body = self.parse_block()?;
            ExprOrBlock::Block(body)
        };

        Ok(Stmt::FnDeclaration(FnDecl {
            name,
            params,
            return_type,
            body: Box::new(body),
        }))
    }

    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.advance();
        if self.peek().kind == TokenType::Newline
            || self.peek().kind == TokenType::CloseBrace
            || self.peek().kind == TokenType::Eof
        {
            Ok(Stmt::Return(None))
        } else {
            let expr = self.parse_expr(0)?;
            Ok(Stmt::Return(Some(expr)))
        }
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.advance();
        let condition = self.parse_expr(0)?;
        self.expect(TokenType::OpenBrace, "'{' for while body")?;
        let body = self.parse_block()?;
        Ok(Stmt::While(condition, body))
    }

    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.advance();
        let var = self.expect(TokenType::Identifier, "loop variable")?.value.clone();
        // consume the 'in' keyword — we allow any identifier here and validate at a higher level
        let _in_token = self.expect(TokenType::Identifier, "'in' keyword")?.value.clone();
        let iter = Box::new(self.parse_expr(0)?);
        self.expect(TokenType::OpenBrace, "'{' for for body")?;
        let body = self.parse_block()?;
        Ok(Stmt::For { var, iter, body })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        while self.peek().kind != TokenType::CloseBrace && self.peek().kind != TokenType::Eof {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
            while self.peek().kind == TokenType::Newline {
                self.advance();
            }
        }
        self.expect(TokenType::CloseBrace, "'}' to close block")?;
        Ok(stmts)
    }

    fn parse_type(&mut self) -> Result<TypeAnnotation, String> {
        let name = self.expect(TokenType::Identifier, "type name")?.value.clone();
        let mut params = Vec::new();
        if self.peek().kind == TokenType::OpenBracket {
            self.advance();
            let param = self.parse_type()?;
            params.push(param);
            self.expect(TokenType::CloseBracket, "']'")?;
        }
        Ok(TypeAnnotation { name, params })
    }

    fn parse_expr(&mut self, min_prec: u8) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        loop {
            if self.peek().kind == TokenType::Eof {
                break;
            }
            let prec = self.peek_precedence();
            if prec < min_prec {
                break;
            }
            let op = self.advance();

            match op.kind {
                TokenType::Equal => {
                    let right = self.parse_expr(prec)?;
                    left = Expr::Assignment {
                        target: Box::new(left),
                        value: Box::new(right),
                    };
                }
                TokenType::Plus | TokenType::Minus | TokenType::Star
                | TokenType::Slash | TokenType::Percent
                | TokenType::EqualEqual | TokenType::BangEqual
                | TokenType::Less | TokenType::Greater
                | TokenType::LessEqual | TokenType::GreaterEqual
                | TokenType::And | TokenType::Or => {
                    let right = self.parse_expr(prec + 1)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        operator: token_to_binop(op.kind),
                        right: Box::new(right),
                    };
                }
                TokenType::OpenParen => {
                    let mut args = Vec::new();
                    if self.peek().kind != TokenType::CloseParen {
                        loop {
                            args.push(self.parse_expr(0)?);
                            if self.peek().kind == TokenType::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(TokenType::CloseParen, "')' after call args")?;
                    left = Expr::Call {
                        callee: Box::new(left),
                        args,
                    };
                }
                TokenType::Dot => {
                    let field = self.expect(TokenType::Identifier, "field name")?.value.clone();
                    left = Expr::FieldAccess {
                        object: Box::new(left),
                        field,
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let tok = self.advance();
        match tok.kind {
            TokenType::Number => {
                let val = tok.value.parse::<i64>().map_err(|e| format!("Invalid number: {}", e))?;
                Ok(Expr::Int(val))
            }
            TokenType::String => Ok(Expr::String(tok.value)),
            TokenType::True => Ok(Expr::Bool(true)),
            TokenType::False => Ok(Expr::Bool(false)),
            TokenType::Null => Ok(Expr::Null),
            TokenType::Identifier => {
                if self.peek().kind == TokenType::OpenBrace {
                    self.advance();
                    let mut fields = Vec::new();
                    if self.peek().kind != TokenType::CloseBrace {
                        loop {
                            let field_name = self.expect(TokenType::Identifier, "field name")?.value.clone();
                            self.expect(TokenType::Colon, "':' in struct field")?;
                            let field_value = self.parse_expr(0)?;
                            fields.push((field_name, field_value));
                            if self.peek().kind == TokenType::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(TokenType::CloseBrace, "'}' for struct literal")?;
                    Ok(Expr::StructLiteral {
                        name: tok.value,
                        fields,
                    })
                } else {
                    Ok(Expr::Identifier(tok.value))
                }
            }
            TokenType::Minus => {
                let expr = self.parse_expr(6)?;
                Ok(Expr::Unary {
                    operator: UnaryOp::Negate,
                    right: Box::new(expr),
                })
            }
            TokenType::Bang => {
                let expr = self.parse_expr(6)?;
                Ok(Expr::Unary {
                    operator: UnaryOp::Not,
                    right: Box::new(expr),
                })
            }
            TokenType::OpenParen => {
                let expr = self.parse_expr(0)?;
                self.expect(TokenType::CloseParen, "')' after expression")?;
                Ok(expr)
            }
            _ => Err(format!(
                "Unexpected token {:?} at line {}",
                tok.kind, tok.line
            )),
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, kind: TokenType, msg: &str) -> Result<Token, String> {
        if self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(format!(
                "Expected {} but got {:?} ('{}') at line {}",
                msg,
                self.peek().kind,
                self.peek().value,
                self.peek().line
            ))
        }
    }

    fn peek_precedence(&self) -> u8 {
        match self.peek().kind {
            TokenType::EqualEqual | TokenType::BangEqual
            | TokenType::Less | TokenType::Greater
            | TokenType::LessEqual | TokenType::GreaterEqual => 2,
            TokenType::And | TokenType::Or => 1,
            TokenType::Plus | TokenType::Minus => 3,
            TokenType::Star | TokenType::Slash | TokenType::Percent => 4,
            TokenType::Equal => 0,
            TokenType::OpenParen => 7,
            TokenType::Dot => 8,
            _ => 0,
        }
    }
}

fn token_to_binop(kind: TokenType) -> BinaryOp {
    match kind {
        TokenType::Plus => BinaryOp::Add,
        TokenType::Minus => BinaryOp::Sub,
        TokenType::Star => BinaryOp::Mul,
        TokenType::Slash => BinaryOp::Div,
        TokenType::Percent => BinaryOp::Mod,
        TokenType::EqualEqual => BinaryOp::Equal,
        TokenType::BangEqual => BinaryOp::NotEqual,
        TokenType::Less => BinaryOp::Less,
        TokenType::Greater => BinaryOp::Greater,
        TokenType::LessEqual => BinaryOp::LessEqual,
        TokenType::GreaterEqual => BinaryOp::GreaterEqual,
        TokenType::And => BinaryOp::And,
        TokenType::Or => BinaryOp::Or,
        _ => unreachable!(),
    }
}
