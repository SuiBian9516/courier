use std::io::{IsTerminal, Write};

use chrono::DateTime;

use crate::{level::Level, record::Record, transport::Transport};

pub struct ConsoleTransport {
  level: Level,
}

impl Transport for ConsoleTransport {
  fn write(&self, record: &Record) {
    let mut stdout = std::io::stdout();
    if stdout.is_terminal() {
      if record.level > self.level {
        return;
      }
      let content = format!("[{}][{}][{}] {}", DateTime::from_timestamp(record.timestamp, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string(), record.level, record.get_namespace(), record.get_message());
      stdout.write(content.as_bytes()).unwrap();
      stdout.flush().unwrap();
    }
  }

  fn writeln(&self, record: &Record) {
    let mut stdout = std::io::stdout();
    if stdout.is_terminal() {
      if record.level > self.level {
        return;
      }
      let content = format!("{} [{}][{}] {}\n", DateTime::from_timestamp(record.timestamp, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string(), record.level, record.get_namespace(), record.get_message());
      stdout.write(content.as_bytes()).unwrap();
    }
  }
}

impl ConsoleTransport {
  pub fn new(level: Level) -> Self {
    Self { level }
  }
}
