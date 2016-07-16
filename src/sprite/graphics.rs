use std::cell::RefCell;
use glium::{ Display, Frame, DrawError };
use texture;
use mesh;
use render;
use Mat;
use texture::Rect;


type Mesh = mesh::Mesh<Vertex>;


/// 2D vertex type.
#[derive(Copy, Clone)]
pub struct Vertex
{
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

impl Vertex
{
    fn new(x: f32, y: f32, u: i32, v: i32) -> Vertex
    {
        Vertex
        {
            position: [x, y],
            tex_coords: [u as f32, v as f32],
        }
    }
}


implement_vertex!{ Vertex, position, tex_coords }



impl render::Graphics for Graphics
{
    type Vertex = Vertex;

    fn vertex() -> &'static str
    {
        include_str!("sprite.vert")
    }

    fn fragment() -> &'static str
    {
        include_str!("sprite.frag")
    }
}


/// Sprite graphics component.
pub struct Graphics
{
    opacity: f32,
    width: f32,
    height: f32,
    texture: texture::Ref,
    texture_rect: Rect,
    mesh_cache: RefCell<Option<Mesh>>,
}

impl Graphics
{
    #[doc(hidden)]
    pub fn new(tex: &texture::Ref, tex_rect: Rect, width: f32, height: f32) -> Graphics
    {
        Graphics
        {
            texture_rect: tex_rect,
            width: width,
            height: height,
            texture: tex.clone(),
            opacity: 1.0,
            mesh_cache: RefCell::new(None),
        }
    }

    fn build_mesh(&self, display: &Display)
    {
        // Get cache.
        let mut mesh_cache = self.mesh_cache.borrow_mut();
        if mesh_cache.is_some() { return }
        // Generate mash.
        let (width, height) = (self.width, self.height);
        let (w, h) = (self.texture_rect.w as i32, self.texture_rect.h as i32);
        let (x, y) = (self.texture_rect.x, self.texture_rect.y);
        let verties = [
            Vertex::new(  0.0, height, x+0, y+h),
            Vertex::new(  0.0,    0.0, x+0, y+0),
            Vertex::new(width,    0.0, x+w, y+0),
            Vertex::new(width,    0.0, x+w, y+0),
            Vertex::new(width, height, x+w, y+h),
            Vertex::new(  0.0, height, x+0, y+h),
        ];
        // Update cache.
        *mesh_cache = Some(Mesh::new(display, &verties).unwrap())
    }

    /// Renders this sprite.
    pub fn render(&self, target: &mut Frame, renderer: &render::Renderer<Self>,
                  camera: &Mat, transform: &Mat)
        -> Result<(), DrawError>
    {
        let camera: &[[_; 4]; 4] = camera.as_ref();
        let transform: &[[_; 4]; 4] = transform.as_ref();
        let uniforms = uniform!
        {
            tex: self.texture.clone(),
            opacity: self.opacity,
            camera: *camera,
            transform: *transform,
        };
        self.build_mesh(&renderer.display);
        let mesh = self.mesh_cache.borrow();
        renderer.draw(target, mesh.as_ref().unwrap(), &uniforms)
    }
}
