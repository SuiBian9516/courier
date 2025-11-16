use crate::record::Record;

pub trait Transport: Send {
  fn write(&self, record: &Record);

  fn writeln(&self, record: &Record);
}
