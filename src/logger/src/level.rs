#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Level {
  FATAL = 0,
  ERROR = 1,
  WARN = 2,
  INFO = 3,
  DEBUG = 4,
  TRACE = 5,
}

impl std::fmt::Display for Level {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::FATAL => write!(f, "FATAL"),
      Self::ERROR => write!(f, "ERROR"),
      Self::WARN => write!(f, "WARN"),
      Self::INFO => write!(f, "INFO"),
      Self::DEBUG => write!(f, "DEBUG"),
      Self::TRACE => write!(f, "TRACE"),
    }
  }
}
