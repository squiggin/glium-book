mod vertex;

#[macro_use]
extern crate glium;

use glium::{Surface, Display, Program};
use vertex::Vertex;

fn compile_shader_program(display: &Display) -> Program {
    let vertex_shader_src = r#"
        #version 410 core

        in vec2 position;
        out vec2 position_frag;
        uniform float elapsed;

        void main() {
            vec2 pos = position;
            pos.x += sin(elapsed);
            pos.y += cos(elapsed);
            gl_Position = vec4(pos, 0.0, 1.0);
            position_frag = position;
        }
    "#;

    let fragment_shader_src = r#"
        #version 410 core

        in vec2 position_frag;

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
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Glium book");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let program = compile_shader_program(&display);
    
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
    let time = std::time::Instant::now();

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

        target.draw(&vertex_buffer, &indices, &program, &uniform! { elapsed: elapsed },
            &Default::default()).unwrap();

        target.finish().unwrap();
    })
}
