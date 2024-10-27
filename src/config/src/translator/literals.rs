#[derive(Debug)]
pub enum Literals {
  None,
  Number(f64),
  String(String),
  Null,
  Boolean(bool),

  AsciiString(String)
}
