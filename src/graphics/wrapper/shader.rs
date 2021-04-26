
use super::TextureManager;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs;
use strum::{EnumCount, EnumIter};
use crate::graphics::UniformData;
use macros::ShaderId;

#[derive(Clone)]
pub enum ProgramType {
    Graphics,
    Compute,
}

///TODO: Figure out a way to unimplement Send and Sync for Shader
pub struct Shader {
    program_id: u32,
    uniform_locations: HashMap<String, i32>,
    metadata : ShaderMetadata
}


#[derive(Clone, Copy, Debug, EnumCount, EnumIter, ShaderId)]
pub enum ShaderIdentifier {
    #[name("Default shader")]
    #[extensionless_path("graphics/shaders/s1")]
    #[is_compute(false)]
    DefaultShader,
    #[name("Sobel shader")]
    #[extensionless_path("graphics/shaders/sobel")]
    #[is_compute(true)]
    SobelShader,
    #[name("Simple Shader")]
    #[extensionless_path("graphics/shaders/simple")]
    #[is_compute(false)]
    SimpleShader,
}

#[derive(Clone)]
pub struct ShaderMetadata {
    pub identifier : ShaderIdentifier,
    /// The uniforms that this shader needs filled out
    /// First string is the name of the uniform, second is the file and line at which it is found (for error reports)
    pub required_uniforms: Vec<(String, String)>,
    pub shader_type: ProgramType,
}

impl Shader {
    unsafe fn compile_shader(source: &CStr, shader_type: u32) -> Result<u32, String> {
        if shader_type != gl::VERTEX_SHADER
            && shader_type != gl::FRAGMENT_SHADER
            && shader_type != gl::COMPUTE_SHADER
        {
            return Err(String::from(
                "Invalid shader type! Only allowed types are vertex, fragment, and compute!",
            ));
        }

        let id = gl::CreateShader(shader_type);

        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);

