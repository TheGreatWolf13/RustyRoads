#![allow(unsafe_op_in_unsafe_fn)]
mod camera;
mod input;
mod graphics;
mod node;
mod float;

use crate::camera::Camera;
use crate::graphics::Graphics;
use crate::input::Input;
use crate::node::{EdgeId, NodeManager};
use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Text};
use ggez::input::keyboard::KeyInput;
use ggez::input::mouse::MouseButton;
use ggez::*;
use std::path::PathBuf;
use tuple_map::TupleMap2;

const CITY_WIDTH: f32 = 100_000.0;

struct Game {
    camera: Camera,
    input: Input,
    graphics: Graphics,
    node_manager: NodeManager,
    current_path: Option<Vec<EdgeId>>,
    explored_paths: Vec<EdgeId>,
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
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(20) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.camera.tick(&self.input, ctx.gfx.drawable_size().into(), ctx.time.delta().as_secs_f32());
        self.input.tick(ctx.gfx.drawable_size().into(), &self.camera, &mut self.node_manager, &mut self.current_path, &mut self.explored_paths);
        ctx.gfx.set_window_title(&format!("{} FPS", ctx.time.fps() as u32));
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_projection(self.camera.get_proj_matrix() * self.camera.get_view_matrix());
        self.node_manager.for_all_edges(|edge| {
            const LENGTH: f32 = 10.0;
            let (a, b) = edge.get_nodes().map(|id| self.node_manager.get_node_pos(id).unwrap());
            let main_dir = (b - a).normalize();
            let perp = main_dir.perp();
            let color = if self.current_path.as_ref().is_some_and(|v| v.contains(&edge.get_id())) {
                Color::YELLOW
            } //
            else if self.explored_paths.contains(&edge.get_id()) {
                Color::WHITE
            } //
            else {
                Color::from_rgb(127, 127, 127)
            };
            if let Ok(quad) = Mesh::new_polygon(
                ctx,
                DrawMode::fill(),
                &[
                    a + 0.5 * LENGTH * perp,
                    a - 0.5 * LENGTH * perp,
                    b - 0.5 * LENGTH * perp,
                    b + 0.5 * LENGTH * perp,
                ],
                color,
            ) {
                canvas.draw(&quad, DrawParam::new().color(Color::WHITE));
            }
        });
        self.node_manager.for_all_nodes(|node| {
            if self.node_manager.start_node == node.get_id() {
                canvas.draw(
                    self.graphics.circle(),
                    DrawParam::new().scale(Vec2::new(10.0, 10.0))
                                    .dest(node.get_pos())
                                    .color(Color::GREEN),
                );
            } //
            else if self.node_manager.end_node == node.get_id() {
                canvas.draw(
                    self.graphics.circle(),
                    DrawParam::new().scale(Vec2::new(10.0, 10.0))
                                    .dest(node.get_pos())
                                    .color(Color::BLUE),
                );
            } //
            else {
                canvas.draw(
                    self.graphics.circle(),
                    DrawParam::new().scale(Vec2::new(5.0, 5.0))
                                    .dest(node.get_pos())
                                    .color(Color::RED),
                );
            }
        });
        canvas.draw(self.graphics.bounds(), DrawParam::new());
        canvas.finish(ctx)?;
        let mut canvas = Canvas::from_frame(ctx, None);
        canvas.draw(
            &Text::new(format!("X: {:.1}", self.camera.get_pos().x)),
            DrawParam::new().dest(Vec2::new(5.0, 5.0))
                            .color(Color::WHITE),
        );
        canvas.draw(
            &Text::new(format!("Y: {:.1}", self.camera.get_pos().y)),
            DrawParam::new().dest(Vec2::new(5.0, 20.0))
                            .color(Color::WHITE),
        );
        canvas.draw(
            &Text::new(format!("Zoom x{}", 1.0 / self.camera.get_zoom())),
            DrawParam::new().dest(Vec2::new(5.0, 35.0))
                            .color(Color::WHITE),
        );
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