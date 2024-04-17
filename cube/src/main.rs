#![allow(unused)]

use std::{f64::consts::PI, time::Duration};

use sdl2::{event::Event, pixels::Color, render::Canvas, video::Window};
use trigger::{matrix::*, *};

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

fn main() {
    let scale = Matrix4x4::scale(2.0);
    let mut cube = Cube::new();
    cube.mesh.apply(scale);
    let fov = 90.0;
    let fov_rad = 1.0 / (fov * 0.5 / (fov * 2.0) * PI).tan();
    let proj = Matrix4x4::projection_3d(800.0 / 800.0, fov_rad, 1000.0, 0.1);

    sdl2_render("Cube", 800, 800, cube, |c, cube| {
        cube.mesh.apply_vec(Matrix4x4::x_rot(0.02));
        cube.mesh.apply_vec(Matrix4x4::z_rot(0.01));
        cube.mesh.apply_vec(Matrix4x4::y_rot(0.02));
        // cube.mesh.apply_vec(proj);
        for verts in cube.mesh.as_vertices() {
            let verts = verts
                .into_iter()
                .map(|mut v| {
                    v.x += 3.0;
                    v.y += 3.0;
                    v.x *= 0.5 * 200.0;
                    v.y *= 0.5 * 200.0;
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

        std::thread::sleep(Duration::from_millis(15));

        Ok(())
    })
    .unwrap()
}

fn sdl2_render<T, F>(
    name: &str,
    width: u32,
    height: u32,
    mut foreign_ctx: T,
    f: F,
) -> anyhow::Result<()>
where
    F: Fn(&mut Canvas<Window>, &mut T) -> Result<(), String>,
{
    let ctx = sdl2::init().unwrap();
    let video_sub = ctx.video().unwrap();
    let mut canvas = video_sub
        .window(name, width, height)
        .position_centered()
        .build()?
        .into_canvas()
        .build()?;
    let mut el = ctx.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);
        f(&mut canvas, &mut foreign_ctx).unwrap();
        canvas.present();
        for event in el.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
    }
    Ok(())
}
