use crate::game::{
    world::{Chunk, Terrain},
    View,
};
use crate::graphics::VertexPack;
use crate::graphics::{GraphicsCapabilities, RenderMessage, RenderMessages, UniformData};
use std::collections::{VecDeque};
use crate::graphics::wrapper::ShaderMetadata;
use std::sync::MutexGuard;
use crate::game::GraphicsStateModel;

/// This creates the vertex pack for a specific chunk. It just goes through all the voxels and adds their faces.
fn create_chunk_pack(chunk: &Chunk) -> VertexPack {
    let mut vertices = Vec::new();
    let mut elements = Vec::new();
    let mut index = 0;
    for (voxel, position) in chunk.iter() {
        let x0 = position.x;
        let x1 = x0 + 1.;
        let y0 = position.y;
        let y1 = y0 + 1.;
        let z0 = position.z;
        let z1 = z0 + 1.;

        if voxel.0 == 1 {
            // Back face
            let (mut vadd, mut eadd) =
                super::cube_faces::back(z0, x0, y0, x1, y1, 1., 0., 0., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);

            //Front face
            let (mut vadd, mut eadd) =
                super::cube_faces::front(z1, x0, y0, x1, y1, 0., 1., 0., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);

            //Left face
            let (mut vadd, mut eadd) =
                super::cube_faces::left(x0, y0, z0, y1, z1, 0., 0., 1., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);

            //Right face
            let (mut vadd, mut eadd) =
                super::cube_faces::right(x1, y0, z0, y1, z1, 1., 1., 0., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);

            //Bottom face
            let (mut vadd, mut eadd) =
                super::cube_faces::bottom(y0, x0, z0, x1, z1, 1., 0., 1., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);

            //Top face
            let (mut vadd, mut eadd) =
                super::cube_faces::top(y1, x0, z0, x1, z1, 0., 1., 1., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);
        }
    }
    let vertex_pack = VertexPack::new(vertices, Some(elements));
    vertex_pack
}

/// Returns the view * projection matrix of the supplied camera.
/// Doesn't get us all the way to mvp (multiply this by the model matrix, and you're there boyo).
pub fn get_vp_matrix(view: &View) -> glm::Mat4 {
    let direction = view.view_direction();
    let chunk = &view.location().chunk;
    let position: glm::Vec3 = view.location().position
        + glm::vec3(
            (chunk.x * 16) as f32,
            (chunk.y * 16) as f32,
            (chunk.z * 16) as f32,
        );
    let center = position + direction;
    let up = view.up();

    let v: glm::Mat4 = glm::look_at(
        &glm::vec3(position[0], position[1], position[2]),
        &glm::vec3(center[0], center[1], center[2]),
        &glm::vec3(up[0], up[1], up[2]),
    );
    let p: glm::Mat4 = glm::perspective_fov(90. / 180. * 3.1415, 600., 400., 0.1, 100.0); //TODO: CORRECT FOV, WIDTH, AND HEIGHT

    p * v
}

/// Breadth first search
struct BFS {
    visited_locations: Vec<glm::IVec3>,
    frontier: VecDeque<glm::IVec3>,
}

impl BFS {
    pub fn new(start_location: glm::IVec3) -> BFS {
        let mut frontier = VecDeque::new();
        frontier.push_back(start_location);
        BFS {
            visited_locations: Vec::new(),
            frontier,
        }
    }

    fn try_add_frontier(
        &mut self,
        point: glm::IVec3,
        view_location: glm::IVec3,
        max_distance: i32,
    ) {
        let v = point - view_location;

        if v.x * v.x + v.y * v.y + v.z * v.z <= (max_distance * max_distance)
            && !self.visited_locations.contains(&point)
            && !self.frontier.contains(&point)
        {
            self.frontier.push_back(point);
        }
    }

