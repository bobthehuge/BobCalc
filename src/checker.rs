use crate::token::Token;

pub fn check_parser(tokens: &Vec<Token>) {
    if tokens.len() > 0 && !tokens[0].is_value_literal() {
        panic!("parsing_checker: Single non numericLiteral detected")
    }
    if tokens.len() != 1 && tokens.last().unwrap().is_value_literal() {
        panic!("parsing_checker: Not single numericLiteral detected")
    }
}
