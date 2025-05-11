use std::{fmt::Debug, marker::PhantomData, ptr::null_mut};

pub struct SinglyLinkedListNode<T> {
  pub next: *mut SinglyLinkedListNode<T>,
  pub element: T,
}

impl<T> SinglyLinkedListNode<T> {
  pub fn new(next: *mut SinglyLinkedListNode<T>, element: T) -> Self {
    Self { next, element }
  }

  pub fn create_with_element(element: T) -> Self {
    Self { next: null_mut(), element }
  }
}

pub struct SinglyLinkedList<T> {
  head: *mut SinglyLinkedListNode<T>,
  length: usize,
}

impl<T> SinglyLinkedList<T> {
  pub fn new() -> Self {
    Self { head: null_mut(), length: 0 }
  }

  pub fn push(&mut self, element: T) {
    let mut node = Box::new(SinglyLinkedListNode::<T>::create_with_element(element));
    node.next = self.head;
    let ptr = Box::into_raw(node);
    self.head = ptr;
    self.length += 1;
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.length == 0 {
      None
    } else {
      let old_head = self.head;
      unsafe {
        let next = (*old_head).next;
        if next.is_null() {
          self.head = null_mut();
          self.length = 0;
        } else {
          self.head = next;
          self.length -= 1;
        }
        let node = Box::from_raw(old_head);
        Some(node.element)
      }
    }
  }

  pub fn count(&self) -> usize {
    self.length
  }

  pub fn is_empty(&self) -> bool {
    self.count() == 0
  }
  pub fn clear(&mut self) {
    let mut current = self.head;

    while !current.is_null() {
      unsafe {
        let next = (*current).next;
        let val = Box::from_raw(current);
        drop(val);
        current = next;
      }
    }

    self.head = null_mut();
    self.length = 0;
  }

  pub fn peek(&self) -> Option<&T> {
    if self.head.is_null() {
      None
    } else {
      unsafe { Some(&(*self.head).element) }
    }
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    if self.head.is_null() {
      None
    } else {
      unsafe { Some(&mut (*self.head).element) }
    }
  }
}

pub struct IntoIter<T> {
  inner: SinglyLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.inner.pop()
  }
}

pub struct Iter<'a, T> {
  position: *mut SinglyLinkedListNode<T>,
  _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.position.is_null() {
      None
    } else {
      unsafe {
        let current = &(*self.position).element;
        self.position = (*self.position).next;
        Some(current)
      }
    }
  }
}

pub struct IterMut<'a, T> {
  position: *mut SinglyLinkedListNode<T>,
  _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.position.is_null() {
      None
    } else {
      unsafe {
        let current = &mut (*self.position).element;
        self.position = (*self.position).next;
        Some(current)
      }
    }
  }
}

impl<T> IntoIterator for SinglyLinkedList<T> {
  type IntoIter = IntoIter<T>;
  type Item = T;

  fn into_iter(self) -> Self::IntoIter {
    IntoIter { inner: self }
  }
}

impl<'a, T> IntoIterator for &'a SinglyLinkedList<T> {
  type IntoIter = Iter<'a, T>;
  type Item = &'a T;

  fn into_iter(self) -> Self::IntoIter {
    Iter { position: self.head, _marker: PhantomData }
  }
}

impl<'a, T> IntoIterator for &'a mut SinglyLinkedList<T> {
  type IntoIter = IterMut<'a, T>;
  type Item = &'a mut T;

  fn into_iter(self) -> Self::IntoIter {
    IterMut { position: self.head, _marker: PhantomData }
  }
}

impl<T: Debug> Debug for SinglyLinkedList<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_list().entries(self.into_iter()).finish()
  }
}

impl<T, const N: usize> From<[T; N]> for SinglyLinkedList<T> {
  fn from(value: [T; N]) -> Self {
    let mut linked_list = Self::new();
    for i in value {
      linked_list.push(i);
    }
    linked_list
  }
}

impl<T> From<Vec<T>> for SinglyLinkedList<T> {
  fn from(value: Vec<T>) -> Self {
    let mut linked_list = Self::new();
    for i in value {
      linked_list.push(i);
    }
    linked_list
  }
}

impl<T: Clone> Clone for SinglyLinkedList<T> {
  fn clone(&self) -> Self {
    let mut new_list = Self::new();
    let mut current = self.head;
    while !current.is_null() {
      unsafe {
        new_list.push((*current).element.clone());
        current = (*current).next;
      }
    }
    new_list
  }
}

impl<T: PartialEq> PartialEq for SinglyLinkedList<T> {
  fn eq(&self, other: &Self) -> bool {
    if self.count() != other.count() {
      return false;
    }
    let mut current_self = self.head;
    let mut current_other = other.head;
    while !current_self.is_null() && !current_other.is_null() {
      unsafe {
        if (*current_self).element != (*current_other).element {
          return false;
        }
        current_self = (*current_self).next;
        current_other = (*current_other).next;
      }
    }
    true
  }
}
