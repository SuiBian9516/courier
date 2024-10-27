#[derive(Debug)]
pub enum TokenType {
  // Single-character tokens
  OpenBrace,
  CloseBrace,
  OpenParen,
  CloseParen,
  Pound,
  VariableDefiner,

  AsciiString,

  // Literals
  String,
  Number,
  Boolean,
  Null,

  // End Symbol
  EOF,
}
