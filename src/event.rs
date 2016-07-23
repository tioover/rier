//! Event notification system.
//!
//! Use the observer design pattern.
//!
//! # Example
//!
//! ```
//! use rier::event::{Notifier, Return};
//!
//! let mut notifier = Notifier::<i32>::new();
//! notifier.register(|e| { assert_eq!(e, &42); Return::Next });
//! notifier.notify(42);
//! ```

/// Callback function returns.
pub enum Return<E> {
    /// Nothing happen.
    Next,
    /// The callback will be moved.
    Dead,
    /// Start notify this new event.
    Spwan(Box<E>),
}

/// Event sender.
pub struct Notifier<E> {
    subscribers: Vec<Box<Fn(&E) -> Return<E>>>,
}


impl<E> Notifier<E> {
    pub fn new() -> Notifier<E> {
        Notifier { subscribers: Vec::new() }
    }

    /// Register event callback function.
    pub fn register<F>(&mut self, callback: F)
        where F: 'static + Fn(&E) -> Return<E>
    {
        self.subscribers.push(Box::new(callback));
    }

    /// Notify new event.
    pub fn notify(&mut self, event: E) {
        self.subscribers.retain(|f| {
            match f(&event) {
                Return::Dead => false,
                Return::Next => true,
                _ => unimplemented!(),
            }
        })
    }
}
