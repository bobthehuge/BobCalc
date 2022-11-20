use crate::token::Token;
use crate::token::TokenType::{ NumericLiteral, OperatorLiteral };

pub fn lexer(string: &str) -> Vec<Token> {
    let str: Vec<_> = string.chars().collect();

    let mut lexed: Vec<Token> = Vec::new();
    let mut i = 0;

    while i < str.len() {
        if is_numeric(str[i]) {
            let mut num = String::from(str[i]);
            i += 1;
            while i < str.len() && is_numeric(str[i]) {
                num.push(str[i]);
                i += 1;
            }
            lexed.push(Token{ttype: NumericLiteral, value: num});
            i -= 1;
        }

        else if is_operator(str[i]) {
            lexed.push(Token{ttype: OperatorLiteral, value: String::from(str[i])})
        }

        i += 1;
    }

    return lexed;
}

fn is_numeric(chr: char) -> bool { return chr.is_numeric(); }
fn is_operator(chr: char) -> bool { return "+-*/%^".contains(chr); }