    fn next(&mut self, view_location: glm::IVec3, max_distance: i32) -> Option<glm::IVec3> {
        if let Some(result) = self.frontier.pop_front().take() {
            self.visited_locations.push(result);

            self.try_add_frontier(result + glm::vec3(1, 0, 0), view_location, max_distance);
            self.try_add_frontier(result + glm::vec3(-1, 0, 0), view_location, max_distance);
            self.try_add_frontier(result + glm::vec3(0, 1, 0), view_location, max_distance);
            self.try_add_frontier(result + glm::vec3(0, -1, 0), view_location, max_distance);
            self.try_add_frontier(result + glm::vec3(0, 0, 1), view_location, max_distance);
            self.try_add_frontier(result + glm::vec3(0, 0, -1), view_location, max_distance);

            Some(result)
        } else {
            None
        }
    }
}

/// Contains state information that is needed by the packing thread.
pub struct RenderState {
    /// Contains a list of packed chunk locations. The index is which buffer they're packed into. None means no chunk is packed into that buffer.
    packed_chunks: Vec<Option<glm::IVec3>>,
    chunk_search: Option<BFS>,
    capabilities: Option<GraphicsCapabilities>,
}

impl RenderState {
    pub fn new() -> RenderState {
        //TODO: Buffer space should be sized according to the number of buffers in the target VertexArray. This should coordinate.
        let packed_chunks = vec![None; 0];

        RenderState {
            packed_chunks,
            chunk_search: None,
            capabilities: None
        }
    }

    fn register_packed_chunk(&mut self, location: glm::IVec3, buffer: usize) {
        /*debug_assert!(
            buffer < self.packed_chunks.len(),
            "Trying to pack a chunk into a buffer outside buffer range!"
        );
        debug_assert!(
            self.packed_chunks[buffer].is_none(),
            "Trying to pack a chunk into an already taken buffer slot!"
        );*/

        self.packed_chunks[buffer] = Some(location);
    }

    fn unregister_packed_chunk(&mut self, buffer: usize) {
        /*debug_assert!(
            buffer < self.packed_chunks.len(),
            "Trying to pack a chunk into a buffer outside buffer range!"
        );
        debug_assert!(
            self.packed_chunks[buffer].is_some(),
            "Trying to remove a chunk from a buffer that does not currently have a bound chunk!"
        );*/

        self.packed_chunks[buffer] = None;
    }

    fn find_free_location(&self) -> Option<usize> {
        let mut counter = 0;
        for c in &self.packed_chunks {
            if c.is_none() {
                return Some(counter);
            }
            counter += 1;
        }
        return None;
    }

    fn pack_chunk(
        &mut self,
        chunk: &Chunk,
        location: glm::IVec3,
        messages: &mut RenderMessages,
    ) -> Result<(), String> {
        if self.packed_chunks.contains(&Some(location)) {
            return Err(String::from("The given chunk has already been packed"));
        }
        if let Some(buffer) = self.find_free_location() {
            messages.add_message(RenderMessage::Pack {
                buffer,
                pack: create_chunk_pack(chunk),
            });

            self.register_packed_chunk(location, buffer);
            Ok(())
        } else {
            Err(String::from("No buffer available for passed chunk!"))
        }
    }

    fn unpack_chunk(&mut self, buffer: usize, messages: &mut RenderMessages) -> Result<(), String> {
        if self.packed_chunks[buffer].is_none() {
            return Err(String::from("Unpacking an unpacked buffer!"));
        }
        messages.add_message(RenderMessage::ClearArray { buffer });

        self.unregister_packed_chunk(buffer);
        Ok(())
    }

    pub fn is_packed(&self, location: glm::IVec3) -> bool {
        self.packed_chunks.contains(&Some(location))
    }

    ///DEPRECATED
    pub fn pack_next_chunk(
        &mut self,
        view_location: glm::IVec3,
        messages: &mut RenderMessages,
        terrain: &Terrain,
    ) {
        //TODO: BFS SHOULD RESET OR SOMETHING SOMETIMES
        if self.chunk_search.is_none() {
            self.chunk_search = Some(BFS::new(view_location));
        }
        if let Some(bfs) = &mut self.chunk_search {
            if let Some(loc) = bfs.next(view_location, 4) {
                if let Some(chunk) = terrain.chunk(loc) {
                    match self.pack_chunk(chunk, loc, messages) {
                        Ok(_) => {}
                        Err(s) => {
                            println!("{}", s)
                        }
                    }
                }
            }
        }
    }

