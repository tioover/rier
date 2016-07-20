//! Object rendering management.
use std::marker::PhantomData;
use glium::{Program, DrawParameters, Surface, Blend};
use glium::uniforms::Uniforms;
use mesh::{Mesh, Vertex};
use context::Context;

pub use glium::{Frame, DrawError};
pub use glium::program::ProgramCreationError;


/// Rendering context object.
pub struct Renderer<G>
    where G: Graphics
{
    pub context: Context,
    program: Program,
    params: DrawParameters<'static>,
    _mark: PhantomData<G>,
}


impl<G: Graphics> Renderer<G> {
    /// Creates default renderer.
    pub fn new(context: Context) -> Result<Renderer<G>, ProgramCreationError> {
        let program = try!(G::build(&context));
        let params = G::draw_parameters();
        let renderer = Renderer {
            context: context,
            program: program,
            params: params,
            _mark: PhantomData,
        };
        Ok(renderer)
    }

    /// Draws.
    pub fn draw<U: Uniforms>(&self,
                             target: &mut Frame,
                             mesh: &Mesh<G::Vertex>,
                             uniforms: &U)
                             -> Result<(), DrawError> {
        target.draw(&mesh.vertices,
                    mesh.indices_source(),
                    &self.program,
                    uniforms,
                    &self.params)
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
        DrawParameters { blend: Blend::alpha_blending(), ..::std::default::Default::default() }
    }

    /// Builds a program.
    fn build(ctx: &Context) -> Result<Program, ProgramCreationError> {
        Program::from_source(&ctx.display,
                             Self::vertex(),
                             Self::fragment(),
                             Self::geometry())
    }
}
