use oxygen_error::Result;
use oxygen_lexer::{tokenize, Token, Tokenizer};
use oxygen_ast::Ast;

struct Parser<'src> {
    tokenizer: Tokenizer<'src>
}

impl<'src> Parser<'src> {
    fn new(input: &'src str) -> Self {
        Parser { 
            tokenizer: tokenize(input)
        }
    }

    fn parse_program(&mut self) -> Result<Ast> {
        todo!();
    }
}

pub fn parse(input: &str) -> Result<()> {
    let mut parser = Parser::new(input);

    parser.parse_program()?;

    Ok(())
}
