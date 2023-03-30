use std::iter::Peekable;

use oxygen_error::Result;
use oxygen_lexer::{tokenize, Tokenizer, Token, TokenKind, Keyword, LiteralKind};
use oxygen_ast::*;

struct Parser<'src> {
    tokenizer: Peekable<Tokenizer<'src>>
}

trait ParseableToken {
    fn should_be_kind(&self, kind: TokenKind) -> Result<()>;
}

impl<'src> ParseableToken for Token<'src> {
    fn should_be_kind(&self, kind: TokenKind) -> Result<()> {
        if self.kind != kind {
            todo!("Handle error: 'Expected token kind {kind:?} but got {:?}'", self.kind);
        }

        Ok(())
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

    fn parse_expression(&mut self) -> Result<Expression> {
        let Some(next) = self.tokenizer.peek() else {
            todo!("Handle error: 'expected expression but no more tokens'");
        };

        match &next.kind {
            TokenKind::Identifier => {
                // Either ident on its own:
                //      x, y, x. etc
                //  or function call:
                //      x(), hfdjksf()
                
                let identifier = self.tokenizer.next().unwrap();
                let Some(next) = self.tokenizer.peek() else {
                    todo!("return an identifier on its own once we add variables as expressions");
                };

                match next.kind {
                    TokenKind::OpenParen => {
                        // Function call
                        self.tokenizer.next().unwrap();
                        self.get_next_token_or_error()?.should_be_kind(TokenKind::CloseParen)?;

                        Ok(
                            Expression::FunctionCall { 
                                name: identifier.string.to_string(), 
                                parameters: None
                            }
                        )
                    },
                    _ => {
                        todo!("return an identifier on its own once we add variables as expressions");
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

    fn parse_function_definition(&mut self) -> Result<Function> {
        // Starts with next token being the identifier
        let identifier = self.get_next_token_or_error()?;
        identifier.should_be_kind(TokenKind::Identifier)?;

        self.get_next_token_or_error()?.should_be_kind(TokenKind::OpenParen)?;
        // TODO: Handle function definition arguments
        self.get_next_token_or_error()?.should_be_kind(TokenKind::CloseParen)?;

        let mut function = Function {
            impure: false,
            name: identifier.string.to_string(),
            parameters: None,
            return_type: None,
            block: None
        };

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
                    Keyword::Impure => {
                        todo!("Handle keyword 'impure'");
                    },
                    Keyword::Func => {
                        Ok(TopLevelItem::Function(
                            self.parse_function_definition()?
                        ))
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
