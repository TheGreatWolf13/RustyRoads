use ggez::glam::Vec2;
use ggez::graphics::{Color, DrawMode, Mesh};
use ggez::{Context, GameResult};

pub struct Graphics {
    circle: Mesh,
}

impl Graphics {
    pub fn new(ctx: &Context) -> GameResult<Graphics> {
        Ok(Graphics {
            circle: Mesh::new_circle(ctx, DrawMode::fill(), Vec2::ZERO, 100.0, 0.25, Color::WHITE)?
        })
    }

    pub fn circle(&self) -> &Mesh {
        &self.circle
    }
}