use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ScannerError {
  UnexpectedValue(String),
  IncompleteNumber,
  UnexpectedTermination,
  UnexpectedStringTermination,
  InvalidNullLiteral,
  InvalidTrueLiteral,
  InvalidFalseLiteral
}

impl Error for ScannerError {}

impl Display for ScannerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::UnexpectedValue(val) => write!(f, "Unexpected value: {}", val),
      Self::IncompleteNumber => write!(f, "Incomplete number"),
      Self::UnexpectedTermination => write!(f, "Unexpected Termination"),
      Self::UnexpectedStringTermination => write!(f, "Unexpected string termination"),
      Self::InvalidNullLiteral => write!(f, "Invalid null literal"),
      Self::InvalidTrueLiteral => write!(f, "Invalid true literal"),
      Self::InvalidFalseLiteral => write!(f, "Invalid false literal"),
    }
  }
}
