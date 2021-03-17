use crate::graphics::VertexPack;
use crate::game::{View, world::{Chunk,Terrain}};
use crate::graphics::{RenderMessages, RenderMessage, UniformData};
use std::collections::VecDeque;

/// This creates the vertex pack for a specific chunk. It just goes through all the voxels and adds their faces.
fn create_chunk_pack(chunk : &Chunk) -> VertexPack {
    let mut vertices = Vec::new();
    let mut elements = Vec::new();
    let mut index = 0;
    for (voxel, position) in chunk.iter() {
        let x0 = position.x;
        let x1 = x0+1.;
        let y0 = position.y;
        let y1 = y0+1.;
        let z0 = position.z;
        let z1 = z0+1.;

        if voxel.0 == 1 {
            
            println!("Creating cube at {:?}",position);

            // Back face 
            let (mut vadd,mut  eadd) = super::cube_faces::back(z0, x0, y0, x1, y1, 1., 0., 0., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);

            //Front face
            let (mut vadd,mut  eadd) = super::cube_faces::front(z1, x0, y0, x1, y1, 0., 1., 0., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);
            
            //Left face
            let (mut vadd,mut  eadd) = super::cube_faces::left(x0, y0, z0, y1, z1, 0., 0., 1., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);
            
            //Right face
            let (mut vadd,mut  eadd) = super::cube_faces::right(x1, y0, z0, y1, z1, 1., 1., 0., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);

            //Bottom face
            let (mut vadd,mut  eadd) = super::cube_faces::bottom(y0, x0, z0, x1, z1, 1., 0., 1., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);
            
            //Top face
            let (mut vadd,mut  eadd) = super::cube_faces::top(y1, x0, z0, x1, z1, 0., 1., 1., index);
            index += vadd.len() as u32;
            vertices.append(&mut vadd);
            elements.append(&mut eadd);
            
        }
    }
    println!("Created {} vertices and {} elements!", vertices.len(), elements.len());
    println!("{:?}",vertices);
    let vertex_pack = VertexPack::new(vertices, Some(elements));
    vertex_pack
}


/// Returns the view * projection matrix of the supplied camera.
/// Doesn't get us all the way to mvp (multiply this by the model matrix, and you're there boyo).
pub fn get_vp_matrix(view : &View) -> glm::Mat4 {

    let direction = view.view_direction();
    let position = view.location().position;
    let center = position + direction;

    let v: glm::Mat4 = glm::look_at(
        &glm::vec3(position[0], position[1], position[2]),
        &glm::vec3(center[0], center[1], center[2]),
        &glm::vec3(0., 1., 0.),
    );
    let p: glm::Mat4 = glm::perspective_fov(90. / 180. * 3.1415, 600., 400., 0.1, 100.0); //TODO: CORRECT FOV, WIDTH, AND HEIGHT

    p * v
}

/// Breadth first search
struct BFS {
    visited_locations : Vec<glm::IVec3>,
    frontier : VecDeque<glm::IVec3>
}

impl BFS {
    pub fn new(start_location : glm::IVec3) -> BFS {
        let mut frontier = VecDeque::new();
        frontier.push_back(start_location);
        BFS { visited_locations : Vec::new(), frontier}
    }

    fn try_add_frontier(&mut self, point : glm::IVec3, view_location : glm::IVec3, max_distance : i32) {
        
        let v = point-view_location;

        if v.x*v.x + v.y*v.y + v.z*v.z <= (max_distance*max_distance) && !self.visited_locations.contains(&point) && !self.frontier.contains(&point) {
            self.frontier.push_back(point);
        }
    }

    fn next(&mut self, view_location : glm::IVec3, max_distance : i32) -> Option<glm::IVec3> {
        if let Some(result) = self.frontier.pop_front().take() {

            self.visited_locations.push(result);

            self.try_add_frontier(result+glm::vec3(1,0,0), view_location, max_distance);
            self.try_add_frontier(result+glm::vec3(-1,0,0), view_location, max_distance);
            self.try_add_frontier(result+glm::vec3(0,1,0), view_location, max_distance);
            self.try_add_frontier(result+glm::vec3(0,-1,0), view_location, max_distance);
            self.try_add_frontier(result+glm::vec3(0,0,1), view_location, max_distance);
            self.try_add_frontier(result+glm::vec3(0,0,-1), view_location, max_distance);

            Some(result)
        } else {
            None
        }
    }
}

