//! Event notification system.
//!
//! Use Observer Design Pattern.


/// Callback function returns.
pub enum Return {
    /// Nothing.
    None,
    /// The callback will be moved.
    Dead,
}

/// Event sender.
///
/// # Example
///
/// ```
/// use rier::event::{Notifier, Return};
///
/// let mut notifier = Notifier::new();
/// notifier.register(|e| {assert_eq!(e, &42i32); Return::None});
/// notifier.notify(42i32);
/// ```
pub struct Notifier<E> {
    subscribers: Vec<Box<Fn(&E) -> Return>>,
}


impl<E> Notifier<E> {
    pub fn new() -> Notifier<E> {
        Notifier { subscribers: Vec::new() }
    }

    /// Register event callback function.
    pub fn register<F>(&mut self, callback: F)
        where F: 'static + Fn(&E) -> Return
    {
        self.subscribers.push(Box::new(callback));
    }

    /// Notify new event.
    pub fn notify(&mut self, event: E) {
        self.subscribers.retain(|f| {
            match f(&event) {
                Return::Dead => false,
                _ => true,
            }
        })
    }
}
