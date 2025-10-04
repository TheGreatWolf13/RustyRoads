use crate::node::fibonacci_heap::Error::{CannotIncreaseKey, KeyNotPresent, KeyNotValid};
use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::rc::{Rc, Weak};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
struct HandleInner<T: Ord>(Rc<RefCell<Node<T>>>);

impl<T: Ord> HandleInner<T> {
    fn new(t: T) -> HandleInner<T> {
        HandleInner(Rc::new(RefCell::new(Node::new(t))))
    }

    fn to_extern(&self) -> Handle<T> {
        Handle(Rc::downgrade(&self.0))
    }
}

pub struct Handle<T: Ord>(Weak<RefCell<Node<T>>>);

struct Node<T> {
    t: T,
    valid: bool,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            t,
            valid: true,
        }
    }
}

impl<T: Ord> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.cmp(&other.t)
    }
}

impl<T: Ord> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Eq for Node<T> {}

impl<T: Ord> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

pub struct FibonacciHeap<T: Ord> {
    inner: BinaryHeap<Reverse<HandleInner<T>>>,
}

impl<T: Ord + Clone> FibonacciHeap<T> {
    pub fn new() -> FibonacciHeap<T> {
        FibonacciHeap {
            inner: BinaryHeap::new()
        }
    }

    #[must_use]
    pub fn push(&mut self, t: T) -> Handle<T> {
        let inner = HandleInner::new(t);
        let handle = inner.to_extern();
        self.inner.push(Reverse(inner));
        handle
    }

    pub fn pop(&mut self) -> Option<T> {
        while let Some(handle) = self.inner.pop() {
            if !handle.0.0.borrow().valid {
                continue;
            }
            return Some(handle.0.0.borrow().t.clone());
        }
        None
    }

    pub fn decrease_key(&mut self, handle: &Handle<T>, new_t: T) -> Result<Handle<T>, Error> {
        match handle.0.upgrade() {
            None => Err(KeyNotPresent),
            Some(rc) => {
                if !rc.borrow().valid {
                    return Err(KeyNotValid);
                }
                if rc.borrow().t <= new_t {
                    return Err(CannotIncreaseKey);
                }
                rc.borrow_mut().valid = false;
                Ok(self.push(new_t))
            }
        }
    }
}

pub enum Error {
    KeyNotPresent,
    KeyNotValid,
    CannotIncreaseKey,
}