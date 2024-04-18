#![allow(unused)]

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, EventPump,
};
use std::{f64::consts::PI, time::Duration};
use trigger::{matrix::*, *};
use trigger_sdl2::*;

struct Cube {
    mesh: Mesh,
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            mesh: Mesh::new(
                [
                    // Front
                    [[0.0, 0.0, 0.0], [0.0, 2.0, 0.0], [2.0, 2.0, 0.0]],
                    [[0.0, 0.0, 0.0], [2.0, 2.0, 0.0], [2.0, 0.0, 0.0]],
                    // East
                    [[2.0, 0.0, 0.0], [2.0, 2.0, 0.0], [2.0, 2.0, 2.0]],
                    [[2.0, 0.0, 0.0], [2.0, 2.0, 2.0], [2.0, 0.0, 2.0]],
                    // Back
                    [[2.0, 0.0, 2.0], [2.0, 2.0, 2.0], [0.0, 2.0, 2.0]],
                    [[2.0, 0.0, 2.0], [0.0, 2.0, 2.0], [0.0, 0.0, 2.0]],
                    // West
                    [[0.0, 0.0, 2.0], [0.0, 2.0, 2.0], [0.0, 2.0, 0.0]],
                    [[0.0, 0.0, 2.0], [0.0, 2.0, 0.0], [0.0, 0.0, 0.0]],
                    // North
                    [[0.0, 2.0, 0.0], [0.0, 2.0, 2.0], [2.0, 2.0, 2.0]],
                    [[0.0, 2.0, 0.0], [2.0, 2.0, 2.0], [2.0, 2.0, 0.0]],
                    //South
                    [[0.0, 0.0, 0.0], [0.0, 0.0, 2.0], [2.0, 0.0, 2.0]],
                    [[0.0, 0.0, 0.0], [2.0, 0.0, 2.0], [2.0, 0.0, 0.0]],
                ]
                .map(|coords| Triangle::from(coords))
                .to_vec(),
            ),
        }
    }
}

struct Ctx {
    cube: Cube,
    x_off: f64,
    y_off: f64,
    z_off: f64,
}

fn main() {
    let settings = SDL2Settings::new().height(1000).width(1000).fps(800.0);
    let mut display = SDL2Display::<Ctx>::new(settings).unwrap();
    display.render_setup(|ctx| {
        let mut cube = Cube::new();
        let scale = Matrix4x4::scale(100.0);
        cube.mesh.apply(scale);
        let x = Ctx {
            cube,
            x_off: 550.0,
            y_off: 350.0,
            z_off: 0.0,
        };
        ctx.ctx = Some(x);
        Ok(())
    });
    display
        .render(|c, ctx| {
            let rot = 1.75 * ctx.delta_time_s;
            let cx = ctx.ctx.as_mut().unwrap();
            for event in ctx.pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return Err("Quitting...".to_string()),
                    _ => {}
                }
            }
            cx.cube.mesh.apply_vec(Matrix4x4::x_rot(rot));
            cx.cube.mesh.apply_vec(Matrix4x4::y_rot(rot));
            cx.cube.mesh.apply_vec(Matrix4x4::z_rot(rot));
            c.set_draw_color(Color::RED);
            for verts in cx.cube.mesh.as_vertices() {
                let verts = verts
                    .into_iter()
                    .map(|mut v| {
                        v.x += cx.x_off;
                        v.y += cx.y_off;
                        v.z += cx.z_off;
                        v
                    })
                    .collect::<Vec<Vertex>>();
                let p1 = ((verts[0].x) as i32, (verts[0].y) as i32);
                let p2 = ((verts[1].x) as i32, (verts[1].y) as i32);
                let p3 = ((verts[2].x) as i32, (verts[2].y) as i32);
                c.draw_line(p2, p1)?;
                c.draw_line(p2, p3)?;
                c.draw_line(p3, p1)?;
            }
            Ok(())
        })
        .unwrap()
}
