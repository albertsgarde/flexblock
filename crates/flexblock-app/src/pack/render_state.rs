use game::{GraphicsStateModel, View};
use graphics::model::{ModelManager, PlacedModel};
use graphics::BufferTarget;
use graphics::ShaderIdentifier;
use graphics::VertexPack;
use graphics::{GraphicsCapabilities, RenderMessage, RenderMessages, UniformData};
use konst::{option::unwrap_or, primitive::parse_u32, result::unwrap_ctx};
use log::error;
use std::collections::VecDeque;
use std::sync::MutexGuard;
use utils::mesh_iterator::MeshIterator;
use world::{self, Chunk, Terrain};

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
                graphics::pack::cube_faces::back(z0, x0, y0, x1, y1, 1., 0., 0., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);

            //Front face
            let (mut vadd, mut eadd) =
                graphics::pack::cube_faces::front(z1, x0, y0, x1, y1, 0., 1., 0., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);

            //Left face
            let (mut vadd, mut eadd) =
                graphics::pack::cube_faces::left(x0, y0, z0, y1, z1, 0., 0., 1., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);

            //Right face
            let (mut vadd, mut eadd) =
                graphics::pack::cube_faces::right(x1, y0, z0, y1, z1, 1., 1., 0., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);

            //Bottom face
            let (mut vadd, mut eadd) =
                graphics::pack::cube_faces::bottom(y0, x0, z0, x1, z1, 1., 0., 1., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);

            //Top face
            let (mut vadd, mut eadd) =
                graphics::pack::cube_faces::top(y1, x0, z0, x1, z1, 0., 1., 1., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);
        }
    }
    VertexPack::new(vertices, Some(elements))
}

/// Returns the view * projection matrix of the supplied camera.
/// Doesn't get us all the way to mvp (multiply this by the model matrix, and you're there boyo).
pub fn get_vp_matrix(view: &View, screen_dimensions: (u32, u32)) -> glm::Mat4 {
    let (width, height) = screen_dimensions;

    let direction = view.view_direction();
    let position = view.location().as_coords();
    let center = position + direction;
    let up = view.up();

    let v: glm::Mat4 = glm::look_at(
        &glm::vec3(position[0], position[1], position[2]),
        &glm::vec3(center[0], center[1], center[2]),
        &glm::vec3(up[0], up[1], up[2]),
    );
    let p: glm::Mat4 = glm::perspective_fov(
        90. / 180. * std::f32::consts::PI,
        width as f32,
        height as f32,
        0.1,
        100.0,
    ); //TODO: CORRECT FOV, WIDTH, AND HEIGHT

    p * v
}

/// Returns a model matrix.
/// TODO: ROTATION
pub fn get_model_matrix(offset: &glm::Vec3) -> glm::Mat4 {
    glm::translation(offset)
}

/// Breadth first search
struct BreadthFirstSearch {
    visited_locations: Vec<glm::IVec3>,
    frontier: VecDeque<glm::IVec3>,
}

impl BreadthFirstSearch {
    pub fn new(start_location: glm::IVec3) -> BreadthFirstSearch {
        let mut frontier = VecDeque::new();
        frontier.push_back(start_location);
        BreadthFirstSearch {
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
    pub(super) packed_chunks: Vec<Option<glm::IVec3>>,
    pub(super) capabilities: Option<GraphicsCapabilities>,
}

/// Keeps track of all state necessary to correctly supply graphics calls to the window each frame.
impl RenderState {
    pub fn new() -> RenderState {
        //TODO: Buffer space should be sized according to the number of buffers in the target VertexArray. This should coordinate.
        let packed_chunks = vec![None; 0];

        RenderState {
            packed_chunks,
            capabilities: None,
        }
    }

    /// Registers that a chunk has been packed into the given buffer
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

    /// Deregisters that a chunk has been packed into the given buffer.
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

    /// Finds a free VBO available for chunk packing
    fn find_free_location(&self) -> Option<usize> {
        for (counter, c) in self.packed_chunks.iter().enumerate() {
            if c.is_none() {
                return Some(counter);
            }
        }
        None
    }

    /// Packs a given chunk
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
            let pack = create_chunk_pack(chunk);
            if !pack.vertices.is_empty() {
                messages.add_message(RenderMessage::Pack {
                    buffer: BufferTarget::WorldBuffer(buffer),
                    pack: create_chunk_pack(chunk),
                });
                self.register_packed_chunk(location, buffer);
            }

            Ok(())
        } else {
            Err(String::from("No buffer available for passed chunk!"))
        }
    }

    /// Unpacks a given chunk
    fn unpack_chunk(&mut self, buffer: usize, messages: &mut RenderMessages) -> Result<(), String> {
        if self.packed_chunks[buffer].is_none() {
            return Err(String::from("Unpacking an unpacked buffer!"));
        }
        messages.add_message(RenderMessage::ClearArray {
            buffer: BufferTarget::WorldBuffer(buffer),
        });

        self.unregister_packed_chunk(buffer);
        Ok(())
    }

    /// Tells whether a chunk is currently packed
    pub fn is_packed(&self, location: glm::IVec3) -> bool {
        self.packed_chunks.contains(&Some(location))
    }

