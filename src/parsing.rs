use crate::token::Token;
use crate::token::TokenType::{IntLiteral, FloatLiteral, OperatorLiteral};

pub fn parser(lexed: Vec<Token>) -> Vec<Token> {
    let mut i = 0;
    let mut operator: Vec<Token> = Vec::new();
    let mut output: Vec<Token> = Vec::new();

    while i < lexed.len() {
        match lexed[i].ttype {
            IntLiteral => output.push(lexed[i].clone()),
            FloatLiteral => output.push(lexed[i].clone()),
            OperatorLiteral => {
                while operator.len() > 0
                    && get_prec(operator.last().unwrap()) >= get_prec(&lexed[i].clone())
                    && !is_left_parr(operator.last().unwrap())
                {
                    output.push(operator.pop().unwrap());
                }

                operator.push(lexed[i].clone());
            }
            _ => break
        }

        i += 1;
    }

    while !operator.is_empty() {
        output.push(operator.pop().unwrap());
    }

    return output;
}

pub fn is_left_parr(token: &Token) -> bool {
    return token.value.as_str() == "(";
}

pub fn get_prec(token: &Token) -> u32 {
    return match token.value.as_str() {
        "+" => 3,
        "-" => 3,
        "*" => 4,
        "/" => 4,
        "^" => 4,
        "%" => 4,
        _ => unreachable!(),
    };
}