    pub fn repack_chunk(
        &mut self,
        location: glm::IVec3,
        messages: &mut RenderMessages,
        terrain: &Terrain,
    ) {
        let mut counter = 0;
        for chunk in &self.packed_chunks {
            if let Some(chunk) = chunk {
                if *chunk == location {
                    self.unregister_packed_chunk(counter);
                    messages.add_message(RenderMessage::ClearArray { buffer: counter });
                    break;
                }
            }
            counter += 1;
        }

        if let Some(chunk) = terrain.chunk(location) {
            match self.pack_chunk(chunk, location, messages) {
                Err(s) => {
                    println!("{}", s)
                }
                Ok(()) => {}
            }
        }
    }

    pub fn clear_distant_chunks(&mut self, location: glm::IVec3, messages: &mut RenderMessages) {
        let mut counter = 0;
        let mut remove = Vec::new();
        for chunk in &self.packed_chunks {
            if let Some(chunk) = chunk {
                let v = chunk - location;
                if v.x * v.x + v.y * v.y + v.z * v.z > 4 {
                    remove.push(counter);
                }
            }
            counter += 1;
        }
        for i in remove {
            match self.unpack_chunk(i, messages) {
                Ok(()) => {}
                Err(s) => {
                    println!("{}", s)
                }
            }
        }
    }

    /// This renders all currently packed chunks
    /// TODO: This needs to work in local coordinates out from the player.
    pub fn render_packed_chunks(
        &self,
        render_messages: &mut RenderMessages,
        vp_matrix: &glm::Mat4,
    ) {
        let mut counter = 0;
        for location in &self.packed_chunks {
            if let Some(location) = location {
                let mvp = glm::translate(
                    vp_matrix,
                    &glm::vec3(
                        location.x as f32 * 16.,
                        location.y as f32 * 16.,
                        location.z as f32 * 16.,
                    ),
                );

                render_messages.add_message(RenderMessage::Uniforms {
                    uniforms: UniformData::new(vec![(mvp, String::from("MVP"))], vec![], vec![(String::from("atlas"), String::from("test_texture"))]),
                });

                render_messages.add_message(RenderMessage::Draw { buffer: counter });
            }
            counter += 1;
        }
    }

    pub fn update_capabilities(&mut self, capabilities: GraphicsCapabilities) {
        let vbo_count = match self.capabilities.take() {
            Some(cap2) => cap2.vbo_count,
            None => 0,
        };

        if capabilities.vbo_count > vbo_count {
            self.packed_chunks
                .append(&mut vec![None; capabilities.vbo_count - vbo_count]);
        } else if capabilities.vbo_count < vbo_count {
            panic!(
                "There is currently no support for a shrink in the number of available VBOs"
            );
        }
        self.capabilities = Some(capabilities);
    }

