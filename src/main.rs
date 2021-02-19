use glfw::{Action, Context, Key};

use gl;
use std::mem;
use std::os::raw::c_char;
use std::sync::mpsc::Receiver;
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
layout(location = 0) out vec4 color;
uniform vec4 u_Color;
void main() {
    color = u_Color;
}
"#;

fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(
            800,
            600,
            "Opengl Playground",
            glfw::WindowMode::Windowed
        ).expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

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

    let uniform_name: *const c_char = "u_Color\0".as_ptr().cast(); 
    let uniform_location;
    unsafe { 
        uniform_location = gl::GetUniformLocation(
            shader_program.renderer_id, 
            uniform_name
        );
    }
    if uniform_location == -1 {
        println!("Uniform not found");
    }

    let mut red: f32 = 0.0;
    let mut increment: f32 = 0.05;

    while !window.should_close() {

        process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            gl::Uniform4f(uniform_location, red, 0.3, 0.8, 1.0);
            va.bind();
            ib.bind();
            
            if red >= 1.0 {
                increment = -0.05;
            } else if red <= 0.0 {
                increment = 0.05;
            }
            red += increment;

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}
