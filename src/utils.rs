//! Utility functions.
use std::cell::UnsafeCell;
use std::rc::Rc;
use std::ops::Deref;
use cgmath::Matrix4;
use glium::uniforms::{AsUniformValue, UniformValue};

/// 4x4 float matrix.
pub type Matrix = Matrix4<f32>;


/// A object that cache the data.
/// # Example
///
/// ```
/// use rier::Cache;
///
/// let mut cache = Cache::<&'static str>::new();
///
/// assert_eq!(cache.get(|| "Madoka"), &"Madoka");
/// assert_eq!(cache.get(|| "Homura"), &"Madoka"); // Nothing change.
/// cache.dirty(); // Make data dirty.
/// assert_eq!(cache.get(|| "Homura"), &"Homura");
/// ```
pub struct Cache<T> {
    data: UnsafeCell<Option<T>>,
}


impl<T> Cache<T> {
    /// Creates a empty cache cell.
    pub fn new() -> Cache<T> {
        Cache { data: UnsafeCell::new(None) }
    }

    /// Mark the data should be updated.
    pub fn dirty(&mut self) {
        self.data = UnsafeCell::new(None)
    }

    /// Gets cached object, If the object has not been cached, creates it first.
    pub fn get<F>(&self, builder: F) -> &T
        where F: FnOnce() -> T
    {
        unsafe {
            let mut cache = self.data.get().as_mut().unwrap();
            if cache.is_none() {
                *cache = Some(builder())
            }
            cache.as_ref().unwrap()
        }
    }

    /// Returns reference only if data not dirty.
    pub fn try_get(&self) -> Option<&T> {
        unsafe { self.data.get().as_ref().unwrap().as_ref() }
    }

    /// Consumes and returning the wrapped value.
    pub fn unwrap(self) -> Option<T> {
        unsafe { self.data.into_inner() }
    }
}


pub trait AsMatrix {
    fn matrix(&self) -> &Matrix;

    fn array(&self) -> &[[f32; 4]; 4] {
        self.matrix().as_ref()
    }
}


impl AsMatrix for Matrix {
    fn matrix(&self) -> &Matrix {
        self
    }
}


/// Reference to the glium uniforms type.
#[derive(Clone)]
pub struct Ref<T: AsUniformValue>(Rc<T>);


impl<T: AsUniformValue> Ref<T> {
    pub fn new(data: T) -> Ref<T> {
        Ref(Rc::new(data))
    }
}


impl<T: AsUniformValue> AsUniformValue for Ref<T> {
    fn as_uniform_value(&self) -> UniformValue {
        use std::mem::transmute;
        let &Ref(ref tex) = self;
        // type system issue.
        unsafe { transmute(tex.deref().as_uniform_value()) }
    }
}


impl<T: AsUniformValue> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let &Ref(ref tex) = self;
        tex.deref()
    }
}
