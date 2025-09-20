mod camera;

use ggez::*;
use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event::EventHandler;
use ggez::input::keyboard::KeyInput;

struct Game;

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(20) {
            
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        ctx.gfx.set_window_title(&format!("{} FPS", ctx.time.fps() as u32));
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> GameResult {
        Ok(())
    }
}

fn main() -> GameResult {
    let game = Game;
    let mut c = Conf::new();
    c.window_mode = WindowMode::default();
    c.window_setup = WindowSetup::from(c.window_setup).vsync(true);
    let (ctx, event_loop) = ContextBuilder::new("rusty_roads", "TheGreatWolf")
        .default_conf(c)
        .build()?;
    event::run(ctx, event_loop, game)
}
