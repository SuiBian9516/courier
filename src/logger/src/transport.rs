use crate::{error::LoggerError, level::Level};

pub trait Transport: Send {
  fn write(&self, level: Level, message: &str) -> Result<(), LoggerError>;
}
