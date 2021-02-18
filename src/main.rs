use glutin::{ 
    window::WindowBuilder,
    event_loop::{ EventLoop, ControlFlow }, 
    ContextBuilder,
    event::{ Event, WindowEvent }
};

use gl;
use std::mem;
use core::convert::TryInto;
use gl_playground::{ 
    shader::Shader,
    vertex_buffer::{
        VertexBuffer,
        VertexBufferLayout,
    },
    vertex_array::VertexArray,
    index_buffer::IndexBuffer,
    Vertex 
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

    let shader_program;
    // Shader initialisation
    unsafe {
        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader.renderer_id);
        gl::AttachShader(shader_program, fragment_shader.renderer_id);
        gl::LinkProgram(shader_program);

        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            const CAP: usize = 1024;
            let mut err_log: Vec<u8> = Vec::with_capacity(CAP);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(
                shader_program,
                CAP as i32,
                &mut log_len,
                err_log.as_mut_ptr().cast(),
            );
            err_log.set_len(log_len.try_into().unwrap());
            panic!("program link error: {}", String::from_utf8_lossy(&err_log));
        }
    }

    const VERTICES: [Vertex; 4] = [
        [0.5, 0.5, 0.0],   //TR
        [-0.5, 0.5, 0.0],  //TL
        [-0.5, -0.5, 0.0], //BL 
        [0.5, -0.5, 0.0],  //BR
    ]; 
    
    const INDICES: [u32; 6] = [
        0, 1, 3,
        1, 2, 3,
    ];


    let mut va = VertexArray::new();

    let mut vb = VertexBuffer::new(
        &VERTICES, 
        mem::size_of_val(&VERTICES)
    );

    let mut layout = VertexBufferLayout::new();
    layout.push(3, gl::FLOAT);
    va.add_buffer(&mut vb, &mut layout);
    
    let mut ib = IndexBuffer::new(
        &INDICES,
        mem::size_of_val(&INDICES)
    );

        
    unsafe {
        gl::DeleteShader(vertex_shader.renderer_id);
        gl::DeleteShader(fragment_shader.renderer_id);
        gl::UseProgram(shader_program);
    }

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
