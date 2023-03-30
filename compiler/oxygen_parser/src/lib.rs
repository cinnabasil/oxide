use std::iter::Peekable;

use oxygen_error::Result;
use oxygen_lexer::{tokenize, Tokenizer, Token, TokenKind, Keyword};
use oxygen_ast::{Ast, TopLevelItem};

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

    fn parse_function_definition(&mut self) -> Result<TopLevelItem> {
        // Starts with next token being the identifier
        let identifier = self.get_next_token_or_error()?;
        identifier.should_be_kind(TokenKind::Identifier)?;

        self.get_next_token_or_error()?.should_be_kind(TokenKind::OpenParen)?;
        // TODO: Handle function definition arguments
        self.get_next_token_or_error()?.should_be_kind(TokenKind::CloseParen)?;

        match self.tokenizer.peek().unwrap_or_else(|| {
            todo!("Handle error: 'function definition not followed by anything'"); 
        }).kind {
            TokenKind::Semicolon => {
                // This could be an undefined function
                // e.g.
                // func whatever(i32 a) ~ i32;
                // ^^ These work similar to C headers, in that they can be used
                // as placeholders when you haven't implemented a function.
                // They will always panic when called.
                self.tokenizer.next();
                return Ok(TopLevelItem::Function {
                    impure: false,
                    name: identifier.string.to_string(),
                    parameters: None,
                    return_type: None,
                    block: None
                });
            },
            TokenKind::OpenCurly => {
                todo!("Handle block expression");
            },
            _ => todo!()
        }
    }

    fn parse_item(&mut self, token: Token<'src>) -> Result<TopLevelItem> {
        match token.kind {
            TokenKind::Keyword(kw) => {
                match kw {
                    Keyword::Impure => {
                        todo!("Handle keyword 'impure'");
                    },
                    Keyword::Func => {
                        Ok(self.parse_function_definition()?)
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

    parser.parse_program()?;

    Ok(())
}
