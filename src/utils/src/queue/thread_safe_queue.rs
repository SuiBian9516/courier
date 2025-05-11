use crate::linked_list::thread_safe_linked_list::{self, ThreadSafeLinkedList};

pub struct ThreadSafeQueue<T> {
  inner: ThreadSafeLinkedList<T>,
}

impl<T> ThreadSafeQueue<T> {
  pub fn new() -> Self {
    Self { inner: ThreadSafeLinkedList::new() }
  }
  pub fn enqueue(&self, value: T) {
    self.inner.push_back(value);
  }

  pub fn dequeue(&self) -> Option<T> {
    self.inner.pop_front()
  }

  pub fn peek(&self) -> Option<T>
  where
    T: Clone,
  {
    self.inner.peek_front().cloned()
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

impl<T> IntoIterator for ThreadSafeQueue<T> {
  type Item = T;
  type IntoIter = thread_safe_linked_list::IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    self.inner.into_iter()
  }
}

impl<'a, T> IntoIterator for &'a ThreadSafeQueue<T> {
  type Item = &'a T;
  type IntoIter = thread_safe_linked_list::Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    (&self.inner).into_iter()
  }
}

impl<'a, T> IntoIterator for &'a mut ThreadSafeQueue<T> {
  type Item = &'a mut T;
  type IntoIter = thread_safe_linked_list::IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    (&mut self.inner).into_iter()
  }
}

impl<T, const N: usize> From<[T; N]> for ThreadSafeQueue<T> {
  fn from(array: [T; N]) -> Self {
    let queue = ThreadSafeQueue::new();
    for item in array {
      queue.enqueue(item);
    }
    queue
  }
}

impl<T> From<Vec<T>> for ThreadSafeQueue<T> {
  fn from(vec: Vec<T>) -> Self {
    let queue = ThreadSafeQueue::new();
    for item in vec {
      queue.enqueue(item);
    }
    queue
  }
}

impl<T: Clone> Clone for ThreadSafeQueue<T> {
  fn clone(&self) -> Self {
    Self { inner: self.inner.clone() }
  }
}

impl<T: PartialEq> PartialEq for ThreadSafeQueue<T>
where
  T: Clone,
{
  fn eq(&self, other: &Self) -> bool {
    self.inner == other.inner
  }
}
