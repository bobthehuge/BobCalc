mod token;
mod lexing;
mod parsing;
mod calculus;
mod checker;

pub fn compute(str: &str) -> f32 {
    let lexed = lexing::lexer(str);
    let parsed = parsing::parser(lexed);
    checker::check_parser(&parsed);
    let result = calculus::calculator(parsed);

    return result;
}