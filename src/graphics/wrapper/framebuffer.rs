use serde::{Serialize, Deserialize};
use strum::{EnumCount, EnumIter};
use super::TextureManager;

#[derive(Clone, Copy, Debug, EnumCount, EnumIter, Serialize, Deserialize)]
pub enum FramebufferIdentifier {
    FirstPassFramebuffer
}

impl FramebufferIdentifier {
    /*pub fn path(&self) -> &'static str{
        match self {
            FramebufferIdentifier::FirstPassFramebuffer => "graphics/framebuffers/firstpassframebuffer.json"
        }
    } TODO: THIS MAY BE REMOVED ONE DAY*/

    pub fn name(&self) -> &'static str {
        match self {
            FramebufferIdentifier::FirstPassFramebuffer => "First pass framebuffer"
        }
    }

    pub fn color_texture(&self) -> Option<&'static str> {
        match self {
            FramebufferIdentifier::FirstPassFramebuffer => Some("fpfcolor")
        }
    }

    pub fn depth_texture(&self) -> Option<&'static str> {
        match self {
            FramebufferIdentifier::FirstPassFramebuffer => None
        }
    }

    pub fn has_depth(&self) -> bool {
        match self {
            FramebufferIdentifier::FirstPassFramebuffer => false
        }
    }

    /// TODO: Figure out a way to make dimensions in terms of screen dimensions.
    pub fn dimensions(&self) -> (usize,usize) {
        match self {
            FramebufferIdentifier::FirstPassFramebuffer => (800,800)
        }
    }
}

pub struct Framebuffer{
    id : u32,
    metadata : FramebufferMetadata,
}

impl Framebuffer {

    pub unsafe fn new(identifier :FramebufferIdentifier, texture_manager : &TextureManager) -> Result<Framebuffer, &'static str>{
        // TODO: VERYIFY THAT TEXTURE SIZES CORRESPOND TO FRAMEBUFFER SIZE
        
        if identifier.depth_texture().is_none() && identifier.color_texture().is_none() {
            return Err("A framebuffer cannot be instantiated with neither a color texture nor a depth texture!");
        }

        if identifier.depth_texture().is_some() && !identifier.has_depth() {
            return Err("Framebuffer told to not have depth, but a depth texture is provided!");
        }

        let mut id = 0;

        gl::GenFramebuffers(1, &mut id);
        gl::BindFramebuffer(gl::FRAMEBUFFER, id);
        if let Some(dt) = identifier.depth_texture() {
            let dt = texture_manager.get_texture(dt);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, dt.get_id(), 0);
        } else {
            if identifier.has_depth() {
                let mut depth_render_buffer = 0;
                gl::GenRenderbuffers(1, &mut depth_render_buffer);
                gl::BindRenderbuffer(gl::RENDERBUFFER, depth_render_buffer);
                gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, identifier.dimensions().0 as i32, identifier.dimensions().1 as i32);
                gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, depth_render_buffer);
            }
        }

        if let Some(ct) = identifier.color_texture() {
            let ct = texture_manager.get_texture(ct);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, ct.get_id(), 0);
            //TODO: DO COLOR TEXTURES WORK??
        } else {
            gl::DrawBuffer(gl::NONE);
            //TODO: DOES NO DRAW BUFFER WORK??
        }

        Ok(Framebuffer {
            id,
            metadata : FramebufferMetadata {
                identifier, width : identifier.dimensions().0 as u32, height : identifier.dimensions().1 as u32,
            }
        })
    }

    pub unsafe fn bind(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
    } 
}

/// Note: This stores width & height, because framebuffers (and their textures) will need reinitialization when the screen is resized.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FramebufferMetadata {
    pub identifier : FramebufferIdentifier,
    pub width : u32,
    pub height : u32,
}

///TODO: We need to be able to update all framebuffers that depend on screen dimensions on resize.
pub struct FramebufferManager {
    /// Framebuffers indexed by FramebufferIdentifier enum discriminants
    framebuffers : Vec<Framebuffer>,
}

impl FramebufferManager {
    pub fn new(framebuffers : Vec<Framebuffer>) -> FramebufferManager {
        let mut count=0;
        for framebuffer in &framebuffers {
            if framebuffer.metadata.identifier as usize != count {
                panic!("Loading framebuffers into the framebuffer manager out of order!")
            }
            count += 1;
        }
        if count < FramebufferIdentifier::COUNT {
            panic!("Not enough framebuffers were supplied for the framebuffer manager")
        }
        FramebufferManager {
            framebuffers
        }
    }

    pub fn get_framebuffer_metadata(&self) -> Vec<FramebufferMetadata> {
        let mut res = Vec::new();
        for framebuffer in &self.framebuffers {
            res.push(framebuffer.metadata.clone());
        }
        res
    }

    ///Passing nothing as the framebuffer will bind the screen - the standard draw buffer.
    pub unsafe fn bind_framebuffer(&self, framebuffer : &Option<FramebufferIdentifier>) {
        match framebuffer { 
            Some(fb) => {
                self.framebuffers[*fb as usize].bind();
            },
            None => {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FramebufferMetadata, FramebufferIdentifier};
    fn serialize_framebuffer_metadata() {
        let metadata = FramebufferMetadata {
            identifier : FramebufferIdentifier::FirstPassFramebuffer,
            width : 800,
            height : 800,
        };
        let j = serde_json::to_string(&metadata).unwrap();

        println!("{}",j);
    }

    fn deserialize_framebuffer_metadata() {
        let j = r#"[
            {"name":"test","has_depth":false,"width":800,"height":800,"color_texture":"a good name","depth_texture":null},
            {"name":"test","has_depth":false,"width":800,"height":800,"color_texture":"a good name","depth_texture":null}
        ]"#;
        let metadata : Vec<FramebufferMetadata> = serde_json::from_str(j).unwrap();

        println!("{:?}",metadata);
    }
}