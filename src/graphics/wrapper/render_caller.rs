
use super::{VertexArray, ShaderManager};
use crate::graphics::{UniformData, RenderMessage, RenderData};
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
    unsafe fn unpack(&mut self, array: &usize, pack: &RenderData) {
        if *array >= self.vertex_array.get_vbos() {
            panic!(
                "Trying to clear an array with index {}, but there's only {} arrays ",
                array,
                self.vertex_array.get_vbos()
            );
        }
        self.vertex_array.fill_vbo(*array, &pack.vertices);
        self.vertex_array.fill_ebo(*array, &pack.elements);
    }

    unsafe fn clear(&mut self, array: &usize) {
        if *array >= self.vertex_array.get_vbos() {
            panic!(
                "Trying to clear an array with index {}, but there's only {} arrays ",
                array,
                self.vertex_array.get_vbos()
            );
        }
        self.vertex_array.clear(*array);
    }

    /// TODO: THIS IS WHERE YOU LEFT OFF, CONTINUE FROM HERE
    unsafe fn choose_shader(&mut self, shader: &String) {
        println!("Choosing shader {}!", shader);
        self.shader_manager.bind_shader(shader).unwrap();
    }

    unsafe fn uniforms(&mut self, uniforms: &UniformData) {
        self.shader_manager.uniforms(uniforms).unwrap();
    }

    pub unsafe fn read_message(&mut self, message: &RenderMessage) {
        match message {
            RenderMessage::Pack { vertex_array, pack } => self.unpack(vertex_array, pack),
            RenderMessage::Clear { vertex_array } => self.clear(vertex_array),
            RenderMessage::ChooseShader { shader } => self.choose_shader(shader),
            RenderMessage::Uniforms { uniforms } => self.uniforms(uniforms),
        }
    }

    pub unsafe fn render(&mut self) {
        for i in 0..(self.vertex_array.get_vbos()) {
            //println!("VBO {} has size {}",i,self.vertex_array.get_size(i));

            if self.vertex_array.get_size(i) > 0 {
                self.vertex_array.draw(i);

                //gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_array.get_size(i) as i32);
            }
        }
    }
}