use crate::world::{Location, chunk::{Chunk, ChunkLocation}, voxel::{VoxelType, Voxel}};
use hashbrown::hash_map::HashMap;

const DEFAULT_VOXEL: VoxelType = VoxelType(0);
const CHUNK_SIZE: i32 = crate::world::chunk::CHUNK_SIZE as i32;

/// Struct that stores all voxels in the world.
pub struct Terrain {
    chunks: HashMap<(i32, i32, i32), Chunk>,
}

fn modulus(lhs: i32, rhs: i32) -> u32 {
    if lhs < 0 {
        (lhs % rhs + rhs) as u32
    } else {
        (lhs % rhs) as u32
    }
}

fn integer_division(lhs: i32, rhs: i32) -> i32 {
    if lhs < 0 {
        (lhs+1)/rhs-1
    } else {
        lhs/rhs
    }
}

/// Transforms a world location into a chunck index and a chunk location.
/// In other words, given a world location, this function finds which chunk that location is in 
/// and the location within that chunk.
fn location_to_chunk_index_and_location(loc: Location) -> ((i32, i32, i32), ChunkLocation) {
    ((integer_division(loc.x, CHUNK_SIZE), integer_division(loc.y, CHUNK_SIZE), integer_division(loc.z, CHUNK_SIZE)), 
        ChunkLocation::new(modulus(loc.x, CHUNK_SIZE), modulus(loc.y, CHUNK_SIZE), modulus(loc.z, CHUNK_SIZE)))
}

impl Terrain {
    /// Returns the type of voxel at the specified location.
    /// If a location outside of all chunks is specified, the default voxel type is returned.
    /// 
    /// # Arguments
    /// 
    /// * `loc` - Will find the type of the voxel at this location.
    pub fn voxel_type(&self, loc: Location) -> VoxelType {
        let (chunk_index, chunk_location) = location_to_chunk_index_and_location(loc);
        match self.chunks.get(&chunk_index) {
            Some(chunk) => chunk.voxel_type_unchecked(chunk_location),
            _ => DEFAULT_VOXEL,
        }
    }

    /// Returns the object representing the voxel at the specified location, 
    /// or none if the voxel has no object.
    /// If a location outside of all chunks is specified, None is returned.
    /// 
    /// # Arguments
    /// 
    /// * `loc` - Will find the object of the voxel at this location.
    pub fn voxel_object(&self, loc: Location) -> Option<&Box<dyn Voxel>> {
        let (chunk_index, chunk_location) = location_to_chunk_index_and_location(loc);
        match self.chunks.get(&chunk_index) {
            Some(chunk) => chunk.voxel_object(chunk_location),
            _ => None,
        }
    }

    /// Returns a tuple of the type and object of the voxel at the specified location.
    /// This is faster than using `voxel_type` and `voxel_object` as some computations can be reused.
    /// If a location outside of all chunks is specified, the default voxel type is returned alongside None.
    /// 
    /// # Arguments
    /// 
    /// * `loc` - Will find the type and object of the voxel at this location.
    pub fn voxel(&self, loc: Location) -> (VoxelType, Option<&Box<dyn Voxel>>) {
        let (chunk_index, chunk_location) = location_to_chunk_index_and_location(loc);
        match self.chunks.get(&chunk_index) {
            Some(chunk) => chunk.voxel_unchecked(chunk_location),
            _ => (DEFAULT_VOXEL, None)
        }
    }
}
