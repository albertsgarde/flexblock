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
    /// A hashmap of shader names and their metadata
    pub shader_metadata: HashMap<String, ShaderMetadata>,
    /// A hashmap of framebuffer names and their metadata
    pub framebuffer_metadata : HashMap<String, FramebufferMetadata>
}
