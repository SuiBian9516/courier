use crate::linked_list::singly_linked_list::{IntoIter, Iter, IterMut, SinglyLinkedList};

pub struct Stack<T> {
  inner: SinglyLinkedList<T>,
}

impl<T> Stack<T> {
  pub fn new() -> Self {
    Self { inner: SinglyLinkedList::<T>::new() }
  }

  pub fn push(&mut self, value: T) {
    self.inner.push(value);
  }

  pub fn pop(&mut self) -> Option<T> {
    self.inner.pop()
  }

  pub fn peek(&self) -> Option<&T> {
    self.inner.peek()
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.inner.peek_mut()
  }

  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  pub fn count(&self) -> usize {
    self.inner.count()
  }

  pub fn clear(&mut self) {
    self.inner.clear()
  }
}

impl<T> IntoIterator for Stack<T> {
  type Item = T;
  type IntoIter = IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    self.inner.into_iter()
  }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
  type Item = &'a T;
  type IntoIter = Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    (&self.inner).into_iter()
  }
}

impl<'a, T> IntoIterator for &'a mut Stack<T> {
  type Item = &'a mut T;
  type IntoIter = IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    (&mut self.inner).into_iter()
  }
}

impl<T, const N: usize> From<[T; N]> for Stack<T> {
  fn from(value: [T; N]) -> Self {
    let mut stack = Self::new();
    for i in value {
      stack.push(i);
    }
    stack
  }
}

impl<T:PartialEq> PartialEq for Stack<T>{
  fn eq(&self, other: &Self) -> bool {
    self.inner == other.inner
  }
}

impl<T: Clone> Clone for Stack<T>{
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.clone()
    }
  }
}

impl<T> From<Vec<T>> for Stack<T> {
  fn from(value: Vec<T>) -> Self {
    let mut stack = Self::new();
    for i in value {
      stack.push(i);
    }
    stack
  }
}
