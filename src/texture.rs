//! Texture management.
use std::path::Path;
use std::rc::Rc;
use std::ops::Deref;
use image::{ImageResult, open};
use glium::texture::{RawImage2d, TextureCreationError, CompressedSrgbTexture2d};
use glium::uniforms::{AsUniformValue, UniformValue};
use loader::Resource;
use context::Gfx;


/// Default texture type.
pub type Texture = CompressedSrgbTexture2d;


/// Raw image data.
pub struct Raw(RawImage2d<'static, u8>);

impl Raw {
    /// Creates texture use data.
    pub fn process(self, ctx: &Gfx) -> Result<Texture, TextureCreationError> {
        let Raw(image) = self;
        Texture::new(&ctx.display, image)
    }
}

impl Resource for Raw {
    type Result = ImageResult<Raw>;

    fn load(path: &Path) -> ImageResult<Raw> {
        let image = try!(open(path)).to_rgba();
        let (dimensions, data) = (image.dimensions(), image.into_raw());
        Ok(Raw(RawImage2d::from_raw_rgba_reversed(data, dimensions)))
    }
}


/// Reference to the Texture.
#[derive(Clone)]
pub struct Ref(Rc<Texture>);


impl Ref {
    pub fn new(texture: Texture) -> Ref {
        Ref(Rc::new(texture))
    }
}


impl AsUniformValue for Ref {
    fn as_uniform_value(&self) -> UniformValue {
        use std::mem::transmute;
        let &Ref(ref tex) = self;
        // type system issue.
        unsafe { transmute(tex.deref().as_uniform_value()) }
    }
}


impl Deref for Ref {
    type Target = Texture;

    fn deref(&self) -> &Texture {
        let &Ref(ref tex) = self;
        tex.deref()
    }
}


/// Texture rectangle.
pub struct Rect {
    /// Rectangle left-up x-coordinate value.
    pub x: i32,
    /// Rectangle left-up y-coordinate value.
    pub y: i32,
    /// Rectangle width.
    pub w: u32,
    /// Rectangle height.
    pub h: u32,
}
