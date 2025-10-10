use crate::node::a_star::AStarHeap;
use crate::CITY_WIDTH;
use ggez::glam::{IVec2, Vec2};
use std::collections::HashMap;
use std::hash::Hash;
use std::mem;
use std::mem::MaybeUninit;
use crate::math::Sqr;

mod a_star;
mod fibonacci_heap;

const CHUNK_SIZE: f32 = 100.0;
const MAX_POS_COMP: i32 = ((CITY_WIDTH / 2.0) / CHUNK_SIZE) as i32 - 1;
const MIN_POS_COMP: i32 = ((-CITY_WIDTH / 2.0) / CHUNK_SIZE) as i32;
const MIN_POS: IVec2 = IVec2::splat(MIN_POS_COMP);
const MAX_POS: IVec2 = IVec2::splat(MAX_POS_COMP);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct ChunkPos(IVec2);

impl From<IVec2> for ChunkPos {
    fn from(pos: IVec2) -> Self {
        ChunkPos(pos)
    }
}

impl ChunkPos {
    const fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }

    fn from_world_pos(world_pos: Vec2) -> Self {
        let pos = (world_pos / CHUNK_SIZE).floor();
        pos.as_ivec2().clamp(MIN_POS, MAX_POS).into()
    }

    fn with_offset(&self, offset: IVec2) -> Self {
        Self(self.0 + offset)
    }

    fn get_area(world_pos: Vec2) -> ChunkPosArea {
        let pos = world_pos / CHUNK_SIZE;
        let max = pos.ceil();
        let min = pos.floor();
        let max_diff = max - pos;
        let min_diff = pos - min;
        let main_pos = min.as_ivec2().clamp(MIN_POS, MAX_POS);
        let mut area = ChunkPosArea::new(main_pos.into());
        if max_diff.x < min_diff.x {
            if main_pos.x < MAX_POS_COMP {
                area = area.expand(IVec2::X);
            }
        } //
        else {
            if main_pos.x > MIN_POS_COMP {
                area = area.expand(IVec2::NEG_X);
            }
        }
        if max_diff.y < min_diff.y {
            if main_pos.y < MAX_POS_COMP {
                area = area.expand(IVec2::Y);
            }
        } //
        else {
            if main_pos.y > MIN_POS_COMP {
                area = area.expand(IVec2::NEG_Y);
            }
        }
        area
    }
}

#[derive(Copy, Clone)]
enum ChunkPosArea {
    One(ChunkPos),
    Two(ChunkPos, ChunkPos),
    Four(ChunkPos, ChunkPos, ChunkPos, ChunkPos),
}

impl ChunkPosArea {
    fn new(pos: ChunkPos) -> Self {
        ChunkPosArea::One(pos)
    }

    fn expand(self, offset: IVec2) -> Self {
        match self {
            ChunkPosArea::One(pos) => ChunkPosArea::Two(pos, pos.with_offset(offset)),
            ChunkPosArea::Two(a, b) => ChunkPosArea::Four(a, b, a.with_offset(offset), b.with_offset(offset)),
            ChunkPosArea::Four(_, _, _, _) => panic!("Already fully defined!")
        }
    }

    fn into_iter(self) -> impl Iterator<Item=ChunkPos> {
        ChunkPosAreaIterator::new(self)
    }
}

#[derive(Copy, Clone)]
enum ChunkPosAreaIterator {
    Zero,
    One(ChunkPos),
    Two(ChunkPos, ChunkPos),
    Three(ChunkPos, ChunkPos, ChunkPos),
    Four(ChunkPos, ChunkPos, ChunkPos, ChunkPos),
}

impl ChunkPosAreaIterator {
    fn new(area: ChunkPosArea) -> Self {
        match area {
            ChunkPosArea::One(a) => Self::One(a),
            ChunkPosArea::Two(a, b) => Self::Two(a, b),
            ChunkPosArea::Four(a, b, c, d) => Self::Four(a, b, c, d),
        }
    }
}

