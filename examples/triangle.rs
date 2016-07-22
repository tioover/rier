#[macro_use] extern crate rier;
#[macro_use] extern crate glium;
extern crate cgmath;

use rier::{AsMatrix, Camera3D};


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex! {Vertex, position, color}

struct Graphics;


impl rier::Graphics for Graphics {
    type Vertex = Vertex;

    fn vertex() -> &'static str {
r#"
#version 140
uniform mat4 matrix;
in vec2 position;
in vec3 color;
out vec3 vColor;
void main()
{
    gl_Position = matrix * vec4(position, 0.0, 1.0);
    vColor = color;
}
"#
    }

    fn fragment() -> &'static str {
r#"
#version 140
in vec3 vColor;
out vec4 f_color;
void main()
{
    f_color = vec4(vColor, 1.0);
}
"#
    }
}



fn main()
{
    let ctx = rier::Context::create("Triangle", (800, 600));
    let renderer = rier::render::Renderer::<Graphics>::new(ctx.clone()).unwrap();
    let mut camera = Camera3D::new(ctx.clone());
    let mesh = rier::Mesh::new(&ctx, &[
            Vertex { position: [-1.0, -1.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 0.0,  1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 1.0, -1.0], color: [1.0, 0.0, 0.0] },
        ]).unwrap();
    'main: loop {
        camera.update();
        for event in ctx.display.poll_events() {
            match event {
                rier::WindowEvent::Closed => break 'main,
                _ => (),
            }
        }
        ctx.draw(|mut target| {
            renderer.draw(&mut target, &mesh, &uniform! { matrix: *camera.array() }).unwrap();
        });
    }
}
