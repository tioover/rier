//! Camera.
use num::One;
use cgmath::{Ortho, PerspectiveFov, Rad, Deg, deg, Point3, vec3};
use glium::uniforms::{AsUniformValue, UniformValue};
use context::Gfx;
use transform::Transform;
use utils::AsMatrix;
use Matrix;


/// 2D Camera generator.
///
/// Converts screen coordinate to OpenGL world coordinate.
pub struct Camera2D {
    gfx: Gfx,
    transform: Transform,
    matrix: Matrix,
}


impl Camera2D {
    pub fn new(gfx: Gfx) -> Camera2D {
        let transform = Transform::new();

        Camera2D {
            matrix: Camera2D::build_matrix(&gfx, transform.matrix()),
            gfx: gfx,
            transform: transform,
        }
    }

    fn build_matrix(gfx: &Gfx, transform: &Matrix) -> Matrix {
        let (w, h) = gfx.display.get_framebuffer_dimensions();
        let ortho = Ortho {
            left: 0.0,
            right: w as f32,
            bottom: 0.0,
            top: h as f32,
            near: -1.0,
            far: 1.0,
        };
        Matrix::from(ortho) * transform
    }

    pub fn update(&mut self) {
        self.matrix = Camera2D::build_matrix(&self.gfx, self.transform.matrix());
    }
}


impl AsMatrix for Camera2D {
    fn matrix(&self) -> &Matrix {
        &self.matrix
    }
}


pub struct Camera3D {
    gfx: Gfx,
    pub pov: Deg<f32>,
    pub near: f32,
    pub far: f32,
    pub eye: Point3<f32>,
    pub center: Point3<f32>,
    matrix: Matrix,
}


impl Camera3D {
    pub fn new(gfx: Gfx) -> Camera3D {
        let mut camera = Camera3D {
            gfx: gfx,
            pov: deg(45.0),
            near: 0.1,
            far: 100.0,
            eye: Point3::new(0.0, 0.0, 1.0),
            center: Point3::new(0.0, 0.0, 0.0),
            matrix: Matrix::one(),
        };
        camera.update();
        camera
    }

    fn build_matrix(&self) -> Matrix {
        let persp = PerspectiveFov {
            fovy: Rad::from(self.pov),
            aspect: self.aspect(),
            near: self.near,
            far: self.far,
        };
        let view = Matrix::look_at(self.eye, self.center, vec3(0.0, 1.0, 0.0));
        Matrix::from(persp) * view
    }

    fn aspect(&self) -> f32 {
        let (w, h) = self.gfx.display.get_framebuffer_dimensions();
        w as f32 / h as f32
    }

    pub fn update(&mut self) {
        self.matrix = self.build_matrix();
    }
}


impl AsMatrix for Camera3D {
    fn matrix(&self) -> &Matrix {
        &self.matrix
    }
}


impl<'a> AsUniformValue for &'a Camera2D {
    fn as_uniform_value(&self) -> UniformValue {
        self.array().as_uniform_value()
    }
}

impl<'a> AsUniformValue for &'a Camera3D {
    fn as_uniform_value(&self) -> UniformValue {
        self.array().as_uniform_value()
    }
}