    pub fn create_render_messages(&mut self, data : &MutexGuard<GraphicsStateModel>) -> RenderMessages{

        // What should happen:
        // 1. Clear color and depth buffer
        // 2. Supply commmon uniforms
        // 3. For every new chunk we're interested in (Or dirty chunks)
        //   3a. Fill the chunk into a vertex array or update the existing vertex array
        // 4. For every chunk already filled into a vertex array
        //   4a. Supply specific uniforms
        //   4b. Draw

        let mut messages = RenderMessages::new();

        if self.capabilities.is_some() {

            messages.add_message(RenderMessage::ChooseShader {
                shader: String::from("s1"),
            });
            messages.add_message(RenderMessage::ClearBuffers {
                color_buffer: true,
                depth_buffer: true,
            });

            // state.pack_next_chunk(data.view.location().chunk, &mut messages, &data.terrain);
            self.repack_chunk(data.view.location().chunk, &mut messages, &data.terrain);
            self.repack_chunk(
                data.view.location().chunk + glm::vec3(1, 0, 0),
                &mut messages,
                &data.terrain,
            );
            self.repack_chunk(
                data.view.location().chunk + glm::vec3(-1, 0, 0),
                &mut messages,
                &data.terrain,
            );
            self.repack_chunk(
                data.view.location().chunk + glm::vec3(0, 1, 0),
                &mut messages,
                &data.terrain,
            );
            self.repack_chunk(
                data.view.location().chunk + glm::vec3(0, -1, 0),
                &mut messages,
                &data.terrain,
            );
            self.repack_chunk(
                data.view.location().chunk + glm::vec3(0, 0, 1),
                &mut messages,
                &data.terrain,
            );
            self.repack_chunk(
                data.view.location().chunk + glm::vec3(0, 0, -1),
                &mut messages,
                &data.terrain,
            );

            self.clear_distant_chunks(data.view.location().chunk, &mut messages);

            let vp = get_vp_matrix(&data.view);
            self.render_packed_chunks(&mut messages, &vp);
        }
        messages
    }

    /// Validates that this contains only allowed render messages in an allowed order
    pub fn validate(&self, messages : &RenderMessages) -> Result<(), &str> {

        let verbose = true;


        if verbose {
            println!("Validating render message pack with {} messages!", messages.size());
        }
        if let Some(capabilities) = &self.capabilities {

            let mut chosen_shader = None;
            let mut bound_uniforms = Vec::new();
            let shader_metadata= &capabilities.shader_metadata;

            for message in messages.iter() {
                match message {
                    RenderMessage::ChooseShader {shader} => {
                        if !shader_metadata.contains_key(shader) {
                            return Err("A choose shader render message is sent, but the shader does not exist!");
                        }
                        chosen_shader = Some(&shader_metadata[shader]);
                        bound_uniforms = Vec::new();

                        if verbose {
                            println!("Choosing shader {}", shader);
                        }
                    },
                    RenderMessage::ClearArray {buffer} => {
                        if *buffer >= capabilities.vbo_count {
                            return Err("Trying to pack into a VBO index that is not available! Out of bounds!");
                        }
                        
                        //TODO: FIGURE OUT A WAY TO VALIDATE WHETHER VBOS ARE FILLED OR NOT!

                        if verbose {
                            println!("Clearing VBO {}", buffer);
                        }
                    },
                    RenderMessage::ClearBuffers {color_buffer, depth_buffer} => {
                        if !color_buffer && !depth_buffer {
                            return Err("A clear buffers render message is sent, but no buffers are cleared!");
                        }
                        if verbose {
                            println!("Clearing color? {} and depth? {}", color_buffer, depth_buffer);
                        }
                    },
                    RenderMessage::Draw{buffer} => {
                        if chosen_shader.is_none() {
                            return Err("Trying to draw without picking a shader first!");
                        }
                        //TODO: FIGURE OUT A WAY TO VALIDATE WHETHER VBOS ARE FILLED OR NOT!

                        if verbose {
                            println!("Drawing buffer {}", buffer);
                        }
                    },
                    RenderMessage::Pack{buffer, pack} => {
                        if *buffer >= capabilities.vbo_count {
                            return Err("Trying to pack into a VBO index that is not available! Out of bounds!");
                        }

                        if pack.elements.len() %3 != 0 || (pack.elements.len() == 0 && pack.vertices.len() % 3 != 0){
                            return Err("Received a vertex pack that does not contain a whole number of triangles!");
                        }
                        if pack.vertices.len() == 0 {
                            return Err("Trying to fil VBO with empty vertex pack! A VBO is cleared by sending a RenderMessage::ClearArray message!");
                        }
                        //TODO: FIGURE OUT A WAY TO VALIDATE WHETHER VBOS ARE FILLED OR NOT!

                        if verbose {
                            println!("filling VBO {}", buffer);
                        }
                    },
                    RenderMessage::Uniforms {uniforms} => {
                        self.validate_uniforms(uniforms, &chosen_shader, &mut bound_uniforms, capabilities)?;
                        if verbose {
                            println!("Filling in uniforms");
                        }
                    }
                }
            }
    
            Ok(())
        }
        else {
            if messages.size() > 0 {
                Err("Trying to send render messages when no graphics capabilities object is available!")
            } else {
                Ok(())
            }
        }

    }

