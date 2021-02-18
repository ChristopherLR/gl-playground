use gl::{self, types::GLenum, FRAGMENT_SHADER, VERTEX_SHADER};
use core::convert::TryInto;

pub enum ShaderLocation {
    FILE(&'static str),
    STR(&'static str)
}

pub struct Shader {
    pub shader_src: ShaderLocation,
    pub renderer_id: u32,
    pub shader_type: GLenum,
}


impl Shader {
    pub fn from_str(shader_str: &'static str, shader_type: GLenum) -> Self {
        let id = Shader::compile_shader(
            ShaderLocation::STR(shader_str),
            shader_type,
        );

        Shader::get_shader_log(id);

        Shader {
            shader_src: ShaderLocation::STR(shader_str),
            renderer_id: id,
            shader_type
        }
        
    }

    pub fn set_uniform(name: &str){

    }

    #[inline]
    fn get_shader_log(id: u32) {
        unsafe {
            let mut success = 0;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut err_log: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(
                    id,
                    1024 as i32,
                    &mut log_len,
                    err_log.as_mut_ptr().cast(),
                );
                err_log.set_len(log_len.try_into().unwrap());
                panic!("Shader compile error: {}", String::from_utf8_lossy(&err_log));
            }
        }
    }

    #[inline]
    fn compile_shader(shader_src: ShaderLocation , shader_type: GLenum) -> u32 {
        let shader_src = match shader_src {
            ShaderLocation::FILE(file) => {
                unimplemented!();
            },
            ShaderLocation::STR(string) => string
        };

        match shader_type {
            VERTEX_SHADER => {
                let id;
                unsafe {
                    id = gl::CreateShader(VERTEX_SHADER);
                    gl::ShaderSource(
                        id,
                        1,
                        &(shader_src.as_bytes().as_ptr().cast()),
                        &(shader_src.len().try_into().unwrap()),
                    );
                    gl::CompileShader(id);
                }
                id
            },
            FRAGMENT_SHADER => {
                let id;
                unsafe {
                    id = gl::CreateShader(FRAGMENT_SHADER);
                    gl::ShaderSource(
                        id,
                        1,
                        &(shader_src.as_bytes().as_ptr().cast()),
                        &(shader_src.len().try_into().unwrap()),
                    );
                    gl::CompileShader(id);
                }
                id
            },
            _ => {
                println!("Undefined shader: {}", shader_type);
                0
            }
        }
    }

    fn get_uniform_location(name: &str){

    }
}

pub struct ShaderProgram {
    renderer_id: u32,
    vertex_shader: Shader,
    fragment_shader: Shader,
}
