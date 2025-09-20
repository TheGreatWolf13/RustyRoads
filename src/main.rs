mod camera;
mod input;

use crate::camera::Camera;
use crate::input::Input;
use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::input::keyboard::KeyInput;
use ggez::input::mouse::MouseButton;
use ggez::*;

struct Game {
    camera: Camera,
    input: Input,
}

impl Game {
    fn new(window_size: Vec2) -> Self {
        Game {
            camera: Camera::new(window_size),
            input: Input::new(),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(20) {
            self.camera.tick(&self.input);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        ctx.gfx.set_window_title(&format!("{} FPS", ctx.time.fps() as u32));
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> GameResult {
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
}

fn main() -> GameResult {
    let mut c = Conf::new();
    c.window_mode = WindowMode::default();
    c.window_setup = WindowSetup::from(c.window_setup).vsync(true);
    let (ctx, event_loop) = ContextBuilder::new("rusty_roads", "TheGreatWolf")
        .default_conf(c)
        .build()?;
    let game = Game::new(ctx.gfx.size().into());
    event::run(ctx, event_loop, game)
}
