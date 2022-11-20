use std::collections::HashMap;
use crate::token::TokenType::{NumericLiteral, OperatorLiteral};
use crate::token::Token;

pub fn parser(lexed: Vec<Token>) -> Vec<Token> {
    let mut i = 0;

    let operator_prec: HashMap<&str, u32> = HashMap::from([
        ("+", 3),
        ("-", 3),
        ("*", 4),
        ("/", 4),
        ("%", 4),
        ("^", 4)
    ]);

    let mut operator: Vec<Token> = Vec::new();
    let mut output: Vec<Token> = Vec::new();

    while i < lexed.len() {
        if lexed[i].ttype == NumericLiteral {
            output.push(lexed[i].clone())
        }

        if lexed[i].ttype == OperatorLiteral {
            while operator.len() > 0 &&
                operator_prec[operator.last().unwrap().value.as_str()]
                    >= operator_prec[&*lexed[i].value.as_str()] {

                output.push(operator.pop().unwrap());
            }

            operator.push(lexed[i].clone());
        }

        i += 1;
    }

    while !operator.is_empty() {
        output.push(operator.pop().unwrap());
    }

    return output;
}