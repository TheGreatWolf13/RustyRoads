#![allow(unsafe_op_in_unsafe_fn)]
mod camera;
mod input;
mod graphics;
mod node;
mod float;
mod math;

use crate::camera::Camera;
use crate::graphics::Graphics;
use crate::input::Input;
use crate::node::{EdgeId, Node, NodeManager};
use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Text};
use ggez::input::keyboard::KeyInput;
use ggez::input::mouse::MouseButton;
use ggez::timer::TimeContext;
use ggez::*;
use std::path::PathBuf;
use std::{any, slice};
use tuple_map::TupleMap2;

const CITY_WIDTH: f32 = 100_000.0;
const TPS: u32 = 20;

struct Game {
    camera: Camera,
    input: Input,
    graphics: Graphics,
    node_manager: NodeManager,
    current_path: Option<Vec<EdgeId>>,
    explored_paths: Vec<EdgeId>,
}

trait Lerp {
    type Output;

    #[inline(always)]
    fn get_alpha(time: &TimeContext) -> f32 {
        let remainder = time.remaining_update_time().as_secs_f32();
        let alpha = remainder / TPS as f32;
        alpha
    }

    fn lerp(self, time: &TimeContext) -> Self::Output;
}

impl Lerp for (f32, f32) {
    type Output = f32;

    fn lerp(self, time: &TimeContext) -> Self::Output {
        self.0 + (self.1 - self.0) * Self::get_alpha(time)
    }
}

impl Game {
    fn new(ctx: &Context, window_size: Vec2) -> GameResult<Self> {
        Ok(Game {
            camera: Camera::new(window_size),
            input: Input::new(),
            graphics: Graphics::new(ctx)?,
            node_manager: NodeManager::new(),
            current_path: None,
            explored_paths: vec![],
        })
    }

    fn draw_node(&self, canvas: &mut Canvas, node: &Node, radius: f32, colour: Color) {
        canvas.draw(self.graphics.circle(), DrawParam::new().scale(Vec2::splat(radius)).dest(node.get_pos()).color(colour));
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(TPS) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.camera.tick(&self.input, ctx.gfx.drawable_size().into(), ctx.time.delta().as_secs_f32());
        self.input.tick(ctx.gfx.drawable_size().into(), &self.camera, &mut self.node_manager, &mut self.current_path, &mut self.explored_paths);
        ctx.gfx.set_window_title(&format!("{} FPS", ctx.time.fps() as u32));
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_projection(self.camera.get_proj_matrix() * self.camera.get_view_matrix());
        for edge in self.node_manager.get_edges() {
            const LENGTH: f32 = 10.0;
            let (a, b) = edge.get_nodes().map(|id| self.node_manager.get_node_pos(id).unwrap());
            let main_dir = (b - a).normalize();
            let perp = main_dir.perp();
            let color = if let Some(selected_edge) = self.node_manager.selected_edge && selected_edge == edge.get_id() {
                Color::GREEN
            } //
            else if let Some(path) = &self.current_path && path.contains(&edge.get_id()) {
                Color::YELLOW
            } //
            else if self.explored_paths.contains(&edge.get_id()) {
                Color::WHITE
            } //
            else {
                Color::from_rgb(127, 127, 127)
            };
            canvas.draw(&Mesh::new_polygon(
                ctx,
                DrawMode::fill(),
                &[
                    a + 0.5 * LENGTH * perp,
                    a - 0.5 * LENGTH * perp,
                    b - 0.5 * LENGTH * perp,
                    b + 0.5 * LENGTH * perp,
                ],
                color,
            )?, DrawParam::new().color(Color::WHITE));
        }
        for node in self.node_manager.get_nodes() {
            if let Some(start) = self.node_manager.start_node && start == node.get_id() {
                self.draw_node(&mut canvas, node, Node::radius(), Color::GREEN);
            } //
            else if let Some(end) = self.node_manager.end_node && end == node.get_id() {
                self.draw_node(&mut canvas, node, Node::radius(), Color::BLUE);
            } //
            else if let Some(selected) = self.node_manager.selected_node && selected == node.get_id() {
                self.draw_node(&mut canvas, node, Node::radius(), Color::YELLOW);
            } //
            else if self.node_manager.tested_nodes.borrow().contains(&node.get_id()) {
                self.draw_node(&mut canvas, node, Node::radius() / 2.0, Color::MAGENTA);
            } //
            else {
                self.draw_node(&mut canvas, node, Node::radius() / 2.0, Color::RED);
            }
        }
        canvas.draw(self.graphics.bounds(), DrawParam::new());
        canvas.finish(ctx)?;
        let mut canvas = Canvas::from_frame(ctx, None);
        canvas.draw(&Text::new(format!("X: {:.1}", self.camera.get_pos().x)), DrawParam::new().dest(Vec2::new(5.0, 5.0)).color(Color::WHITE));
        canvas.draw(&Text::new(format!("Y: {:.1}", self.camera.get_pos().y)), DrawParam::new().dest(Vec2::new(5.0, 20.0)).color(Color::WHITE));
        canvas.draw(&Text::new(format!("Zoom x{}", 1.0 / self.camera.get_zoom())), DrawParam::new().dest(Vec2::new(5.0, 35.0)).color(Color::WHITE));
        canvas.finish(ctx)?;
        self.input.end_tick();
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) -> GameResult {
        self.input.handle_release(button.into());
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) -> GameResult {
        self.input.handle_down(button.into());
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) -> GameResult {
        self.input.handle_mouse_pos(x, y);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, repeated: bool) -> GameResult {
        if repeated {
            self.input.handle_repeat(input.event.physical_key.into());
        } //
        else {
            self.input.handle_down(input.event.physical_key.into());
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        self.input.handle_release(input.event.physical_key.into());
        Ok(())
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) -> GameResult {
        self.input.scroll = Vec2::new(x, y);
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("rusty_roads", "TheGreatWolf")
        .window_setup(WindowSetup::default().title("").vsync(true).samples(NumSamples::Four))
        .window_mode(WindowMode::default().dimensions(800.0, 600.0).resizable(true))
        .add_resource_path(PathBuf::from("./resources"))
        .build()?;
    let game = Game::new(&ctx, ctx.gfx.drawable_size().into())?;
    event::run(ctx, event_loop, game)
}

#[allow(dead_code)]
fn print_memory<T>(input: &T) {
    unsafe {
        println!("{} = {:?}", any::type_name::<T>(), slice::from_raw_parts(
            input as *const _ as *const u8,
            size_of::<T>(),
        ));
    }
}