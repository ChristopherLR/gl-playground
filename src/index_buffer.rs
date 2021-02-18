use gl;


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
