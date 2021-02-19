use glutin::{ 
    window::WindowBuilder,
    event_loop::{ EventLoop, ControlFlow }, 
    ContextBuilder,
    event::{ Event, WindowEvent }
};

use gl;
use std::mem;
use gl_playground::{ 
    shader::{
        Shader,
        ShaderProgram
    },
    buffer::{
        Buffer,
        VertexBufferLayout,
    },
    vertex_array::VertexArray,
};

const VERT_SHADER: &str = r#"
#version 330 core
layout (location = 0) in vec3 pos;
void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
}
"#;

const FRAG_SHADER: &str = r#"
#version 330 core
out vec4 final_color;
void main() {
    final_color = vec4(0.7, 0.5, 0.6, 0.5);
}
"#;

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Playground")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    
    let gl_window = ContextBuilder::new()
        .build_windowed(wb, &el)
        .unwrap();
    
    let gl_window = unsafe { gl_window.make_current() }.unwrap();

    gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    let vertex_shader = Shader::from_str(VERT_SHADER, gl::VERTEX_SHADER);

    let fragment_shader = Shader::from_str(FRAG_SHADER, gl::FRAGMENT_SHADER);

    let shader_program = ShaderProgram::new(&vertex_shader, &fragment_shader);

    const VERTICES: [f32; 12] = [
         0.5,  0.5, 0.0, //TR
        -0.5,  0.5, 0.0, //TL
        -0.5, -0.5, 0.0, //BL 
         0.5, -0.5, 0.0, //BR
    ]; 
    
    const INDICES: [u32; 6] = [
        0, 1, 3,
        1, 2, 3,
    ];


    let mut va = VertexArray::new();

    let mut vb = Buffer::new(
        &VERTICES, 
        mem::size_of_val(&VERTICES)
    );

    let mut layout = VertexBufferLayout::new();
    layout.push(3, gl::FLOAT);
    va.add_buffer(&mut vb, &mut layout);
    
    let mut ib = Buffer::new(
        &INDICES,
        mem::size_of_val(&INDICES)
    );
    
    shader_program.use_program();
    vertex_shader.delete_shader();
    fragment_shader.delete_shader();

    el.run( move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                println!("redraw");
                unsafe {
                    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    
                    //gl::DrawArrays(gl::TRIANGLES, 0, 3);

                    va.bind();
                    ib.bind();

                    // DrawElements replaces draw arrays when using and EBO
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
                }
                gl_window.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}
