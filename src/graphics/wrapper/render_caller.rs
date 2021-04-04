use super::{ShaderManager, TextureManager, FramebufferManager, Texture, Framebuffer, VertexArray};
use crate::graphics::{RenderMessage, UniformData, VertexPack};
use crate::utils::{Vertex3D, read_png};

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
    framebuffer_manager : FramebufferManager
}

impl RenderCaller {
    ///
    /// Marked as unsafe because it calls GL code
    pub unsafe fn new() -> RenderCaller {
        let vertex_array = VertexArray::new(Vertex3D::dummy()).unwrap();

        let shader_manager = RenderCaller::load_shaders();
        let texture_manager = RenderCaller::load_textures();
        let framebuffer_manager = RenderCaller::load_framebuffers(&texture_manager);

        RenderCaller {
            vertex_array,
            shader_manager,
            texture_manager,
            framebuffer_manager
        }
    }

    unsafe fn load_shaders() -> ShaderManager{

        let mut shader_manager = ShaderManager::new();

        //TODO: This should maybe not be called from the RenderCaller new. Some decision has to be made.
        shader_manager.load_shaders("graphics/shaders");

        shader_manager
    }

    unsafe fn load_textures() -> TextureManager {
        let mut texture_manager = TextureManager::new();

        let entries = crate::utils::dir_entries(&std::path::Path::new("./graphics/textures"), "");
        let entries = match entries {
            Ok(e) => e,
            Err(error) => { panic!("Could not load textures! {:?}", error)}
        };
        for entry in entries {
            if entry.1.ends_with(".png") {
                let data = match read_png(&entry.0.path()) {
                    Ok(d) => d,
                    Err(error) => {
                        println!("Failed to load texture in png file {:?}! Error {:?}",entry.0.path(), error);
                        continue;
                    }
                };


                let mut t1 = Texture::new(data.width, data.height, data.format, &entry.1);
                t1.fill(data.data);
                println!("Loaded texture {}!", &t1.metadata.name);
                texture_manager.add_texture(t1);
            } else if entry.1.ends_with(".json") {
                todo!("Do .json files!");
            }
        }
        //let mut t1 = Texture::new(800, 800, TextureFormat::RGB, "atlas");
        //t1.fill(crate::utils::read_png("textures/atlas.png"));
        //texture_manager.add_texture(t1);

        texture_manager
    }

    unsafe fn load_framebuffers(texture_manager : &TextureManager) -> FramebufferManager {
        let mut framebuffer_manager = FramebufferManager::new();
        framebuffer_manager.add_framebuffer(Framebuffer::new("f1", Some(texture_manager.get_texture("/atlas.png")), None, 800, 800, true).unwrap());

        framebuffer_manager
    }


    ///
    /// This is supposed to turn a packed render into something that can then be rendered directly. So
    /// this has access to OpenGL calls.
    /// TODO: Enforce requirements on RenderPack<T> to make this safe.
    unsafe fn unpack(&mut self, buffer: &usize, pack: &VertexPack) {
        if *buffer >= self.vertex_array.get_vbo_count() {
            panic!(
                "Trying to clear a buffer with index {}, but there's only {} buffers ",
                buffer,
                self.vertex_array.get_vbo_count()
            );
        }
        self.vertex_array.fill_vbo(*buffer, &pack.vertices);
        self.vertex_array.fill_ebo(*buffer, &pack.elements);
    }

    unsafe fn clear(&mut self, buffer: &usize) {
        if *buffer >= self.vertex_array.get_vbo_count() {
            panic!(
                "Trying to clear an array with index {}, but there's only {} arrays ",
                buffer,
                self.vertex_array.get_vbo_count()
            );
        }
        self.vertex_array.clear(*buffer);
    }

    unsafe fn choose_shader(&mut self, shader: &str) {
        match self.shader_manager.bind_shader(shader) {
            Err(s) => {
                println!("{}", s)
            } //TODO: LOG INSTEAD
            _ => (),
        }
    }

    unsafe fn uniforms(&mut self, uniforms: &UniformData) {
        match self
            .shader_manager
            .uniforms(uniforms, &self.texture_manager)
        {
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
            RenderMessage::ChooseFramebuffer {framebuffer} => self.choose_framebuffer(&framebuffer),
        }
    }

    pub unsafe fn choose_framebuffer(&mut self, framebuffer : &Option<String>) {
        self.framebuffer_manager.bind_framebuffer(&framebuffer);
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
