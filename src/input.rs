use crate::camera::Camera;
use crate::input::BindingType::{Backward, Forward, Left, Pathfind, PlaceNode, Right, RotateLeft, RotateRight, SelectEdge, SelectNode, SetEnd, SetStart};
use crate::node::{EdgeId, NodeManager};
use enum_map::{Enum, EnumMap};
use ggez::glam::{Vec2, Vec4};
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyCode::{KeyA, KeyD, KeyE, KeyQ, KeyS, KeyW, KeyX, KeyZ};
use ggez::input::mouse::MouseButton;
use ggez::winit::keyboard::PhysicalKey;
use rustc_hash::FxHashMap;

pub struct KeyBinding {
    click_count: u16,
    is_down: bool,
}

impl KeyBinding {
    fn new() -> Self {
        KeyBinding {
            click_count: 0,
            is_down: false,
        }
    }

    pub fn is_down(&self) -> bool {
        self.is_down
    }

    pub fn consume_all_clicks(&mut self) -> bool {
        if self.click_count > 0 {
            self.click_count = 0;
            return true;
        }
        false
    }

    pub fn consume_click(&mut self) -> bool {
        if self.click_count > 0 {
            self.click_count -= 1;
            return true;
        }
        false
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum PhysicalBinding {
    Keyboard(PhysicalKey),
    Mouse(MouseButton),
}

impl From<PhysicalKey> for PhysicalBinding {
    fn from(value: PhysicalKey) -> Self {
        PhysicalBinding::Keyboard(value)
    }
}

impl From<MouseButton> for PhysicalBinding {
    fn from(value: MouseButton) -> Self {
        PhysicalBinding::Mouse(value)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug, Enum)]
pub enum BindingType {
    RotateLeft,
    RotateRight,
    Forward,
    Backward,
    Right,
    Left,
    PlaceNode,
    Pathfind,
    SelectNode,
    SelectEdge,
    SetStart,
    SetEnd,
}

pub struct Input {
    bindings_by_key: FxHashMap<PhysicalBinding, Vec<BindingType>>,
    bindings: EnumMap<BindingType, KeyBinding>,
    pub scroll: Vec2,
    mouse_pos: Vec2,
}

impl Input {
    pub fn tick(&mut self, window_size: Vec2, camera: &Camera, node_manager: &mut NodeManager, current_path: &mut Option<Vec<EdgeId>>, explored_paths: &mut Vec<EdgeId>) {
        while self.get_mut(PlaceNode).consume_click() {
            node_manager.add_node(self.get_world_pos_from_screen_pos(window_size, &camera));
        }
        if self.get_mut(Pathfind).consume_all_clicks() {
            if let Some(start) = node_manager.start_node && let Some(end) = node_manager.end_node {
                let (path, explored) = node_manager.a_star(start, end, |a, b| a.distance(b) / 2.0);
                *current_path = path;
                *explored_paths = explored;
            }
        }
        if self.get_mut(SelectNode).consume_all_clicks() {
            if let Some(id) = node_manager.try_node_collision(self.get_world_pos_from_screen_pos(window_size, &camera)) {
                node_manager.selected_node = Some(id);
            }
        }
        if self.get_mut(SelectEdge).consume_all_clicks() {
            if let Some(id) = node_manager.try_edge_collision(self.get_world_pos_from_screen_pos(window_size, &camera)) {
                node_manager.selected_edge = Some(id);
            }
        }
        if self.get_mut(SetStart).consume_all_clicks() {
            if let Some(selected) = node_manager.selected_node {
                node_manager.start_node = Some(selected);
            }
        }
        if self.get_mut(SetEnd).consume_all_clicks() {
            if let Some(selected) = node_manager.selected_node {
                node_manager.end_node = Some(selected);
            }
        }
    }

    pub fn handle_mouse_pos(&mut self, x: f32, y: f32) {
        self.mouse_pos = Vec2::new(x, y);
    }

    pub fn end_tick(&mut self) {
        self.scroll = Vec2::ZERO;
    }

    pub fn new() -> Self {
        let mut input = Input {
            bindings_by_key: FxHashMap::default(),
            bindings: EnumMap::from_fn(|_| KeyBinding::new()),
            scroll: Vec2::ZERO,
            mouse_pos: Vec2::ZERO,
        };
        input.bind(keyboard(KeyQ), RotateLeft);
        input.bind(keyboard(KeyE), RotateRight);
        input.bind(keyboard(KeyW), Forward);
        input.bind(keyboard(KeyS), Backward);
        input.bind(keyboard(KeyA), Left);
        input.bind(keyboard(KeyD), Right);
        input.bind(mouse(MouseButton::Left), PlaceNode);
        input.bind(keyboard(KeyCode::Enter), Pathfind);
        input.bind(mouse(MouseButton::Right), SelectNode);
        input.bind(mouse(MouseButton::Middle), SelectEdge);
        input.bind(keyboard(KeyZ), SetStart);
        input.bind(keyboard(KeyX), SetEnd);
        input
    }

    pub fn get(&self, binding: BindingType) -> &KeyBinding {
        &self.bindings[binding]
    }

    pub fn get_mut(&mut self, binding: BindingType) -> &mut KeyBinding {
        &mut self.bindings[binding]
    }

    fn bind(&mut self, bind: PhysicalBinding, binding: BindingType) {
        self.bindings_by_key.entry(bind).or_insert(Vec::new()).push(binding);
    }

    pub fn handle_down(&mut self, binding: PhysicalBinding) {
        self.handle_binding(binding, |inner| {
            inner.click_count += 1;
            inner.is_down = true;
        });
    }

    fn handle_binding(&mut self, binding: PhysicalBinding, f: fn(&mut KeyBinding)) {
        if let Some(bindings) = self.bindings_by_key.get(&binding) {
            for binding_type in bindings {
                f(&mut self.bindings[*binding_type]);
            }
        }
    }

    pub fn handle_repeat(&mut self, binding: PhysicalBinding) {
        self.handle_binding(binding, |key_bind| {
            key_bind.click_count += 1;
        });
    }

    pub fn handle_release(&mut self, binding: PhysicalBinding) {
        self.handle_binding(binding, |key_bind| {
            key_bind.is_down = false;
        });
    }

    fn get_world_pos_from_screen_pos(&self, window_size: Vec2, cam: &Camera) -> Vec2 {
        let vec = Vec4::new((2.0 * self.mouse_pos.x - window_size.x) / window_size.x, (window_size.y - 2.0 * self.mouse_pos.y) / window_size.y, -1.0, 1.0);
        let vec = cam.get_inv_proj_matrix().mul_vec4(vec);
        let vec = cam.get_inv_view_matrix().mul_vec4(vec);
        Vec2::new(vec.x, vec.y)
    }
}

#[inline]
fn keyboard(key: KeyCode) -> PhysicalBinding {
    PhysicalBinding::Keyboard(PhysicalKey::Code(key))
}

#[inline]
fn mouse(button: MouseButton) -> PhysicalBinding {
    PhysicalBinding::Mouse(button)
}