use glium::{Display, Frame, DrawError};
use texture;
use mesh;
use render;
use Mat;
use texture::Rect;
use utils::Cache;


type Mesh = mesh::Mesh<Vertex>;


/// 2D vertex type.
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    fn new(x: f32, y: f32, u: i32, v: i32) -> Vertex {
        Vertex {
            position: [x, y],
            tex_coords: [u as f32, v as f32],
        }
    }
}


implement_vertex!{ Vertex, position, tex_coords }



impl render::Graphics for Graphics {
    type Vertex = Vertex;

    fn vertex() -> &'static str {
        include_str!("sprite.vert")
    }

    fn fragment() -> &'static str {
        include_str!("sprite.frag")
    }
}


/// Sprite graphics component.
pub struct Graphics {
    opacity: f32,
    width: f32,
    height: f32,
    texture: texture::Ref,
    texture_rect: Rect,
    mesh_cache: Cache<Mesh>,
}

impl Graphics {
    #[doc(hidden)]
    pub fn new(tex: &texture::Ref, tex_rect: Rect, width: f32, height: f32) -> Graphics {
        Graphics {
            texture_rect: tex_rect,
            width: width,
            height: height,
            texture: tex.clone(),
            opacity: 1.0,
            mesh_cache: Cache::new(),
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn get_mesh<'a>(&'a self, display: &Display) -> &'a Mesh {
        self.mesh_cache.get(|| {

            // Generate mash.
            let (width, height) = (self.width, self.height);
            let (w, h) = (self.texture_rect.w as i32, self.texture_rect.h as i32);
            let (x, y) = (self.texture_rect.x, self.texture_rect.y);
            let verties = [Vertex::new(  0.0, height, x + 0, y + h),
                           Vertex::new(  0.0,    0.0, x + 0, y + 0),
                           Vertex::new(width,    0.0, x + w, y + 0),
                           Vertex::new(width,    0.0, x + w, y + 0),
                           Vertex::new(width, height, x + w, y + h),
                           Vertex::new(  0.0, height, x + 0, y + h)];
            Mesh::new(display, &verties).unwrap()
        })
    }

    /// Renders this sprite.
    pub fn render(&self,
                  target: &mut Frame,
                  renderer: &render::Renderer<Self>,
                  camera: &Mat,
                  transform: &Mat)
                  -> Result<(), DrawError> {
        let camera: &[[_; 4]; 4] = camera.as_ref();
        let transform: &[[_; 4]; 4] = transform.as_ref();
        let uniforms = uniform!
        {
            tex: self.texture.clone(),
            opacity: self.opacity,
            camera: *camera,
            transform: *transform,
        };
        let mesh = self.get_mesh(&renderer.display);
        renderer.draw(target, mesh, &uniforms)
    }
}
