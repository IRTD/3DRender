// Defines the traits and shared values for display services, using the trigger 3D rendering
// engine

/// The total engine responsible for displaying the vertices passed in
pub trait DisplayEngine {
    fn setup(settings: impl Settings) -> Self;
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