impl Iterator for ChunkPosAreaIterator {
    type Item = ChunkPos;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Zero => None,
            Self::One(pos) => {
                let ret = Some(*pos);
                *self = Self::Zero;
                ret
            }
            Self::Two(a, b) => {
                let ret = Some(*b);
                *self = Self::One(*a);
                ret
            }
            Self::Three(a, b, c) => {
                let ret = Some(*c);
                *self = Self::Two(*a, *b);
                ret
            }
            Self::Four(a, b, c, d) => {
                let ret = Some(*d);
                *self = Self::Three(*a, *b, *c);
                ret
            }
        }
    }
}

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
        for node_id in self.edges.iter().map(|edge_id| (node_manager.get_edge(*edge_id).unwrap().get_other_node(self.id), *edge_id)) {
            vec.push(node_id);
        }
    }

    pub fn find_edge(&self, other_node: NodeId, node_manager: &NodeManager) -> Option<EdgeId> {
        for edge_id in &self.edges {
            if node_manager.get_edge(*edge_id).unwrap().get_other_node(self.id) == other_node {
                return Some(*edge_id);
            }
        }
        None
    }

    #[inline(always)]
    pub const fn radius() -> f32 {
        10.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NodeId(u64);

impl FromRawId for NodeId {
    fn from_raw(id: u64) -> Self {
        NodeId(id)
    }
}

pub struct NodeManager {
    nodes: Inner<NodeId, Node>,
    edges: Inner<EdgeId, Edge>,
    node_lookup: HashMap<ChunkPos, Vec<NodeId>>,
    pub start_node: NodeId,
    pub end_node: NodeId,
}

trait FromRawId {
    fn from_raw(id: u64) -> Self;
}

struct Inner<I: FromRawId, N> {
    id_maker: u64,
    map: HashMap<I, N>,
}

impl<I: FromRawId, N> Inner<I, N> {
    fn get_id(&mut self) -> I {
        let id = I::from_raw(self.id_maker);
        self.id_maker += 1;
        id
    }
}

impl NodeManager {
    pub fn new() -> Self {
        let mut manager = NodeManager {
            nodes: Inner {
                map: HashMap::new(),
                id_maker: 0,
            },
            edges: Inner {
                map: HashMap::new(),
                id_maker: 0,
            },
            node_lookup: HashMap::new(),
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

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.map.get(&id)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.map.get_mut(&id)
    }

    pub fn get_node_pos(&self, id: NodeId) -> Option<Vec2> {
        self.get_node(id).map(|node| node.pos)
    }

    pub fn get_edge(&self, id: EdgeId) -> Option<&Edge> {
        self.edges.map.get(&id)
    }

    pub fn add_node(&mut self, pos: Vec2) -> NodeId {
        let id = self.nodes.get_id();
        self.nodes.map.insert(id, Node {
            id,
            pos,
            edges: vec![],
        });
        id
    }

    pub fn make_edge(&mut self, node_a: NodeId, node_b: NodeId, speed: f32) -> EdgeId {
        let id = self.edges.get_id();
        self.edges.map.insert(id, Edge {
            nodes: (node_a, node_b),
            id,
            speed,
        });
        self.get_node_mut(node_a).unwrap().edges.push(id);
        self.get_node_mut(node_b).unwrap().edges.push(id);
        id
    }

    pub fn for_all_nodes(&self, mut f: impl FnMut(&Node)) {
        for (_, node) in &self.nodes.map {
            f(node);
        }
    }

    pub fn for_all_edges(&self, mut f: impl FnMut(&Edge)) {
        for (_, edge) in &self.edges.map {
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
        while let Some(current) = open_set.pop() {
            if current == goal {
                return (Some(self.reconstruct_path(came_from, goal)), explored_paths);
            }
            self.get_node(current).map(|node| node.get_neighbours(&self, &mut neighbours));
            for (neighbour, path) in &neighbours {
                explored_paths.push(*path);
                let tentative_g_score = g_score[&current] + self.get_node_pos(current).unwrap().distance(self.get_node_pos(*neighbour).unwrap()) / self.get_edge(*path).unwrap().speed;
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
            vec.push(self.get_node(last_node).unwrap().find_edge(*next_node, &self).unwrap());
            last_node = *next_node;
        }
        vec
    }

    pub fn try_node_collision(&self, pos: Vec2) -> Option<NodeId> {
        for chunk_pos in ChunkPos::get_area(pos).into_iter() {
            if let Some(vec) = self.node_lookup.get(&chunk_pos) {
                for id in vec {
                    if self.get_node_pos(*id).unwrap().distance_squared(pos) <= Node::radius().sqr() {
                        return Some(*id);
                    }
                }
            }
        }
        None
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct EdgeId(u64);

impl FromRawId for EdgeId {
    fn from_raw(id: u64) -> Self {
        EdgeId(id)
    }
}

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