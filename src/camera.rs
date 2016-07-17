//! Camera.

use glium::Display;
use transform::Transform;
use Mat;


/// Camera types.
pub trait Camera {
    /// Returns a camera matrix.
    fn matrix(&self) -> Mat;

    fn array(&self) -> [[f32; 4]; 4] {
        self.matrix().into()
    }
}


/// 2D Camera generator.
///
/// Converts screen coordinate to OpenGL world coordinate.
pub struct Camera2D {
    display: Display,
    transform: Transform,
}


impl Camera2D {
    pub fn new(display: &Display) -> Camera2D {
        Camera2D {
            display: display.clone(),
            transform: Transform::new(),
        }
    }
}


impl Camera for Camera2D {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn matrix(&self) -> Mat {
        let factor = self.display.get_window().unwrap().hidpi_factor();
        let (w, h) = self.display.get_framebuffer_dimensions();
        let (w, h) = (w as f32, h as f32);
        let f = factor * 2.0;
        Mat::new(
             f/w,  0.0,  0.0, 0.0,
             0.0, -f/h,  0.0, 0.0,
             0.0,  0.0, -1.0, 0.0,
            -1.0,  1.0,  0.0, 1.0,
        ) * self.transform.matrix()
    }
}
