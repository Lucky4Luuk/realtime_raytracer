use std::ffi::{CString, CStr};

pub struct Program {
    pub handle: u32,
}

impl Program {
    pub fn from_shaders(shaders: Vec<Shader>) -> Self {
        let program = unsafe { gl::CreateProgram() };

        unsafe {
            for shader in &shaders {
                gl::AttachShader(program, shader.handle);
            }
            gl::LinkProgram(program);
            for shader in &shaders {
                gl::DetachShader(program, shader.handle);
            }
        }

        Self {
            handle: program
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}

pub struct Shader {
    pub handle: u32,
}

impl Shader {
    pub fn from_compute(source: &str) -> Self {
        Shader::from_source(source, gl::COMPUTE_SHADER)
    }

    pub fn from_vertex(source: &str) -> Self {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_fragment(source: &str) -> Self {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn from_source(source: &str, shader_type: gl::types::GLenum) -> Self {
        unsafe {
            let c_source_tmp = CString::new(source).unwrap();
            let c_source = c_source_tmp.as_c_str();

            let shader = gl::CreateShader(shader_type);
            gl::ShaderSource(shader, 1, &c_source.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            let mut is_compiled = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut is_compiled);
            if is_compiled == 0 {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                buffer.extend([b' '].iter().cycle().take(len as usize));
                let error: CString = CString::from_vec_unchecked(buffer);
                gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
                error!("{}", error.to_string_lossy().into_owned());
                panic!("shader broken");
            }

            Self {
                handle: shader
            }
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.handle);
        }
    }
}
