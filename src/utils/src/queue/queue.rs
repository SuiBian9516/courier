use crate::linked_list::{linked_list, linked_list::LinkedList};

pub struct Queue<T>{
  inner: LinkedList<T>
}

impl<T> Queue<T> {
  pub fn new() -> Self {
    Self {
      inner: LinkedList::<T>::new()
    }
  }

  pub fn enqueue(&mut self, value: T) {
    self.inner.push_back(value);
  }

  pub fn dequeue(&mut self) -> Option<T> {
    self.inner.pop_front()
  }
  
  pub fn peek(&self) -> Option<&T> {
    self.inner.peek_front()
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.inner.peek_front_mut()
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

impl<T> IntoIterator for Queue<T> {
  type Item = T;
  type IntoIter = linked_list::IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    self.inner.into_iter()
  }
}

impl<'a, T> IntoIterator for &'a Queue<T> {
  type Item = &'a T;
  type IntoIter = linked_list::Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    (&self.inner).into_iter()
  }
}

impl<'a, T> IntoIterator for &'a mut Queue<T> {
  type Item = &'a mut T;
  type IntoIter = linked_list::IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    (&mut self.inner).into_iter()
  }
}

impl<T, const N: usize> From<[T;N]> for Queue<T> {
  fn from(array: [T; N]) -> Self {
    let mut queue = Queue::new();
    for item in array {
      queue.enqueue(item);
    }
    queue
  }
}

impl<T> From<Vec<T>> for Queue<T> {
  fn from(vec: Vec<T>) -> Self {
    let mut queue = Queue::new();
    for item in vec {
      queue.enqueue(item);
    }
    queue
  }
}

impl<T:Clone> Clone for Queue<T> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.clone()
    }
  }
}

impl<T:PartialEq> PartialEq for Queue<T> {
  fn eq(&self, other: &Self) -> bool {
    self.inner == other.inner
  }
}