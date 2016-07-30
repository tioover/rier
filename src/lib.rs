//! A simple graphics rendering library.

extern crate num;
extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate time;
extern crate rand;
extern crate image;
extern crate fps_counter;


mod id;
mod utils;
mod transform;
pub mod main_loop;
pub mod context;
pub mod camera;
pub mod event;
pub mod loader;
pub mod texture;
pub mod mesh;
pub mod render;



pub use id::Id;
pub use mesh::Mesh;
pub use main_loop::Loop;
pub use render::{Renderer, Shader};
pub use camera::{Camera2D, Camera3D};
pub use transform::Transform;
pub use context::{Gfx, Context};
pub use utils::{Matrix, AsMatrix, Cache};
pub use glium::glutin::Event as WindowEvent;
