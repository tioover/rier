//! Utility functions.
use std::cell::UnsafeCell;
use num::NumCast;


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
/// # Example
///
/// ```
/// use rier::utils::Cache;
///
/// let cache = Cache::new();
/// assert_eq!(cache.get(|| 42i32), &42i32);
/// ```
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
