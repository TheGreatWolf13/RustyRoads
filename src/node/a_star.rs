use crate::float::F32;
use crate::node::fibonacci_heap::{Error, FibonacciHeap, Handle};
use rustc_hash::FxHashMap;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

pub struct AStarHeap<T: Eq> {
    map: FxHashMap<T, Handle<AStarNode<T>>>,
    heap: FibonacciHeap<AStarNode<T>>,
}

impl<T: Copy + Eq + Hash> AStarHeap<T> {
    pub fn new() -> AStarHeap<T> {
        AStarHeap {
            heap: FibonacciHeap::new(),
            map: FxHashMap::default(),
        }
    }

    pub fn push(&mut self, node: T, weight: f32) {
        if !self.map.contains_key(&node) {
            self.map.insert(node, self.heap.push(AStarNode(node, weight.into())));
        } //
        else {
            match self.heap.decrease_key(&self.map[&node], AStarNode(node, weight.into())) {
                Ok(handle) => {
                    self.map.insert(node, handle);
                }
                Err(e) => match e {
                    Error::KeyNotPresent => {
                        self.map.insert(node, self.heap.push(AStarNode(node, weight.into())));
                    }
                    Error::KeyNotValid => panic!("Should never happen!"),
                    Error::CannotIncreaseKey => (),
                }
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|a| {
            self.map.remove(&a.0);
            a.0
        })
    }
}

#[derive(Copy, Clone, Debug)]
struct AStarNode<T>(T, F32);

impl<T: Eq> Eq for AStarNode<T> {}

impl<T: Eq> PartialEq for AStarNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Hash + Eq> Hash for AStarNode<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T: Eq> Ord for AStarNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl<T: Eq> PartialOrd for AStarNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}