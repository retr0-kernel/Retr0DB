use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

#[derive(Debug)]
pub enum ParseError {
    Error(String),
}

pub fn parse(input: &str) -> Result<Vec<String>, ParseError> {
    let parser = parser::SelectParser::new();
    match parser.parse(input) {
        Ok(result) => Ok(result),
        Err(e) => Err(ParseError::Error(format!("Parse error: {:?}", e))),
    }
}