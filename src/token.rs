#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    EmptyLiteral,
    IntLiteral,
    FloatLiteral,
    OperatorLiteral,
}

#[derive(Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub value: String,
}

#[allow(dead_code)]
impl Token {
    pub fn print(&self) {
        println!("(Type: {:?}; Value: {})", self.ttype, self.value)
    }
    pub fn is_value_literal(&self) -> bool {
        self.ttype == TokenType::IntLiteral || self.ttype == TokenType::FloatLiteral
    }
}
