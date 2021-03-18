use super::{ShaderManager, VertexArray};
use crate::graphics::{RenderMessage, UniformData, VertexPack};
use crate::utils::Vertex3D;

///
/// TODO
/// This struct is in charge of rendering one round. Also in terms of setting opengl settings.
///
/// It needs to know what things change and what things stay the same, so it doesn't change anything that makes
/// opengl slow. I actually basically need a function for changing opengl things that knows if they're actually
/// changed.
///
pub struct RenderCaller {
    vertex_array: VertexArray<Vertex3D>,
    pub shader_manager: ShaderManager,
}

impl RenderCaller {
    ///
    /// Marked as unsafe because it calls GL code
    pub unsafe fn new() -> RenderCaller {
        let vertex_array = VertexArray::new(Vertex3D::dummy()).unwrap();
        let mut shader_manager = ShaderManager::new();

        //TODO: This should probably not be called from the RenderCaller new.
        shader_manager.load_shaders("shaders");

        RenderCaller {
            vertex_array,
            shader_manager,
        }
    }

    ///
    /// This is supposed to turn a packed render into something that can then be rendered directly. So
    /// this has access to OpenGL calls.
    /// TODO: Enforce requirements on RenderPack<T> to make this safe.
    unsafe fn unpack(&mut self, buffer: &usize, pack: &VertexPack) {
        if *buffer >= self.vertex_array.get_vbos() {
            panic!(
                "Trying to clear a buffer with index {}, but there's only {} buffers ",
                buffer,
                self.vertex_array.get_vbos()
            );
        }
        self.vertex_array.fill_vbo(*buffer, &pack.vertices);
        self.vertex_array.fill_ebo(*buffer, &pack.elements);
    }

    unsafe fn clear(&mut self, buffer: &usize) {
        if *buffer >= self.vertex_array.get_vbos() {
            panic!(
                "Trying to clear an array with index {}, but there's only {} arrays ",
                buffer,
                self.vertex_array.get_vbos()
            );
        }
        self.vertex_array.clear(*buffer);
    }

    /// TODO: THIS IS WHERE YOU LEFT OFF, CONTINUE FROM HERE
    unsafe fn choose_shader(&mut self, shader: &String) {
        match self.shader_manager.bind_shader(shader) {
            Err(s) => {
                println!("{}", s)
            } //TODO: LOG INSTEAD
            _ => (),
        }
    }

    unsafe fn uniforms(&mut self, uniforms: &UniformData) {
        match self.shader_manager.uniforms(uniforms) {
            Err(s) => {
                println!("{}", s)
            } //TODO: LOG INSTEAD
            _ => (),
        }
    }

    pub unsafe fn read_message(&mut self, message: &RenderMessage) {
        match message {
            RenderMessage::Pack { buffer, pack } => self.unpack(buffer, pack),
            RenderMessage::ClearArray { buffer } => self.clear(buffer),
            RenderMessage::ChooseShader { shader } => self.choose_shader(shader),
            RenderMessage::Uniforms { uniforms } => self.uniforms(uniforms),
            RenderMessage::Draw { buffer } => self.render(buffer),
            RenderMessage::ClearBuffers {
                color_buffer,
                depth_buffer,
            } => self.clear_buffers(color_buffer, depth_buffer),
        }
    }

    pub unsafe fn render(&mut self, buffer: &usize) {
        debug_assert!(
            self.vertex_array.get_size(*buffer) > 0,
            "A render call was made on an empty vertex array!"
        );
        self.vertex_array.draw(*buffer);
    }

    pub unsafe fn clear_buffers(&mut self, color_buffer: &bool, depth_buffer: &bool) {
        debug_assert!(*color_buffer || *depth_buffer, "A clear buffer call should never be made when neither color nor depth buffer is cleared!");
        gl::Clear(
            (if *color_buffer {
                gl::COLOR_BUFFER_BIT
            } else {
                0
            }) | (if *depth_buffer {
                gl::DEPTH_BUFFER_BIT
            } else {
                0
            }),
        );
    }
}
