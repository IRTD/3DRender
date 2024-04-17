#![allow(unused)]

use std::time::Duration;

use sdl2::{event::Event, pixels::Color, render::Canvas, video::Window};
use trigger::matrix::Matrix4x4;

fn main() {
    todo!()
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