        let mut success: gl::types::GLint = 1;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            //Get the length of the error message
            let mut len: gl::types::GLint = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);

            let error = create_whitespace_cstring_with_len(len as usize);

            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(id)
    }

    unsafe fn load_shader(
        file: &str,
        shader_type: u32,
    ) -> Result<(u32, Vec<(String, String)>), String> {
        let shader_source = match fs::read_to_string(file) {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("Couldn't open file {}. {:?}", file, e));
            }
        };
        let shader_source = shader_source + "\0";

        let id = match Self::compile_shader(
            &CStr::from_bytes_with_nul(shader_source.as_bytes()).unwrap(),
            shader_type,
        ) {
            Ok(vsid) => vsid,
            Err(s) => return Err(s),
        };

        Ok((id, Self::find_uniforms(&shader_source, file)))
    }

    pub unsafe fn new(
        identifier : ShaderIdentifier
    ) -> Result<Shader, String> {
        if identifier.is_compute() {
            Shader::new_compute(identifier)
        } else {
            Shader::new_graphical(identifier)
        }
    }

    unsafe fn new_graphical(identifier : ShaderIdentifier) -> Result<Shader, String> {
        if identifier.is_compute() {
            return Err(format!("Compute shader identifier {} passed as graphical!", identifier.name()))
        }
        let name = identifier.name();
        let extensionless_path = identifier.extensionless_path();
        let vertex_file = format!("{}.vert",extensionless_path);
        let fragment_file = format!("{}.frag",extensionless_path);
        println!("Loading shader {}", name);
        let (vsid, mut vsuniforms) = match Self::load_shader(&vertex_file, gl::VERTEX_SHADER) {
            Ok(id) => id,
            Err(s) => return Err(s),
        };
        let (fsid, mut fsuniforms) = match Self::load_shader(&fragment_file, gl::FRAGMENT_SHADER) {
            Ok(id) => id,
            Err(s) => return Err(s),
        };

        vsuniforms.append(&mut fsuniforms);
        let required_uniforms = vsuniforms;

        let program_id = gl::CreateProgram();

        gl::AttachShader(program_id, vsid);
        gl::AttachShader(program_id, fsid);
        gl::LinkProgram(program_id);

        let mut success: gl::types::GLint = 1;
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);

            let error = create_whitespace_cstring_with_len(len as usize);

            gl::GetProgramInfoLog(
                program_id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );

            return Err(error.to_string_lossy().into_owned());
        }

        //TODO: Why do I validate the program?
        gl::ValidateProgram(program_id);

        gl::DetachShader(program_id, vsid);
        gl::DetachShader(program_id, fsid);
        gl::DeleteShader(vsid);
        gl::DeleteShader(fsid);

        let mut uniform_locations: HashMap<String, i32> = HashMap::new();

        gl::UseProgram(0);

        gl::UseProgram(program_id);
        for entry in &required_uniforms {
            let ename = format!("{}{}", entry.0, "\0");
            let ename = ename.as_bytes();
            let ename = CStr::from_bytes_with_nul(ename).unwrap();
            let id =
                gl::GetUniformLocation(program_id, (ename.as_ptr()) as *const gl::types::GLchar);
            println!(
                "Creating a uniform location for uniform {:?} at {}",
                ename, id
            );

            uniform_locations.insert(String::from(&entry.0), id);
        }
        gl::UseProgram(0);



        Ok(Shader {
            program_id,
            uniform_locations: uniform_locations,
            metadata : ShaderMetadata {
                identifier,
                required_uniforms,
                shader_type: ProgramType::Graphics,
            }
        })
    }

    unsafe fn new_compute(identifier : ShaderIdentifier) -> Result<Shader, String> {
        if !identifier.is_compute() {
            return Err(format!("Graphical shader identifier {} passed as compute!", identifier.name()))
        }
        let extensionless_path = identifier.extensionless_path();
        let compute_file = format!("{}.comp",extensionless_path);
        let (id, required_uniforms) = match Self::load_shader(&compute_file, gl::COMPUTE_SHADER) {
            Ok(id) => id,
            Err(s) => return Err(s),
        };

        let program_id = gl::CreateProgram();

        gl::AttachShader(program_id, id);
        gl::LinkProgram(program_id);
        //gl::ValidateProgram(program_id);

        let mut success: gl::types::GLint = 1;
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);

            let error = create_whitespace_cstring_with_len(len as usize);

            gl::GetProgramInfoLog(
                program_id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );

            return Err(error.to_string_lossy().into_owned());
        }

        //gl::DetachShader(program_id, id);
        gl::DeleteShader(id);

        let mut uniform_locations: HashMap<String, i32> = HashMap::new();

        gl::UseProgram(program_id);
        for entry in &required_uniforms {
            let id = gl::GetUniformLocation(
                program_id,
                ((*entry.0).as_ptr()) as *const gl::types::GLchar,
            );
            println!("Creating a uniform location for uniform {}", &entry.0);
            uniform_locations.insert(String::from(&entry.0), id);
            //uniform_locations.
        }
        gl::UseProgram(0);

        Ok(Shader {
            program_id,
            uniform_locations,
            metadata : ShaderMetadata {
                identifier,
                required_uniforms,
                shader_type: ProgramType::Graphics,
            }
        })
    }

    ///TODO: This should work for any valid notation.
    fn find_uniforms(source: &str, filename: &str) -> Vec<(String, String)> {
        let mut uniforms: Vec<(String, String)> = Vec::new();
        let mut counter = 0;
        for line in source.lines() {
            counter += 1;
            if line.starts_with("uniform") {
                let next = &line[8..];
                let re = regex::Regex::new(r"\w+").unwrap();
                let mut ms = re.captures_iter(next);
                ms.next();
                if let Some(type_name) = ms.next() {
                    uniforms.push((
                        String::from(&type_name[0]),
                        format!("{}:{}", filename, counter),
                    ));
                }
            }
        }

        uniforms
    }

    pub unsafe fn bind(&self) {
        gl::UseProgram(self.program_id);
    }

    pub unsafe fn unbind(&self) {
        gl::UseProgram(0);
    }

    pub fn get_metadata(&self) -> &ShaderMetadata {
        &self.metadata
    }
}

/// Tests don't have a GL Context, so they can't drop the shader.
/// From outside the shader::test module, shaders can only be instantiated by the unsafe function Shader::new,
/// which requires a GL context.
#[cfg(not(test))]
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub struct ShaderManager {
    /// Shaders indexed by their count in the ShaderIdentifier enum
    shaders: Vec<Shader>,
    // Name of chosen shader.
    bound_shader: Option<ShaderIdentifier>,
}

