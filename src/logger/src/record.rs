use bytes::Bytes;
use chrono::Local;

use crate::{level::Level, loggable::Loggable};

pub struct Record {
  pub level: Level,
  message: Bytes,
  namespace: Bytes,
  pub timestamp: i64,
}

impl Record {
  pub fn new(level: Level, message: impl Loggable, namespace: impl Loggable) -> Self {
    let now = Local::now();
    Self { level, message: message.format(), namespace: namespace.format(), timestamp: now.timestamp() }
  }

  pub fn get_message(&self) -> &str {
    unsafe { str::from_utf8_unchecked(&self.message) }
  }

  pub fn get_namespace(&self) -> &str {
    unsafe { str::from_utf8_unchecked(&self.namespace) }
  }
}
