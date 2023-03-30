use std::iter::Peekable;

use oxygen_error::Result;
use oxygen_lexer::{tokenize, Tokenizer, Token, TokenKind, Keyword, LiteralKind, BinaryOperation};
use oxygen_ast::*;

struct Parser<'src> {
    tokenizer: Peekable<Tokenizer<'src>>
}

trait ParseableToken {
    fn should_be_kind(&self, kind: TokenKind) -> Result<()>;
    fn get_precedence(&self) -> Precedence;
}

impl<'src> ParseableToken for Token<'src> {
    fn should_be_kind(&self, kind: TokenKind) -> Result<()> {
        if self.kind != kind {
            todo!("Handle error: 'Expected token kind {kind:?} but got {:?}'", self.kind);
        }

        Ok(())
    }

    fn get_precedence(&self) -> Precedence {
        match &self.kind {
            TokenKind::Eq | TokenKind::BinOpEq(_) => {
                Precedence::Assign
            },
            TokenKind::BinOp(b) => {
                match b {
                    BinaryOperation::Plus | BinaryOperation::Minus => Precedence::Sum,
                    BinaryOperation::Star | BinaryOperation::Slash => Precedence::Product,
                    BinaryOperation::And => Precedence::BitAnd,
                    BinaryOperation::Or => Precedence::BitOr
                }
            },
            TokenKind::EqEq | TokenKind::NotEq => Precedence::Equality,
            TokenKind::Greater | TokenKind::GreaterEq | TokenKind::Less | TokenKind::LessEq =>
                Precedence::Comparison,
            TokenKind::OpenParen | TokenKind::OpenBracket | TokenKind::OpenCurly =>
                Precedence::Call,
            _ => Precedence::None
        }
    }
}


impl<'src> Parser<'src> {
    fn new(input: &'src str) -> Self {
        Parser { 
            tokenizer: tokenize(input).peekable()
        }
    }

    fn get_next_token_or_error(&mut self) -> Result<Token<'src>> {
        let Some(token) = self.tokenizer.next() else {
            todo!("Handle error: 'error in get_next_token_or_error: expected token'");
        };

