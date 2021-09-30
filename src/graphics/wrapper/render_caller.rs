use super::{
    BufferTarget, FramebufferIdentifier, FramebufferManager, ShaderIdentifier, ShaderManager,
    TextureManager, VertexArray,
};
use crate::graphics::{RenderMessage, UniformData, VertexPack};
use crate::utils::Vertex3D;
use log::{debug, error};

const VERBOSE: bool = false;

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
    texture_manager: TextureManager,
    framebuffer_manager: FramebufferManager,
    screen_dimensions: (u32, u32),
}

impl RenderCaller {
    ///
    /// Marked as unsafe because it calls GL code
    pub unsafe fn new(screen_dimensions: (u32, u32)) -> RenderCaller {
        let vertex_array = VertexArray::new(Vertex3D::default()).unwrap();

        let shader_manager = super::loader::load_shaders();
        let texture_manager = super::loader::load_textures(screen_dimensions);
        let framebuffer_manager =
            super::loader::load_framebuffers(&texture_manager, screen_dimensions);

        RenderCaller {
            vertex_array,
            shader_manager,
            texture_manager,
            framebuffer_manager,
            screen_dimensions,
        }
    }

    pub unsafe fn update_screen_dimensions(&mut self, screen_dimensions: (u32, u32)) {
        self.screen_dimensions = screen_dimensions;
        self.texture_manager
            .update_screen_dimensions(screen_dimensions);
        self.framebuffer_manager
            .update_screen_dimensions(&self.texture_manager, screen_dimensions);
    }

    ///
    /// This is supposed to turn a packed render into something that can then be rendered directly. So
    /// this has access to OpenGL calls.
    /// TODO: Enforce requirements on RenderPack<T> to make this safe.
    unsafe fn pack(&mut self, buffer: &BufferTarget, pack: &VertexPack) {
        let target_id = buffer.get_target_id();
        if target_id >= self.vertex_array.get_vbo_count() {
            panic!(
                "Trying to clear {}, but there's only {} normal buffers ",
                buffer,
                self.vertex_array.get_vbo_count() - 1
            );
        }
        if VERBOSE {
            debug!("Packing buffer {}", buffer);
        }
        self.vertex_array.fill_vbo(target_id, &pack.vertices);
        self.vertex_array.fill_ebo(target_id, &pack.elements);
    }

    unsafe fn clear(&mut self, buffer: &BufferTarget) {
        let target_id = buffer.get_target_id();
        if target_id >= self.vertex_array.get_vbo_count() {
            panic!(
                "Trying to clear {}, but there's only {} normal buffers ",
                buffer,
                self.vertex_array.get_vbo_count()
            );
        }
        if VERBOSE {
            debug!("Clearing buffer {}", buffer);
        }
        self.vertex_array.clear(target_id);
    }

    unsafe fn choose_shader(&mut self, shader: ShaderIdentifier) {
        if let Err(s) = self.shader_manager.bind_shader(shader) {
            error!("{}", s)
        }
        if VERBOSE {
            debug!("Choosing shader {}", shader.name());
        }
    }

    unsafe fn uniforms(&mut self, uniforms: &UniformData) {
        if let Err(s) = self
            .shader_manager
            .uniforms(uniforms, &self.texture_manager)
        {
            error!("{}", s);
        }
        if VERBOSE {
            debug!("Passing uniforms");
        }
    }

    unsafe fn switch_to_2d(&mut self) {
        gl::Clear(gl::DEPTH_BUFFER_BIT);
        gl::Disable(gl::DEPTH_TEST);
        gl::Disable(gl::CULL_FACE); // TODO: This should not actually be done.
                                    // TODO: IS ANYTHING ELSE NEEDED FOR THE SWITCH TO 2D?
                                    // SHADER HAS TO BE HANDLED BY THE RENDERER
                                    // gl::Disable(gl::DEPTH_TEST);
    }

    pub unsafe fn read_message(&mut self, message: &RenderMessage) {
        match message {
            RenderMessage::Pack { buffer, pack } => self.pack(buffer, pack),
            RenderMessage::ClearArray { buffer } => self.clear(buffer),
            RenderMessage::ChooseShader { shader } => self.choose_shader(*shader),
            RenderMessage::Uniforms { uniforms } => self.uniforms(uniforms),
            RenderMessage::Draw { buffer } => self.render(buffer),
            RenderMessage::ClearBuffers {
                color_buffer,
                depth_buffer,
            } => self.clear_buffers(color_buffer, depth_buffer),
            RenderMessage::ChooseFramebuffer { framebuffer } => {
                self.choose_framebuffer(&framebuffer)
            }
            RenderMessage::Compute {
                output_texture,
                dimensions,
            } => self.dispatch_compute(output_texture, dimensions),
            RenderMessage::SwitchTo2D {} => {
                self.switch_to_2d();
            }
        }
    }

    pub unsafe fn choose_framebuffer(&mut self, framebuffer: &Option<FramebufferIdentifier>) {
        self.framebuffer_manager.bind_framebuffer(&framebuffer);
        if VERBOSE {
            debug!("Choosing framebuffer {:?}", framebuffer);
        }
    }

    pub unsafe fn render(&mut self, buffer: &BufferTarget) {
        let target_id = buffer.get_target_id();
        debug_assert!(
            self.vertex_array.get_size(target_id) > 0,
            "A render call was made on an empty vertex array!"
        );
        self.vertex_array.draw(target_id);
        if VERBOSE {
            debug!("Rendering buffer {}", buffer);
        }
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

    pub unsafe fn dispatch_compute(&mut self, output_texture: &str, dimensions: &(u32, u32, u32)) {
        let tex = self.texture_manager.get_texture(output_texture);
        gl::BindImageTexture(
            0,
            tex.get_id(),
            0,
            gl::FALSE,
            0,
            gl::WRITE_ONLY,
            tex.metadata.internal_format.to_gl(),
        );
        gl::DispatchCompute(dimensions.0, dimensions.1, dimensions.2);
        if VERBOSE {
            debug!(
                "Dispatching compute shader generating texture {}, id {}",
                output_texture,
                tex.get_id()
            );
        }
    }

    pub fn get_vbo_count(&self) -> usize {
        self.vertex_array.get_vbo_count()
    }

    pub fn get_texture_manager(&self) -> &TextureManager {
        &self.texture_manager
    }

    pub fn get_shader_manager(&self) -> &ShaderManager {
        &self.shader_manager
    }

    pub fn get_framebuffer_manager(&self) -> &FramebufferManager {
        &self.framebuffer_manager
    }
}
