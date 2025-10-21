use crate::math::vec::Vec2CompWise;
use crate::math::{if_else, vec::Vec2Axis, Sqr};
use crate::node::a_star::AStarHeap;
use crate::traffic::LaneDefinition;
use crate::CITY_WIDTH;
use ggez::glam::{IVec2, Vec2};
use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::hash::Hash;
use std::mem;
use std::mem::MaybeUninit;
use std::num::NonZeroU64;

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

#[derive(Copy, Clone)]
struct LocalPos(Vec2);

impl From<Vec2> for LocalPos {
    fn from(pos: Vec2) -> Self {
        let mut pos = pos % CHUNK_SIZE;
        pos.x = if_else!(pos.x < 0.0 => pos.x + CHUNK_SIZE ; pos.x);
        pos.y = if_else!(pos.y < 0.0 => pos.y + CHUNK_SIZE ; pos.y);
        LocalPos(pos)
    }
}

impl LocalPos {
    fn get_delta(mut self, direction: Vec2) -> Vec2 {
        if direction.x.signum() != self.0.x.signum() {
            self.0.x = CHUNK_SIZE - self.0.x;
        }
        if direction.y.signum() != self.0.y.signum() {
            self.0.y = CHUNK_SIZE - self.0.y;
        }
        self.0
    }
}

impl ChunkPos {
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
pub struct NodeId(NonZeroU64);

impl FromRawId for NodeId {
    fn from_raw(id: NonZeroU64) -> Self {
        NodeId(id)
    }
}

pub struct NodeManager {
    nodes: Inner<NodeId, Node>,
    edges: Inner<EdgeId, Edge>,
    node_lookup: FxHashMap<ChunkPos, Vec<NodeId>>,
    edge_lookup: FxHashMap<ChunkPos, Vec<EdgeId>>,
    pub start_node: Option<NodeId>,
    pub end_node: Option<NodeId>,
    pub selected_node: Option<NodeId>,
    pub selected_edge: Option<EdgeId>,
    pub tested_nodes: RefCell<Vec<NodeId>>,
}

trait FromRawId {
    fn from_raw(id: NonZeroU64) -> Self;
}

struct Inner<I: FromRawId, N> {
    id_maker: u64,
    map: FxHashMap<I, N>,
}

impl<I: FromRawId, N> Inner<I, N> {
    fn get_id(&mut self) -> I {
        self.id_maker += 1;
        I::from_raw(NonZeroU64::new(self.id_maker).unwrap())
    }
}

impl NodeManager {
    pub fn new() -> Self {
        let mut manager = NodeManager {
            nodes: Inner {
                map: FxHashMap::default(),
                id_maker: 0,
            },
            edges: Inner {
                map: FxHashMap::default(),
                id_maker: 0,
            },
            node_lookup: FxHashMap::default(),
            edge_lookup: FxHashMap::default(),
            start_node: None,
            end_node: None,
            selected_node: None,
            selected_edge: None,
            tested_nodes: RefCell::new(vec![]),
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
                    if x == 0 {
                        manager.make_edge(last, node, 2.0, 16);
                    } else {
                        manager.make_edge(last, node, 1.0, 12);
                    }
                }
                last_node = Some(node);
            }
        }
        for y in -RADIUS..=RADIUS {
            let mut last_node = None;
            for x in -RADIUS..=RADIUS {
                let node = ids[(x + RADIUS) as usize][(y + RADIUS) as usize];
                if let Some(last) = last_node {
                    manager.make_edge(last, node, 1.0, 12);
                }
                last_node = Some(node);
            }
        }
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

    pub fn get_nodes(&self) -> impl Iterator<Item=&Node> {
        self.nodes.map.values().into_iter()
    }

    pub fn get_edges(&self) -> impl Iterator<Item=&Edge> {
        self.edges.map.values().into_iter()
    }

    pub fn add_node(&mut self, pos: Vec2) -> NodeId {
        let id = self.nodes.get_id();
        self.nodes.map.insert(id, Node {
            id,
            pos,
            edges: vec![],
        });
        self.node_lookup.entry(ChunkPos::from_world_pos(pos)).or_insert_with(|| Vec::new()).push(id);
        id
    }