    fn validate_uniforms(&self, uniforms : &UniformData, chosen_shader : &Option<&ShaderMetadata>, bound_uniforms : &mut Vec<String>, capabilities : &GraphicsCapabilities) -> Result<(), &str> {

        //TODO: Enforce uniform type matching to shader known type

        if let Some(s) = &chosen_shader {

            // Test if every passed texture exists in the graphics capabilities.
            for entry in &uniforms.textures {
                if !capabilities.texture_names.contains_key(&entry.0) {
                    return Err("Trying to pass a texture as shader uniform that does not exist in the graphics capabilities object!");
                }
            }

            // Test if the shader wants every uniform passed to it
            // (TODO: This may be overzealous, maybe you should be let off with a warning.)
            for uniform in uniforms.get_uniform_locations() {
                let req_uniforms = &s.required_uniforms;
                let mut contained = false;
                for req_uni in req_uniforms {
                    if req_uni.0 == *uniform {
                        contained = true
                    }
                }

                if !contained {
                    return Err("Trying to pass a uniform to a shader that does not want it! (This is non-critical, should maybe be a warning instead)");
                }
                
                if !bound_uniforms.contains(&uniform) {
                    bound_uniforms.push(String::from(uniform));
                }
            }

        } else {
            return Err("Trying to pass uniforms without picking a shader first!");
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::RenderState;
    use crate::graphics::GraphicsCapabilities;
    use crate::graphics::wrapper::{ShaderMetadata, ProgramType};
    use crate::graphics::{RenderMessage, RenderMessages, UniformData, VertexPack};
    use std::collections::HashMap;

    fn create_shader_metadata() -> HashMap<String, ShaderMetadata> {
        let mut res = HashMap::new();
        let t1 = ShaderMetadata {
            name : String::from("s1"),
            required_uniforms : vec![(String::from("test_texture"),String::from(""))],
            shader_type : ProgramType::Graphics
        };
        res.insert(String::from(&t1.name), t1);

        res
    }

    /// Creates a basic render state that has a capabilities object with:
    ///  - one shader (s1) that needs one uniform (test_texture)
    ///  - one texture (atlas)
    ///  - 100 vbos
    fn create_render_state() -> RenderState {
        let mut rs = RenderState::new();

        let shader_metadata = create_shader_metadata();
        let mut texture_names = HashMap::new();
        texture_names.insert(String::from("atlas"), 0);

        rs.update_capabilities(GraphicsCapabilities {vbo_count : 100, texture_names, shader_metadata});

        rs
    }

    fn create_triangle_pack() -> VertexPack {

        let mut vertices = Vec::new();
        let mut elements = Vec::new();
        let x0 = 0.;
        let x1 = x0 + 1.;
        let y0 = 0.;
        let y1 = y0 + 1.;
        let z0 = 0.;

        // Back face
        let (mut vadd, mut eadd) =
            super::super::cube_faces::back(z0, x0, y0, x1, y1, 1., 0., 0., 0);
        vertices.append(&mut vadd);
        elements.append(&mut eadd);
        let vertex_pack = VertexPack::new(vertices, Some(elements));
        vertex_pack
    }

    #[test]
    fn basic_validation() {
        let rs = create_render_state();

        let mut render_messages = RenderMessages::new();


        render_messages.add_message(RenderMessage::ChooseShader { shader : String::from("s1") } );
        render_messages.add_message(RenderMessage::Uniforms { uniforms : UniformData::new(vec![], vec![], vec![(String::from("atlas"), String::from("test_texture"))]) } );
        render_messages.add_message(RenderMessage::Pack { buffer : 0, pack : create_triangle_pack()} );

        assert!(rs.validate(&render_messages).unwrap() == ());
    }
}