    /// Packs a chunk if it hasn't been packed before, or unpacks and packs it if it has been packed before.
    pub fn repack_chunk(
        &mut self,
        location: glm::IVec3,
        messages: &mut RenderMessages,
        terrain: &Terrain,
    ) {
        for (counter, chunk) in self.packed_chunks.iter().enumerate() {
            if let Some(chunk) = chunk {
                if *chunk == location {
                    self.unregister_packed_chunk(counter);
                    messages.add_message(RenderMessage::ClearArray {
                        buffer: BufferTarget::WorldBuffer(counter),
                    });
                    break;
                }
            }
        }

        if let Some(chunk) = terrain.chunk(location) {
            if let Err(s) = self.pack_chunk(chunk, location, messages) {
                error!("{}", s);
            }
        }
    }

    /// Clears chunks that are further than four chunks from the given location.
    pub fn clear_distant_chunks(&mut self, location: glm::IVec3, messages: &mut RenderMessages) {
        let mut remove = Vec::new();
        for (counter, chunk) in self.packed_chunks.iter().enumerate() {
            if let Some(chunk) = chunk {
                let v = chunk - location;
                if v.x * v.x + v.y * v.y + v.z * v.z > 4 {
                    remove.push(counter);
                }
            }
        }
        for i in remove {
            match self.unpack_chunk(i, messages) {
                Ok(()) => {}
                Err(s) => {
                    error!("{}", s)
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
        for (counter, location) in self.packed_chunks.iter().enumerate() {
            if let Some(location) = location {
                let mvp = glm::translate(vp_matrix, &world::chunk_index_to_position(*location));
                let mut ud = UniformData::new();
                ud.texture(String::from("/atlas.png"), String::from("test_texture"));
                ud.mat4(mvp, String::from("MVP"));
                render_messages.add_message(RenderMessage::Uniforms {
                    uniforms: Box::new(ud),
                });

                render_messages.add_message(RenderMessage::Draw {
                    buffer: BufferTarget::WorldBuffer(counter),
                });
            }
        }
    }

    /// When the window supplies new capabilities, update local capabilities to those.
    pub fn update_capabilities(&mut self, capabilities: GraphicsCapabilities) {
        let vbo_count = match self.capabilities.take() {
            Some(cap2) => cap2.vbo_count,
            None => 0,
        };

        match capabilities.vbo_count {
            c if c > vbo_count => {
                self.packed_chunks.append(&mut vec![None; c - vbo_count]);
            }
            c if c < vbo_count => {
                panic!(
                    "There is currently no support for a shrink in the number of available VBOs"
                );
            }
            _ => {}
        }
        self.capabilities = Some(capabilities);
    }

    pub fn is_render_ready(&self) -> bool {
        self.capabilities.is_some()
    }

    pub fn render_capabilities(&self) -> &Option<GraphicsCapabilities> {
        &self.capabilities
    }

    /// Fills in world render messages for a tick
    pub fn create_render_messages(
        &mut self,
        data: &MutexGuard<GraphicsStateModel>,
        model_manager: &ModelManager,
    ) -> RenderMessages {
        // What should happen:
        // 1. Clear color and depth buffer
        // 2. Supply commmon uniforms
        // 3. For every new chunk we're interested in (Or dirty chunks)
        //   3a. Fill the chunk into a vertex array or update the existing vertex array
        // 4. For every chunk already filled into a vertex array
        //   4a. Supply specific uniforms
        //   4b. Draw

        let mut messages = RenderMessages::new();

        if !self.is_render_ready() {
            return messages;
        }

        let (width, height) = match &self.capabilities {
            Some(cap) => cap.screen_dimensions,
            None => unreachable!(),
        };

        messages.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Default,
        });
        messages.add_message(RenderMessage::ClearBuffers {
            color_buffer: true,
            depth_buffer: true,
        });

        // Read chunk render distance from environment variable. Default is 1 chunk and the value should be given in thousandths of chunks.
        const REPACK_CHUNK_RADIUS: f32 = unwrap_ctx!(parse_u32(unwrap_or!(
            option_env!("FLEXBLOCK_RENDER_RADIUS"),
            "1100"
        ))) as f32
            / 1000.;
        let repack_chunk_vec = glm::TVec3::new(
            REPACK_CHUNK_RADIUS.floor() as i32,
            REPACK_CHUNK_RADIUS.floor() as i32,
            REPACK_CHUNK_RADIUS.floor() as i32,
        );

        // Create a mesh iterator walking through all possible chunks to draw.
        let chunk_mesh = MeshIterator::create(repack_chunk_vec.map(|x| x * 2 + 1))
            .map(|chunk_vec| chunk_vec - repack_chunk_vec);

        // Walk through the mesh iterator and repack the chunks that are close enough.
        for chunk_vec in chunk_mesh {
            if chunk_vec.map(|x| x as f32).norm() <= REPACK_CHUNK_RADIUS {
                self.repack_chunk(
                    data.view.location().chunk + chunk_vec,
                    &mut messages,
                    &data.terrain,
                );
            }
        }
        self.clear_distant_chunks(data.view.location().chunk, &mut messages);

        let vp = get_vp_matrix(&data.view, (width, height));
        self.render_packed_chunks(&mut messages, &vp);

        let k = glm::vec3(2.0, 3.0, 1.0);
        let models = vec![("/test.obj", k)]; // TODO: USE INFO FROM THE GRAPHICS STATE HERE

        let models = models.into_iter().map(|(model_name, location)| {
            PlacedModel::new(model_name.into(), location, glm::vec3(1.0, 1.0, 1.0))
        });

        messages.merge_current(model_manager.draw_models(
            models.collect(),
            &get_vp_matrix(&data.view, (width, height)),
        ));

        messages
    }
}
