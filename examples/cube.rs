extern crate rier;
#[macro_use] extern crate glium;
extern crate cgmath;
use glium::DrawParameters;
use rier::render::Renderer;
use cgmath::{Quaternion, Rotation3, Rad};

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
    fn new(gfx: &rier::Gfx) -> Cube {
        Cube {
            mesh: Cube::mesh(gfx),
            transform: rier::Transform::new(),
        }
    }

    fn mesh(gfx: &rier::Gfx) -> rier::Mesh<Vertex> {
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
        rier::Mesh::with_indices(gfx, &vertices, &indices).unwrap()
    }

    fn render(&self, renderer: &Renderer<Graphics>, camera: &rier::Camera3D) {
        use rier::AsMatrix;

        let matrix: [[_; 4]; 4] = (camera.matrix() * self.transform.matrix()).into();
        renderer.draw(&self.mesh, &uniform! { matrix: matrix }).unwrap();
    }
}


fn main()
{
    let gfx = rier::Context::create("Cube", (800, 600)).gfx();
    let renderer = Renderer::<Graphics>::new(gfx.clone()).unwrap();
    let mut camera = rier::Camera3D::new(gfx.clone());
    camera.eye = cgmath::Point3::new(4.0, 3.0, 3.0);
    let mut cube = Cube::new(&gfx);
    let mut x = 0.0f32;
    'main: loop {
        camera.update();
        x += 0.0025;
        cube.transform.modify(|_, mut rotation, _| {
            *rotation = Quaternion::from_angle_y(Rad::new(x));
        });
        for event in gfx.display.poll_events() {
            match event {
                rier::WindowEvent::Closed => break 'main,
                _ => (),
            }
        }
        gfx.frame(|| {
            cube.render(&renderer, &camera);
        }).unwrap();
    }
}
