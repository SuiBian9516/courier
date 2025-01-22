use std::fmt::Display;

#[derive(Debug)]
pub enum LexerError {
  UnexpectedValue(char, (usize, usize)),
  UnexpectedTermination((usize, usize)),
  InvalidNewLineInString((usize, usize)),
  NoSetsFound((usize,usize))
}

impl Display for LexerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::UnexpectedValue(val, position) => write!(f, "Unexpected value {}, occurred at line {}, column {}", val, position.0, position.1),
      Self::UnexpectedTermination(position) => write!(f, "Unexpected termination occurred at line {}, column {}", position.0, position.1),
      Self::InvalidNewLineInString(position) => write!(f, "INvalid new line occurred at line {}, column {}", position.0, position.1),
      Self::NoSetsFound(position) => write!(f, "Undefined set involved at line {}, column {}", position.0, position.1),
    }
  }
}
