use std::fmt::Debug;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct ThreadSafeLinkedListNode<T> {
  pub front: Option<NonNull<ThreadSafeLinkedListNode<T>>>,
  pub back: Option<NonNull<ThreadSafeLinkedListNode<T>>>,
  pub element: T,
}

impl<T> ThreadSafeLinkedListNode<T> {
  pub fn new(front: Option<NonNull<ThreadSafeLinkedListNode<T>>>, back: Option<NonNull<ThreadSafeLinkedListNode<T>>>, element: T) -> Self {
    Self { front, back, element }
  }

  pub fn create_with_element(element: T) -> Self {
    Self { front: None, back: None, element }
  }
}

struct LinkedListInner<T> {
  head: Option<NonNull<ThreadSafeLinkedListNode<T>>>,
  tail: Option<NonNull<ThreadSafeLinkedListNode<T>>>,
  length: usize,
}

impl<T> LinkedListInner<T> {
  fn new() -> Self {
    Self { head: None, tail: None, length: 0 }
  }
}

pub struct ThreadSafeLinkedList<T> {
  inner: Arc<Mutex<LinkedListInner<T>>>,
}

impl<T> ThreadSafeLinkedList<T> {
  pub fn new() -> Self {
    Self { inner: Arc::new(Mutex::new(LinkedListInner::new())) }
  }

  pub fn push_front(&self, element: T) {
    let mut inner = self.inner.lock().unwrap();

    let node = Box::new(ThreadSafeLinkedListNode::create_with_element(element));
    let node_ptr = NonNull::new(Box::into_raw(node)).unwrap();

    if inner.length == 0 {
      inner.head = Some(node_ptr);
      inner.tail = Some(node_ptr);
    } else {
      let old_head = inner.head.unwrap();

      unsafe {
        (*node_ptr.as_ptr()).back = Some(old_head);

        (*old_head.as_ptr()).front = Some(node_ptr);
      }

      inner.head = Some(node_ptr);
    }

    inner.length += 1;
  }

  pub fn push_back(&self, element: T) {
    let mut inner = self.inner.lock().unwrap();

    let node = Box::new(ThreadSafeLinkedListNode::create_with_element(element));
    let node_ptr = NonNull::new(Box::into_raw(node)).unwrap();

    if inner.length == 0 {
      inner.head = Some(node_ptr);
      inner.tail = Some(node_ptr);
    } else {
      let old_tail = inner.tail.unwrap();

      unsafe {
        (*node_ptr.as_ptr()).front = Some(old_tail);

        (*old_tail.as_ptr()).back = Some(node_ptr);
      }

      inner.tail = Some(node_ptr);
    }

    inner.length += 1;
  }

  pub fn pop_front(&self) -> Option<T> {
    let mut inner = self.inner.lock().unwrap();

    if inner.length == 0 {
      return None;
    }

    let old_head = inner.head.unwrap();

    unsafe {
      let next = (*old_head.as_ptr()).back;

      if next.is_none() {
        inner.head = None;
        inner.tail = None;
        inner.length = 0;
      } else {
        let next_ptr = next.unwrap();
        (*next_ptr.as_ptr()).front = None;

        inner.head = Some(next_ptr);
        inner.length -= 1;
      }

      let node = Box::from_raw(old_head.as_ptr());
      Some(node.element)
    }
  }

  pub fn pop_back(&self) -> Option<T> {
    let mut inner = self.inner.lock().unwrap();

    if inner.length == 0 {
      return None;
    }

    let old_tail = inner.tail.unwrap();

    unsafe {
      let prev = (*old_tail.as_ptr()).front;

      if prev.is_none() {
        inner.head = None;
        inner.tail = None;
        inner.length = 0;
      } else {
        let prev_ptr = prev.unwrap();
        (*prev_ptr.as_ptr()).back = None;

        inner.tail = Some(prev_ptr);
        inner.length -= 1;
      }

      let node = Box::from_raw(old_tail.as_ptr());
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
        let next = (*ptr.as_ptr()).back;
        let val = Box::from_raw(ptr.as_ptr());
        drop(val);
        current = next;
      }
    }

