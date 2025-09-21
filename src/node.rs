use ggez::glam::Vec2;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Node {
    id: NodeId,
    pos: Vec2,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Node {
    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct NodeId(u64);

pub struct NodeManager {
    inner: RefCell<Inner>,
}

struct Inner {
    id_maker: u64,
    nodes: HashMap<NodeId, Node>,
}

impl NodeManager {
    pub fn new() -> Self {
        NodeManager {
            inner: RefCell::new(Inner {
                nodes: HashMap::new(),
                id_maker: 0,
            })
        }
    }

    pub fn add_node(&self, pos: Vec2) {
        let inner = &self.inner;
        let id = NodeId(inner.borrow().id_maker);
        inner.borrow_mut().id_maker += 1;
        inner.borrow_mut().nodes.insert(id, Node {
            id,
            pos,
        });
    }

    pub fn for_all_nodes(&self, mut f: impl FnMut(&Node)) {
        for (_, node) in &self.inner.borrow().nodes {
            f(node);
        }
    }
}