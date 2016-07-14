#[macro_use] extern crate rier;
#[macro_use] extern crate glium;
extern crate cgmath;
use glium::uniforms::EmptyUniforms;

#[derive(Copy, Clone)]
struct Vertex
{
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex! { Vertex, position, color }

struct Graphics;


impl rier::Graphics for Graphics
{
    type Vertex = Vertex;

    fn vertex() -> &'static str
    {
r#"
#version 140
in vec2 position;
in vec3 color;
out vec3 vColor;
void main()
{
    gl_Position = vec4(position, 0.0, 1.0);
    vColor = color;
}
"#
    }

    fn fragment() -> &'static str
    {
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
    let display = rier::utils::build_display("Triangle", (800, 600));
    let renderer = rier::render::Renderer::<Graphics>::new(&display).unwrap();
    let mesh = rier::mesh::Mesh::new(&display, &[
            Vertex { position: [-1.0, -1.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 0.0,  1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 1.0, -1.0], color: [1.0, 0.0, 0.0] },
        ]).unwrap();
    rier::Loop::new(&display,
        move |mut target|
        {
            let _ = renderer.draw(target, &mesh, &EmptyUniforms).unwrap();
        }).start();
}
