use crate::UniformData;
use crate::{wrapper::ShaderIdentifier, RenderMessage, RenderMessages};

pub struct ComputePipeline {
    dispatches: Vec<ComputeDispatch>,
}

pub struct ComputeDispatch {
    shader: ShaderIdentifier,
    uniforms: UniformData,
    output_texture: String,
    dimensions: (u32, u32, u32),
}

impl ComputePipeline {
    pub fn new() -> ComputePipeline {
        ComputePipeline {
            dispatches: Vec::new(),
        }
    }

    pub fn add_dispatch(&mut self, dispatch: ComputeDispatch) {
        self.dispatches.push(dispatch);
    }

    pub fn get_messages(self) -> RenderMessages {
        let mut messages = RenderMessages::new();

        for dispatch in self.dispatches {
            messages.merge_current(dispatch.create_messages());
        }
        messages
    }
}

impl Default for ComputePipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl ComputeDispatch {
    pub fn new<T: Into<String>>(
        shader: ShaderIdentifier,
        uniforms: UniformData,
        output_texture: T,
        dimensions: (u32, u32, u32),
    ) -> ComputeDispatch {
        ComputeDispatch {
            shader,
            uniforms,
            output_texture: output_texture.into(),
            dimensions,
        }
    }

    pub fn create_messages(self) -> RenderMessages {
        let mut messages = RenderMessages::new();

        messages.add_message(RenderMessage::ChooseShader {
            shader: self.shader,
        });
        messages.add_message(RenderMessage::Uniforms {
            uniforms: Box::new(self.uniforms),
        });
        messages.add_message(RenderMessage::Compute {
            output_texture: self.output_texture,
            dimensions: self.dimensions,
        });

        messages
    }
}
