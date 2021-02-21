use crate::{world::{*, voxel::{Voxel, VoxelType}}};
use hashbrown::hash_map::HashMap;

/// The side length of a chunk.
/// The number of voxels per chunk is this value to the third power.
pub const CHUNK_SIZE: u32 = 16;

/// The number of elements in a chunk's voxel vector.
const CHUNK_LENGTH: usize = (CHUNK_SIZE*CHUNK_SIZE*CHUNK_SIZE) as usize;

/// Represents a voxel location in a chunk.
#[derive(PartialEq, Eq, Hash)]
pub struct ChunkLocation {
    index: usize,
}


impl ChunkLocation {
    pub fn new(x: u32, y: u32, z: u32) -> ChunkLocation {
        ChunkLocation{index: (CHUNK_SIZE*CHUNK_SIZE*x+CHUNK_SIZE*y+z) as usize}
    }
}

/// Represents a cube-shaped block of voxels.
pub enum Chunk {
    SingleType(VoxelType),
    MultiType(Vec<VoxelType>, HashMap<ChunkLocation, Box<dyn Voxel>>),
}

impl Chunk {
    /// Create a new chunk with all voxels set to the default type.
    pub fn new() -> Chunk {
        Chunk::SingleType(voxel::DEFAULT_TYPE)
    }

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
            Chunk::MultiType(array, _) => array[loc.index]
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
            Chunk::MultiType(array, voxel_map) => (array[loc.index], voxel_map.get(&loc))
        }
    }

    /// Sets the voxel at the given location to the given type.
    pub fn set_voxel_type_unchecked(&mut self, loc: ChunkLocation, voxel_type: VoxelType) {
        if let Chunk::SingleType(single_type) = self {
            if voxel_type != *single_type {
                let mut voxel_vec = vec![*single_type; CHUNK_LENGTH];
                voxel_vec[loc.index] = voxel_type;
                *self = Chunk::MultiType(voxel_vec, HashMap::new());
            }
        } else if let Chunk::MultiType(voxel_vec, _) = self {
            voxel_vec[loc.index] = voxel_type;
        }
    }

    /// Checks if all voxels in the chunk are of a single type and have no voxel object.
    /// If this is the case, change the chunk to a single typed chunk.
    pub fn single_type(&mut self) {
        if let Chunk::MultiType(voxel_vec, voxel_map) = self {
            if voxel_map.is_empty() {
                let single_type = voxel_vec[0];
                if voxel_vec.iter().all( |voxel_type| *voxel_type == single_type ) {
                    *self = Chunk::SingleType(single_type);
                }
            }
        }
    }
}
