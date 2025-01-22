#[derive(Debug, Clone)]
pub struct Position(usize, usize);

impl Position {
  pub fn new(line: usize, column: usize) -> Self {
    Self(line, column)
  }

  pub fn create_from(object: &Position) -> Self {
    Self(object.0, object.1)
  }

  #[inline(always)]
  pub fn add_line_by(&mut self, count: usize) {
    self.0 += count;
  }

  #[inline(always)]
  pub fn set_line_to(&mut self, count: usize) {
    self.0 = count;
  }

  #[inline(always)]
  pub fn add_column_by(&mut self, count: usize) {
    self.1 += count;
  }

  #[inline(always)]
  pub fn set_column_to(&mut self, count: usize) {
    self.1 = count;
  }

  #[inline(always)]
  pub fn get_line(&self) -> usize {
    self.0
  }

  #[inline(always)]
  pub fn get_column(&self) -> usize {
    self.0
  }
}
