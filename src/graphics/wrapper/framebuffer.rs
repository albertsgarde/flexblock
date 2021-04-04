use super::Texture;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub struct Framebuffer{
    id : u32,
    metadata : FramebufferMetadata,
}

impl Framebuffer {

    pub unsafe fn new(name : &str, color_texture : Option<&Texture>, depth_texture : Option<&Texture>, width : u32, height : u32, has_depth : bool) -> Result<Framebuffer, &'static str>{
        // TODO: VERYIFY THAT TEXTURE SIZES CORRESPOND TO FRAMEBUFFER SIZE
        
        if depth_texture.is_none() && color_texture.is_none() {
            return Err("A framebuffer cannot be instantiated with neither a color texture nor a depth texture!");
        }

        if depth_texture.is_some() && !has_depth {
            return Err("Framebuffer told to not have depth, but a depth texture is provided!");
        }

        let mut id = 0;

        gl::GenFramebuffers(1, &mut id);
        gl::BindFramebuffer(gl::FRAMEBUFFER, id);
        
        if let Some(dt) = depth_texture {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, dt.get_id(), 0);
        } else {
            if has_depth {
                let mut depth_render_buffer = 0;
                gl::GenRenderbuffers(1, &mut depth_render_buffer);
                gl::BindRenderbuffer(gl::RENDERBUFFER, depth_render_buffer);
                gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width as i32, height as i32);
                gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, depth_render_buffer);
                //TODO: DOES NO DEPTH WORK??
                //TODO: DOES DEPTH WORK??
            }
        }

        if let Some(ct) = color_texture {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, ct.get_id(), 0);
            //TODO: DO COLOR TEXTURES WORK??
        } else {
            gl::DrawBuffer(gl::NONE);
            //TODO: DOES NO DRAW BUFFER WORK??
        }

        Ok(Framebuffer {
            id,
            metadata : FramebufferMetadata {
                name : String::from(name), has_depth, width, height,
                color_texture : match color_texture { Some(tex) => Some(String::from(&tex.metadata.name)), None => None},
                depth_texture : match depth_texture { Some(tex) => Some(String::from(&tex.metadata.name)), None => None}
            }
        })
    }

    pub unsafe fn bind(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
    } 
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FramebufferMetadata {
    pub name : String,
    pub has_depth : bool,
    pub width : u32,
    pub height : u32,
    pub color_texture : Option<String>,
    pub depth_texture : Option<String>,
}

pub struct FramebufferManager {
    framebuffers : Vec<Framebuffer>,
    framebuffer_names : HashMap<String, usize>
}

impl FramebufferManager {
    pub fn new() -> FramebufferManager {
        FramebufferManager {
            framebuffers : Vec::new(),
            framebuffer_names : HashMap::new()
        }
    }

    pub fn add_framebuffer(&mut self, framebuffer : Framebuffer) {
        self.framebuffer_names.insert(String::from(&framebuffer.metadata.name), self.framebuffers.len());
        self.framebuffers.push(framebuffer);
    }

    pub fn get_framebuffer_metadata(&self) -> HashMap<String, FramebufferMetadata> {
        let mut res = HashMap::new();
        for framebuffer in &self.framebuffers {
            res.insert(String::from(&framebuffer.metadata.name), framebuffer.metadata.clone());
        }
        res
    }

    ///Passing nothing as the framebuffer will bind the screen - the standard draw buffer.
    pub unsafe fn bind_framebuffer(&self, framebuffer : &Option<String>) {
        match framebuffer { 
            Some(fb) => {
                let index = self.framebuffer_names.get(fb).unwrap();
                self.framebuffers[*index].bind();
            },
            None => {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FramebufferMetadata;
    fn serialize_framebuffer_metadata() {
        let metadata = FramebufferMetadata {
            name : "test".to_owned(),
            has_depth : false,
            width : 800,
            height : 800,
            color_texture : Some("a good name".to_owned()),
            depth_texture : None
        };
        let j = serde_json::to_string(&metadata).unwrap();

        println!("{}",j);
    }

    #[test]
    fn deserialize_framebuffer_metadata() {
        let j = r#"[
            {"name":"test","has_depth":false,"width":800,"height":800,"color_texture":"a good name","depth_texture":null},
            {"name":"test","has_depth":false,"width":800,"height":800,"color_texture":"a good name","depth_texture":null}
        ]"#;
        let metadata : Vec<FramebufferMetadata> = serde_json::from_str(j).unwrap();

        println!("{:?}",metadata);
    }
}