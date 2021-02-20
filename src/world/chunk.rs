use crate::{world::voxel::{Voxel, VoxelType}};
use hashbrown::hash_map::HashMap;

/// The side length of a chunk.
/// The number of voxels per chunk is this value to the third power.
pub const CHUNK_SIZE: u32 = 16;

/// Represents a voxel location in a chunk.
#[derive(PartialEq, Eq, Hash)]
pub struct ChunkLocation {
    x: u32,
    y: u32,
    z: u32,
}

impl ChunkLocation {
    pub fn new(x: u32, y: u32, z: u32) -> ChunkLocation {
        ChunkLocation{x, y, z}
    }
}

/// Represents a cube-shaped block of voxels.
pub enum Chunk {
    SingleType(VoxelType),
    MultiType(Vec<VoxelType>, HashMap<ChunkLocation, Box<dyn Voxel>>),
}

impl Chunk {
    /// Returns the type of the voxel at the specified location within the chunk.
    /// The location is not bound-checked and the function will either panic or 
    /// return an arbitrary voxel if the location is out of bounds.
    /// 
    /// # Arguments
    /// 
    /// * `loc` - Will find the type of the voxel at this location in the chunk.
    pub fn voxel_type_unchecked(&self, loc: ChunkLocation) -> VoxelType {
        match self {
            Chunk::SingleType(voxel_type) => *voxel_type,
            Chunk::MultiType(array, _) => array[(CHUNK_SIZE*CHUNK_SIZE*loc.x+CHUNK_SIZE*loc.y+loc.z) as usize]
        }
    }

    /// Returns the object of the voxel at the specified location within the chunk or None 
    /// if the voxel has no object.
    /// The location is not bound-checked and the function will return None if 
    /// the location is out of bounds.
    /// 
    /// # Arguments
    /// 
    /// * `loc` - Will find the object of the voxel at this location in the chunk.
    pub fn voxel_object(&self, loc: ChunkLocation) -> Option<&Box<dyn Voxel>> {
        match self {
            Chunk::SingleType(_) => None,
            Chunk::MultiType(_, voxel_map) => voxel_map.get(&loc),
        }
    }

    

    /// Returns a tuple of the type and object of the voxel at the specified location within the chunk.
    /// The location is not bound-checked and the function will either panic or 
    /// return an undefined voxel if the location is out of bounds.
    /// 
    /// # Arguments
    /// 
    /// * `loc` - Will find the type of the voxel at this location in the chunk.
    pub fn voxel_unchecked(&self, loc: ChunkLocation) -> (VoxelType, Option<&Box<dyn Voxel>>) {
        match self {
            Chunk::SingleType(voxel_type) => (*voxel_type, None),
            Chunk::MultiType(array, voxel_map) => (array[(CHUNK_SIZE*CHUNK_SIZE*loc.x+CHUNK_SIZE*loc.y+loc.z) as usize], voxel_map.get(&loc))
        }
    }
}
