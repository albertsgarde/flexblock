use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs;

use crate::graphics::UniformData;

pub enum ProgramType {
    Graphics,
    Compute,
}

///
/// TODO: There should be a shader manager that holds all the shaders.
/// Then shaders can only be changed by the shader manager.
pub struct Shader {
    program_id: u32,
    name: String,
    required_uniforms: Vec<(String, String)>,
    uniform_locations: HashMap<String, i32>,
    bound_uniforms: Vec<String>,
    shader_type: ProgramType,
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

        println!("Src: {:?}", source);
        let id = gl::CreateShader(shader_type);

        println!("ID? {}", id);

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
        vertex_file: &str,
        fragment_file: &str,
        name: &str,
    ) -> Result<Shader, String> {
        let (vsid, mut vsuniforms) = match Self::load_shader(vertex_file, gl::VERTEX_SHADER) {
            Ok(id) => id,
            Err(s) => return Err(s),
        };
        let (fsid, mut fsuniforms) = match Self::load_shader(fragment_file, gl::FRAGMENT_SHADER) {
            Ok(id) => id,
            Err(s) => return Err(s),
        };

        vsuniforms.append(&mut fsuniforms);
        let required_uniforms = vsuniforms;

        for uniform in &required_uniforms {
            println!("Registering uniform {}!", uniform.0);
        }

        let program_id = gl::CreateProgram();

        gl::AttachShader(program_id, vsid);
        gl::AttachShader(program_id, fsid);
        gl::LinkProgram(program_id);

        let mut success: gl::types::GLint = 1;
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            //println!("No luck here!");
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
            name: String::from(name),
            required_uniforms,
            bound_uniforms: Vec::new(),
            uniform_locations: uniform_locations,
            shader_type: ProgramType::Graphics,
        })
    }

    pub unsafe fn new_compute(compute_file: &str, name: &str) -> Result<Shader, String> {
        let (id, required_uniforms) = match Self::load_shader(compute_file, gl::COMPUTE_SHADER) {
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
            //println!("No luck here!");
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

        gl::DetachShader(program_id, id);
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
        println!("Now we have uniform locations {:?}", uniform_locations);
        gl::UseProgram(0);

        Ok(Shader {
            program_id,
            required_uniforms,
            uniform_locations,
            name: String::from(name),
            bound_uniforms: Vec::new(),
            shader_type: ProgramType::Compute,
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

    /// TODO: Shader manager should make sure this is done for every uniform call
    /// This is a debug function
    pub fn fill_uniform(&mut self, uniform_name: &str) {
        self.bound_uniforms.push(String::from(uniform_name));
    }

    /// This function validates that all uniforms are bound when the render is called
    /// This is a debug function
    pub fn ready_to_render(&self) -> Result<String, String> {
        for (k, v) in self.required_uniforms.iter() {
            if !self.bound_uniforms.contains(k) {
                return Err(format!("uniform {} not filled for {}!", k, v));
            }
        }
        return Ok(String::from(""));
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

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
    shaders: Vec<Shader>, //Shaders indexed by given names
    shader_names: HashMap<String, usize>,
    // Name of chosen shader - so disgusting.
    bound_shader: Option<usize>,
}

impl<'a> ShaderManager {
    pub fn new() -> ShaderManager {
        ShaderManager {
            shaders: Vec::new(),
            shader_names: HashMap::new(),
            bound_shader: None,
        }
    }

    pub unsafe fn add_shader(&mut self, shader: Shader) {
        self.shader_names
            .insert(String::from(shader.get_name()), self.shaders.len());
        self.shaders.push(shader);
    }

    pub unsafe fn bind_shader(&mut self, shader: &str) -> Result<String, String> {
        if !self.shader_names.contains_key(shader) {
            return Err(format!("No shader by the name {} is loaded!", shader));
        }
        if let Some(s) = self.bound_shader.take() {
            self.shaders[s].unbind();
        }
        let shader_index = self.shader_names.get(shader).unwrap(); //TODO: MAKE NO SHADER MESSAGE
        let shader_real = self.shaders.get(*shader_index).unwrap();
        shader_real.bind();
        self.bound_shader = Some(*shader_index);

        Ok(format!(""))
    }

    pub unsafe fn uniforms(&mut self, uniforms: &UniformData) -> Result<u32, String> {
        //TODO: THERE COULD BE NO CURRENT SHADER
        if let None = self.bound_shader {
            return Err(String::from(
                "Uniforms were sent, but there's no bound shader!",
            ));
        }

        let s = self.shaders.get(self.bound_shader.unwrap()).unwrap();
        for entry in &uniforms.mat4s {
            let loc = (s).uniform_locations.get(&entry.1);
            let mat = entry.0;

            gl::UniformMatrix4fv(*loc.unwrap(), 1, gl::FALSE, (mat.as_ptr()) as *const f32);
            //TODO: HOW TO DO THIS?? s.fill_uniform(&entry.1);
        }

        for entry in &uniforms.vec4s {
            println!("Trying to find a location for uniform {}.", &entry.1);
            let loc = s.uniform_locations.get(&entry.1);

            println!("{:?}", s.uniform_locations);
            let vec = entry.0;

            gl::Uniform4fv(*loc.unwrap(), 1, (vec.as_ptr()) as *const f32)
        }
        Ok(0)
    }

    pub fn get_active_shader_name(&self) -> Option<String> {
        if let Some(index) = self.bound_shader {
            Some(String::from(self.shaders[index].get_name()))
        } else {
            None
        }
    }

    // Loads fragment and vertex shader pairs from folder, and TODO: compute shaders individually
    // UNSAFE because it runs gl commands
    pub unsafe fn load_shaders(&mut self, folder: &str) {
        let mut fragment_shaders: Vec<String> = Vec::new();
        let mut vertex_shaders: Vec<String> = Vec::new();

        // First, we find every file in the folder we're loading from, and see if it's a shader file
        for entry in fs::read_dir(folder).unwrap() {
            let e = entry.unwrap();
            let name = e.file_name().into_string().unwrap();
            if name.ends_with(".vertexshader") {
                let name = &name[0..(name.len() - 13)];
                vertex_shaders.push(String::from(name));
            } else if name.ends_with(".fragmentshader") {
                let name = &name[0..(name.len() - 15)];
                fragment_shaders.push(String::from(name));
            } else {
                eprintln!("File {} does not contain a shader!", name);
            }
        }

        for vs in vertex_shaders {
            if !fragment_shaders.contains(&vs) {
                eprintln!(
                    "Vertex shader {} does not have a fragment shader partner!",
                    vs
                );
            }
            fragment_shaders.retain(|x| *x != vs);

            let shader = match Shader::new(
                &(format!("{}/{}.vertexshader", folder, vs)),
                &(format!("{}/{}.fragmentshader", folder, vs)),
                &vs,
            ) {
                Ok(s) => s,
                Err(s) => {
                    eprintln!("Loading shader {} failed! Error: {}", vs, s);
                    continue;
                }
            };

            self.add_shader(shader);
        }
        // Remaining fragment shaders are the ones that didn't have a vertex shader partner
        for fs in fragment_shaders {
            eprintln!(
                "Fragment shader {} does not have a vertex shader partner!",
                fs
            );
        }
    }
}
