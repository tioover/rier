//! Polygon mesh.

use glium;
use glium::index::{PrimitiveType, NoIndices, IndicesSource};
use glium::vertex::{IntoVerticesSource, VerticesSource};
use either::{Either, Left, Right};
use context::Gfx;

pub use glium::VertexBuffer;
pub use glium::Vertex;
pub use glium::index::BufferCreationError as IndexCreationError;
pub use glium::vertex::BufferCreationError as VertexCreationError;


/// A integer represent the vertex index.
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
    pub fn new(gfx: &Gfx, vertices: &[T]) -> Result<Mesh<T>, VertexCreationError> {
        Ok(Mesh {
            vertices: try!(VertexBuffer::new(&gfx.display, vertices)),
            indices: Right(NoIndices(PrimitiveType::TrianglesList)),
        })
    }

    pub fn with_indices(gfx: &Gfx,
                        vertices: &[T],
                        indices: &[Index])
                        -> Result<Mesh<T>, CreationError> {
        Ok(Mesh {
            vertices: try!(VertexBuffer::new(&gfx.display, vertices)),
            indices: Left(try!(IndexBuffer::new(&gfx.display,
                                                PrimitiveType::TrianglesList,
                                                indices))),
        })
    }

    /// Create a mesh with the given buffers.
    pub fn buffer(vertices: VertexBuffer<T>, indices: IndexBuffer) -> Mesh<T> {
        Mesh {
            vertices: vertices,
            indices: Left(indices),
        }
    }
}


impl<'a, T: Vertex> Into<IndicesSource<'a>> for &'a Mesh<T> {
    fn into(self) -> IndicesSource<'a> {
        match self.indices {
            Left(ref x) => x.into(),
            Right(ref x) => x.into(),
        }
    }
}


impl<'a, T: Vertex> IntoVerticesSource<'a> for &'a Mesh<T> {
    fn into_vertices_source(self) -> VerticesSource<'a> {
        self.vertices.into_vertices_source()
    }
}


/// Errors which can occur when attempting to create a mesh.
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