    pub fn make_edge(&mut self, node_a: NodeId, node_b: NodeId, speed: f32, size: u8) -> EdgeId {
        let id = self.edges.get_id();
        self.edges.map.insert(id, Edge {
            nodes: (node_a, node_b),
            id,
            speed,
            lane_def: LaneDefinition::new(size),
        });
        let node_a = self.get_node_mut(node_a).unwrap();
        node_a.edges.push(id);
        let a = node_a.pos;
        let node_b = self.get_node_mut(node_b).unwrap();
        node_b.edges.push(id);
        let b = node_b.pos;
        //Add edge to lookup
        let chunk_a = ChunkPos::from_world_pos(a);
        let chunk_b = ChunkPos::from_world_pos(b);
        if chunk_a == chunk_b {
            //Too small
            self.edge_lookup.entry(chunk_a).or_insert_with(|| Vec::new()).push(id);
        } //
        else if chunk_a.0.x == chunk_b.0.x || chunk_a.0.y == chunk_b.0.y {
            //Straight line
            let spam = chunk_b.0 - chunk_a.0;
            let axis = spam.abs().get_max_axis().unwrap();
            let steps = spam.abs().get_comp(axis) + 1;
            self.edge_lookup.entry(chunk_a).or_insert_with(|| Vec::new()).push(id);
            self.edge_lookup.entry(chunk_b).or_insert_with(|| Vec::new()).push(id);
            if steps > 2 {
                let chunk = chunk_a.0;
                let offset = spam.signum().get_comp(axis);
                for step in 1..steps - 1 {
                    self.edge_lookup.entry(chunk.with_offset_on(axis, step * offset).into()).or_insert_with(|| Vec::new()).push(id);
                }
            }
        } //
        else {
            let ab = b - a;
            let spam = chunk_b.0 - chunk_a.0;
            if let Some(axis) = ab.get_max_axis() {
                let steps = spam.abs().get_comp(axis) + 1;
                self.edge_lookup.entry(chunk_a).or_insert_with(|| Vec::new()).push(id);
                self.edge_lookup.entry(chunk_b).or_insert_with(|| Vec::new()).push(id);
                if steps > 2 {
                    let mut chunk = chunk_a.0;
                    let offset = spam.signum().get_comp(axis);
                    for _ in 1..steps - 1 {
                        chunk = chunk.with_offset_on(axis, offset);
                        let main_comp = chunk.get_comp(axis) as f32 * CHUNK_SIZE + CHUNK_SIZE / 2.0;
                        let other_comp = (b.get_comp(axis.other()) - a.get_comp(axis.other())) / (b.get_comp(axis) - a.get_comp(axis)) * (main_comp - a.get_comp(axis)) + a.get_comp(axis.other());
                        let other_comp = (other_comp / CHUNK_SIZE).floor() as i32;
                        self.edge_lookup.entry(chunk.with_comp(axis.other(), other_comp).into()).or_insert_with(|| Vec::new()).push(id);
                    }
                }
            } //
            else {
                //Perfectly diagonal vector
                let steps = spam.abs().get_comp(Vec2Axis::X) + 1;
                self.edge_lookup.entry(chunk_a).or_insert_with(|| Vec::new()).push(id);
                self.edge_lookup.entry(chunk_b).or_insert_with(|| Vec::new()).push(id);
                if steps > 2 {
                    let mut chunk = chunk_a.0;
                    let offset = spam.signum().get_comp(Vec2Axis::X);
                    for _ in 1..steps - 1 {
                        chunk = chunk.with_offset_on(Vec2Axis::X, offset).with_offset_on(Vec2Axis::Y, offset);
                        self.edge_lookup.entry(chunk.into()).or_insert_with(|| Vec::new()).push(id);
                    }
                }
            }
        }
        id
    }

    pub fn a_star(&self, start: NodeId, goal: NodeId, h: fn(Vec2, Vec2) -> f32) -> (Option<Vec<EdgeId>>, Vec<EdgeId>) {
        let mut open_set = AStarHeap::new();
        let mut explored_paths = vec![];
        let goal_pos = self.get_node_pos(goal).unwrap();
        open_set.push(start, h(self.get_node_pos(start).unwrap(), goal_pos));
        let mut came_from = FxHashMap::<NodeId, NodeId>::default();
        let mut g_score = FxHashMap::default();
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

    fn reconstruct_path(&self, came_from: FxHashMap<NodeId, NodeId>, goal: NodeId) -> Vec<EdgeId> {
        let mut vec = vec![];
        let mut last_node = goal;
        while let Some(next_node) = came_from.get(&last_node) {
            vec.push(self.get_node(last_node).unwrap().find_edge(*next_node, &self).unwrap());
            last_node = *next_node;
        }
        vec
    }

    pub fn try_node_collision(&self, pos: Vec2) -> Option<NodeId> {
        self.tested_nodes.borrow_mut().clear();
        for chunk_pos in ChunkPos::get_area(pos).into_iter() {
            if let Some(vec) = self.node_lookup.get(&chunk_pos) {
                for id in vec {
                    self.tested_nodes.borrow_mut().push(*id);
                    if self.get_node_pos(*id).unwrap().distance_squared(pos) <= Node::radius().sqr() {
                        return Some(*id);
                    }
                }
            }
        }
        None
    }

    pub fn try_edge_collision(&self, pos: Vec2) -> Option<EdgeId> {
        for chunk_pos in ChunkPos::get_area(pos).into_iter() {
            if let Some(vec) = self.edge_lookup.get(&chunk_pos) {
                for id in vec {
                    if self.get_edge(*id).unwrap().distance_to_sqr(&self, pos) <= (Node::radius() / 2.0).sqr() {
                        return Some(*id);
                    }
                }
            }
        }
        None
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct EdgeId(NonZeroU64);

impl FromRawId for EdgeId {
    fn from_raw(id: NonZeroU64) -> Self {
        EdgeId(id)
    }
}

pub struct Edge {
    id: EdgeId,
    nodes: (NodeId, NodeId),
    speed: f32,
    lane_def: LaneDefinition,
}

impl Edge {
    pub fn get_id(&self) -> EdgeId {
        self.id
    }

    pub fn get_nodes(&self) -> (NodeId, NodeId) {
        self.nodes
    }

    pub fn distance_to_sqr(&self, node_manager: &NodeManager, pos: Vec2) -> f32 {
        let a = node_manager.get_node_pos(self.nodes.0).unwrap();
        let b = node_manager.get_node_pos(self.nodes.1).unwrap();
        let ab = b - a;
        let ap = pos - a;
        let t = ap.dot(ab) / ab.length_squared();
        let c = if 0.0 <= t && t <= 1.0 {
            a + t * ab
        } //
        else if t < 0.0 {
            a
        } //
        else {
            b
        };
        c.distance_squared(pos)
    }

    pub fn get_other_node(&self, node: NodeId) -> NodeId {
        if self.nodes.0 == node {
            return self.nodes.1;
        }
        debug_assert_eq!(self.nodes.1, node, "This edge does not contain the given node!");
        self.nodes.0
    }

    pub fn get_size(&self) -> u8 {
        self.lane_def.get_size()
    }
}