//! Component that describes the transform of object.

use num::{Zero, One};
use cgmath::{Vector3, Matrix4, Basis3, Quaternion};
use utils::{AsMatrix, Matrix};

pub type Position = Vector3<f32>;
pub type Scale = f32;
pub type Rotation = Quaternion<f32>;


/// Position, rotation and scale of an object.
/// # Example
///
/// ```
/// use rier::{AsMatrix, Transform};
///
/// let mut transform = Transform::new();
/// transform.set_position(100.0, 100.0, 0.0);
/// let _ = transform.matrix();
/// ```
pub struct Transform {
    /// Ojbect scale, default `1`.
    pub scale: Scale,
    /// Ojbect translation, default `(0, 0, 0)`.
    pub position: Position,
    /// Object rotation, default do nothing.
    pub rotation: Rotation,
    matrix: Matrix,
}


impl Transform {
    /// Creates an empty transform.
    pub fn new() -> Transform {
        Transform {
            scale: 1.0,
            position: Vector3::zero(),
            rotation: Quaternion::zero(),
            matrix: Matrix::one(),
        }
    }

    fn build_matrix(&self) -> Matrix {
        Matrix4::from_translation(self.position) *
        Matrix4::from(*Basis3::from(self.rotation).as_ref()) *
        Matrix4::from_scale(self.scale)
    }

    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut Position, &mut Rotation, &mut Scale)
    {
        f(&mut self.position, &mut self.rotation, &mut self.scale);
        self.matrix = self.build_matrix();
    }

    /// Sets transform position.
    #[inline]
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.modify(|mut pos, _, _| *pos = Position::new(x, y, z));
    }

    /// Sets transform scale.
    #[inline]
    pub fn set_scale(&mut self, n: f32) {
        self.modify(|_, _, mut scale| *scale = n);
    }

    /// Transforms point from local space to world space.
    pub fn apply(&self, point: Vector3<f32>) -> Vector3<f32> {
        self.rotation * &(point * self.scale) + self.position
    }
}


impl AsMatrix for Transform {
    fn matrix(&self) -> &Matrix {
        &self.matrix
    }
}
