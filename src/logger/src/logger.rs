use std::sync::{Mutex, OnceLock};

use utils::queue::thread_safe_queue::ThreadSafeQueue;

use crate::{error::LoggerError, level::Level, loggable::Loggable, transport::Transport};

pub struct Logger {
  level: Level,
  transports: ThreadSafeQueue<Box<dyn Transport>>,
}

impl Logger {
  pub fn new<const N: usize>(level: Level, transports: [Box<dyn Transport>; N]) -> &'static Mutex<Self> {
    static INSTANCE: OnceLock<Mutex<Logger>> = OnceLock::new();

    INSTANCE.get_or_init(move || Mutex::new(Self { level, transports: ThreadSafeQueue::from(transports) }))
  }

  pub fn get_instance() -> &'static Mutex<Self> {
    Self::new(Level::INFO, [])
  }

  fn log(&self, level: Level, message: impl Loggable) -> Result<(), LoggerError> {
    if level <= self.level {
      for transport in &self.transports {
        let result = transport.write(level, &message.format().as_str());
        match result {
          Ok(_) => continue,
          Err(e) => return Err(e),
        }
      }
    }
    Ok(())
  }

  pub fn info(&self, message: impl Loggable) -> Result<(), LoggerError> {
    self.log(Level::INFO, message)
  }

  pub fn warn(&self, message: impl Loggable) -> Result<(), LoggerError> {
    self.log(Level::WARN, message)
  }

  pub fn error(&self, message: impl Loggable) -> Result<(), LoggerError> {
    self.log(Level::ERROR, message)
  }

  pub fn fatal(&self, message: impl Loggable) -> Result<(), LoggerError> {
    self.log(Level::FATAL, message)
  }

  pub fn debug(&self, message: impl Loggable) -> Result<(), LoggerError> {
    self.log(Level::DEBUG, message)
  }

  pub fn trace(&self, message: impl Loggable) -> Result<(), LoggerError> {
    self.log(Level::TRACE, message)
  }
}