impl<'a> ShaderManager {
    pub fn new(shaders : Vec<Shader>) -> ShaderManager {
        let mut count=0;
        for shader in &shaders {
            if shader.metadata.identifier as usize != count {
                panic!("Loading shaders into the shader manager out of order!")
            }
            count += 1;
        }
        if count < ShaderIdentifier::COUNT {
            panic!("Not enough shaders were supplied for the shader manager. Likely some loading failed; look further up for compilation errors.")
        }
        ShaderManager {
            shaders,
            bound_shader: None,
        }
    }

    //pub unsafe fn add_shader(&mut self, shader: Shader) {
    //    self.shaders.push(shader);
    //}

    pub unsafe fn bind_shader(&mut self, shader: ShaderIdentifier) -> Result<String, String> {
        
        if let Some(s) = self.bound_shader.take() {
            self.shaders[s as usize].unbind();
        }
        self.shaders[shader as usize].bind(); //TODO: MAKE NO SHADER MESSAGE
        self.bound_shader = Some(shader);

        Ok(format!(""))
    }

    pub unsafe fn uniforms(
        &mut self,
        uniforms: &UniformData,
        texture_manager: &TextureManager,
    ) -> Result<u32, String> {
        //TODO: THERE COULD BE NO CURRENT SHADER
        if let None = self.bound_shader {
            return Err(String::from(
                "Uniforms were sent, but there's no bound shader!",
            ));
        }

        let s = &self.shaders[self.bound_shader.unwrap() as usize];
        for entry in &uniforms.mat4s {
            let loc = (s).uniform_locations.get(&entry.1);
            let mat = entry.0;

            gl::UniformMatrix4fv(*loc.unwrap(), 1, gl::FALSE, (mat.as_ptr()) as *const f32);
            //TODO: HOW TO DO THIS?? s.fill_uniform(&entry.1);
        }

        for entry in &uniforms.vec3s {
            let loc = s.uniform_locations.get(&entry.1);
            let vec = entry.0;

            gl::Uniform4fv(*loc.unwrap(), 1, (vec.as_ptr()) as *const f32)
        }

        let mut texture_slot: i32 = 0;
        for entry in &uniforms.textures {
            let loc = s.uniform_locations.get(&entry.1);
            let tex_name = &entry.0;
            let tex = texture_manager.get_texture(tex_name);

            gl::ActiveTexture(gl::TEXTURE0 + texture_slot as u32);
            tex.bind();
            gl::Uniform1i(*loc.unwrap(), texture_slot);
            texture_slot += 1;
        }
        Ok(0)
    }

    pub fn get_active_shader_name(&self) -> Option<String> {
        if let Some(index) = self.bound_shader {
            Some(String::from(index.name()))
        } else {
            None
        }
    }

    /// Returns the metadata of every Shader indexed by its ShaderIdentifier.
    pub fn get_shader_metadata(&self) -> Vec<ShaderMetadata> {
        let mut res = Vec::new();
        for shader in &self.shaders {
            res.push(shader.metadata.clone());
        }
        res
    }

}


#[cfg(test)]
mod tests {
    use super::{Shader, ShaderIdentifier, ShaderManager, ShaderMetadata, ProgramType};
    use std::collections::HashMap;
    use strum::IntoEnumIterator;
    #[test]
    #[should_panic]
    fn no_shader_test() {
        ShaderManager::new(vec![]);
    }

    #[test]
    fn right_shader_count_test() {
        let mut shaders = Vec::new();
        for identifier in ShaderIdentifier::iter() {
            shaders.push(Shader {
                program_id : 0,
                uniform_locations : HashMap::new(),
                metadata : ShaderMetadata {
                    identifier : identifier,
                    required_uniforms : Vec::new(),
                    shader_type : if identifier.is_compute() {ProgramType::Compute} else { ProgramType::Graphics}
                }
            });
        }
        ShaderManager::new(shaders);
    }

    #[test]
    #[should_panic]
    fn one_shader_too_many_test() {
        let mut shaders = Vec::new();
        for identifier in ShaderIdentifier::iter() {
            shaders.push(Shader {
                program_id : 0,
                uniform_locations : HashMap::new(),
                metadata : ShaderMetadata {
                    identifier : identifier,
                    required_uniforms : Vec::new(),
                    shader_type : if identifier.is_compute() {ProgramType::Compute} else { ProgramType::Graphics}
                }
            });
        }
        shaders.push(Shader {
            program_id : 0,
            uniform_locations : HashMap::new(),
            metadata : ShaderMetadata {
                identifier : ShaderIdentifier::DefaultShader,
                required_uniforms : Vec::new(),
                shader_type : if ShaderIdentifier::DefaultShader.is_compute() {ProgramType::Compute} else { ProgramType::Graphics}
            }
        });
        ShaderManager::new(shaders);
    }

}