pub mod vertex_buffer;
pub mod index_buffer;
pub mod vertex_array;
pub mod shader;

pub type Vertex = [f32; 3];

pub enum GLType {
    UINT,
    FLOAT,
    BYTE,
}
