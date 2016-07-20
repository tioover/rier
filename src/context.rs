//! Manage OpenGL context and window.
use std::string::ToString;
use glium::{Display, DisplayBuild, Frame, Surface};
use glium::glutin::WindowBuilder;


/// Context handle object.
///
/// Proxy of `glium::Display`.
#[derive(Clone)]
pub struct Context {
    pub display: Display,
}


impl Context {
    /// Builds OpenGL context and create a window.
    pub fn create<T: ToString>(title: T, (width, height): (u32, u32)) -> Context {
        let display = WindowBuilder::new()
            .with_title(title.to_string())
            .with_dimensions(width, height)
            .build_glium()
            .unwrap();
        Context { display: display }
    }

    /// Start draw a new frame.
    pub fn draw<F>(&self, f: F)
        where F: FnOnce(&mut Frame)
    {
        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 0.0);
        f(&mut frame);
        frame.finish().unwrap();
    }

    /// Returns the ratio between the backing framebuffer resolution
    /// and the window size in screen pixels.
    pub fn hidpi_factor(&self) -> f32 {
        self.display.get_window().unwrap().hidpi_factor()
    }
}
