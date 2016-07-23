extern crate rier;
#[macro_use] extern crate glium;


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
    let gfx = rier::Context::create("Triangle", (800, 600)).gfx();
    let renderer = rier::render::Renderer::<Graphics>::new(gfx.clone()).unwrap();
    let mut camera = rier::Camera3D::new(gfx.clone());
    let mesh = rier::Mesh::new(&gfx, &[
            Vertex { position: [-1.0, -1.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 0.0,  1.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 1.0, -1.0], color: [1.0, 0.0, 0.0] },
        ]).unwrap();
    'main: loop {
        camera.update();
        for event in gfx.display.poll_events() {
            match event {
                rier::WindowEvent::Closed => break 'main,
                _ => (),
            }
        }
        gfx.frame(|| {
            renderer.draw(&mesh, &uniform! { matrix: &camera }).unwrap();
        }).unwrap();
    }
}
