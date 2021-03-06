extern crate rier;
#[macro_use] extern crate glium;
extern crate cgmath;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex! {Vertex, position, color}

struct Shader;

impl rier::Shader for Shader {
    type Vertex = Vertex;

    fn vertex() -> &'static str {
        r#"
        #version 330 core
        uniform mat4 matrix;
        in vec2 position;
        in vec3 color;
        out vec3 v_color;
        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
            v_color = color;
        }
        "#
    }

    fn fragment() -> &'static str {
        r#"
        #version 330 core
        in vec3 v_color;
        out vec4 f_color;
        void main() {
            f_color = vec4(v_color, 1.0);
        }
        "#
    }
}



fn main() {
    let gfx = rier::Context::create("Triangle", (800, 600)).gfx();
    let renderer = rier::Renderer::<Shader>::new(gfx.clone()).unwrap();
    let mut camera = rier::Camera3D::new(gfx.clone());
    camera.eye = cgmath::Point3::new(4.0, 3.0, 3.0);
    let mesh = rier::Mesh::new(&renderer, &[
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
