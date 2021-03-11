use crate::game::world::{
    voxel::{Voxel, VoxelType},
    *,
};
use cgmath::Vector3;
use hashbrown::hash_map::HashMap;
use serde::{Deserialize, Serialize};

/// The side length of a chunk.
/// The number of voxels per chunk is this value to the third power.
pub const CHUNK_SIZE: u32 = 16;

/// The number of elements in a chunk's voxel vector.
const CHUNK_LENGTH: usize = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize;

/// Represents a voxel location in a chunk.
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChunkLocation {
    index: usize,
}

impl ChunkLocation {
    /// Create a new ChunkLocation.
    /// The coordinates are not bound checked and undefined behaviour
    /// will occur if any are >= CHUNK_SIZE.
    pub fn new(x: u32, y: u32, z: u32) -> ChunkLocation {
        ChunkLocation {
            index: (CHUNK_SIZE * CHUNK_SIZE * x + CHUNK_SIZE * y + z) as usize,
        }
    }
}

impl From<Vector3<f32>> for ChunkLocation {
    fn from(position: Vector3<f32>) -> ChunkLocation {
        debug_assert!(
            position.x >= 0.
                && position.y >= 0.
                && position.z >= 0.
                && position.x <= CHUNK_SIZE as f32
                && position.y <= CHUNK_SIZE as f32
                && position.z <= CHUNK_SIZE as f32,
            "Position must be between 0 and chunk size on all coordinates. Position: ({}, {}, {})",
            position.x,
            position.y,
            position.z
        );
        ChunkLocation::new(position.x as u32, position.y as u32, position.z as u32)
    }
}

/// Represents a cube-shaped block of voxels.
#[derive(Serialize, Deserialize)]
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
            Chunk::MultiType(array, _) => array[loc.index],
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
    pub fn voxel_object(&self, loc: ChunkLocation) -> Option<&dyn Voxel> {
        match self {
            Chunk::SingleType(_) => None,
            // `voxel` is a reference to a box. To get a reference to the contents, we
            // dereference the reference, dereference the box and then reference the result.
            Chunk::MultiType(_, voxel_map) => voxel_map.get(&loc).map(|voxel| &**voxel),
        }
    }

    /// Returns a tuple of the type and object of the voxel at the specified location within the chunk.
    /// The location is not bound-checked and the function will either panic or
    /// return an undefined voxel if the location is out of bounds.
    ///
    /// # Arguments
    ///
    /// * `loc` - Will find the type of the voxel at this location in the chunk.
    pub fn voxel_unchecked(&self, loc: ChunkLocation) -> (VoxelType, Option<&dyn Voxel>) {
        match self {
            Chunk::SingleType(voxel_type) => (*voxel_type, None),
            Chunk::MultiType(array, voxel_map) => {
                (array[loc.index], voxel_map.get(&loc).map(|voxel| &**voxel))
            }
        }
    }

    /// Sets the voxel at the given location to the given type.
    /// The location is not bound-checked and the function will either panic or
    /// set an undefined voxel if the location is out of bounds.
    pub fn set_voxel_type_unchecked(&mut self, loc: ChunkLocation, voxel_type: VoxelType) {
        if let Chunk::SingleType(single_type) = self {
            if voxel_type != *single_type {
                let mut voxel_vec = vec![*single_type; CHUNK_LENGTH];
                voxel_vec[loc.index] = voxel_type;
                *self = Chunk::MultiType(voxel_vec, HashMap::new());
            }
        } else if let Chunk::MultiType(voxel_vec, voxel_map) = self {
            voxel_map.remove(&loc);
            voxel_vec[loc.index] = voxel_type;
        }
    }

    /// Checks if all voxels in the chunk are of a single type and have no voxel object.
    /// If this is the case, change the chunk to a single typed chunk.
    pub fn single_type(&mut self) {
        if let Chunk::MultiType(voxel_vec, voxel_map) = self {
            if voxel_map.is_empty() {
                let single_type = voxel_vec[0];
                if voxel_vec
                    .iter()
                    .all(|voxel_type| *voxel_type == single_type)
                {
                    *self = Chunk::SingleType(single_type);
                }
            }
        }
    }
}
