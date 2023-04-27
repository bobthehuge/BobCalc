use std::collections::HashMap;
use crate::token::Token;
use crate::token::TokenType::{IntLiteral, OperatorLiteral};

pub fn calculator(tokens: Vec<Token>) -> f32{

    let mut i = 0;
    let mut output: Vec<Token> = Vec::new();

    let operator_args: HashMap<&str, u32> = HashMap::from([
        ("+", 2),
        ("-", 2),
        ("*", 2),
        ("/", 2),
        ("^", 2),
        ("%", 2)
    ]);

    while i < tokens.len() {
        if tokens[i].is_value_literal() {
            output.push(tokens[i].clone())
        }

        else if tokens[i].ttype == OperatorLiteral {
            let mut args_num = operator_args[&*tokens[i].value];
            let mut args: Vec<f32> = Vec::new();

            while args_num <= output.len() as u32 && args_num > 0 {
                args.push(output.pop().unwrap().value.parse().unwrap());
                args_num -= 1;
            }

            let value = operator(&tokens[i].value, args).unwrap();

            output.push(Token{ttype: IntLiteral, value: value.to_string()})
        }

        i += 1;
    }

    match output.pop() {
        None => 0.0,
        e => e.unwrap().value.parse::<f32>().unwrap()
    }
}

fn operator(func: &str, args: Vec<f32>) -> Result<f32, String> {
    match &*func {
        "+" => Ok(args[1] + args[0]),
        "-" => Ok(args[1] - args[0]),
        "*" => Ok(args[1] * args[0]),
        "/" => Ok(args[1] / args[0]),
        "^" => Ok(args[1].powi(args[0] as i32)),
        "%" => Ok(args[1] % args[0]),
        _ => Err(format!("unexpected operator '{func}'"))
    }
}