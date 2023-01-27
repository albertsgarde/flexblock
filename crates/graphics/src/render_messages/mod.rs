mod uniform_data;
pub use uniform_data::{UniformData, UniformValue};

mod vertex_pack;
pub use vertex_pack::VertexPack;

use crate::wrapper::BufferTarget;
use crate::wrapper::{FramebufferIdentifier, ShaderIdentifier};
use std::slice::Iter;

#[derive(Debug)]
pub struct RenderMessages {
    messages: Vec<RenderMessage>,
    /// TODO: THIS IS SUCH A BAD NAME
    /// Indicates the index where the first new message occurs. Before this index, only persistent render messages occur.
    old_new_split_index: usize,
}
///
/// Holds everything needed for one render pass, but in CPU memory.
///
/// This struct does not call any Gl, all Gl is called later
///
impl RenderMessages {
    pub fn new() -> RenderMessages {
        let messages = Vec::new();
        RenderMessages {
            messages,
            old_new_split_index: 0,
        }
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

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
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

    /// Merges a current render pack onto the end of this one.
    pub fn merge_current(&mut self, new_pack: RenderMessages) {
        let mut new_pack = new_pack;
        self.messages.append(&mut new_pack.messages);
    }
}

impl Default for RenderMessages {
    fn default() -> Self {
        Self::new()
    }
}

/// One render message to the graphics thread.
/// TODO: Documentation & contract checking
#[derive(Debug)]
pub enum RenderMessage {
    /// buffer = which buffer in the vertex array to target
    Pack {
        buffer: BufferTarget,
        pack: VertexPack,
    },
    /// buffer = which buffer in the vertex array to target.
    ClearArray {
        buffer: BufferTarget,
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
        uniforms: Box<UniformData>,
    },
    /// buffer = which buffer in the vertex array to target.
    Draw {
        buffer: BufferTarget,
    },
    /// framebuffer = which framebuffer to draw to. None for the screen.
    ChooseFramebuffer {
        framebuffer: Option<FramebufferIdentifier>,
    },
    /// Runs the currently bound compute shader.
    Compute {
        output_texture: String,
        dimensions: (u32, u32, u32),
    },
    /// Switches the rendering to a 2D context.
    SwitchTo2D {},
    SwitchTo3D {},
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
            RenderMessage::ChooseFramebuffer { framebuffer: _ } => false,
            RenderMessage::Compute {
                output_texture: _,
                dimensions: _,
            } => false,
            RenderMessage::SwitchTo2D {} => false,
            RenderMessage::SwitchTo3D {} => false,
        }
    }
}
