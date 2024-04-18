#![allow(unused)]

use std::time::{Duration, Instant};

use sdl2::{pixels::Color, render::Canvas, video::Window, EventPump};
pub use trigger_display::*;

pub struct SDL2Display<T> {
    canvas: Canvas<Window>,
    context: SDL2Context<T>,
    bg_color: Color,
    draw_color: Color,
    time: Instant,
    delta: f64,
}

pub struct SDL2Context<T> {
    pub pump: EventPump,
    frames: f32,
    pub delta_time_s: f64,
    pub ctx: Option<T>,
}

impl<T> SDL2Context<T> {
    pub fn set_fps(&mut self, new: f32) {
        self.frames = new;
    }
}

impl<T> DisplayEngine for SDL2Display<T> {
    type Settings = SDL2Settings;
    type Canvas = Canvas<Window>;
    type Context = SDL2Context<T>;
    type CreateError = String;
    type RenderError = String;
    fn new(settings: Self::Settings) -> Result<Self, Self::CreateError> {
        let sdl = sdl2::init()?;
        let canvas = sdl
            .video()?
            .window(&settings.name, settings.width, settings.height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;
        let context = SDL2Context {
            pump: sdl.event_pump()?,
            frames: settings.fps,
            delta_time_s: 1.0,
            ctx: None,
        };

        Ok(SDL2Display {
            canvas,
            context,
            bg_color: Color::BLACK,
            draw_color: Color::WHITE,
            time: Instant::now(),
            delta: 1.0,
        })
    }

    fn render_setup<F>(&mut self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut Self::Context) -> anyhow::Result<()>,
    {
        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();
        f(&mut self.context)
    }

    fn render<F>(&mut self, f: F) -> Result<(), Self::RenderError>
    where
        F: Fn(&mut Self::Canvas, &mut Self::Context) -> Result<(), Self::RenderError>,
    {
        loop {
            self.time = Instant::now();
            self.context.delta_time_s = self.delta;
            self.canvas.set_draw_color(self.bg_color);
            self.canvas.clear();
            self.canvas.set_draw_color(self.draw_color);
            f(&mut self.canvas, &mut self.context)?;
            self.canvas.present();
            let fps = 1_000.0 / self.context.frames;
            std::thread::sleep(Duration::from_millis(fps as u64));
            self.delta = self.time.elapsed().as_secs_f64();
        }
    }
}

pub struct SDL2Settings {
    fps: f32,
    name: String,
    width: u32,
    height: u32,
    vsync: bool,
}

impl Settings for SDL2Settings {
    fn new() -> Self {
        SDL2Settings {
            fps: 60.0,
            name: String::from("SDL2Display"),
            width: 400,
            height: 400,
            vsync: false,
        }
    }

    fn fps(mut self, fps: f32) -> Self {
        self.fps = fps;
        self
    }
    fn name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }
    fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
}
