
use crate::utils::vertex::{Vertex3D, AttributePointerList};
use std::slice::Iter;

pub struct RenderPack {
    messages: Vec<RenderMessage>,
}
///
/// Holds everything needed for one render pass, but in CPU memory.
///
/// This struct does not call any Gl, all Gl is called later
///
impl RenderPack {
    pub fn new() -> RenderPack {
        let messages = Vec::new();
        RenderPack { messages }
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
}


pub struct RenderData {
    pub vertices: Vec<Vertex3D>,
    pub elements: Vec<u32>,
    attributes: AttributePointerList,
}

pub enum RenderMessage {
    Pack {
        vertex_array: usize,
        pack: RenderData,
    },
    Clear {
        vertex_array: usize,
    },
    ChooseShader {
        shader: String,
    },
    Uniforms {
        uniforms: UniformData,
    },
}

impl RenderData {
    ///TODO: Make this follow the contract
    pub fn new(
        vertices: Vec<Vertex3D>,
        elements: Option<Vec<u32>>,
        attributes: AttributePointerList,
    ) -> RenderData {
        let elements = match elements {
            Some(e) => e,
            None => Vec::new(),
        };
        RenderData {
            vertices,
            elements,
            attributes,
        }
    }

    pub fn get_stride(&self) -> usize {
        self.attributes.get_stride()
    }
}


//TODO: Add mat 3, 2, and vec 3, 2, and f32, u32, i32
pub struct UniformData {
    pub mat4s: Vec<(cgmath::Matrix4<f32>, String)>,
    pub vec4s: Vec<(cgmath::Vector4<f32>, String)>,
}

impl UniformData {
    pub fn new(mat4s: Vec<(cgmath::Matrix4<f32>, String)>, vec4s: Vec<(cgmath::Vector4<f32>, String)>) -> UniformData {
        UniformData { mat4s, vec4s }
    }
}
