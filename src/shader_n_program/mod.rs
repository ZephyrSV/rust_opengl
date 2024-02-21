pub struct Shader {
    id: gl::types::GLuint,
}

pub enum ShaderType {
    Vertex,
    Fragment,
}

#[derive(Debug)]
pub enum ShaderError {
    SrcFileNotFound,
    CompileError(String),
    LinkError(String),
}

impl Shader{
    pub fn new(location: &str, shader_type: ShaderType) -> Result<Self, ShaderError> {
        let shader_src = std::ffi::CString::new(
            std::fs::read_to_string(location).map_err(|_| ShaderError::SrcFileNotFound)?,
        ).unwrap();
        let mut shader = Self {
            id: 0,
        };
        unsafe {
            match shader_type {
                ShaderType::Vertex => shader.id = gl::CreateShader(gl::VERTEX_SHADER),
                ShaderType::Fragment => shader.id = gl::CreateShader(gl::FRAGMENT_SHADER),
            }
            gl::ShaderSource(shader.id, 1, &shader_src.as_ptr(), std::ptr::null());
            gl::CompileShader(shader.id);
        }
        let mut success = 0;
        unsafe {
            gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);
        }
        if success == 1 {
            return Ok(shader);
        }
        // compile error
        let mut error_log_size: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
        }
        let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
        unsafe {
            gl::GetShaderInfoLog(
                shader.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );
            error_log.set_len(error_log_size as usize);
        }
        let log = String::from_utf8(error_log).unwrap();
        Err(ShaderError::CompileError(log))
    }
    
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}


pub struct ShaderProgram {
    id: gl::types::GLuint,
}

impl ShaderProgram {
    pub fn new(shaders: &[Shader]) -> Result<Self, ShaderError> {
        let mut program = Self {
            id: 0,
        };
        unsafe{
            program.id = gl::CreateProgram();
            for shader in shaders {
                gl::AttachShader(program.id, shader.id);
            }

            gl::LinkProgram(program.id);
        }
        let mut success: gl::types::GLint = 0;
        unsafe {
            gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);
        }

        
        if success == 1 {
            return Ok(program);
        }
        let mut error_log_size: gl::types::GLint = 0;
        let mut error_log: Vec<u8>;
        unsafe {
            gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            error_log = Vec::with_capacity(error_log_size as usize);
            gl::GetProgramInfoLog(
                program.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );
            error_log.set_len(error_log_size as usize);
        }
        
        let log = String::from_utf8(error_log).unwrap();
        Err(ShaderError::LinkError(log))
    }
    
    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn get_attrib_location(&self, attrib: &str) -> Result<gl::types::GLuint, std::ffi::NulError> {
        let attrib = std::ffi::CString::new(attrib)?;
        Ok(gl::GetAttribLocation(self.id, attrib.as_ptr()) as gl::types::GLuint)
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}


#[macro_export]
macro_rules! set_attribute {
    ($vbo:ident, $pos:tt, $t:ident :: $field:tt) => {{
        let dummy = core::mem::MaybeUninit::<$t>::uninit();
        let dummy_ptr = dummy.as_ptr();
        let member_ptr = core::ptr::addr_of!((*dummy_ptr).$field);
        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        let member_offset = member_ptr as i32 - dummy_ptr as i32;
        $vbo.set_attribute::<$t>(
            $pos,
            (size_of_raw(member_ptr) / core::mem::size_of::<f32>()) as i32,
            member_offset,
        )
    }};
}