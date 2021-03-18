use crate::utils::vertex::Vertex3D;
use std::slice::Iter;

pub struct RenderMessages {
    messages: Vec<RenderMessage>,
}
///
/// Holds everything needed for one render pass, but in CPU memory.
///
/// This struct does not call any Gl, all Gl is called later
///
impl RenderMessages {
    pub fn new() -> RenderMessages {
        let messages = Vec::new();
        RenderMessages { messages }
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

    /// Merges an old, unused render pack into the render pack, putting it first and only keeping persistent render messages
    /// TODO: Make things cancel each other out, like filling and emptying the same vertex array should cancel out - this is probably actually a very inconsequential optimization
    pub fn merge_old(&mut self, old_pack : RenderMessages) {
        let mut new_messages : Vec<RenderMessage>= old_pack.messages.into_iter().filter(|x|  x.is_persistent()).collect();
        new_messages.append(&mut self.messages);
        self.messages= new_messages;
    }
}

pub struct VertexPack {
    pub vertices: Vec<Vertex3D>,
    pub elements: Vec<u32>,
}


/// One render message to the graphics thread.
/// TODO: Documentation & contract checking
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
        color_buffer : bool,
        // Whether to clear the depth buffer
        depth_buffer : bool,
    },
    ChooseShader {
        shader: String,
    },
    Uniforms {
        uniforms: UniformData,
    },
    /// buffer = which buffer in the vertex array to target.
    Draw { buffer : usize }
}

impl RenderMessage {
    /// Whether a render message is persistent across graphics ticks or not.
    /// The prototypical persistent render message is Pack, which packs a vertex array, and this vertex array will then be used again and again
    /// The prototypical impersistent render message is Draw. Everything that is drawn is 100% drawn every tick.
    pub fn is_persistent(&self) -> bool {
        match self {
            RenderMessage::Pack{buffer: _,pack: _} => true,
            RenderMessage::ClearArray{buffer: _} => true,
            RenderMessage::ClearBuffers { color_buffer:_, depth_buffer:_ } => false,
            RenderMessage::ChooseShader {shader: _} => false,
            RenderMessage::Uniforms { uniforms: _ } => false,
            RenderMessage::Draw { buffer: _ } => false,
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

//TODO: Add mat 3, 2, and vec 3, 2, and f32, u32, i32
pub struct UniformData {
    pub mat4s: Vec<(glm::Mat4, String)>,
    pub vec4s: Vec<(glm::Vec3, String)>,
}

impl UniformData {
    pub fn new(mat4s: Vec<(glm::Mat4, String)>, vec4s: Vec<(glm::Vec3, String)>) -> UniformData {
        UniformData { mat4s, vec4s }
    }
}
