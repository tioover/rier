extern crate rier;
#[macro_use] extern crate glium;
extern crate cgmath;
use glium::DrawParameters;
use cgmath::Rad;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

implement_vertex! {Vertex, position, color}


struct Shader;

type Renderer = rier::Renderer<Shader>;

impl rier::Shader for Shader {
    type Vertex = Vertex;

    fn vertex() -> &'static str {
        r#"
        #version 330 core
        uniform mat4 camera;
        uniform mat4 transform;
        in vec3 position;
        in vec3 color;
        out vec3 v_color;

        void main()
        {
            gl_Position = camera * transform * vec4(position, 1.0);
            v_color = color;
        }
        "#
    }

    fn fragment() -> &'static str {
        r#"
        #version 330 core
        in vec3 position;
        in vec3 v_color;
        out vec4 f_color;

        void main()
        {
            f_color = vec4(v_color  , 1.0);
        }
        "#
    }

    fn draw_parameters() -> DrawParameters<'static> {
        use std::default::Default;
        use glium::draw_parameters::{Depth, DepthTest, BackfaceCullingMode};

        let depth = Depth {
            test: DepthTest::IfLess,
            write: true,
            .. Default::default()
        };

        DrawParameters {
            depth: depth,
            backface_culling: BackfaceCullingMode::CullClockwise,
            ..Default::default()
        }
    }
}


struct Cube {
    mesh: rier::Mesh<Vertex>,
    transform: rier::Transform,
}


impl Cube {
    fn new(renderer: &Renderer) -> Cube {
        Cube {
            mesh: Cube::mesh(renderer),
            transform: rier::Transform::new(),
        }
    }

    fn mesh(renderer: &Renderer) -> rier::Mesh<Vertex> {
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
            7, 2, 5
        ];
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
        rier::Mesh::with_indices(renderer, &vertices, &indices).unwrap()
    }

    fn render(&self, renderer: &Renderer, camera: &rier::Camera3D) {
        renderer.draw(&self.mesh, &uniform! {
            camera: camera,
            transform: &self.transform
        }).unwrap();
    }
}


fn main()
{
    let gfx = rier::Context::create("Cube", (800, 600)).gfx();
    let renderer = Renderer::new(gfx.clone()).unwrap();
    let mut camera = rier::Camera3D::new(gfx.clone());
    camera.eye = cgmath::Point3::new(4.0, 3.0, 3.0);
    let mut cube = Cube::new(&renderer);
    let mut x = 0.0f32;
    let main_loop = rier::Loop::new(move |delta| {
        use rier::main_loop::Return::*;

        camera.update();
        x += delta.subsec_nanos() as f32 / 1000000000.0;
        cube.transform.set_rotation(Rad::new(x), Rad::new(x), Rad::new(0.0));
        cube.transform.dirty();
        for event in gfx.display.poll_events() {
            match event {
                rier::WindowEvent::Closed => return Exit,
                _ => (),
            }
        }
        gfx.frame(|| {
            cube.render(&renderer, &camera);
        }).unwrap();
        Next
    });
    main_loop.start();
}
