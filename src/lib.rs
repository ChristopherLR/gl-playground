use gl::{self, types::{ GLuint, GLfloat, GLbyte, GLenum, GLboolean }};
use std::mem;
use core::convert::TryInto;

pub type Vertex = [GLfloat; 3];

pub enum GLType {
    UINT,
    FLOAT,
    BYTE,
}

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

#[derive(Debug)]
pub struct IndexBuffer {
    renderer_id: u32,
}

impl IndexBuffer {
    pub fn new(data: &[u32], size: usize) -> Self {
        let mut buffer = 0;
        unsafe { 
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
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
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.renderer_id) };
    }

    pub fn un_bind(){
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) };
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self){
        println!("Dropping: {}", self.renderer_id);
        unsafe { gl::DeleteBuffers(1, &self.renderer_id) };
    }
}

pub struct VertexBufferElement {
    count: u32,
    element_type: GLenum,
    normalised: GLboolean,
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
                self.stride += stride;
            },
            gl::BYTE => {
                normalised = gl::TRUE;
                let stride: u32 = mem::size_of::<GLbyte>().try_into().unwrap();
                self.stride += stride;
            },
            gl::INT => {
                normalised = gl::FALSE;
                let stride: u32 = mem::size_of::<GLuint>().try_into().unwrap();
                self.stride += stride;
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

pub struct VertexArray {
    renderer_id: u32
}

impl VertexArray {
    pub fn new() -> Self {
        let mut arrays = 0;
        unsafe { gl::GenVertexArrays(1, &mut arrays); }
        VertexArray {
            renderer_id: arrays
        }
    }

    pub fn bind(&self) {
       unsafe { gl::BindVertexArray(self.renderer_id) } 
    }

    pub fn un_bind() {
       unsafe { gl::BindVertexArray(0) } 
    }

    pub fn add_buffer(&mut self, vb: &mut VertexBuffer, layout: &mut VertexBufferLayout){
        self.bind();
        vb.bind();
        let stride = layout.get_stride();
        let elements = layout.get_elements();
        let mut offset = 0;

        for (pos, e) in elements.iter().enumerate() {
            unsafe {
                gl::EnableVertexAttribArray(pos as u32);
                
                gl::VertexAttribPointer(
                    pos as u32,
                    e.count as i32,
                    e.element_type,
                    e.normalised,
                    stride as i32,
                    offset as *const _,
                );
            }
            offset += e.get_size() * e.count; 
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self){
        println!("Dropping: {}", self.renderer_id);
        unsafe { gl::DeleteVertexArrays(1, &self.renderer_id) };
    }
}
