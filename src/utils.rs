//! Utility functions.
use std::string::ToString;
use std::cell::UnsafeCell;
use num::NumCast;
use glium::glutin::WindowBuilder;
use glium::{Display, DisplayBuild};


/// Builds OpenGL context and create a window.
pub fn build_display<T>(title: T, (width, height): (u32, u32)) -> Display
    where T: ToString
{
    WindowBuilder::new()
        .with_title(title.to_string())
        .with_dimensions(width, height)
        .with_vsync()
        .build_glium()
        .unwrap()
}


/// Number types cast function.
/// # Example
///
/// ```
/// use rier::utils::cast;
///
/// fn foobar(x: u32) {}
///
/// foobar(cast(42i32));
/// ```
#[inline]
pub fn cast<T, U>(x: T) -> U
    where T: NumCast,
          U: NumCast
{
    U::from(x).unwrap()
}


/// Puts the current thread to sleep for the specified amount of milliseconds.
pub fn sleep_ms(ms: u32) {
    use std::thread::sleep;
    use std::time::Duration;
    sleep(Duration::from_millis(ms as u64));
}


/// Cache cell.
pub struct Cache<T> {
    data: UnsafeCell<Option<T>>,
}


impl<T> Cache<T> {
    /// Creates a empty cache object.
    pub fn new() -> Cache<T> {
        Cache { data: UnsafeCell::new(None) }
    }

    /// Mark data dirty.
    pub fn dirty(&mut self) {
        self.data = UnsafeCell::new(None)
    }

    /// Gets cached object, If object is uninitialized, initialize it first.
    pub fn get<'a, F>(&'a self, builder: F) -> &'a T
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
}
