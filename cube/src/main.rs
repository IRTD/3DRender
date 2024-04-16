#![allow(unused)]

use std::time::Duration;

use sdl2::{event::Event, pixels::Color, render::Canvas, video::Window};
use trigger::matrix::Matrix4x4;

fn main() {
    let t1 = Matrix4x4::x_rot(0.01);
    let t2 = Matrix4x4::y_rot(0.03);
    let mut s1 = Matrix4x4::default();
    s1[(0, 0)] = 50.0;
    s1[(1, 1)] = 100.0;
    let mut s2 = Matrix4x4::default();
    s2[(0, 0)] = 20.0;
    s2[(1, 1)] = 20.0;
    sdl2_render("Tets", 800, 800, (s1, s2, t1, t2), |c, ctx| {
        let v1: trigger::Vertex = ctx.0.clone().into();
        let v2: trigger::Vertex = ctx.1.clone().into();
        let p1 = (v1.x as i32, v1.y as i32);
        let p2 = (v2.x as i32, v2.y as i32);
        c.draw_line(p1, p2)?;
        c.draw_line((p1.0, p2.1), (p2.0, p1.1))?;
        ctx.0 = ctx.3 * (ctx.2 * ctx.0);
        ctx.1 = ctx.3 * (ctx.2 * ctx.1);
        std::thread::sleep(Duration::from_millis(15));
        Ok(())
    })
    .unwrap();
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
