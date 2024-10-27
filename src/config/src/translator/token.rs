use super::{literals::Literals, token_type};

#[derive(Debug)]
pub struct Token {
  token_type: token_type::TokenType,
  literal: Literals,
  lexeme: Option<String>,
  line: i32,
}

impl Token {
  pub fn new(
    token_type: token_type::TokenType,
    literal: Literals,
    lexeme: Option<String>,
    line: i32,
  ) -> Self {
    Self {
      token_type,
      literal,
      lexeme,
      line,
    }
  }
}
