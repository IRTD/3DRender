#![allow(unused)]

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, EventPump,
};
use std::{f64::consts::PI, time::Duration};
use trigger::{matrix::*, *};
use trigger_sdl2::*;

#[derive(Clone)]
struct Cube {
    mesh: Mesh,
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            mesh: Mesh::new(
                [
                    // Front
                    [
                        [10.0, 10.0, 10.0],
                        [10.0, 100.0, 10.0],
                        [100.0, 100.0, 10.0],
                    ],
                    [
                        [10.0, 10.0, 10.0],
                        [100.0, 100.0, 10.0],
                        [100.0, 10.0, 10.0],
                    ],
                    // East
                    [
                        [100.0, 10.0, 10.0],
                        [100.0, 100.0, 10.0],
                        [100.0, 100.0, 100.0],
                    ],
                    [
                        [100.0, 10.0, 10.0],
                        [100.0, 100.0, 100.0],
                        [100.0, 10.0, 100.0],
                    ],
                    // Back
                    [
                        [100.0, 10.0, 100.0],
                        [100.0, 100.0, 100.0],
                        [10.0, 100.0, 100.0],
                    ],
                    [
                        [100.0, 10.0, 100.0],
                        [10.0, 100.0, 100.0],
                        [10.0, 10.0, 100.0],
                    ],
                    // West
                    [
                        [10.0, 10.0, 100.0],
                        [10.0, 100.0, 100.0],
                        [10.0, 100.0, 10.0],
                    ],
                    [[10.0, 10.0, 100.0], [10.0, 100.0, 10.0], [10.0, 10.0, 10.0]],
                    // North
                    [
                        [10.0, 100.0, 10.0],
                        [10.0, 100.0, 100.0],
                        [100.0, 100.0, 100.0],
                    ],
                    [
                        [10.0, 100.0, 10.0],
                        [100.0, 100.0, 100.0],
                        [100.0, 100.0, 10.0],
                    ],
                    //South
                    [
                        [10.0, 10.0, 10.0],
                        [10.0, 10.0, 100.0],
                        [100.0, 10.0, 100.0],
                    ],
                    [
                        [10.0, 10.0, 10.0],
                        [100.0, 10.0, 100.0],
                        [100.0, 10.0, 10.0],
                    ],
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
    let settings = SDL2Settings::new().height(1000).width(1000).fps(144.0);
    let mut display = SDL2Display::<Ctx>::new(settings).unwrap();
    display.render_setup(|ctx| {
        let mut cube = Cube::new();
        let x = Ctx {
            cube,
            x_off: 550.0,
            y_off: 350.0,
            z_off: 10.0,
        };
        ctx.ctx = Some(x);
        Ok(())
    });
    display
        .render(|c, ctx| {
            let rot = 0.5 * ctx.delta_time_s;
            let cx = ctx.ctx.as_mut().unwrap();
            for event in ctx.pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return Err("Quitting...".to_string()),
                    _ => {}
                }
            }
            cx.cube.mesh.apply_vec(Matrix4x4::y_rot(rot));
            let mut display_cube = cx.cube.clone();
            let proj = Matrix4x4::projection_3d(90.0, 1000.0 / 1000.0, 1000.0, 0.1);
            display_cube.mesh.apply_vec(proj);
            display_cube.mesh.apply(Matrix4x4::scale(10.0));
            c.set_draw_color(Color::RED);
            for verts in display_cube.mesh.as_vertices() {
                let verts = verts
                    .into_iter()
                    .map(|mut v| {
                        v.x += 400.0;
                        v.y += 400.0;
                        v
                    })
                    .collect::<Vec<Vertex>>();
                let p1 = ((verts[0].x) as i32, (verts[0].y) as i32);
                let p2 = ((verts[1].x) as i32, (verts[1].y) as i32);
                let p3 = ((verts[2].x) as i32, (verts[2].y) as i32);
                c.draw_line(p1, p2)?;
                c.draw_line(p2, p3)?;
                c.draw_line(p3, p1)?;
            }
            // std::thread::sleep(Duration::from_secs(1));
            Ok(())
        })
        .unwrap()
}
