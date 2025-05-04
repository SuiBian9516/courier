use std::{marker::PhantomData, ptr::null_mut};

pub struct LinkedListNode<T> {
  pub front: *mut LinkedListNode<T>,
  pub back: *mut LinkedListNode<T>,
  pub element: T,
}

impl<T> LinkedListNode<T> {
  pub fn new(front: *mut LinkedListNode<T>, back: *mut LinkedListNode<T>, element: T) -> Self {
    Self { front, back, element }
  }

  pub fn create_with_element(element: T) -> Self {
    Self { front: null_mut(), back: null_mut(), element }
  }
}

pub struct LinkedList<T> {
  head: *mut LinkedListNode<T>,
  tail: *mut LinkedListNode<T>,
  length: usize,
}

impl<T> LinkedList<T> {
  pub fn new() -> Self {
    Self { head: null_mut(), tail: null_mut(), length: 0 }
  }

  pub fn push_front(&mut self, element: T) {
    if self.length == 0 {
      let node = Box::new(LinkedListNode::<T>::create_with_element(element));
      let ptr = Box::into_raw(node);
      self.head = ptr;
      self.tail = ptr;
      self.length += 1;
    } else {
      let old_head = self.head;
      let mut node = Box::new(LinkedListNode::<T>::create_with_element(element));
      (*node).back = old_head;
      let ptr = Box::into_raw(node);
      unsafe {
        (*old_head).front = ptr;
      }
      self.head = ptr;
      self.length += 1;
    }
  }

  pub fn push_back(&mut self, element: T) {
    if self.length == 0 {
      let node = Box::new(LinkedListNode::<T>::create_with_element(element));
      let ptr = Box::into_raw(node);  // 将 Box 转换为原始指针
      self.head = ptr;
      self.tail = ptr;
      self.length += 1;
    } else {
      let old_tail = self.tail;
      let mut node = Box::new(LinkedListNode::<T>::create_with_element(element));
      (*node).front = old_tail;
      let ptr = Box::into_raw(node);
      unsafe {
        (*old_tail).back = ptr;
      }
      self.tail = ptr;
      self.length += 1;
    }
  }

  pub fn pop_front(&mut self) -> Option<T> {
    if self.length == 0 {
      None
    } else {
      let old_head = self.head;
      unsafe {
        let next = (*old_head).back;
        if next.is_null() {
          self.head = null_mut();
          self.tail = null_mut();
          self.length = 0;
          let node = Box::from_raw(old_head);
          Some(node.element)
        } else {
          (*next).front = null_mut();
          self.head = next;
          self.length -= 1;
          let node = Box::from_raw(old_head);
          Some(node.element)
        }
      }
    }
  }

  pub fn pop_back(&mut self) -> Option<T> {
    if self.length == 0 {
      None
    } else {
      let old_tail = self.tail;
      unsafe {
        let prev = (*old_tail).front;
        if prev.is_null() {
          self.head = null_mut();
          self.tail = null_mut();
          self.length = 0;
          let node = Box::from_raw(old_tail);
          Some(node.element)
        } else {
          (*prev).back = null_mut();
          self.tail = prev;
          self.length -= 1;
          let node = Box::from_raw(old_tail);
          Some(node.element)
        }
      }
    }
  }

  pub fn count(&self) -> usize {
    self.length
  }

  pub fn is_empty(&self) -> bool {
    self.count() == 0
  }
}

pub struct IntoIter<T>{
  inner: LinkedList<T>
}

impl<T> Iterator for IntoIter<T>{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.inner.pop_front()
  }
}

pub struct Iter<'a,T>{
  position: *mut LinkedListNode<T>,
  _marker: PhantomData<&'a T>
}

impl<'a,T> Iterator for Iter<'a,T>{
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.position.is_null(){
      None
    }else{
      unsafe {
        let current = &(*self.position).element;
        self.position = (*self.position).back;
        Some(current)
      }
    }
  }
}

pub struct IterMut<'a,T>{
  position: *mut LinkedListNode<T>,
  _marker: PhantomData<&'a T>
}

impl<'a,T> Iterator for IterMut<'a,T>{
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.position.is_null(){
      None
    }else{
      unsafe {
        let current = &mut (*self.position).element;
        self.position = (*self.position).back;
        Some(current)
      }
    }
  }
}

impl<T> IntoIterator for LinkedList<T>{
  type IntoIter = IntoIter<T>;
  type Item = T;

  fn into_iter(self) -> Self::IntoIter {
    IntoIter {
      inner: self
    }
  }
}

impl<'a, T> IntoIterator for &'a LinkedList<T>{
  type IntoIter = Iter<'a,T>;
  type Item = &'a T;

  fn into_iter(self) -> Self::IntoIter {
    Iter {
      position: self.head,
      _marker: PhantomData
    }
  }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T>{
  type IntoIter = IterMut<'a,T>;
  type Item = &'a mut T;

  fn into_iter(self) -> Self::IntoIter {
    IterMut {
      position: self.head,
      _marker: PhantomData
    }
  }
}