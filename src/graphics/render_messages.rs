use crate::utils::vertex::Vertex3D;
use std::slice::Iter;
use std::fmt;

use super::wrapper::{ShaderIdentifier, FramebufferIdentifier};

#[derive(Debug)]
pub struct RenderMessages {
    messages: Vec<RenderMessage>,
    /// TODO: THIS IS SUCH A BAD NAME
    /// Indicates the index where the first new message occurs. Before this index, only persistent render messages occur.
    old_new_split_index : usize,
}
///
/// Holds everything needed for one render pass, but in CPU memory.
///
/// This struct does not call any Gl, all Gl is called later
///
impl RenderMessages {
    pub fn new() -> RenderMessages {
        let messages = Vec::new();
        RenderMessages { messages, old_new_split_index : 0 }
    }

    pub fn add_message(&mut self, message: RenderMessage) {
        self.messages.push(message);
    }

    pub fn iter(&self) -> Iter<'_, RenderMessage> {
        self.messages.iter()
    }

    pub fn size(&self) -> usize {
        self.messages.len()
    }

    pub fn old_new_split_index(&self) -> usize {
        self.old_new_split_index
    }

    /// Merges an old, unused render pack into the render pack, putting it first and only keeping persistent render messages
    /// TODO: Make things cancel each other out, like filling and emptying the same vertex array should cancel out - this is probably actually a very inconsequential optimization
    pub fn merge_old(&mut self, old_pack: RenderMessages) {
        let mut new_messages: Vec<RenderMessage> = old_pack
            .messages
            .into_iter()
            .filter(|x| x.is_persistent())
            .collect();
        self.old_new_split_index = new_messages.len();
        new_messages.append(&mut self.messages);
        self.messages = new_messages;
    }
}

pub struct VertexPack {
    pub vertices: Vec<Vertex3D>,
    pub elements: Vec<u32>,
}

/// One render message to the graphics thread.
/// TODO: Documentation & contract checking
#[derive(Debug)]
pub enum RenderMessage {
    /// buffer = which buffer in the vertex array to target
    Pack {
        buffer: usize,
        pack: VertexPack,
    },
    /// buffer = which buffer in the vertex array to target.
    ClearArray {
        buffer: usize,
    },
    ClearBuffers {
        /// Whether to clear the color buffer
        color_buffer: bool,
        // Whether to clear the depth buffer
        depth_buffer: bool,
    },
    ChooseShader {
        shader: ShaderIdentifier,
    },
    Uniforms {
        uniforms: UniformData,
    },
    /// buffer = which buffer in the vertex array to target.
    Draw {
        buffer: usize,
    },
    /// framebuffer = which framebuffer to
    ChooseFramebuffer {
        framebuffer : Option<FramebufferIdentifier>,
    },
}

impl RenderMessage {
    /// Whether a render message is persistent across graphics ticks or not.
    /// The prototypical persistent render message is Pack, which packs a vertex array, and this vertex array will then be used again and again
    /// The prototypical impersistent render message is Draw. Everything that is drawn is 100% drawn every tick.
    pub fn is_persistent(&self) -> bool {
        match self {
            RenderMessage::Pack { buffer: _, pack: _ } => true,
            RenderMessage::ClearArray { buffer: _ } => true,
            RenderMessage::ClearBuffers {
                color_buffer: _,
                depth_buffer: _,
            } => false,
            RenderMessage::ChooseShader { shader: _ } => false,
            RenderMessage::Uniforms { uniforms: _ } => false,
            RenderMessage::Draw { buffer: _ } => false,
            RenderMessage::ChooseFramebuffer {framebuffer : _} => false,
        }
    }
}

impl VertexPack {
    ///TODO: Make this follow the contract
    pub fn new(vertices: Vec<Vertex3D>, elements: Option<Vec<u32>>) -> VertexPack {
        let elements = match elements {
            Some(e) => e,
            None => Vec::new(),
        };
        VertexPack { vertices, elements }
    }
}

impl fmt::Debug for VertexPack {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("VertexPack")
    }
}

//TODO: Add mat 3, 2, and vec 3, 2, and f32, u32, i32, and texture
pub struct UniformData {
    pub mat4s: Vec<(glm::Mat4, String)>,
    pub vec3s: Vec<(glm::Vec3, String)>,
    /// The first string is the texture name, second is the uniform location name
    pub textures: Vec<(String, String)>,
}

impl UniformData {
    ///
    /// textures = (texture name, uniform location name)
    pub fn new(
        mat4s: Vec<(glm::Mat4, String)>,
        vec3s: Vec<(glm::Vec3, String)>,
        textures: Vec<(String, String)>,
    ) -> UniformData {
        UniformData {
            mat4s,
            vec3s,
            textures,
        }
    }

    /// Gets the list of all uniforms referred to by this set of uniform data.
    pub fn get_uniform_locations(&self) -> Vec<&String> {
        let mut res = Vec::new();

        for entry in &self.mat4s {
            res.push(&entry.1);
        }
        for entry in &self.vec3s {
            res.push(&entry.1);
        }
        for entry in &self.textures {
            res.push(&entry.1);
        }

        res
    }
}

impl fmt::Debug for UniformData {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("UniformData")
    }
}