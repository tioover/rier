extern crate rier;
#[macro_use] extern crate glium;
extern crate cgmath;
extern crate wavefront_obj;

use std::fs::File;
use std::io::Read;
use wavefront_obj::obj;
use glium::DrawParameters;
use cgmath::{Rad, Point3};


type Mesh = rier::Mesh<Vertex>;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

implement_vertex! {Vertex, position, normal}


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
        in vec3 normal;
        out vec3 v_normal;
        out vec3 pixel_pos;

        void main() {
            gl_Position = camera * transform * vec4(position, 1.0);
            pixel_pos = vec3(transform * vec4(position, 1.0f));
            v_normal = normalize(mat3(transpose(inverse(transform))) * normal);
        }
        "#
    }

    fn fragment() -> &'static str {
        r#"
        #version 330 core
        uniform vec3 light_pos;
        uniform vec3 view_pos;
        in vec3 v_normal;
        in vec3 pixel_pos;
        out vec4 f_color;

        vec3 light_color = vec3(1.0, 1.0, 1.0);
        vec3 light_dir = normalize(light_pos - pixel_pos);

        vec3 diffuse() {
            float diff = max(dot(v_normal, light_dir), 0.0);
            return diff * light_color;
        }

        vec3 ambient() {
            float strength = 0.1f;
            return strength * light_color;
        }

        vec3 specular() {
            float strength = 0.5f;
            vec3 view_dir = normalize(view_pos - pixel_pos);
            vec3 reflect_dir = reflect(-light_dir, v_normal);
            float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
            return strength * spec * light_color;
        }

        void main() {
            vec3 obj_color = vec3(1.0, 0.5, 0.31);
            vec3 result = (ambient()+diffuse()+specular()) * obj_color;
            f_color = vec4(result, 1.0);
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

struct Model {
    mesh: Mesh,
    transform: rier::Transform,
}


impl Model {
    fn new(renderer: &Renderer) -> Model {
        let mesh = Model::mesh(renderer);


        Model {
            mesh: mesh,
            transform: rier::Transform::new(),
        }
    }

    fn load() -> obj::Object {
        let mut file = File::open("./examples/assets/model.obj").unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        let mut objects = obj::parse(s).unwrap();
        objects.objects.pop().unwrap()
    }

    fn make_vertex(object: &obj::Object, index: obj::VTNIndex) -> Vertex {
        let (v, _, n) = index;
        let n = n.unwrap();
        let vertex = object.vertices[v];
        let normal = object.normals[n];
        Vertex {
            position: [vertex.x as f32, vertex.y as f32, vertex.z as f32],
            normal  : [normal.x as f32, normal.y as f32, normal.z as f32],
        }
    }

    fn mesh(renderer: &Renderer) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let object = Model::load();
        for geometry in &object.geometry {
            for shape in &geometry.shapes {
                match shape {
                    &obj::Shape::Triangle(a, b, c) => {
                        vertices.push(Model::make_vertex(&object, a));
                        vertices.push(Model::make_vertex(&object, b));
                        vertices.push(Model::make_vertex(&object, c));
                    }
                    _ => unimplemented!(),
                }

            }
        }
        Mesh::new(renderer, &*vertices).unwrap()
    }

    fn render(&self, renderer: &Renderer, camera: &rier::Camera3D, light: Point3<f32>) {
        type Point = [f32; 3];
        let light: Point = light.into();
        let view_pos: Point = camera.eye.into();

        let uniforms = uniform! {
            camera: camera,
            transform: &self.transform,
            light_pos: light,
            view_pos: view_pos,
        };
        renderer.draw(&self.mesh, &uniforms).unwrap();
    }
}


fn main() {
    let gfx = rier::Context::create("Cube", (800, 600)).gfx();
    let renderer = Renderer::new(gfx.clone()).unwrap();
    let mut camera = rier::Camera3D::new(gfx.clone());
    camera.eye = Point3::new(4.0, 3.0, 3.0);
    let mut model = Model::new(&renderer);
    let mut x = 0.0f32;
    let main_loop = rier::Loop::new(move |delta| {
        use rier::main_loop::Return::*;

        camera.update();
        x += delta.subsec_nanos() as f32 / 1000000000.0;
        model.transform.set_rotation(Rad::new(x), Rad::new(x), Rad::new(0.0));
        model.transform.dirty();
        for event in gfx.display.poll_events() {
            match event {
                rier::WindowEvent::Closed => return Exit,
                _ => (),
            }
        }
        gfx.frame(|| {
            model.render(&renderer, &camera, Point3::new(2.2, 1.0, 2.0));
        }).unwrap();
        Next
    });
    main_loop.start();
}
