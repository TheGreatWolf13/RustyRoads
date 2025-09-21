use crate::input::BindingType::{Backward, Forward, Left, Right, RotateLeft, RotateRight};
use enum_map::{Enum, EnumMap};
use ggez::glam::Vec2;
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
            self.inner.borrow_mut().click_count += 1;
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
}

pub struct Input {
    bindings_by_key: HashMap<PhysicalBinding, Vec<BindingType>>,
    bindings: EnumMap<BindingType, KeyBinding>,
    pub scroll: Vec2,
}

impl Input {
    pub fn end_tick(&mut self) {
        self.scroll = Vec2::ZERO;
    }

    pub fn new() -> Self {
        let mut input = Input {
            bindings_by_key: HashMap::new(),
            bindings: EnumMap::from_fn(|_| KeyBinding::new()),
            scroll: Vec2::ZERO,
        };
        input.bind(keyboard(KeyQ), RotateLeft);
        input.bind(keyboard(KeyE), RotateRight);
        input.bind(keyboard(KeyW), Forward);
        input.bind(keyboard(KeyS), Backward);
        input.bind(keyboard(KeyA), Left);
        input.bind(keyboard(KeyD), Right);
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
}

#[inline]
fn keyboard(key: KeyCode) -> PhysicalBinding {
    PhysicalBinding::Keyboard(PhysicalKey::Code(key))
}

#[inline]
fn mouse(button: MouseButton) -> PhysicalBinding {
    PhysicalBinding::Mouse(button)
}