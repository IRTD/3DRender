// Defines the traits and shared values for display services, using the trigger 3D rendering
// engine

pub use anyhow;

/// The total engine responsible for displaying the vertices passed in
pub trait DisplayEngine {
    type Context;
    type Canvas;
    type Settings: Settings;
    type CreateError;
    type RenderError;
    fn new(settings: Self::Settings) -> Result<Self, Self::CreateError>
    where
        Self: Sized;
    fn render_setup<F>(&mut self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut Self::Context) -> anyhow::Result<()>;

    fn render<F>(&mut self, f: F) -> Result<(), Self::RenderError>
    where
        F: Fn(&mut Self::Canvas, &mut Self::Context) -> Result<(), Self::RenderError>;
}

/// Settings used for the Setup of the engine
pub trait Settings {
    fn new() -> Self;
    fn name(self, name: impl ToString) -> Self;
    fn height(self, height: u32) -> Self;
    fn width(self, width: u32) -> Self;
    fn fps(self, fps: f32) -> Self;
    fn vsync(self, _vsync: bool) -> Self
    where
        Self: Sized,
    {
        unimplemented!("This feature is not supported by the current DisplayEngine");
    }
}
