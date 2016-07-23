//! 2D Texture management.
use std::path::Path;
use image::{ImageResult, open};
use glium::texture::{RawImage2d, TextureCreationError, CompressedSrgbTexture2d};
use loader::Resource;
use context::Gfx;


/// Default texture type.
pub type Texture = CompressedSrgbTexture2d;


/// Raw image data.
pub struct RawImage(RawImage2d<'static, u8>);

impl RawImage {
    /// Creates texture use data.
    pub fn process(self, ctx: &Gfx) -> Result<Texture, TextureCreationError> {
        let RawImage(image) = self;
        Texture::new(&ctx.display, image)
    }
}

impl Resource for RawImage {
    type Result = ImageResult<RawImage>;

    fn load(path: &Path) -> ImageResult<RawImage> {
        let image = try!(open(path)).to_rgba();
        let (dimensions, data) = (image.dimensions(), image.into_raw());
        Ok(RawImage(RawImage2d::from_raw_rgba_reversed(data, dimensions)))
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
