use crate::node::a_star::AStarHeap;
use ggez::glam::Vec2;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::mem;
use std::mem::MaybeUninit;

mod a_star;
mod fibonacci_heap;

pub struct Node {
    id: NodeId,
    pos: Vec2,
    edges: Vec<EdgeId>,
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

    pub fn get_id(&self) -> NodeId {
        self.id
    }

    pub fn get_neighbours(&self, node_manager: &NodeManager, vec: &mut Vec<(NodeId, EdgeId)>) {
        vec.clear();
        for node_id in self.edges.iter().map(|edge_id| (node_manager.for_edge(*edge_id, |edge| edge.get_other_node(self.id)).unwrap(), *edge_id)) {
            vec.push(node_id);
        }
    }

    pub fn find_edge(&self, other_node: NodeId, node_manager: &NodeManager) -> Option<EdgeId> {
        for edge_id in &self.edges {
            if node_manager.for_edge(*edge_id, |edge| edge.get_other_node(self.id)).unwrap() == other_node {
                return Some(*edge_id);
            }
        }
        None
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NodeId(u64);

impl NodeId {
    pub fn from_raw(id: u64) -> Self {
        NodeId(id)
    }
}

pub struct NodeManager {
    nodes: RefCell<Inner<NodeId, Node>>,
    edges: RefCell<Inner<EdgeId, Edge>>,
    pub start_node: NodeId,
    pub end_node: NodeId,
}

struct Inner<I, N> {
    id_maker: u64,
    map: HashMap<I, N>,
}

impl NodeManager {
    pub fn new() -> Self {
        let mut manager = NodeManager {
            nodes: RefCell::new(Inner {
                map: HashMap::new(),
                id_maker: 0,
            }),
            edges: RefCell::new(Inner {
                map: HashMap::new(),
                id_maker: 0,
            }),
            start_node: NodeId(0),
            end_node: NodeId(0),
        };
        const RADIUS: i32 = 5;
        const LEN: usize = 2 * RADIUS as usize + 1;
        let mut ids = [[MaybeUninit::<NodeId>::uninit(); LEN]; LEN];
        for x in -RADIUS..=RADIUS {
            for y in -RADIUS..=RADIUS {
                let current_node = manager.add_node(Vec2::new(x as f32 * 100.0, y as f32 * 100.0));
                ids[(x + RADIUS) as usize][(y + RADIUS) as usize].write(current_node);
            }
        }
        let ids = unsafe {
            mem::transmute::<_, [[NodeId; LEN]; LEN]>(ids)
        };
        for x in -RADIUS..=RADIUS {
            let ids = ids[(x + RADIUS) as usize];
            let mut last_node = None;
            for node in ids {
                if let Some(last) = last_node {
                    manager.make_edge(last, node, if x == 0 { 2.0 } else { 1.0 });
                }
                last_node = Some(node);
            }
        }
        for y in -RADIUS..=RADIUS {
            let mut last_node = None;
            for x in -RADIUS..=RADIUS {
                let node = ids[(x + RADIUS) as usize][(y + RADIUS) as usize];
                if let Some(last) = last_node {
                    manager.make_edge(last, node, 1.0);
                }
                last_node = Some(node);
            }
        }
        manager.start_node = ids[0][0];
        manager.end_node = ids[(LEN - 1) / 2][LEN - 1];
        manager
    }

    pub fn get_node_pos(&self, id: NodeId) -> Option<Vec2> {
        self.for_node(id, |node| node.get_pos())
    }

    pub fn for_node_mut<T>(&self, id: NodeId, mut f: impl FnMut(&mut Node) -> T) -> Option<T> {
        if let Some(node) = self.nodes.borrow_mut().map.get_mut(&id) {
            return Some(f(node));
        }
        None
    }

    pub fn for_node<T>(&self, id: NodeId, mut f: impl FnMut(&Node) -> T) -> Option<T> {
        if let Some(node) = self.nodes.borrow().map.get(&id) {
            return Some(f(node));
        }
        None
    }

    pub fn for_edge<T>(&self, id: EdgeId, f: impl Fn(&Edge) -> T) -> Option<T> {
        if let Some(edge) = self.edges.borrow().map.get(&id) {
            return Some(f(edge));
        }
        None
    }

    pub fn add_node(&self, pos: Vec2) -> NodeId {
        let nodes = &self.nodes;
        let id = NodeId(nodes.borrow().id_maker);
        nodes.borrow_mut().id_maker += 1;
        nodes.borrow_mut().map.insert(id, Node {
            id,
            pos,
            edges: vec![],
        });
        id
    }

    pub fn make_edge(&self, node_a: NodeId, node_b: NodeId, speed: f32) -> EdgeId {
        let edges = &self.edges;
        let id = EdgeId(edges.borrow().id_maker);
        edges.borrow_mut().id_maker += 1;
        edges.borrow_mut().map.insert(id, Edge {
            nodes: (node_a, node_b),
            id,
            speed,
        });
        self.for_node_mut(node_a, |node| node.edges.push(id));
        self.for_node_mut(node_b, |node| node.edges.push(id));
        id
    }

    pub fn for_all_nodes(&self, mut f: impl FnMut(&Node)) {
        for (_, node) in &self.nodes.borrow().map {
            f(node);
        }
    }

    pub fn for_all_edges(&self, mut f: impl FnMut(&Edge)) {
        for (_, edge) in &self.edges.borrow().map {
            f(edge);
        }
    }

    pub fn a_star(&self, start: NodeId, goal: NodeId, h: fn(Vec2, Vec2) -> f32) -> (Option<Vec<EdgeId>>, Vec<EdgeId>) {
        let mut open_set = AStarHeap::new();
        let mut explored_paths = vec![];
        let goal_pos = self.get_node_pos(goal).unwrap();
        open_set.push(start, h(self.get_node_pos(start).unwrap(), goal_pos));
        let mut came_from = HashMap::<NodeId, NodeId>::new();
        let mut g_score = HashMap::new();
        g_score.insert(start, 0.0);
        let mut neighbours = vec![];
        while !open_set.is_empty() {
            let current = open_set.pop().unwrap();
            if current == goal {
                return (Some(self.reconstruct_path(came_from, goal)), explored_paths);
            }
            self.for_node(current, |node| node.get_neighbours(&self, &mut neighbours));
            for (neighbour, path) in &neighbours {
                explored_paths.push(*path);
                let tentative_g_score = g_score[&current] + self.get_node_pos(current).unwrap().distance(self.get_node_pos(*neighbour).unwrap()) /*/ self.for_edge(*path, |edge| edge.speed).unwrap()*/;
                if tentative_g_score < *g_score.get(&neighbour).unwrap_or(&f32::INFINITY) {
                    came_from.insert(*neighbour, current);
                    g_score.insert(*neighbour, tentative_g_score);
                    let f_score = tentative_g_score + h(self.get_node_pos(*neighbour).unwrap(), goal_pos);
                    open_set.push(*neighbour, f_score);
                }
            }
        }
        (None, explored_paths)
    }

    fn reconstruct_path(&self, came_from: HashMap<NodeId, NodeId>, goal: NodeId) -> Vec<EdgeId> {
        let mut vec = vec![];
        let mut last_node = goal;
        while let Some(next_node) = came_from.get(&last_node) {
            vec.push(self.for_node(last_node, |node| node.find_edge(*next_node, &self)).unwrap().unwrap());
            last_node = *next_node;
        }
        vec
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct EdgeId(u64);

pub struct Edge {
    id: EdgeId,
    nodes: (NodeId, NodeId),
    speed: f32,
}

impl Edge {
    pub fn get_id(&self) -> EdgeId {
        self.id
    }

    pub fn get_nodes(&self) -> (NodeId, NodeId) {
        self.nodes
    }

    pub fn get_other_node(&self, node: NodeId) -> NodeId {
        if self.nodes.0 == node {
            return self.nodes.1;
        }
        debug_assert_eq!(self.nodes.1, node, "This edge does not contain the given node!");
        self.nodes.0
    }
}