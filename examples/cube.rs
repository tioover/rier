extern crate rier;
#[macro_use] extern crate glium;
extern crate cgmath;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

implement_vertex! {Vertex, position, color}

struct Graphics;


impl rier::Graphics for Graphics {
    type Vertex = Vertex;

    fn vertex() -> &'static str {
r#"
#version 330 core
uniform mat4 matrix;
in vec3 position;
in vec3 color;
out vec3 vColor;

void main()
{
    gl_Position = matrix * vec4(position, 1.0);
    vColor = color;
}
"#
    }

    fn fragment() -> &'static str {
r#"
#version 330 core
in vec3 position;
in vec3 vColor;
out vec4 f_color;

void main()
{
    f_color = vec4(vColor, 1.0);
}
"#
    }
}


fn cube_model(gfx: &rier::Gfx) -> rier::Mesh<Vertex> {
    let indices = [
    0, 1, 2,
    3, 0, 4,
    5, 0, 6,
    3, 6, 0,
    0, 2, 4,
    5, 1, 0,
    2, 1, 5,
    7, 6, 3,
    6, 7, 5,
    7, 3, 4,
    7, 4, 2,
    7, 2, 5];
    let vertices = [
        Vertex { position: [-1.0,-1.0,-1.0], color: [0.6, 0.2, 0.3] },
        Vertex { position: [-1.0,-1.0, 1.0], color: [0.1, 0.8, 0.4] },
        Vertex { position: [-1.0, 1.0, 1.0], color: [0.3, 0.5, 0.5] },
        Vertex { position: [ 1.0, 1.0,-1.0], color: [1.0, 0.6, 1.0] },
        Vertex { position: [-1.0, 1.0,-1.0], color: [1.0, 1.0, 1.0] },
        Vertex { position: [ 1.0,-1.0, 1.0], color: [0.7, 0.9, 0.6] },
        Vertex { position: [ 1.0,-1.0,-1.0], color: [0.0, 0.0, 0.0] },
        Vertex { position: [ 1.0, 1.0, 1.0], color: [0.9, 0.2, 0.5] },
    ];
    rier::Mesh::with_indices(gfx, &vertices, &indices).unwrap()
}

fn main()
{
    let gfx = rier::Context::create("Cube", (800, 600)).gfx();
    let renderer = rier::render::Renderer::<Graphics>::new(gfx.clone()).unwrap();
    let mut camera = rier::Camera3D::new(gfx.clone());
    camera.eye = cgmath::Point3::new(4.0, 3.0, 3.0);
    let mesh = cube_model(&gfx);
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