    inner.head = None;
    inner.tail = None;
    inner.length = 0;
  }

  fn lock_inner(&self) -> MutexGuard<LinkedListInner<T>> {
    self.inner.lock().unwrap()
  }

  pub fn peek_front(&self) -> Option<&T> {
    let inner = self.inner.lock().unwrap();
    if inner.length == 0 {
      None
    } else {
      unsafe { Some(&(*inner.head.unwrap().as_ptr()).element) }
    }
  }

  pub fn peek_back(&self) -> Option<&T> {
    let inner = self.inner.lock().unwrap();
    if inner.length == 0 {
      None
    } else {
      unsafe { Some(&(*inner.tail.unwrap().as_ptr()).element) }
    }
  }

  pub fn peek_front_mut(&self) -> Option<&mut T> {
    let inner = self.inner.lock().unwrap();
    if inner.length == 0 {
      None
    } else {
      unsafe { Some(&mut (*inner.head.unwrap().as_ptr()).element) }
    }
  }

  pub fn peek_back_mut(&self) -> Option<&mut T> {
    let inner = self.inner.lock().unwrap();
    if inner.length == 0 {
      None
    } else {
      unsafe { Some(&mut (*inner.tail.unwrap().as_ptr()).element) }
    }
  }
}

pub struct IntoIter<T> {
  list: ThreadSafeLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.list.pop_front()
  }
}

pub struct Iter<'a, T> {
  current: Option<NonNull<ThreadSafeLinkedListNode<T>>>,
  _guard: MutexGuard<'a, LinkedListInner<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(ptr) = self.current {
      unsafe {
        let current = &(*ptr.as_ptr());
        self.current = current.back;
        Some(&current.element)
      }
    } else {
      None
    }
  }
}

pub struct IterMut<'a, T> {
  current: Option<NonNull<ThreadSafeLinkedListNode<T>>>,
  _guard: MutexGuard<'a, LinkedListInner<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(ptr) = self.current {
      unsafe {
        let current = &mut (*ptr.as_ptr());
        self.current = current.back;
        Some(&mut current.element)
      }
    } else {
      None
    }
  }
}

impl<T> IntoIterator for ThreadSafeLinkedList<T> {
  type IntoIter = IntoIter<T>;
  type Item = T;

  fn into_iter(self) -> Self::IntoIter {
    IntoIter { list: self }
  }
}

impl<'a, T> IntoIterator for &'a ThreadSafeLinkedList<T> {
  type IntoIter = Iter<'a, T>;
  type Item = &'a T;

  fn into_iter(self) -> Self::IntoIter {
    let guard = self.lock_inner();
    let current = guard.head;

    Iter { current, _guard: guard }
  }
}

impl<'a, T> IntoIterator for &'a mut ThreadSafeLinkedList<T> {
  type IntoIter = IterMut<'a, T>;
  type Item = &'a mut T;

  fn into_iter(self) -> Self::IntoIter {
    let guard = self.lock_inner();
    let current = guard.head;

    IterMut { current, _guard: guard }
  }
}

impl<T: Debug> Debug for ThreadSafeLinkedList<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_list().entries(self.into_iter()).finish()
  }
}

impl<T, const N: usize> From<[T; N]> for ThreadSafeLinkedList<T> {
  fn from(value: [T; N]) -> Self {
    let list = Self::new();
    for i in value {
      list.push_back(i);
    }
    list
  }
}

impl<T> From<Vec<T>> for ThreadSafeLinkedList<T> {
  fn from(value: Vec<T>) -> Self {
    let list = Self::new();
    for i in value {
      list.push_back(i);
    }
    list
  }
}

impl<T> Drop for ThreadSafeLinkedList<T> {
  fn drop(&mut self) {
    if let Ok(mut inner) = self.inner.lock() {
      let mut current = inner.head;

      while let Some(node_ptr) = current {
        unsafe {
          let next = (*node_ptr.as_ptr()).back;

          let _box = Box::from_raw(node_ptr.as_ptr());
          current = next;
        }
      }

      inner.head = None;
      inner.tail = None;
      inner.length = 0;
    }
  }
}

impl<T> Clone for ThreadSafeLinkedList<T> {
  fn clone(&self) -> Self {
    Self { inner: Arc::clone(&self.inner) }
  }
}

impl<T: PartialEq> PartialEq for ThreadSafeLinkedList<T> {
  fn eq(&self, other: &Self) -> bool {
    if self.count() != other.count() {
      return false;
    }

    let mut self_iter = self.into_iter();
    let mut other_iter = other.into_iter();

    loop {
      match (self_iter.next(), other_iter.next()) {
        (Some(a), Some(b)) if a == b => {}
        (None, None) => break,
        _ => return false,
      }
    }

    true
  }
}

unsafe impl<T: Send> Send for ThreadSafeLinkedList<T> {}
unsafe impl<T: Sync> Sync for ThreadSafeLinkedList<T> {}
