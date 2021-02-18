use gl::{self, types::{ GLuint, GLfloat, GLbyte, GLenum, GLboolean }};
use std::mem;
use core::convert::TryInto;

use crate::Vertex;

#[derive(Debug)]
pub struct VertexBuffer {
    renderer_id: u32,
}

impl VertexBuffer {
    pub fn new(data: &[Vertex], size: usize) -> Self {
        let mut buffer = 0;
        unsafe { 
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size as isize,
                data.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
        }
        Self {
            renderer_id: buffer,
        }
    }

    pub fn bind(&mut self){
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.renderer_id) };
    }

    pub fn un_bind(){
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) };
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self){
        println!("Dropping: {}", self.renderer_id);
        unsafe { gl::DeleteBuffers(1, &self.renderer_id)};
    }
}


pub struct VertexBufferElement {
    pub count: u32,
    pub element_type: GLenum,
    pub normalised: GLboolean,
}

impl VertexBufferElement {
    pub fn get_size(&self) -> u32 {
        match self.element_type {
            gl::FLOAT => mem::size_of::<GLfloat>().try_into().unwrap(),
            gl::BYTE => mem::size_of::<GLbyte>().try_into().unwrap(),
            gl::INT => mem::size_of::<GLuint>().try_into().unwrap(),
            _ => {
                println!("Undefined type: {}", self.element_type);
                0
            }
        }
    }
}

pub struct VertexBufferLayout {
    pub elements: Vec<VertexBufferElement>,
    pub stride: u32,
}

impl VertexBufferLayout {
    pub fn new() -> Self {
        VertexBufferLayout {
            elements: Vec::new(),
            stride: 0
        }
    }
    pub fn push(&mut self, count: u32, e_type: GLenum){
        let normalised;

        match e_type {
            gl::FLOAT => {
                normalised = gl::FALSE;
                let stride: u32 = mem::size_of::<GLfloat>().try_into().unwrap();
                self.stride += count * stride;
            },
            gl::BYTE => {
                normalised = gl::TRUE;
                let stride: u32 = mem::size_of::<GLbyte>().try_into().unwrap();
                self.stride += count * stride;
            },
            gl::INT => {
                normalised = gl::FALSE;
                let stride: u32 = mem::size_of::<GLuint>().try_into().unwrap();
                self.stride += count * stride;
            },
            _ => {
                println!("Undefined type: {}", e_type);
                normalised = gl::FALSE;
            }
        }

        self.elements.push(
            VertexBufferElement {
                count,
                element_type: e_type,
                normalised
            }
        );
    }
    
    pub fn get_stride(&mut self) -> u32 {
        self.stride
    }

    pub fn get_elements(&mut self) -> &Vec<VertexBufferElement> {
        &self.elements
    }

}
