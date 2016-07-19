//! Camera.
use cgmath::Ortho;
use context::Context;
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
    context: Context,
    transform: Transform,
}


impl Camera2D {
    pub fn new(context: Context) -> Camera2D {
        Camera2D {
            context: context,
            transform: Transform::new(),
        }
    }
}


impl Camera for Camera2D {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn matrix(&self) -> Mat {
        let (w, h) = self.context.display.get_framebuffer_dimensions();
        let ortho = Ortho {
            left: 0.0,
            right: w as f32,
            bottom: 0.0,
            top: h as f32,
            near: -1.0,
            far: 1.0,
        };
        Mat::from(ortho) * self.transform.matrix()
    }
}
