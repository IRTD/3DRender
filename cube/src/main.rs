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
    ship: Mesh,
    camera: Camera,
}

fn main() {
    let settings = SDL2Settings::new().width(800).height(800).fps(150.0);
    let mut display = SDL2Display::<Ctx>::new(settings).unwrap();
    display
        .render_setup(|ctx| {
            let mut ship = Mesh::load_obj("../ship.obj")?;
            let c = Ctx {
                ship: Cube::new().mesh,
                camera: Camera::new([200.0, 200.0, 20.0], 90.0),
            };
            ctx.ctx = Some(c);
            Ok(())
        })
        .unwrap();
    display
        .render(|c, ctx| {
            let mut ct = ctx.ctx.as_mut().unwrap();

            // Take the angle times the time difference between frames
            let theta = 2.0 * ctx.delta_time_s;
            let mat_proj = Matrix4x4::projection_3d(ct.camera.fov, 800.0 / 800.0, 1000.0, 0.1);
            let mut cube = ct.ship.clone();
            let mut rot = Matrix4x4::x_rot(theta);
            rot *= Matrix4x4::y_rot(-theta);
            // cube.apply_vec(rot);
            cube.apply_vec(Matrix4x4::translate(2.0, 2.0, 1.0));

            // Check for any events and if Quit is called exit
            for event in ctx.pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return Err("Quitting...".to_string()),
                    Event::KeyDown {
                        timestamp,
                        window_id,
                        keycode,
                        scancode,
                        keymod,
                        repeat,
                    } => {
                        let k = match keycode {
                            Some(k) => k,
                            None => continue,
                        };

                        match k {
                            Keycode::W => ct.camera.pos.z -= 0.5,
                            Keycode::S => ct.camera.pos.z += 0.5,
                            Keycode::D => ct.camera.pos.x -= 6.5,
                            Keycode::A => ct.camera.pos.x += 6.5,
                            Keycode::Q => cube.apply_vec(Matrix4x4::y_rot(-theta)),
                            Keycode::E => cube.apply_vec(Matrix4x4::y_rot(theta)),
                            _ => {}
                        }
                    }
                    _ => continue,
                }
            }

            // Clone the cube for display only, do not want to alter the original cube with the
            // perspective projection matrix
            for tri in &mut cube.tris {
                // Scale into view on the Z axis
                tri.scale_add(ct.camera.pos.z, Axis::Z);
                tri.apply_vec(mat_proj);

                //Scale up to size
                tri.scale_mul(1500.0, Axis::X);
                tri.scale_mul(1500.0, Axis::Y);

                // Set into the middle of the window
                tri.scale_add(ct.camera.pos.x, Axis::X);
                tri.scale_add(ct.camera.pos.y, Axis::Y);

                // Create points of vertices and draw them with lines in between
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
