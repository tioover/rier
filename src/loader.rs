//! Resource loader.
use std::sync::mpsc::{Receiver, Sender, channel};
use std::path::{Path, PathBuf};


/// Asynchronous file loader.
pub struct Loader<T: Resource> {
    tx: Sender<(PathBuf, T::Result)>,
    rx: Receiver<(PathBuf, T::Result)>,
}

impl<T: Resource> Loader<T> {
    /// Creates new loader.
    pub fn new() -> Loader<T> {
        let (tx, rx) = channel();
        Loader { tx: tx, rx: rx }
    }

    /// Start load a file.
    pub fn load(&self, path: PathBuf) {
        use std::thread::spawn;
        let tx = self.tx.clone();

        spawn(move || {
            let x = T::load(path.as_path());
            tx.send((path, x))
        });
    }

    /// Gets loaded data.
    pub fn get(&self) -> Vec<(PathBuf, T::Result)> {
        let mut loaded = Vec::new();
        while let Ok(data) = self.rx.try_recv() {
            loaded.push(data)
        }
        loaded
    }
}


/// The type need to load.
pub trait Resource: Sized {
    /// Load result type.
    ///
    /// For example, we need load `Image`, `Result` is `Result<Image, Error>`.
    type Result: Send + 'static;

    /// Synchronize load.
    fn load(path: &Path) -> Self::Result;

    /// Creates resource loader.
    fn loader() -> Loader<Self> {
        Loader::new()
    }
}
