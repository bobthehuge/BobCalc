use crate::token::Token;
use crate::token::TokenType::{EmptyLiteral, IntLiteral, OperatorLiteral};

pub fn lexer(string: &str) -> Vec<Token> {
    let str: Vec<_> = string.chars().collect();

    let mut lexed: Vec<Token> = Vec::new();
    let mut i: usize = 0;

    while i < str.len() {
        if is_numeric(str[i]) {
            let (num, end) = num_handler(&str, i);
            lexed.push(Token {
                ttype: IntLiteral,
                value: num,
            });

            i = end - 1;
        }
        else if is_unary(str[i])
            && (last_ttype(&lexed) == OperatorLiteral) {
            let (num, end) = unary_handler(&str, i);
            lexed.push(Token {
                ttype: IntLiteral,
                value: num,
            });

            i = end - 1;
        }
        else if is_operator(str[i]) {
            lexed.push(Token {
                ttype: OperatorLiteral,
                value: String::from(str[i]),
            })
        }

        i += 1;
    }

    return lexed;
}

fn last_ttype(tokens: &Vec<Token>) -> crate::token::TokenType {
    match tokens.last() {
        None => EmptyLiteral,
        e => e.unwrap().ttype
    }
}

fn is_unary(chr: char) -> bool {
    return chr == '+' || chr == '-';
}
fn is_numeric(chr: char) -> bool {
    return chr.is_numeric();
}
fn is_operator(chr: char) -> bool {
    return "+-*/%^()".contains(chr);
}

fn num_handler(str: &[char], start: usize) -> (String, usize) {
    let mut i = start;
    let mut num = String::from(str[i]);

    i += 1;

    while i < str.len() && is_numeric(str[i]) {
        num.push(str[i]);
        i += 1;
    }

    return (num, i);
}

fn unary_handler(str: &[char], start: usize) -> (String, usize) {
    let mut i: usize = start;
    let mut num: String = String::new();
    let mut count: u32 = 0;

    while i < str.len() && !is_numeric(str[i]) {
        if str[i] == '-' {
            count += 1;
        }

        i += 1;
    }

    if count % 2 != 0 {
        num.push('-');
    }

    let (res, end) = num_handler(&str, i);

    num.push_str(&res);

    return (num, i + end);
}
