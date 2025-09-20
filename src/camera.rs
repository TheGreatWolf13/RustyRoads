use crate::input::BindingType::{Backward, Forward, Left, Right, RotateLeft, RotateRight};
use crate::input::Input;
use ggez::glam::{Mat4, Vec2};
use std::f32::consts::PI;

const ACCELERATION: f32 = 8.0 / 9.0;
const DAMPING: f32 = 0.9;

pub struct Camera {
    pos: Vec2,
    velocity: Vec2,
    zoom: f32,
    roll: f32,
    proj_matrix: Mat4,
    view_matrix: Mat4,
    inv_proj_matrix: Mat4,
    inv_view_matrix: Mat4,
}

impl Camera {
    pub fn new(window_size: Vec2) -> Self {
        let mut cam = Camera {
            pos: Vec2::ZERO,
            velocity: Vec2::ZERO,
            zoom: 1.0,
            roll: PI / 2.0,
            proj_matrix: Mat4::IDENTITY,
            view_matrix: Mat4::IDENTITY,
            inv_proj_matrix: Mat4::IDENTITY,
            inv_view_matrix: Mat4::IDENTITY,
        };
        cam.adjust_projection(window_size);
        cam
    }

    pub fn adjust_projection(&mut self, window_size: Vec2) {
        let window_size = window_size * 0.5 * self.zoom;
        self.proj_matrix = Mat4::orthographic_rh_gl(-window_size.x, window_size.x, -window_size.y, window_size.y, 0.0, 100.0);
        self.inv_proj_matrix = self.proj_matrix.inverse();
    }

    pub fn tick(&mut self, input: &Input) {
        if input.get(RotateLeft).is_down() {
            self.roll += PI / 180.0;
        }
        if input.get(RotateRight).is_down() {
            self.roll -= PI / 180.0;
        }
        if input.get(Forward).is_down() {
            self.velocity.y += ACCELERATION;
        }
        if input.get(Backward).is_down() {
            self.velocity.y -= ACCELERATION;
        }
        if input.get(Right).is_down() {
            self.velocity.x += ACCELERATION;
        }
        if input.get(Left).is_down() {
            self.velocity.x -= ACCELERATION;
        }
        self.velocity *= DAMPING;
        self.pos += self.velocity;
        // float lastZoom = this.zoom;
        // float scrollY = MouseListener.getScrollY();
        // if (scrollY > 0) {
        //     this.zoom /= 2;
        //     if (this.zoom < 1 / 16.0f) {
        //         this.zoom = 1 / 16.0f;
        //     }
        // }
        // else if (scrollY < 0) {
        //     this.zoom *= 2;
        //     if (this.zoom > 16) {
        //         this.zoom = 16;
        //     }
        // }
        // if (lastZoom != this.zoom) {
        //     this.adjustProjection();
        // }
    }
}
