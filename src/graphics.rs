use crate::CITY_WIDTH;
use ggez::glam::Vec2;
use ggez::graphics::{Color, Mesh, MeshBuilder, MeshData, Vertex};
use ggez::{Context, GameResult};
use std::f32::consts::PI;

pub struct Graphics {
    circle: Mesh,
    bounds: Mesh,
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
            },
            bounds: {
                let mut builder = MeshBuilder::new();
                Mesh::from_data(ctx, {
                    make_dashed_line(&mut builder, Vec2::new(-CITY_WIDTH / 2.0, CITY_WIDTH / 2.0), Vec2::new(CITY_WIDTH / 2.0, CITY_WIDTH / 2.0), 10.0, Color::WHITE, 50.0, 50.0, true, true)?;
                    make_dashed_line(&mut builder, Vec2::new(-CITY_WIDTH / 2.0, -CITY_WIDTH / 2.0), Vec2::new(CITY_WIDTH / 2.0, -CITY_WIDTH / 2.0), 10.0, Color::WHITE, 50.0, 50.0, true, true)?;
                    make_dashed_line(&mut builder, Vec2::new(CITY_WIDTH / 2.0, -CITY_WIDTH / 2.0), Vec2::new(CITY_WIDTH / 2.0, CITY_WIDTH / 2.0), 10.0, Color::WHITE, 50.0, 50.0, true, true)?;
                    make_dashed_line(&mut builder, Vec2::new(-CITY_WIDTH / 2.0, -CITY_WIDTH / 2.0), Vec2::new(-CITY_WIDTH / 2.0, CITY_WIDTH / 2.0), 10.0, Color::WHITE, 50.0, 50.0, true, true)?;
                    builder.build()
                })
            },
        })
    }

    pub fn circle(&self) -> &Mesh {
        &self.circle
    }

    pub fn bounds(&self) -> &Mesh {
        &self.bounds
    }
}

fn make_dashed_line(builder: &mut MeshBuilder, from: Vec2, to: Vec2, width: f32, colour: Color, dash_len: f32, spacing: f32, butt: bool, cramped: bool) -> GameResult {
    let mut len = (to - from).length();
    if butt {
        len += width / 2.0;
    }
    let inner_len = len - 2.0 * dash_len;
    if inner_len <= 0.0 {
        //No space for dashing, just draw a line
        builder.line(&[from, to], width, colour)?;
    } //
    else if inner_len <= spacing {
        //Just enough space for two dashes and a little space in between
        builder.line(&[from, from + (to - from).normalize() * dash_len], width, colour)?;
        builder.line(&[to - (to - from).normalize() * dash_len, to], width, colour)?;
    } //
    else {
        //We have enough space
        let rem_len = len - dash_len;
        let mut times = rem_len / (dash_len + spacing);
        if cramped {
            times = times.ceil();
        } //
        else {
            times = times.floor();
        }
        let actual_spacing = (rem_len - times * dash_len) / times;
        let dir = (to - from).normalize();
        builder.line(&[from, from + dir * dash_len], width, colour)?;
        for i in 1..=times as i32 {
            builder.line(&[from + dir * (i as f32 * (dash_len + actual_spacing)), from + dir * (i as f32 * (dash_len + actual_spacing) + dash_len)], width, colour)?;
        }
    }
    Ok(())
}