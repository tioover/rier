//! Component that describes the transform of object.
use std::default::Default;
use num::Zero;
use cgmath::{Vector3, Matrix4, Quaternion, Rotation3, Rad};
use utils::{AsMatrix, Matrix, Cache};
use glium::uniforms::{AsUniformValue, UniformValue};


/// Position, rotation and scale of an object.
/// # Example
///
/// ```
/// use rier::{AsMatrix, Transform};
///
/// let mut transform = Transform::new();
/// transform.set_position(100.0, 100.0, 0.0);
/// transform.dirty();
/// let _ = transform.matrix();
/// ```
pub struct Transform {
    /// Object scale, default `1`.
    pub scale: f32,
    /// Object translation, default `(0, 0, 0)`.
    pub position: Vector3<f32>,
    /// Object rotation, default do nothing.
    pub rotation: Quaternion<f32>,
    matrix: Cache<Matrix>,
}


impl Transform {
    /// Creates an empty transform.
    pub fn new() -> Transform {
        Transform {
            scale: 1.0,
            position: Vector3::zero(),
            rotation: Quaternion::zero(),
            matrix: Cache::new(),
        }
    }


    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Vector3::new(x, y, z);
    }

    pub fn set_scale(&mut self, n: f32) {
        self.scale = n;
    }

    pub fn set_rotation(&mut self, pitch: Rad<f32>, yaw: Rad<f32>, roll: Rad<f32>) {
        self.rotation = Quaternion::from_angle_x(pitch) * Quaternion::from_angle_y(yaw) *
                        Quaternion::from_angle_z(roll);
    }

    /// Mart the data were dirty.
    ///
    /// after modifying the transform data, you must call this method.
    pub fn dirty(&mut self) {
        self.matrix.dirty()
    }

    fn build_matrix(&self) -> Matrix {
        let translation = Matrix4::from_translation(self.position);
        let rotation = Matrix4::from(self.rotation);
        let scale = Matrix4::from_scale(self.scale);
        translation * rotation * scale
    }


    /// Transforms point from local space to world space.
    pub fn compute(&self, point: Vector3<f32>) -> Vector3<f32> {
        self.rotation * (&(point * self.scale) + self.position)
    }
}


impl AsMatrix for Transform {
    fn matrix(&self) -> &Matrix {
        self.matrix.get(|| self.build_matrix())
    }
}


impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> AsUniformValue for &'a Transform {
    fn as_uniform_value(&self) -> UniformValue {
        self.array().as_uniform_value()
    }
}
