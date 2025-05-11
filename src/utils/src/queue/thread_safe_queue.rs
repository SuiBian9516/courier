use crate::linked_list::linked_list::LinkedList;
use std::sync::{Arc, Mutex};

pub struct ThreadSafeQueue<T> {
    inner: Arc<Mutex<LinkedList<T>>>
}

impl<T> ThreadSafeQueue<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(LinkedList::<T>::new()))
        }
    }

    pub fn enqueue(&self, value: T) {
        let mut list = self.inner.lock().unwrap();
        list.push_back(value);
    }

    pub fn dequeue(&self) -> Option<T> {
        let mut list = self.inner.lock().unwrap();
        list.pop_front()
    }
    
    pub fn peek(&self) -> Option<T> 
    where 
        T: Clone
    {
        let list = self.inner.lock().unwrap();
        list.peek_front().cloned()
    }

    pub fn is_empty(&self) -> bool {
        let list = self.inner.lock().unwrap();
        list.is_empty()
    }

    pub fn count(&self) -> usize {
        let list = self.inner.lock().unwrap();
        list.count()
    }

    pub fn clear(&self) {
        let mut list = self.inner.lock().unwrap();
        list.clear()
    }
}

impl<T, const N: usize> From<[T;N]> for ThreadSafeQueue<T> {
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
        let list = self.inner.lock().unwrap();
        let cloned_list = list.clone();
        Self {
            inner: Arc::new(Mutex::new(cloned_list))
        }
    }
}

impl<T: PartialEq> PartialEq for ThreadSafeQueue<T> 
where
    T: Clone
{
    fn eq(&self, other: &Self) -> bool {
        let self_list = self.inner.lock().unwrap();
        let other_list = other.inner.lock().unwrap();
        *self_list == *other_list
    }
}

unsafe impl<T> Send for ThreadSafeQueue<T> {}
unsafe impl<T> Sync for ThreadSafeQueue<T> {}