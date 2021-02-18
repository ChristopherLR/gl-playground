use gl;

use crate::vertex_buffer::{ VertexBufferLayout, VertexBuffer };

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
