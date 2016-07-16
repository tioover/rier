//! Component that describes the transform of object.
use num::Zero;
use cgmath::{ Vector3, Matrix4, Basis3, Quaternion };
use utils::Cache;
use Mat;

/// Position, rotation and scale of an object.
pub struct Transform
{
    /// Ojbect scale, default `1`.
    pub scale: f32,
    /// Ojbect translation, default `(0, 0, 0)`.
    pub position: Vector3<f32>,
    /// Object rotation, default do nothing.
    pub rotation: Quaternion<f32>,
    cache: Cache<Mat>,
}


impl Transform
{
    //! Creates an empty transform.
    //! Corresponding with unit matrix.
    pub fn new() -> Transform
    {
        Transform
        {
            scale: 1.0,
            position: Vector3::zero(),
            rotation: Quaternion::zero(),
            cache: Cache::new(),
        }
    }

    /// Transforms point from local space to world space.
    pub fn apply(&self, point: Vector3<f32>) -> Vector3<f32>
    {
        self.rotation * &(point * self.scale) + self.position
    }

    /// Builds transform matrix.
    pub fn matrix<'a>(&'a self) -> &'a Mat
    {
        self.cache.get(|| {
            Matrix4::from_translation(self.position) *
            Matrix4::from(*Basis3::from(self.rotation).as_ref()) *
            Matrix4::from_scale(self.scale)
        })
    }
}
