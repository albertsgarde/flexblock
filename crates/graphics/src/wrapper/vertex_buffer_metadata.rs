use crate::BufferTarget;

///
/// Contains info about where buffer targets go in the range of available opengl buffers.
/// 
#[allow(dead_code)]
pub struct VertexBufferMetadata {
    gui_buffer : usize,
    world_buffer_start : usize,
    model_buffer_start : usize,
    max_buffer_id : usize,
}

pub const VERTEX_BUFFER_METADATA : VertexBufferMetadata = VertexBufferMetadata {
    gui_buffer : 0,
    world_buffer_start : 1,
    model_buffer_start : 100,
    max_buffer_id : 200,
};

impl VertexBufferMetadata {
    pub fn new(model_buffer_start : usize, max_buffer_id : usize ) -> Self {
        if model_buffer_start <= 1 {
            panic!("Passed invalid data into VertexBufferMetadata! (graphics::wrapper::vertex_buffer_metadata l. 12");
        }
        if max_buffer_id <= model_buffer_start {
            panic!("Passed invalid data into VertexBufferMetadata! (graphics::wrapper::vertex_buffer_metadata l. 16");
        }
        Self {
            gui_buffer : 0, world_buffer_start : 1, model_buffer_start, max_buffer_id
        }
    }

    pub fn valid_target(&self, buffer_target : &BufferTarget) -> bool {
        match buffer_target {
            BufferTarget::GuiBuffer => true,
            BufferTarget::WorldBuffer(i) => *i < self.model_buffer_start - self.world_buffer_start,
            BufferTarget::ModelBuffer(i) => *i < self.max_buffer_id - self.model_buffer_start
        }
    }

    pub fn world_buffer_start(&self) -> usize {
        self.world_buffer_start
    }

    pub fn model_buffer_start(&self) -> usize {
        self.model_buffer_start
    }

    pub fn max_buffer_id(&self)-> usize {
        self.max_buffer_id
    }
}