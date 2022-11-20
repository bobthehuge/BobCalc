#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    NumericLiteral,
    OperatorLiteral,
}
#[derive(Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub value: String,
}

impl Token {
    pub fn print(&self) {
        println!("(Type: {:?}; Value: {})", self.ttype, self.value)
    }
}