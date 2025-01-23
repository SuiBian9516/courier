use std::fmt::Display;

#[derive(Debug)]
pub enum DeserializerError {
  UnexpectedValue(char, (usize, usize)),
  UnexpectedTermination((usize, usize)),
  InvalidNewLine((usize, usize)),
  NoSetsFound((usize, usize)),
  InvalidCommand(String, (usize, usize)),
}

impl Display for DeserializerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::UnexpectedValue(val, position) => write!(f, "Lexing: Unexpected value {}, occurred at line {}, column {}", val, position.0, position.1),
      Self::UnexpectedTermination(position) => write!(f, "Lexing: Unexpected termination occurred at line {}, column {}", position.0, position.1),
      Self::InvalidNewLine(position) => write!(f, "Lexing: Invalid new line occurred at line {}, column {}", position.0, position.1),
      Self::NoSetsFound(position) => write!(f, "Lexing: Undefined set involved at line {}, column {}", position.0, position.1),
      Self::InvalidCommand(name, position) => write!(f, "Lexing: Invalid command {} involved at line {}, column {}", name, position.0, position.1),
    }
  }
}
