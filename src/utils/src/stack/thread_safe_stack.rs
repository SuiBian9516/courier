use crate::linked_list::thread_safe_singly_linked_list::{IntoIter, Iter, IterMut, ThreadSafeSinglyLinkedList};

pub struct ThreadSafeStack<T> {
  inner: ThreadSafeSinglyLinkedList<T>
}

impl<T> ThreadSafeStack<T> {
  pub fn new() -> Self {
    Self {
      inner: ThreadSafeSinglyLinkedList::<T>::new()
    }
  }

  pub fn push(&self, value: T) {
    self.inner.push(value);
  }

  pub fn pop(&self) -> Option<T> {
    self.inner.pop()
  }

  pub fn peek(&self) -> Option<&T> {
    self.inner.peek()
  }

  pub fn peek_mut(&self) -> Option<&mut T> {
    self.inner.peek_mut()
  }

  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  pub fn count(&self) -> usize {
    self.inner.count()
  }

  pub fn clear(&self) {
    self.inner.clear()
  }
}

impl<T> IntoIterator for ThreadSafeStack<T> {
  type Item = T;
  type IntoIter = IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    self.inner.into_iter()
  }
}

impl<'a, T> IntoIterator for &'a ThreadSafeStack<T> {
  type Item = &'a T;
  type IntoIter = Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    (&self.inner).into_iter()
  }
}

impl<'a, T> IntoIterator for &'a mut ThreadSafeStack<T> {
  type Item = &'a mut T;
  type IntoIter = IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    (&mut self.inner).into_iter()
  }
}

impl<T> Clone for ThreadSafeStack<T> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.clone()
    }
  }
}

unsafe impl<T: Send> Send for ThreadSafeStack<T> {}
unsafe impl<T: Sync> Sync for ThreadSafeStack<T> {}
