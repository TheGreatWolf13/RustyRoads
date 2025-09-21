use crate::camera::Camera;
use crate::input::BindingType::{Backward, Forward, Left, PlaceNode, Right, RotateLeft, RotateRight};
use enum_map::{Enum, EnumMap};
use ggez::glam::{Mat4, Vec2, Vec4};
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyCode::{KeyA, KeyD, KeyE, KeyQ, KeyS, KeyW};
use ggez::input::mouse::MouseButton;
use ggez::winit::keyboard::PhysicalKey;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct KeyBinding {
    inner: RefCell<InnerBinding>,
}

struct InnerBinding {
    click_count: u16,
    is_down: bool,
}

impl KeyBinding {
    fn new() -> Self {
        KeyBinding {
            inner: RefCell::new(InnerBinding {
                click_count: 0,
                is_down: false,
            })
        }
    }

    pub fn is_down(&self) -> bool {
        self.inner.borrow().is_down
    }

    pub fn consume_click(&self) -> bool {
        if self.inner.borrow().click_count > 0 {
            self.inner.borrow_mut().click_count -= 1;
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
}

pub struct Input {
    bindings_by_key: HashMap<PhysicalBinding, Vec<BindingType>>,
    bindings: EnumMap<BindingType, KeyBinding>,
    pub scroll: Vec2,
    mouse_pos: Vec2,
}

impl Input {
    pub fn tick(&self, window_size: Vec2, cam: &Camera) {
        while self.get(PlaceNode).consume_click() {
            println!("Clicked at {}", self.get_world_pos_from_screen_pos(window_size, cam.get_inv_proj_matrix(), cam.get_inv_view_matrix()))
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
            bindings_by_key: HashMap::new(),
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
        input
    }

    pub fn get(&self, binding: BindingType) -> &KeyBinding {
        &self.bindings[binding]
    }

    fn bind(&mut self, bind: PhysicalBinding, binding: BindingType) {
        self.bindings_by_key.entry(bind).or_insert(Vec::new()).push(binding);
    }

    pub fn handle_down(&self, binding: PhysicalBinding) {
        self.handle_binding(binding, |inner| {
            inner.borrow_mut().click_count += 1;
            inner.borrow_mut().is_down = true;
        });
    }

    fn handle_binding(&self, binding: PhysicalBinding, f: fn(&RefCell<InnerBinding>)) {
        if let Some(bindings) = self.bindings_by_key.get(&binding) {
            for binding_type in bindings {
                f(&self.bindings[*binding_type].inner);
            }
        }
    }

    pub fn handle_repeat(&self, binding: PhysicalBinding) {
        self.handle_binding(binding, |inner| {
            inner.borrow_mut().click_count += 1;
        });
    }

    pub fn handle_release(&self, binding: PhysicalBinding) {
        self.handle_binding(binding, |inner| {
            inner.borrow_mut().is_down = false;
        });
    }

    fn get_world_pos_from_screen_pos(&self, window_size: Vec2, inverse_proj: Mat4, inverse_view: Mat4) -> Vec2 {
        let vec = Vec4::new((2.0 * self.mouse_pos.x - window_size.x) / window_size.x, (window_size.y - 2.0 * self.mouse_pos.y) / window_size.y, -1.0, 1.0);
        let vec = inverse_proj.mul_vec4(vec);
        let vec = inverse_view.mul_vec4(vec);
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