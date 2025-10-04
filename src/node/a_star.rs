use crate::float::F32;
use crate::node::fibonacci_heap::{FibonacciHeap, NodePtr};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub struct AStarHeap<T> {
    map: HashMap<T, NodePtr<AStarNode<T>>>,
    heap: FibonacciHeap<AStarNode<T>>,
}

impl<T: Copy + Eq + Hash> AStarHeap<T> {
    pub fn new() -> AStarHeap<T> {
        AStarHeap {
            map: HashMap::new(),
            heap: FibonacciHeap::new(),
        }
    }

    pub fn push(&mut self, node: T, weight: f32) {
        if !self.map.contains_key(&node) {
            self.map.insert(node, self.heap.push(AStarNode(node, weight.into())));
        } //
        else {
            self.heap.decrease_key(&self.map[&node], AStarNode(node, weight.into()));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|node| node.0)
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