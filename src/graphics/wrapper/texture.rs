use std::collections::HashMap;
use std::ptr::null;
use serde::{Serialize, Deserialize};
use crate::utils::ColorFormat;

pub struct Texture {
    id: u32,
    filled: bool,
    pub metadata : TextureMetadata
}

impl Texture {
    ///
    ///Creates a new, empty texture, with the specified width, height, and format
    pub unsafe fn new(width: u32, height: u32, format: ColorFormat, name : &str) -> Texture {
        let glf = format.gl_format();
        let mut id = 0;

        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32); //TODO: WHAT THE HELL IS GOING ON WITH THIS CONVERSION TO I32???
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32); //WHY IS IT NECESSARY??
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_COMPARE_FUNC, gl::LEQUAL as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            glf as i32,
            width as i32,
            height as i32,
            0,
            glf,
            gl::UNSIGNED_BYTE,
            null(),
        );
        gl::BindTexture(gl::TEXTURE_2D, 0);

        Texture {
            id,
            filled: false,
            metadata : TextureMetadata { format, width, height, name : String::from(name)} 
        }
    }

    /// Fills the texture with the given data
    /// TODO: THIS SHOULD NOT REINITIALIZE THE TEXTURE IMAGE EACH TIME IT IS FILLED; IT SHOULD INSTEAD OVERWRITE THE EXISTING BUFFER
    pub unsafe fn fill(&mut self, data: Vec<u8>) {
        assert_eq!(
            data.len(),
            (self.metadata.width * self.metadata.height) as usize  * self.metadata.format.bytes(),
            "Tried to fill a {}x{} {}-byte-stride texture with {} length data!",
            self.metadata.width,
            self.metadata.height,
            self.metadata.format.bytes(),
            data.len()
        );

        gl::BindTexture(gl::TEXTURE_2D, self.id);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            self.metadata.format.gl_format() as i32,
            self.metadata.width as i32,
            self.metadata.height as i32,
            0,
            self.metadata.format.gl_format(),
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const gl::types::GLvoid,
        );
        gl::BindTexture(gl::TEXTURE_2D, 0);
        self.filled = true;
    }

    /// Binds the texture
    /// Watch out; this is stateful.
    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TextureMetadata {
    pub format : ColorFormat,
    pub width : u32,
    pub height : u32,
    pub name : String
}

pub struct TextureManager {
    textures: Vec<Texture>,
    texture_names: HashMap<String, usize>,
}

impl TextureManager {
    pub fn new() -> TextureManager {
        TextureManager {
            textures: vec![],
            texture_names: HashMap::new(),
        }
    }

    pub fn get_texture_metadata(&self) -> HashMap<String, TextureMetadata> {
        let mut res = HashMap::new();
        for texture in &self.textures {
            res.insert(String::from(&texture.metadata.name), texture.metadata.clone());
        }
        res
    }

    pub fn add_texture(&mut self, texture: Texture) -> Result<(), String>{
        if self.texture_names.contains_key(&texture.metadata.name) {
            return Err(format!("Texture with name {} was just added to TextureManager, but it already exists!", &texture.metadata.name));
        }
        self.texture_names
            .insert(String::from(&texture.metadata.name), self.textures.len());
        self.textures.push(texture);
        Ok(())
    }

    pub fn get_texture(&self, name: &str) -> &Texture {
        let index = match self.texture_names.get(name) {
           Some(i) => i,
           None => panic!("Texture {} was requested, but was not in texture manager!",name) 
        };
        &self.textures[*index]
    }

    /// Fills data into the texture of the given name.
    pub unsafe fn fill_texture(&mut self, name: &str, data: Vec<u8>) {
        let index = match self.texture_names.get(name) {
            Some(i) => i,
            None => panic!("Texture {} was requested, but was not in texture manager!",name) 
         };
        self.textures[*index].fill(data);
    }

    pub fn contains_texture(&self, name: &str) -> bool {
        self.texture_names.contains_key(name)
    }
}
