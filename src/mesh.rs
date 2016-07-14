//! Polygon mesh.

use glium;
use glium::Display;
use glium::{ index, vertex };
use glium::index::{ PrimitiveType, NoIndices, IndicesSource };
use either::{ Either, Left, Right };

pub use glium::VertexBuffer;
pub use glium::Vertex;


/// Use `u16` represent index data.
pub type Index = u16;
/// A list of indices loaded in the graphics card's memory.
pub type IndexBuffer = glium::IndexBuffer<Index>;


/// Mesh is a collection of vertices, edges and faces.
pub struct Mesh<T: Vertex>
{
    /// Vertex buffer.
    pub vertices: VertexBuffer<T>,
    /// Index buffer or none.
    pub indices: Either<IndexBuffer, NoIndices>,
}


impl<T: Vertex> Mesh<T>
{
    /// Creates a simple mesh object.
    /// Primitive type is triangles list, no indices need.
    pub fn new(display: &Display, vertices: &[T])
        -> Result<Mesh<T>, CreationError>
    {
        Ok(Mesh
        {
            vertices: try!(VertexBuffer::new(display, vertices)),
            indices: Right(NoIndices(PrimitiveType::TrianglesList)),
        })
    }

    /// Create a mesh with the given buffers.
    pub fn buffer(vertices: VertexBuffer<T>, indices: IndexBuffer) -> Mesh<T>
    {
        Mesh { vertices: vertices, indices: Left(indices) }
    }

    #[doc(hidden)]
    pub fn indices_source<'a>(&'a self) -> IndicesSource<'a>
    {
        return match self.indices
        {
            Left(ref x) => x.into(),
            Right(ref x) => x.into(),
        };
    }
}


/// Errors which can occur when attempting to create a Mesh.
#[derive(Debug)]
pub enum CreationError
{
    /// Vertex buffer create failure.
    Vertex(vertex::BufferCreationError),
    /// Index buffer create failure.
    Index(index::BufferCreationError),
}


impl From<index::BufferCreationError> for CreationError
{
    fn from(err: index::BufferCreationError) -> CreationError
    {
        CreationError::Index(err)
    }
}


impl From<vertex::BufferCreationError> for CreationError
{
    fn from(err: vertex::BufferCreationError) -> CreationError
    {
        CreationError::Vertex(err)
    }
}
