use glfw;
use gl::types::{GLuint, GLint};

type Pos = [f32; 2];
type Color = [f32; 3];

#[repr(C, packed)]
pub struct Vertex(pub Pos, pub Color);


pub struct VBO {
    pub id: gl::types::GLuint,
    target: gl::types::GLuint,
}

impl VBO {
    pub fn new(target: gl::types::GLuint) -> Self {
        let mut id = 0;
        unsafe{
            gl::GenBuffers(1, &mut id);
        }
        VBO { id, target }
    }
    pub unsafe fn bind(&self) {
        gl::BindBuffer(self.target, self.id);
    }

    pub unsafe fn set_data<D>(&self, data: &[D], usage: gl::types::GLuint) {
        self.bind();
        let (_, aligned_data, _) = data.align_to::<u8>();
        gl::BufferData(
            self.target,
            aligned_data.len() as gl::types::GLsizeiptr,
            aligned_data.as_ptr() as *const _,
            usage
        )
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

pub struct VAO {
    pub id: GLuint
}

impl VAO {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe{
            gl::GenVertexArrays(1, &mut id);
        }
        Self { id }
    }
    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }
    pub unsafe fn set_attribute<V: Sized>(
        &self,
        attrib_pos: GLuint,
        components: GLint,
        offset: GLint,
    ) {
        self.bind();
        gl::VertexAttribPointer(
            attrib_pos,
            components,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<V>() as GLint,
            offset as *const _,
        );
        gl::EnableVertexAttribArray(attrib_pos);
    }
}


impl Drop for VAO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}
