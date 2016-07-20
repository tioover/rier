//! A simple graphics rendering library.
extern crate num;
extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate time;
extern crate rand;
extern crate image;
extern crate either;


mod id;
mod transform;
mod context;
pub mod event;
pub mod loader;
pub mod texture;
pub mod mesh;
pub mod camera;
pub mod render;
pub mod utils;


/// 4x4 float matrix.
pub type Mat = cgmath::Matrix4<f32>;


pub use id::Id;
pub use mesh::Mesh;
pub use render::Graphics;
pub use transform::Transform;
pub use context::{Context, SwapBuffersError};
pub use glium::glutin::Event as WindowEvent;
