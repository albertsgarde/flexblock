use std::collections::HashMap;
use crate::graphics::wrapper::{ShaderMetadata, TextureMetadata, FramebufferMetadata};

///TODO
///Contains the capabilities that the Graphics wrapper makes available to the packer.
pub struct GraphicsCapabilities {
    /// The number of avaliable VBOS in the 3d vertex array
    pub vbo_count: usize,
    /// A hashmap of texture names and texture ids
    /// TODO: Convert to same format as shaders
    pub texture_metadata: HashMap<String, TextureMetadata>,
    /// A vector of shader metadata indexed by identifiers
    pub shader_metadata: Vec<ShaderMetadata>,
    /// A hashmap of framebuffer names and their metadata
    pub framebuffer_metadata : Vec<FramebufferMetadata>,
    /// A tuple of current (width,height) of the screen.
    pub screen_dimensions : (u32,u32)
}