        Ok(token)
    }

    fn parse_call_params(&mut self) -> Result<CallParameters> {
        let mut params = CallParameters::new();
        
        'parse_params: loop {
            let expr = self.parse_expression()?;

            params.push(expr);

            let Some(next) = self.tokenizer.peek() else {
                // The end of the file is technically
                // not a comma, so we return params, since
                // it's not parse_call_params' job to 
                // do anything other than parse the parameters.
                return Ok(params);
            };

            match next.kind {
                TokenKind::Comma => {
                    self.tokenizer.next();
                },
                _ => break 'parse_params
            };
        };

        return Ok(params);
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        let Some(next) = self.tokenizer.peek() else {
            todo!("Handle error: 'expected expression but no more tokens'");
        };

        match &next.kind {
            TokenKind::BinOp(BinaryOperation::Minus) => {
                // Labelled as binary operation in tokenizer,
                // but actually they are used as unary operations here
                self.tokenizer.next();
                Ok(Expression::Unary { operator: UnaryOperator::Negate, right: Box::new(self.parse_expression()?) })
            },
            TokenKind::Bang => {
                self.tokenizer.next();
                Ok(Expression::Unary { operator: UnaryOperator::Not, right: Box::new(self.parse_expression()?) })
            },
            TokenKind::Identifier => {
                // Either ident on its own:
                //      x, y, x. etc
                //  or function call:
                //      x(), hfdjksf()
                
                let identifier = self.tokenizer.next().unwrap();
                let Some(next) = self.tokenizer.peek() else {
                    return Ok(
                        Expression::Ident(identifier.string.to_string())
                    );
                };

                match next.kind {
                    TokenKind::OpenParen => {
                        // Function call
                        self.tokenizer.next().unwrap();

                        let Some(close_paren_or_arg) = self.tokenizer.peek() else {
                            todo!("Handle error: 'expected close of function call or arguments but no more tokens'");
                        };

                        let mut parameters: Option<CallParameters> = None;

                        match close_paren_or_arg.kind {
                            TokenKind::CloseParen => {},
                            _ => {
                                parameters = Some(self.parse_call_params()?);
                            }
                        }

                        self.get_next_token_or_error()?.should_be_kind(TokenKind::CloseParen)?;

                        Ok(
                            Expression::FunctionCall { 
                                name: identifier.string.to_string(), 
                                parameters
                            }
                        )
                    },
                    _ => {
                        Ok(
                            Expression::Ident(identifier.string.to_string())
                        )
                    }
                } 
            },
            TokenKind::Literal { kind, suffix_start: _ } => {
                match kind {
                    LiteralKind::Str(terminated) => {
                        if !terminated {
                            todo!("Handle error: 'unterminated string literal'"); 
                        }

                        let literal_token = self.tokenizer.next().unwrap();

                        Ok(
                            Expression::Literal(LiteralType::String(literal_token.string.to_string()))
                        )
                    },
                    _ => todo!("Handle other literal kinds")
                } 
            },
            TokenKind::Keyword(k) => {
                match k {
                    Keyword::If => {
                        self.tokenizer.next();
                        let condition = Box::new(self.parse_expression()?);

                        let block = self.parse_block()?;

                        Ok(Expression::IfExpression { condition, block })
                    },
                    _ => todo!("Handle error: 'can't start expression with {k:?}'")
                }
            },
            _ => {
                todo!("Handle error: 'can't start expression with {:?}'", next.kind); 
            }
        } 
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        let expr = self.parse_expression()?;
        self.get_next_token_or_error()?.should_be_kind(TokenKind::Semicolon)?;
        Ok(Statement::Expression(expr))
    }

    fn parse_block(&mut self) -> Result<Block> {
        self.get_next_token_or_error()?.should_be_kind(TokenKind::OpenCurly)?;

        let mut block = Block::new();

        // Parse statements
        'parse_statements: loop {
            let Some(next_token) = self.tokenizer.peek() else {
                todo!("Handle error: 'expected statement or end of function def, but no more tokens'");
            };

            match next_token.kind {
                TokenKind::CloseCurly => break 'parse_statements,
                _ => {
                    block.push(self.parse_statement()?);
                }
            };
        }

        // Close curly already found inside parse_statements
        self.tokenizer.next();

        Ok(block)
    }

    #[allow(dead_code)]
    fn parse_return_type(&mut self) -> Result<ReturnType> {
        let r#type = self.parse_type()?;
        
        match self.tokenizer.peek() {
            Some(t) => {
                match t.kind {
                    TokenKind::Bang => {
                        self.tokenizer.next();
                        return Ok((r#type, true));
                    },
                    _ => return Ok((r#type, false))
                }
            },
            None => return Ok((r#type, false))
        }
    }

    fn parse_type(&mut self) -> Result<Type> {
        // Only handle compiler types for now, but
        // later on we want to be able to have user
        // defined types!
         
        match self.tokenizer.next() {
            Some(t) => {
                match &t.kind {
                    TokenKind::Keyword(k) => {
                        match k {
                            Keyword::I32 => Ok(Type::I32),
                            _ => todo!("Handle error: 'expected type but got {k:?}'")
                        }
                    },
                    k => {
                        todo!("Handle error: 'expected type but got {k:?}'");
                    }
                }
            },
            None => todo!("Handle error: 'expected type but no more tokens'")
        }
    }

    fn parse_function_parameters(&mut self) -> Result<FunctionParameters> {
        let mut params = FunctionParameters::new();
        
        'parse_params: loop {
            let r#type = self.parse_type()?; 

            let identifier = self.get_next_token_or_error()?;
            identifier.should_be_kind(TokenKind::Identifier)?;

            if let Some(v) = params.insert(identifier.string.to_string(), r#type) {
                todo!("Handle error: 'Tried to define the parameter {} again (was defined as {v:?})'",
                    identifier.string);
            }

            match self.tokenizer.peek() {
                Some(t) => {
                    match &t.kind {
                        TokenKind::Comma => {
                            self.tokenizer.next();
                        },
                        _ => break 'parse_params
                    }
                },
                None => break 'parse_params
            }
        };

        return Ok(params);
    }

    fn parse_function_definition(&mut self) -> Result<Function> {
        // Starts with next token being the identifier
        let identifier = self.get_next_token_or_error()?;
        identifier.should_be_kind(TokenKind::Identifier)?;

        self.get_next_token_or_error()?.should_be_kind(TokenKind::OpenParen)?;
        
        let mut parameters: Option<FunctionParameters> = None;

        match self.tokenizer.peek().unwrap_or_else(|| {
            todo!("Handle error: 'expected function def params or block, but no more tokens'");
        }).kind {
            TokenKind::CloseParen => {},
            _ => {
                parameters = Some(self.parse_function_parameters()?);
            }
        }

        self.get_next_token_or_error()?.should_be_kind(TokenKind::CloseParen)?;

        let mut function = Function {
            impure: false,
            name: identifier.string.to_string(),
            parameters,
            return_type: None,
            block: None
        };
        // We need to do this match twice:
        // 1. Handle a return type
        // 2. Handle either `;` or `{}`
        match self.tokenizer.peek().unwrap_or_else(|| {
            todo!("Handle error: 'function definition not followed by anything'"); 
        }).kind {
            TokenKind::Tilde => {
                self.tokenizer.next();
                let r#type = self.parse_return_type()?;
                function.return_type = Some(r#type);
            },
            _ => {}
        }

        match self.tokenizer.peek().unwrap_or_else(|| {
            todo!("Handle error: 'function definition not followed by anything'"); 
        }).kind {
            TokenKind::Semicolon => { self.tokenizer.next(); },
            TokenKind::OpenCurly => { function.block = Some(self.parse_block()?); },
            _ => todo!("Handle error: 'function definition followed by invalid token'")
        };

        Ok(function)
    }

    fn parse_item(&mut self, token: Token<'src>) -> Result<TopLevelItem> {
        match token.kind {
            TokenKind::Keyword(kw) => {
                match kw {
                    Keyword::Func => {
                        Ok(TopLevelItem::Function(
                            self.parse_function_definition()?
                        ))
                    }
                    _ => {
                        todo!("Handle keyword {kw:?}");
                    }
                }
            },
            _ => todo!("Handle error: 'Tried to start an item with an invalid token'") 
        }
    }

    fn parse_program(&mut self) -> Result<Ast> {
        let mut ast = Ast::new();

        while let Some(token) = self.tokenizer.next() {
            ast.push(self.parse_item(token)?); 
        }

        Ok(ast)
    }
}

pub fn parse(input: &str) -> Result<()> {
    let mut parser = Parser::new(input);

    let ast = parser.parse_program()?;

    println!("{ast:#?}");

    Ok(())
}
