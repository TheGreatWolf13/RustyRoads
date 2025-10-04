use std::cell::RefCell;
use std::rc::Rc;
use std::{mem, ptr};

pub struct FibonacciHeap<T> {
    n: usize,
    min: *mut Node<T>,
}

#[derive(Clone)]
pub struct NodePtr<T>(Rc<RefCell<NodePtrInternal<T>>>);

struct NodePtrInternal<T> {
    invalidated: bool,
    ptr: *mut Node<T>,
    heap_ptr: *const FibonacciHeap<T>,
}

struct Node<T> {
    key: T,
    left: *mut Node<T>,
    right: *mut Node<T>,
    parent: *mut Node<T>,
    child: *mut Node<T>,
    degree: usize,
    mark: bool,
    outside_ref: Rc<RefCell<NodePtrInternal<T>>>,
}

impl<T: Ord> FibonacciHeap<T> {
    pub const fn new() -> FibonacciHeap<T> {
        FibonacciHeap {
            n: 0,
            min: ptr::null_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.min.is_null()
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn push(&mut self, item: T) -> NodePtr<T> {
        let node = Box::into_raw(Box::new(Node {
            key: item,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            parent: ptr::null_mut(),
            child: ptr::null_mut(),
            degree: 0,
            mark: false,
            outside_ref: Rc::new(RefCell::new(NodePtrInternal {
                ptr: ptr::null_mut(),
                heap_ptr: ptr::addr_of!(*self),
                invalidated: false,
            })),
        }));
        unsafe {
            (*node).outside_ref.borrow_mut().ptr = node;
            if self.min.is_null() {
                (*node).left = node;
                (*node).right = node;
                self.min = node;
            } //
            else {
                Self::add_node_to_nonempty_circular_list(node, self.min);
                if (*node).key < (*self.min).key {
                    self.min = node;
                }
            }
            self.n += 1;
            NodePtr((*node).outside_ref.clone())
        }
    }

    pub fn decrease_key(&mut self, elem: &NodePtr<T>, new_key: T) {
        if elem.0.borrow().invalidated {
            return;
        }
        if &new_key > unsafe { &(*elem.0.borrow().ptr).key } {
            //Can only decrease, not increase
            return;
        }
        if !ptr::eq(elem.0.borrow().heap_ptr, ptr::addr_of!(*self)) {
            panic!("Element was never in this heap!");
        }
        unsafe {
            let node = elem.0.borrow().ptr;
            (*node).key = new_key;
            let parent = (*node).parent;
            if !parent.is_null() && (*node).key < (*parent).key {
                self.cut(node, parent);
                self.cascading_cut(parent);
            }
            if (*node).key < (*self.min).key {
                self.min = node;
            }
        }
    }

    unsafe fn cut(&mut self, node: *mut Node<T>, parent: *mut Node<T>) {
        (*parent).degree -= 1;
        Self::remove_from_circular_list(node);
        Self::add_node_to_nonempty_circular_list(node, self.min);
        (*node).parent = std::ptr::null_mut();
        (*node).mark = false;
    }

    unsafe fn cascading_cut(&mut self, node: *mut Node<T>) {
        let parent = (*node).parent;
        if !parent.is_null() {
            if !(*node).mark {
                (*node).mark = true;
            } else {
                self.cut(node, parent);
                self.cascading_cut(parent);
            }
        }
    }

    pub fn delete(&mut self, elem: NodePtr<T>) -> Option<T> {
        if elem.0.borrow().invalidated {
            return None;
        }
        if !ptr::eq(elem.0.borrow().heap_ptr, ptr::addr_of!(*self)) {
            panic!("Element was never in this heap!");
        }
        let node = elem.0.borrow().ptr;
        unsafe {
            let parent = (*node).parent;
            if !parent.is_null() {
                self.cut(node, parent);
                self.cascading_cut(node);
            }
        }
        self.min = node;
        self.pop()
    }

    pub fn pop(&mut self) -> Option<T> {
        let popped = self.min;
        if popped.is_null() {
            None
        } //
        else {
            unsafe {
                (*popped).outside_ref.borrow_mut().invalidated = true;
                let mut child = (*popped).child;
                if !child.is_null() {
                    while !(*child).parent.is_null() {
                        (*child).parent = ptr::null_mut();
                        child = (*child).right;
                    }
                }
                Self::concatenate_circular_lists(child, popped);
                if (*popped).right != popped {
                    Self::remove_from_circular_list(popped);
                    self.min = (*popped).right;
                    self.consolidate();
                } //
                else {
                    self.min = ptr::null_mut();
                }
            }
            self.n -= 1;
            unsafe {
                Some(Box::from_raw(popped).key)
            }
        }
    }

    unsafe fn consolidate(&mut self) {
        let mut arr: Vec<*mut Node<T>> = vec![ptr::null_mut(); (self.n as f64).log((1.0 + 5.0f64.sqrt()) / 2.0).floor() as usize + 1];
        let last = (*self.min).left;
        let mut node_it = last;
        let mut finish = false;
        while !finish {
            node_it = (*node_it).right;
            let mut x = node_it;
            if ptr::eq(x, last) {
                finish = true;
            }
            let mut d = (*x).degree;
            while !arr[d].is_null() {
                let mut y = arr[d];
                if (*x).key > (*y).key {
                    mem::swap(&mut x, &mut y);
                }
                if node_it == y {
                    node_it = (*node_it).left;
                }
                Self::remove_from_circular_list(y);
                (*x).degree += 1;
                if !(*x).child.is_null() {
                    Self::add_node_to_nonempty_circular_list(y, (*x).child);
                } //
                else {
                    (*y).left = y;
                    (*y).right = y;
                    (*x).child = y;
                }
                (*y).mark = false;
                (*y).parent = x;
                arr[d] = ptr::null_mut();
                d += 1;
            }
            arr[d] = x;
        }
        self.min = ptr::null_mut();
        let mut min: *mut Node<T> = ptr::null_mut();
        for node in arr {
            if !node.is_null() && (min.is_null() || (*node).key < (*min).key) {
                min = node;
            }
        }
        self.min = min;
    }

    unsafe fn add_node_to_nonempty_circular_list(new_item: *mut Node<T>, list: *mut Node<T>) {
        (*new_item).right = (*list).right;
        (*new_item).left = list;
        (*(*list).right).left = new_item;
        (*list).right = new_item;
    }

    unsafe fn concatenate_circular_lists(list1: *mut Node<T>, list2: *mut Node<T>) {
        if list1.is_null() || list2.is_null() {
            return;
        }
        let prev_list1_right = (*list1).right;
        let prev_list2_left = (*list2).left;
        (*list1).right = list2;
        (*list2).left = list1;
        (*prev_list1_right).left = prev_list2_left;
        (*prev_list2_left).right = prev_list1_right;
    }

    unsafe fn remove_from_circular_list(elem: *const Node<T>) {
        (*(*elem).right).left = (*elem).left;
        (*(*elem).left).right = (*elem).right;
    }
}

impl<T> Drop for FibonacciHeap<T> {
    fn drop(&mut self) {
        unsafe fn drop_recursive<T>(mut elem: *mut Node<T>) {
            if elem.is_null() {
                return;
            }
            (*(*elem).left).right = ptr::null_mut();
            loop {
                drop_recursive((*elem).child);
                if (*elem).right.is_null() {
                    let _ = Box::from_raw(elem);
                    break;
                }
                (*elem).outside_ref.borrow_mut().invalidated = true;
                elem = (*elem).right;
                let _ = Box::from_raw((*elem).left);
            }
        }
        unsafe {
            drop_recursive(self.min);
        }
    }
}

impl<T: Ord, const N: usize> From<[T; N]> for FibonacciHeap<T> {
    fn from(elems: [T; N]) -> Self {
        let mut heap = FibonacciHeap::new();
        for elem in elems {
            heap.push(elem);
        }
        heap
    }
}

impl<T: Ord> From<Vec<T>> for FibonacciHeap<T> {
    fn from(elems: Vec<T>) -> Self {
        let mut heap = FibonacciHeap::new();
        for elem in elems {
            heap.push(elem);
        }
        heap
    }
}

impl<T: Ord> From<FibonacciHeap<T>> for Vec<T> {
    fn from(mut heap: FibonacciHeap<T>) -> Vec<T> {
        let mut res = vec![];
        while let Some(popped) = heap.pop() {
            res.push(popped);
        }
        res
    }
}