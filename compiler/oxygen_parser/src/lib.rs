use oxygen_error::Result;
use oxygen_lexer::tokenize;

pub fn parse(input: &str) -> Result<()> {
    for token in tokenize(input) {
        println!("{token:?}");
    }

    Ok(())
}
