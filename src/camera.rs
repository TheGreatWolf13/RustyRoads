use crate::input::BindingType::{Backward, Forward, Left, Right, RotateLeft, RotateRight};
use crate::input::Input;
use ggez::glam::{Mat2, Mat4, Vec2, Vec3};
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
    last_window_size: Vec2,
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
            last_window_size: Vec2::ZERO,
        };
        cam.adjust_projection(window_size);
        cam.adjust_view();
        cam
    }

    pub fn get_proj_matrix(&self) -> Mat4 {
        self.proj_matrix
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        self.view_matrix
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    fn adjust_view(&mut self) {
        self.view_matrix = Mat4::look_at_rh(Vec3::new(self.pos.x, self.pos.y, 100.0), Vec3::new(self.pos.x, self.pos.y, -1.0), Vec3::new(self.roll.cos(), self.roll.sin(), 0.0));
        self.inv_view_matrix = self.view_matrix.inverse();
    }

    pub fn adjust_projection(&mut self, window_size: Vec2) {
        let window_size = window_size * 0.5 * self.zoom;
        self.proj_matrix = Mat4::orthographic_rh_gl(-window_size.x, window_size.x, -window_size.y, window_size.y, 0.0, 100.0);
        self.inv_proj_matrix = self.proj_matrix.inverse();
    }

    pub fn get_inv_proj_matrix(&self) -> Mat4 {
        self.inv_proj_matrix
    }

    pub fn get_inv_view_matrix(&self) -> Mat4 {
        self.inv_view_matrix
    }

    pub fn tick(&mut self, input: &Input, window_size: Vec2, delta_time: f32) {
        let delta_time = delta_time / (1.0 / 75.0);
        let last_roll = self.roll;
        if input.get(RotateLeft).is_down() {
            self.roll += PI / 180.0 * delta_time;
        }
        if input.get(RotateRight).is_down() {
            self.roll -= PI / 180.0 * delta_time;
        }
        let mut move_vec = Vec2::ZERO;
        if input.get(Forward).is_down() {
            move_vec.y += 1.0;
        }
        if input.get(Backward).is_down() {
            move_vec.y -= 1.0;
        }
        if input.get(Right).is_down() {
            move_vec.x += 1.0;
        }
        if input.get(Left).is_down() {
            move_vec.x -= 1.0;
        }
        move_vec *= ACCELERATION * delta_time;
        let rotation = Mat2::from_angle(self.roll - PI / 2.0);
        let move_vec = rotation.mul_vec2(move_vec);
        self.velocity += move_vec;
        self.velocity *= DAMPING * delta_time;
        let last_pos = self.pos;
        self.pos += self.velocity * self.zoom * delta_time;
        if last_roll != self.roll || last_pos != self.pos {
            self.adjust_view();
        }
        let last_zoom = self.zoom;
        let scroll = input.scroll.y;
        if scroll > 0.0 {
            self.zoom /= 2.0;
            if self.zoom < 1.0 / 16.0 {
                self.zoom = 1.0 / 16.0;
            }
        } //
        else if scroll < 0.0 {
            self.zoom *= 2.0;
            if self.zoom > 16.0 {
                self.zoom = 16.0;
            }
        }
        if last_zoom != self.zoom || self.last_window_size != window_size {
            self.adjust_projection(window_size);
            self.last_window_size = window_size;
        }
    }
}
