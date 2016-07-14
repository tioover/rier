//! Object rendering management.
use std::marker::PhantomData;
use glium::{ Display, Program, DrawParameters, Frame, Surface, Blend, DrawError };
use glium::program::ProgramCreationError;
use glium::uniforms::Uniforms;
use mesh::{ Mesh, Vertex };


/// Rendering context object.
pub struct Renderer<G>
    where G: Graphics
{
    pub display: Display,
    program: Program,
    params: DrawParameters<'static>,
    _mark: PhantomData<G>,
}


impl<G: Graphics> Renderer<G>
{
    /// Creates default renderer.
    pub fn new(display: &Display)
        -> Result<Renderer<G>, ProgramCreationError>
    {
        let program = try!(G::build(display));
        let params = G::draw_parameters();
        let renderer = Renderer
        {
            display: display.clone(),
            program: program,
            params: params,
            _mark: PhantomData,
        };
        Ok(renderer)
    }

    /// Draws.
    pub fn draw<U: Uniforms>(&self, target: &mut Frame, mesh: &Mesh<G::Vertex>, uniforms: &U)
        -> Result<(), DrawError>
    {
        target.draw(&mesh.vertices, mesh.indices_source(), &self.program, uniforms, &self.params)
    }
}


/// A marker that provides renderer sittings.
pub trait Graphics
{
    type Vertex: Vertex;

    /// Source code of the vertex shader.
    fn vertex() -> &'static str;

    /// Source code of the fragment shader.
    fn fragment() -> &'static str;

    /// Source code of the geometry shader.
    fn geometry() -> Option<&'static str> { None }

    /// Represents the parameters to use when drawing.
    fn draw_parameters() -> DrawParameters<'static>
    {
        DrawParameters
        {
            blend: Blend::alpha_blending(),
            ..::std::default::Default::default()
        }
    }

    /// Builds a program.
    fn build(display: &Display)
        -> Result<Program, ProgramCreationError>
    {
        Program::from_source(
            display,
            Self::vertex(),
            Self::fragment(),
            Self::geometry()
        )
    }
}
