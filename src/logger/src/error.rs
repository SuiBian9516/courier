use std::io::Error;

#[derive(Debug)]
pub enum LoggerError {
  UnknownError,
  SystemError(Error),
}

impl std::fmt::Display for LoggerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::UnknownError => write!(f, "Unknown error"),
      Self::SystemError(e) => write!(f, "System error: {}", e),
    }
  }
}
