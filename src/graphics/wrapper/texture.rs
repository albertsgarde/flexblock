use std::ptr::null;
use std::collections::HashMap;


pub enum TextureFormat {
    RGB,
    RGBA
}
pub struct Texture {
    id : u32,
    format : u32,
    filled : bool,
    width : u32,
    height : u32,
}

impl Texture {
    ///
    ///Creates a new, empty texture, with the specified width, height, and format
    pub unsafe fn new(width : u32, height : u32, format : TextureFormat) -> Texture {
        let format = match format {
            TextureFormat::RGB => gl::RGB,
            TextureFormat::RGBA => gl::RGBA
        };
        let mut id=0;

        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32); //TODO: WHAT THE HELL IS GOING ON WITH THIS CONVERSION??? 
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32); //WHY IS IT NECESSARY??
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_COMPARE_FUNC, gl::LEQUAL as i32);
	    gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, width as i32, height as i32, 0, format, gl::UNSIGNED_BYTE, null());
        
        Texture { id, format, filled : false, width, height}
    }

    pub unsafe fn fill(&mut self, data : Vec<u8>) {
        assert_eq!(data.len(), (self.width*self.height) as usize, "Tried to fill a {}x{} texture with {} length data!",self.width,self.height,data.len());

	    gl::TexImage2D(gl::TEXTURE_2D, 0, self.format as i32, self.width as i32, self.height as i32, 0, self.format, gl::UNSIGNED_BYTE, null());
        self.filled = true;
    }
}

pub struct TextureManager {
    textures : Vec<Texture>,
    texture_names : HashMap<String, usize>
}

impl TextureManager {
    pub fn new() -> TextureManager {
        TextureManager {textures : vec![], texture_names : HashMap::new()}
    }

    pub fn get_texture_names(&self) -> HashMap<String, usize> {
        self.texture_names.clone()
    }

    pub fn add_texture(&mut self, texture : Texture, name : &str) {
        self.textures.push(texture);
        self.texture_names.insert(String::from(name), self.textures.len()-1);
    }
}

