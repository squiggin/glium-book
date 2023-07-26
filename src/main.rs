mod vertex;
mod camera;

#[macro_use]
extern crate glium;

use std::f32::consts::PI;

use glam::Mat4;
use glium::{Surface, Display, Program};
use vertex::Vertex;
use camera::Camera;

const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 768.0;

fn compile_shader_program(display: &Display) -> Program {
    let vertex_shader_src = r#"
        #version 410 core

        in vec3 position;
        out vec3 position_frag;
        uniform float elapsed;
        uniform mat4 view_proj;

        void main() {
            vec3 pos = position;
            pos.x += sin(elapsed);
            pos.y += cos(elapsed);
            gl_Position = view_proj * vec4(pos, 1.0);
            position_frag = position;
        }
    "#;

    let fragment_shader_src = r#"
        #version 410 core

        in vec3 position_frag;

        out vec4 color;

        void main() {
            color = vec4(1.0, position_frag.x + 0.5, position_frag.y + 0.5, 1.0);
        }
    "#;

    glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}


fn main() {
    let events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .with_title("Glium book");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let vertex1 = Vertex { position: [-0.5, -0.5, 0.0] };
    let vertex2 = Vertex { position: [ 0.0,  0.5, 0.0] };
    let vertex3 = Vertex { position: [ 0.5, -0.25, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let program = compile_shader_program(&display);
    
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
    let time = std::time::Instant::now();
    let mut cam = Camera::new();

    events_loop.run(move |ev, _, control_flow| {
        match ev {
            glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                glium::glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                },
                _ => (),
            },
            _ => (),
        };

        
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        
        let elapsed = time.elapsed().as_secs_f32();

        let view_proj = cam.gen_view_projection_matrix().to_cols_array_2d();
        target.draw(&vertex_buffer, &indices, &program, &uniform! { elapsed: elapsed, view_proj: view_proj },
            &Default::default()).unwrap();

        target.finish().unwrap();
    })
}
