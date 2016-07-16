//! [Rier](https://github.com/tioover/rier) is a simple graphics rendering library.
extern crate num;
extern crate cgmath;
#[macro_use] extern crate glium;
extern crate time;
extern crate rand;
extern crate image;
extern crate either;


mod id;
mod transform;
mod event_loop;
pub mod loader;
pub mod texture;
pub mod mesh;
pub mod camera;
pub mod render;
pub mod utils;
pub mod sprite;


/// 4x4 float matrix.
pub type Mat = cgmath::Matrix4<f32>;


pub use id::Id;
pub use render::Graphics;
pub use event_loop::Loop;
pub use transform::Transform;
