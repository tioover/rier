use rier::{Context, Transform, Camera2D, AsMatrix, Cache, texture, mesh};
use rier::render;
use rier::render::{Frame, DrawError, Renderer};


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
    texture_rect: texture::Rect,
    mesh_cache: Cache<Mesh>,
}

impl Graphics {
    pub fn new(texture: &texture::Ref, tex_rect: texture::Rect, width: f32, height: f32)
        -> Graphics
    {
        Graphics {
            texture_rect: tex_rect,
            width: width,
            height: height,
            texture: texture.clone(),
            opacity: 1.0,
            mesh_cache: Cache::new(),
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn get_mesh<'a>(&'a self, context: &Context) -> &'a Mesh {
        self.mesh_cache.get(|| {
            // Generate mash.
            let (width, height) = (self.width, self.height);
            let (w, h) = (self.texture_rect.w as i32, self.texture_rect.h as i32);
            let (x, y) = (self.texture_rect.x, self.texture_rect.y);
            let verties = [Vertex::new(  0.0, height, x + 0, y + h),
                           Vertex::new(  0.0,    0.0, x + 0, y + 0),
                           Vertex::new(width,    0.0, x + w, y + 0),
                           Vertex::new(width, height, x + w, y + h),];
            let indices = [0, 1, 2, 3, 2, 0];
            Mesh::with_indices(context, &verties, &indices).unwrap()
        })
    }

    /// Renders this sprite.
    pub fn render(&self,
                  target: &mut Frame,
                  renderer: &Renderer<Self>,
                  camera: &Camera2D,
                  transform: &Transform)
                  -> Result<(), DrawError> {


        let camera = camera.array();
        let transform = transform.array();

        let uniforms = uniform!
        {
            tex: self.texture.clone(),
            opacity: self.opacity,
            camera: *camera,
            transform: *transform,
        };

        let mesh = self.get_mesh(&renderer.context);
        renderer.draw(target, mesh, &uniforms)
    }
}
