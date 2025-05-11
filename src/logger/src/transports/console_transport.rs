use std::io::{stdout, Write};

use crate::transport::Transport;

pub struct ConsoleTransport;

impl Transport for ConsoleTransport {
  fn write(&self, level: crate::level::Level, message: &str) -> Result<(), crate::error::LoggerError> {
    match stdout().lock().write(format!("[{}] {}", level.to_string(), message).as_bytes()) {
      Ok(_) => Ok(()),
      Err(e) => Err(crate::error::LoggerError::SystemError(e)),
    }
  }
}

impl ConsoleTransport {
  pub fn new() -> Box<Self> {
    Box::new(Self)
  }
}
