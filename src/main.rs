mod camera;
mod input;
mod graphics;

use crate::camera::Camera;
use crate::graphics::Graphics;
use crate::input::Input;
use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, Drawable, Text};
use ggez::input::keyboard::KeyInput;
use ggez::input::mouse::MouseButton;
use ggez::*;
use std::path::PathBuf;

struct Game {
    camera: Camera,
    input: Input,
    graphics: Graphics,
}

impl Game {
    fn new(ctx: &Context, window_size: Vec2) -> GameResult<Self> {
        Ok(Game {
            camera: Camera::new(window_size),
            input: Input::new(),
            graphics: Graphics::new(ctx)?,
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
        self.input.tick(ctx.gfx.drawable_size().into(), &self.camera);
        ctx.gfx.set_window_title(&format!("{} FPS", ctx.time.fps() as u32));
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_projection(self.camera.get_proj_matrix() * self.camera.get_view_matrix());
        canvas.draw(self.graphics.circle(),
                    DrawParam::new().offset(-Vec2::ZERO)
                                    .color(Color::BLUE),
        );
        canvas.draw(self.graphics.circle(),
                    DrawParam::new().offset(-Vec2::new(20.0, 0.0))
                                    .color(Color::RED),
        );
        canvas.finish(ctx)?;
        let mut canvas = Canvas::from_frame(ctx, None);
        canvas.draw(&Text::new(format!("Zoom x{}", 1.0 / self.camera.get_zoom())),
                    DrawParam::new().dest(Vec2::new(5.0, 5.0))
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