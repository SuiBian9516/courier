use std::fmt::Debug;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct ThreadSafeSinglyLinkedListNode<T> {
  pub next: Option<NonNull<ThreadSafeSinglyLinkedListNode<T>>>,
  pub element: T,
}

impl<T> ThreadSafeSinglyLinkedListNode<T> {
  pub fn new(next: Option<NonNull<ThreadSafeSinglyLinkedListNode<T>>>, element: T) -> Self {
    Self { next, element }
  }

  pub fn create_with_element(element: T) -> Self {
    Self { next: None, element }
  }
}

struct LinkedListInner<T> {
  head: Option<NonNull<ThreadSafeSinglyLinkedListNode<T>>>,
  length: usize,
}

impl<T> LinkedListInner<T> {
  fn new() -> Self {
    Self { head: None, length: 0 }
  }
}

pub struct ThreadSafeSinglyLinkedList<T> {
  inner: Arc<Mutex<LinkedListInner<T>>>,
}

impl<T> ThreadSafeSinglyLinkedList<T> {
  pub fn new() -> Self {
    Self { inner: Arc::new(Mutex::new(LinkedListInner::new())) }
  }
  pub fn push(&self, element: T) {
    let mut inner = self.inner.lock().unwrap();

    let node = Box::new(ThreadSafeSinglyLinkedListNode::create_with_element(element));
    let node_ptr = NonNull::new(Box::into_raw(node)).unwrap();

    unsafe {
      (*node_ptr.as_ptr()).next = inner.head;
    }
    inner.head = Some(node_ptr);
    inner.length += 1;
  }
  pub fn pop(&self) -> Option<T> {
    let mut inner = self.inner.lock().unwrap();

    if inner.length == 0 {
      return None;
    }

    let old_head = inner.head.unwrap();

    unsafe {
      let next = (*old_head.as_ptr()).next;

      if next.is_none() {
        inner.head = None;
        inner.length = 0;
      } else {
        inner.head = next;
        inner.length -= 1;
      }

      let node = Box::from_raw(old_head.as_ptr());
      Some(node.element)
    }
  }

  pub fn count(&self) -> usize {
    let inner = self.inner.lock().unwrap();
    inner.length
  }

  pub fn is_empty(&self) -> bool {
    self.count() == 0
  }
  pub fn clear(&self) {
    let mut inner = self.inner.lock().unwrap();

    let mut current = inner.head;

    while let Some(ptr) = current {
      unsafe {
        let next = (*ptr.as_ptr()).next;
        let val = Box::from_raw(ptr.as_ptr());
        drop(val);
        current = next;
      }
    }

    inner.head = None;
    inner.length = 0;
  }

  fn lock_inner(&self) -> MutexGuard<LinkedListInner<T>> {
    self.inner.lock().unwrap()
  }

  pub fn clone(&self) -> Self {
    Self { inner: Arc::clone(&self.inner) }
  }
}

pub struct IntoIter<T> {
  list: ThreadSafeSinglyLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.list.pop()
  }
}

pub struct Iter<'a, T> {
  current: Option<NonNull<ThreadSafeSinglyLinkedListNode<T>>>,
  _guard: MutexGuard<'a, LinkedListInner<T>>,
}

impl<'a, T: Clone> Iterator for Iter<'a, T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(ptr) = self.current {
      unsafe {
        let current = &(*ptr.as_ptr());
        self.current = current.next;
        Some(current.element.clone())
      }
    } else {
      None
    }
  }
}

impl<T> IntoIterator for ThreadSafeSinglyLinkedList<T> {
  type IntoIter = IntoIter<T>;
  type Item = T;

  fn into_iter(self) -> Self::IntoIter {
    IntoIter { list: self }
  }
}

impl<'a, T: Clone> IntoIterator for &'a ThreadSafeSinglyLinkedList<T> {
  type IntoIter = Iter<'a, T>;
  type Item = T;

  fn into_iter(self) -> Self::IntoIter {
    let guard = self.lock_inner();
    let current = guard.head;

    Iter { current, _guard: guard }
  }
}

impl<T: Debug + Clone> Debug for ThreadSafeSinglyLinkedList<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let items: Vec<T> = self.into_iter().collect();
    f.debug_list().entries(items).finish()
  }
}

impl<T, const N: usize> From<[T; N]> for ThreadSafeSinglyLinkedList<T> {
  fn from(value: [T; N]) -> Self {
    let list = Self::new();
    for i in value {
      list.push(i);
    }
    list
  }
}

impl<T> From<Vec<T>> for ThreadSafeSinglyLinkedList<T> {
  fn from(value: Vec<T>) -> Self {
    let list = Self::new();
    for i in value {
      list.push(i);
    }
    list
  }
}

impl<T> Drop for ThreadSafeSinglyLinkedList<T> {
  fn drop(&mut self) {
    if let Ok(mut inner) = self.inner.lock() {
      let mut current = inner.head;

      while let Some(node_ptr) = current {
        unsafe {
          let next = (*node_ptr.as_ptr()).next;

          let _box = Box::from_raw(node_ptr.as_ptr());
          current = next;
        }
      }

      inner.head = None;
      inner.length = 0;
    }
  }
}

unsafe impl<T: Send> Send for ThreadSafeSinglyLinkedList<T> {}
unsafe impl<T: Sync> Sync for ThreadSafeSinglyLinkedList<T> {}
