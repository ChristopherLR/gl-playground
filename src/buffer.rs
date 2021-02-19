use gl::{self, types::{ GLuint, GLfloat, GLbyte, GLenum, GLboolean }};
use std::mem;
use core::convert::TryInto;

use crate::Vertex;

pub trait BufferType {
    fn buffer_type(&self) -> GLenum;
}

impl BufferType for f32 {
    fn buffer_type(&self) -> GLenum {
        gl::ARRAY_BUFFER
    }
}

impl BufferType for Vertex {
    fn buffer_type(&self) -> GLenum {
        gl::ARRAY_BUFFER
    }
}

impl BufferType for u32 {
    fn buffer_type(&self) -> GLenum {
        gl::ELEMENT_ARRAY_BUFFER
    }
}

#[derive(Debug)]
pub struct Buffer {
    renderer_id: u32,
    buffer_type: GLenum
}

impl Buffer {
    pub fn new<T: BufferType>(data: &[T], size: usize) -> Self {
        let mut buffer = 0;
        unsafe { 
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(data[0].buffer_type(), buffer);
            gl::BufferData(
                data[0].buffer_type(),
                size as isize,
                data.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
        }
        Self {
            renderer_id: buffer,
            buffer_type: data[0].buffer_type()
        }
    }

    pub fn bind(&mut self){
        unsafe { gl::BindBuffer(self.buffer_type, self.renderer_id) };
    }

    pub fn un_bind(&mut self){
        unsafe { gl::BindBuffer(self.buffer_type, 0) };
    }
}

impl Drop for Buffer {
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
