//! Object rendering management.
use std::marker::PhantomData;
use std::default::Default;
use glium::{Program, DrawParameters, Blend};
use glium::uniforms::Uniforms;
use mesh::{Mesh, Vertex};
use context::{Gfx, Surface, DrawError};

pub use glium::program::ProgramCreationError;


/// A context object for rendering.
pub struct Renderer<G>
    where G: Graphics
{
    pub gfx: Gfx,
    program: Program,
    params: DrawParameters<'static>,
    _mark: PhantomData<G>,
}


impl<G: Graphics> Renderer<G> {
    /// Creates default renderer.
    pub fn new(gfx: Gfx) -> Result<Renderer<G>, ProgramCreationError> {
        let program = try!(G::build(&gfx));
        let params = G::draw_parameters();
        Ok(Renderer {
            gfx: gfx,
            program: program,
            params: params,
            _mark: PhantomData,
        })
    }

    /// Draw with current frame.
    pub fn draw<U: Uniforms>(&self, mesh: &Mesh<G::Vertex>, uniforms: &U) -> Result<(), DrawError> {
        let mut target = self.gfx.get_frame_mut();
        self.draw_with_target(&mut *target, mesh, uniforms)
    }


    /// Draws with specified surface.
    pub fn draw_with_target<T, U>(&self,
                                  target: &mut T,
                                  mesh: &Mesh<G::Vertex>,
                                  uniforms: &U)
                                  -> Result<(), DrawError>
        where T: Surface,
              U: Uniforms
    {
        target.draw(mesh, mesh, &self.program, uniforms, &self.params)
    }
}


/// A marker that provides renderer sittings.
pub trait Graphics {
    type Vertex: Vertex;

    /// Source code of the vertex shader.
    fn vertex() -> &'static str;

    /// Source code of the fragment shader.
    fn fragment() -> &'static str;

    /// Source code of the geometry shader.
    fn geometry() -> Option<&'static str> {
        None
    }

    /// Represents the parameters to use when drawing.
    fn draw_parameters() -> DrawParameters<'static> {
        DrawParameters { blend: Blend::alpha_blending(), ..Default::default() }
    }

    /// Builds a program.
    fn build(ctx: &Gfx) -> Result<Program, ProgramCreationError> {
        Program::from_source(&ctx.display,
                             Self::vertex(),
                             Self::fragment(),
                             Self::geometry())
    }
}
