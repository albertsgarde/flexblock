use crate::game::world::{
    raytrace::Ray,
    voxel::{Voxel, VoxelType},
    *,
};
use glm::Vec3;
use hashbrown::hash_map::HashMap;
use serde::{Deserialize, Serialize};

/// The side length of a chunk.
/// The number of voxels per chunk is this value to the third power.
pub const CHUNK_SIZE: u32 = 16;
pub const CHUNK_SIZE_F: f32 = CHUNK_SIZE as f32;

/// The number of elements in a chunk's voxel vector.
const CHUNK_LENGTH: usize = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize;

/// Represents a voxel location in a chunk.
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
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

impl From<Vec3> for ChunkLocation {
    fn from(position: Vec3) -> ChunkLocation {
        Chunk::debug_assert_within_chunk(position);
        ChunkLocation::new(position.x as u32, position.y as u32, position.z as u32)
    }
}

/// Represents a cube-shaped block of voxels.
#[derive(Serialize, Deserialize, Clone)]
pub enum Chunk {
    SingleType(VoxelType),
    MultiType(Vec<VoxelType>, HashMap<ChunkLocation, Box<dyn Voxel>>),
}

impl Chunk {
    /// Create a new chunk with all voxels set to the default type.
    pub fn new() -> Chunk {
        Chunk::SingleType(voxel::DEFAULT_TYPE)
    }

    pub fn within_chunk(position: Vec3) -> bool {
        position.x >= 0.
            && position.y >= 0.
            && position.z >= 0.
            && position.x <= CHUNK_SIZE_F
            && position.y <= CHUNK_SIZE_F
            && position.z <= CHUNK_SIZE_F
    }

    pub fn debug_assert_within_chunk(position: Vec3) {
        debug_assert!(
            Chunk::within_chunk(position),
            "Position must be between 0 and chunk size on all coordinates. Position: ({}, {}, {})",
            position.x,
            position.y,
            position.z
        );
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

    fn ignore_voxel(&self, voxel: ChunkLocation) -> bool {
        raytrace::ignore_voxel_type(self.voxel_type_unchecked(voxel))
    }

    /// Traces a ray within the chunk returning the first non-ignored voxel hit or the
    /// first position outside the chunk if none is hit.
    /// Undefined behaviour occurs if the ray origin is outside chunk bounds.
    pub fn trace_ray(&self, ray: Ray) -> Option<Vec3> {
        Chunk::debug_assert_within_chunk(ray.origin);
        if let Chunk::SingleType(voxel_type) = self {
            if raytrace::ignore_voxel_type(*voxel_type) {
                //raytrace::round(ray.point_at(raytrace::voxel_exit(ray.origin, ray.direction, Vector3::zero(), CHUNK_SIZE_F) + 1e-4))
                None
            } else {
                Some(ray.voxel_exit(raytrace::round(ray.origin), 1.).unwrap())
            }
        } else {
            let mut voxel = ray.voxel_exit(raytrace::round(ray.origin), 1.).unwrap();
            while Chunk::within_chunk(voxel) {
                if !self.ignore_voxel(voxel.into()) {
                    return Some(voxel);
                }
                voxel = ray.voxel_exit(voxel, 1.).unwrap();
            }
            None
        }
    }
}
