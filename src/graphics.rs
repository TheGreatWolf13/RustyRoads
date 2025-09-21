use ggez::graphics::{Mesh, MeshData, Vertex};
use ggez::{Context, GameResult};
use std::f32::consts::PI;

pub struct Graphics {
    circle: Mesh,
}

impl Graphics {
    pub fn new(ctx: &Context) -> GameResult<Graphics> {
        Ok(Graphics {
            circle: {
                const CIRCLE_QUALITY: usize = 64;
                let mut vertices = Vec::with_capacity(CIRCLE_QUALITY + 1);
                let mut indices = Vec::with_capacity(CIRCLE_QUALITY * 3);
                vertices.push(Vertex {
                    position: [0.0, 0.0],
                    uv: [0.0, 0.0],
                    color: [1.0, 1.0, 1.0, 1.0],
                });
                for i in 0..CIRCLE_QUALITY {
                    let angle = 2.0 * PI / CIRCLE_QUALITY as f32 * i as f32;
                    vertices.push(Vertex {
                        position: [angle.cos(), angle.sin()],
                        uv: [0.0, 0.0],
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                }
                for i in 0..CIRCLE_QUALITY as u32 {
                    indices.push(0);
                    indices.push(i + 1);
                    indices.push(i + 2);
                }
                indices.pop();
                indices.push(1);
                Mesh::from_data(ctx, MeshData {
                    indices: &indices,
                    vertices: &vertices,
                })
            }
        })
    }

    pub fn circle(&self) -> &Mesh {
        &self.circle
    }
}