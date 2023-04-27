mod token;
mod lexing;
mod parsing;
mod calculus;
mod checker;

#[cfg_attr(feature = "db", derive(Debug))]
use rustc_lexer;

fn main() {
    println!("{}", compute("5"));

    let tokens: Vec<rustc_lexer::Token> = rustc_lexer::tokenize("5.42 && 4").collect();

    tokens.iter().for_each(|x| println!("{:?}", x.kind));

    println!("{:?}", tokens.first().unwrap().kind);
}

fn compute(str: &str) -> f32 {
        
    let lexed = lexing::lexer(str);
    let parsed = parsing::parser(lexed);

    checker::check_parser(&parsed);

    let result = calculus::calculator(parsed);

    return result;
}