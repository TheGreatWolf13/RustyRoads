use crate::CITY_WIDTH;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, MeshData, Vertex};
use ggez::{Context, GameResult};
use std::f32::consts::PI;
use crate::node::{Edge, EdgeId, NodeManager};
use tuple_map::TupleMap2;

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

    pub fn draw_ege(&self, canvas: &mut Canvas, ctx: &mut Context, edge: &Edge, node_manager: &NodeManager, current_path: &Option<Vec<EdgeId>>, explored_paths: &Vec<EdgeId>) -> GameResult {
        const LENGTH: f32 = 10.0;
        let (a, b) = edge.get_nodes().map(|id| node_manager.get_node_pos(id).unwrap());
        let main_dir = (b - a).normalize();
        let perp = main_dir.perp();
        let color = if let Some(selected_edge) = node_manager.selected_edge && selected_edge == edge.get_id() {
            Color::GREEN
        } //
        else if let Some(path) = &current_path && path.contains(&edge.get_id()) {
            Color::YELLOW
        } //
        else if explored_paths.contains(&edge.get_id()) {
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
        Ok(())
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