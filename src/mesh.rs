//! Polygon mesh.

use glium;
use glium::index::{PrimitiveType, NoIndices, IndicesSource};
use either::{Either, Left, Right};
use context::Context;

pub use glium::VertexBuffer;
pub use glium::Vertex;
pub use glium::index::BufferCreationError as IndexCreationError;
pub use glium::vertex::BufferCreationError as VertexCreationError;


/// Use `u16` represent index data.
pub type Index = u16;
/// A list of indices loaded in the graphics card's memory.
pub type IndexBuffer = glium::IndexBuffer<Index>;


/// Mesh is a collection of vertices, edges and faces.
pub struct Mesh<T: Vertex> {
    /// Vertex buffer.
    pub vertices: VertexBuffer<T>,
    /// Index buffer or none.
    pub indices: Either<IndexBuffer, NoIndices>,
}


impl<T: Vertex> Mesh<T> {
    /// Creates a simple mesh object.
    /// Primitive type is triangles list, no indices need.
    pub fn new(ctx: &Context, vertices: &[T]) -> Result<Mesh<T>, CreationError> {
        Ok(Mesh {
            vertices: try!(VertexBuffer::new(&ctx.display, vertices)),
            indices: Right(NoIndices(PrimitiveType::TrianglesList)),
        })
    }

    /// Create a mesh with the given buffers.
    pub fn buffer(vertices: VertexBuffer<T>, indices: IndexBuffer) -> Mesh<T> {
        Mesh {
            vertices: vertices,
            indices: Left(indices),
        }
    }

    #[doc(hidden)]
    pub fn indices_source<'a>(&'a self) -> IndicesSource<'a> {
        return match self.indices {
            Left(ref x) => x.into(),
            Right(ref x) => x.into(),
        };
    }
}


/// Errors which can occur when attempting to create a Mesh.
#[derive(Debug)]
pub enum CreationError {
    /// Vertex buffer create failure.
    Vertex(VertexCreationError),
    /// Index buffer create failure.
    Index(IndexCreationError),
}


impl From<IndexCreationError> for CreationError {
    fn from(err: IndexCreationError) -> CreationError {
        CreationError::Index(err)
    }
}


impl From<VertexCreationError> for CreationError {
    fn from(err: VertexCreationError) -> CreationError {
        CreationError::Vertex(err)
    }
}