/// Contains state information that is needed by the packing thread.
pub struct RenderState {
    /// Contains a list of packed chunk locations. The index is which buffer they're packed into. None means no chunk is packed into that buffer.
    packed_chunks : Vec<Option<glm::IVec3>>,
    chunk_search : Option<BFS>
}

impl RenderState {
    pub fn new() -> RenderState {

        //TODO: Buffer space should be sized according to the number of buffers in the target VertexArray. This should coordinate.
        let packed_chunks = vec![None;20];

        RenderState {
            packed_chunks,
            chunk_search : None
        }
    }

    fn register_packed_chunk(&mut self, location : glm::IVec3, buffer : usize) {
        debug_assert!(buffer < self.packed_chunks.len(), "Trying to pack a chunk into a buffer outside buffer range!");
        debug_assert!(self.packed_chunks[buffer].is_none(), "Trying to pack a chunk into an already taken buffer slot!");

        self.packed_chunks[buffer] = Some(location);
    }

    fn unregister_packed_chunk(&mut self, buffer : usize) {
        debug_assert!(buffer < self.packed_chunks.len(), "Trying to pack a chunk into a buffer outside buffer range!");
        debug_assert!(self.packed_chunks[buffer].is_none(), "Trying to remove a chunk from a buffer that does not currently have a bound chunk!");

        self.packed_chunks[buffer] = None;
    }

    fn find_free_location(&self) -> Option<usize> {
        let mut counter=0;
        for c in &self.packed_chunks {
            if c.is_none() {
                return Some(counter);
            }
            counter += 1;
        }
        return None
    }

    fn pack_chunk(&mut self, chunk : &Chunk, location : glm::IVec3, messages : &mut RenderMessages) -> Result<(), String> {
        if self.packed_chunks.contains(&Some(location)) {
            return Err(String::from("The given chunk has already been packed"));
        }
        if let Some(buffer) = self.find_free_location() {
            println!("Packing to {}", buffer);
            messages.add_message(RenderMessage::Pack {
                buffer,
                pack : create_chunk_pack(chunk)
            });

            self.register_packed_chunk(location, buffer);
            Ok(())
        } else {
            Err(String::from("No buffer available for passed chunk!"))
        }
    }

    pub fn is_packed(&self, location : glm::IVec3) -> bool {
        self.packed_chunks.contains(&Some(location))
    }

    pub fn pack_next_chunk(&mut self, view_location : glm::IVec3, messages : &mut RenderMessages, terrain : &Terrain) { 
        //TODO: BFS SHOULD RESET OR SOMETHING SOMETIMES
        if self.chunk_search.is_none() { 
            self.chunk_search = Some(BFS::new(view_location));
        }
        if let Some(bfs) = &mut self.chunk_search {
            if let Some(loc) = bfs.next(view_location, 4) {
                if let Some(chunk) = terrain.chunk(loc) {
                    match self.pack_chunk(chunk, loc, messages) {
                        Ok(_) => { },
                        Err(s) => {println!("{}",s)}
                    }
                }
            }
        }
    }

    /// This renders all currently packed chunks
    /// TODO: This needs to work in local coordinates out from the player.
    pub fn render_packed_chunks(&self, render_messages : &mut RenderMessages, vp_matrix : &glm::Mat4) {
        let mut counter=0;
        for location in &self.packed_chunks {
            if let Some(location) = location {
                let mvp = glm::translate(vp_matrix, &glm::vec3(location.x as f32 *16., location.y as f32 *16., location.z as f32 *16.));

                render_messages.add_message(RenderMessage::Uniforms {
                    uniforms : UniformData::new( vec![(mvp, String::from("MVP"))], vec![])
                });

                render_messages.add_message(RenderMessage::Draw {
                    buffer : counter
                });
            }
            counter += 1;
        }
    }
}
