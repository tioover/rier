//! Texture management.
use std::path::PathBuf;
use std::collections::{ HashMap, HashSet };
use std::sync::mpsc::{ Receiver, Sender, channel };
use std::thread;
use std::rc::{ Rc, Weak };
use std::ops::Deref;
use std::cell::RefCell;
use image::{ ImageResult, open };
use glium::Display;
use glium::texture::RawImage2d;
use glium::uniforms::{ AsUniformValue, UniformValue };

type RawImage = RawImage2d<'static, u8>;
/// Use a path as key in order to get specific texture.
pub type Key = PathBuf;
/// Default texture type.
pub type Texture = ::glium::texture::CompressedSrgbTexture2d;


/// Reference to the Texture.
#[derive(Clone)]
pub struct TexRef(Rc<Texture>);




impl AsUniformValue for TexRef
{
    fn as_uniform_value(&self) -> UniformValue
    {
        UniformValue::CompressedSrgbTexture2d(&self.0, None)
    }
}


impl Deref for TexRef
{
    type Target = Texture;

    fn deref(&self) -> &Texture { self.0.deref() }
}


fn load_texture_raw_image(path: &Key) -> ImageResult<RawImage>
{
    let image = try!(open(&path)).to_rgba();
    let (dimensions, data) = (image.dimensions(), image.into_raw());
    Ok(RawImage2d::from_raw_rgba_reversed(data, dimensions))
}


/// Texture loader and manager.
pub struct Manager
{
    display: Display,
    textures: RefCell<HashMap<Key, Weak<Texture>>>,
    buffer: RefCell<HashMap<Key, Rc<Texture>>>,
    loading: RefCell<HashSet<Key>>,
    rx: Receiver<(Key, RawImage)>,
    tx: Sender<Key>,
}


impl Manager
{
    /// Creates new texture manager.
    pub fn new(display: &Display) -> Manager
    {
        let (tx, cmd_rx) = channel::<Key>();
        let (data_tx, rx) = channel::<(Key, RawImage)>();

        thread::spawn(move || {

            while let Ok(path) = cmd_rx.recv()
            {
                let image = load_texture_raw_image(&path).unwrap();
                data_tx.send((path, image)).unwrap();
            }
        });

        Manager
        {
            tx: tx,
            rx: rx,
            display: display.clone(),
            textures: RefCell::new(HashMap::new()),
            buffer: RefCell::new(HashMap::new()),
            loading: RefCell::new(HashSet::new()),
        }
    }

    /// Start a new texture asynchronous loading.
    ///
    /// If the texture has already begun to loading, this function will return `None`.
    pub fn push(&self, path: &Key) -> Option<()>
    {
        let textures = self.textures.borrow();
        let buffer = self.buffer.borrow();
        let mut loading = self.loading.borrow_mut();

        if textures.contains_key(path) || buffer.contains_key(path) || loading.contains(path)
        {
            return None
        }

        loading.insert(path.clone());
        self.tx.send(path.clone()).unwrap();
        Some(())
    }

    /// Start a new texture synchronization loading.
    pub fn load(&self, path: &Key) -> TexRef
    {
        let mut textures = self.textures.borrow_mut();
        if textures.contains_key(path)
        {
            if let Some(x) = textures[path].upgrade() { return TexRef(x) }
        }
        let image = load_texture_raw_image(&path).unwrap();
        let tex = Rc::new(Texture::new(&self.display, image).unwrap());
        textures.insert(path.clone(), Rc::downgrade(&tex));
        TexRef(tex)
    }

    /// Get loaded textures.
    ///
    /// If return `None`, the texture is loading.
    /// # Panics
    /// * Texture has never been loaded.
    /// * Texture has been released.
    pub fn get(&self, key: &Key)
        -> Option<TexRef>
    {
        let mut textures = self.textures.borrow_mut();
        let mut buffer = self.buffer.borrow_mut();
        if buffer.contains_key(key)
        {
            let tex_ref = buffer.remove(key).unwrap();
            textures.insert(key.clone(), Rc::downgrade(&tex_ref));
            return Some(TexRef(tex_ref));
        }
        match textures.get(key) {
            None =>
            {
                let loading = self.loading.borrow();
                if loading.contains(key) { None }
                else { panic!(format!("Get a texture must load first. texture path: {:?}", key)) }
            }
            Some(tex) => match tex.upgrade()
            {
                None => panic!(format!("This texture was released. texture path: {:?}", key)),
                Some(tex) => Some(TexRef(tex)),
            }
        }
    }

    /// Receive and process data from asynchronous queue.
    pub fn receive(&self)
    {
        let mut buffer = self.buffer.borrow_mut();
        let mut loading = self.loading.borrow_mut();
        while let Ok((key, data)) = self.rx.try_recv()
        {
            let tex = Texture::new(&self.display, data).unwrap();
            let key_in_loading = loading.remove(&key);
            assert!(key_in_loading);
            buffer.insert(key, Rc::new(tex));
        }
    }
}
