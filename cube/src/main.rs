#![allow(unused)]

use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode};
use trigger::matrix::*;
use trigger::*;
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
                    [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
                    [[0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]],
                    // East
                    [[1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0]],
                    [[1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0, 1.0]],
                    // Back
                    [[1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]],
                    [[1.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 0.0, 1.0]],
                    // West
                    [[0.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0]],
                    [[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]],
                    // North
                    [[0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0]],
                    [[0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0]],
                    //South
                    [[0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0]],
                    [[0.0, 0.0, 0.0], [1.0, 0.0, 1.0], [1.0, 0.0, 0.0]],
                ]
                .map(|coords| Triangle::from(coords))
                .to_vec(),
            ),
        }
    }
}

#[derive(Clone)]
struct Ctx {
    cube: Cube,
    fov: f64,
    x_shift: f64,
}

fn main() {
    let settings = SDL2Settings::new().width(800).height(800).fps(150.0);
    let mut display = SDL2Display::<Ctx>::new(settings).unwrap();
    display
        .render_setup(|ctx| {
            let mut cube = Cube::new();
            let c = Ctx {
                cube,
                fov: 90.0,
                x_shift: 0.0,
            };
            ctx.ctx = Some(c);
            Ok(())
        })
        .unwrap();
    display
        .render(|c, ctx| {
            let mut ct = ctx.ctx.as_mut().unwrap();
            let theta = 1.0 * ctx.frame_delta_s;
            let cube = &mut ct.cube;
            cube.mesh.apply_vec(Matrix4x4::x_rot(theta));
            cube.mesh.apply_vec(Matrix4x4::z_rot(theta));
            for event in ctx.pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return Err("Quitting...".to_string()),
                    _ => {}
                }
            }
            let mat_proj = Matrix4x4::projection_3d(ct.fov, 800.0 / 800.0, 1000.0, 0.1);

            let mut cube = ct.cube.clone();
            for tri in &mut cube.mesh.tris {
                tri.scale_add(10.0, Axis::Z);
                tri.apply_vec(mat_proj);
                tri.scale_mul(1500.0, Axis::X);
                tri.scale_mul(1500.0, Axis::Y);
                tri.scale_add(0.5 * 800.0, Axis::X);
                tri.scale_add(0.5 * 800.0, Axis::Y);

                let p1 = (tri.vertices[0].x as i32, tri.vertices[0].y as i32);
                let p2 = (tri.vertices[1].x as i32, tri.vertices[1].y as i32);
                let p3 = (tri.vertices[2].x as i32, tri.vertices[2].y as i32);
                c.draw_line(p1, p2)?;
                c.draw_line(p2, p3)?;
                c.draw_line(p3, p1)?;
            }

            Ok(())
        })
        .unwrap()
}
