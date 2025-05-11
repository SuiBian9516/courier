use std::fmt::Display;

pub trait Loggable {
  fn format(&self) -> String;
}

impl<T: Display> Loggable for T {
  fn format(&self) -> String {
    self.to_string()
  }
}
