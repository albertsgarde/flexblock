use crate::game::world::{
    voxel::{Voxel, VoxelType},
    *,
};
use glm::Vec3;
use hashbrown::hash_map::HashMap;
use konst::{option::unwrap_or, primitive::parse_u32, result::unwrap_ctx};
use serde::{Deserialize, Serialize};

/// The side length of a chunk.
/// The number of voxels per chunk is this value to the third power.
pub const CHUNK_SIZE: u32 = unwrap_ctx!(parse_u32(unwrap_or!(
    option_env!("FLEXBLOCK_CHUNK_SIZE"),
    "16"
)));
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
        let index = (CHUNK_SIZE * CHUNK_SIZE * x + CHUNK_SIZE * y + z) as usize;
        debug_assert!(
            index < CHUNK_LENGTH,
            "Invalid chunk location ({}, {}, {})",
            x,
            y,
            z
        );
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

pub fn chunk_index_to_position(chunk_index: IVec3) -> Vec3 {
    Vec3::new(
        chunk_index.x as f32,
        chunk_index.y as f32,
        chunk_index.z as f32,
    ) * CHUNK_SIZE_F
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
            && position.x < CHUNK_SIZE_F
            && position.y < CHUNK_SIZE_F
            && position.z < CHUNK_SIZE_F
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
    pub unsafe fn voxel_type_unchecked(&self, loc: ChunkLocation) -> VoxelType {
        debug_assert!(loc.index < CHUNK_LENGTH);
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
        debug_assert!(loc.index < CHUNK_LENGTH);
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
    pub unsafe fn voxel_unchecked(&self, loc: ChunkLocation) -> (VoxelType, Option<&dyn Voxel>) {
        debug_assert!(loc.index < CHUNK_LENGTH);
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
        debug_assert!(loc.index < CHUNK_LENGTH);
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

    pub unsafe fn ignore_voxel(&self, voxel: ChunkLocation) -> bool {
        raytrace::ignore_voxel_type(self.voxel_type_unchecked(voxel))
    }

    /// If the ray hits anything in the chunk return the distance from origin and the position of the voxel hit, else return None.
    ///
    /// # Panics
    ///
    /// Panics if `direction` is the zero vector.
    /// Panics if `origin` is outside of chunk bounds.
    pub fn trace_ray(&self, origin: Vec3, direction: Vec3) -> Option<(f32, Vec3)> {
        Chunk::debug_assert_within_chunk(origin);
        if let Chunk::SingleType(voxel_type) = self {
            if raytrace::ignore_voxel_type(*voxel_type) {
                //raytrace::round(ray.point_at(raytrace::voxel_exit(ray.origin, ray.direction, Vector3::zero(), CHUNK_SIZE_F) + 1e-4))
                None
            } else {
                Some(raytrace::voxel_exit(origin, direction, raytrace::round(origin), 1.).unwrap())
            }
        } else {
            let (mut t, mut voxel) =
                raytrace::voxel_exit(origin, direction, raytrace::round(origin), 1.).unwrap();
            while Chunk::within_chunk(voxel) {
                if !unsafe { self.ignore_voxel(voxel.into()) } {
                    return Some((t, voxel));
                }
                let temp = raytrace::voxel_exit(origin, direction, voxel, 1.).unwrap();
                t = temp.0;
                voxel = temp.1;
            }
            None
        }
    }

    pub fn iter<'a>(&'a self) -> ChunkIterator<'a> {
        self.into_iter()
    }
}

pub enum ChunkIterator<'a> {
    SingleType {
        voxel_type: VoxelType,
        position: Vec3,
    },
    MultiType {
        iter: std::slice::Iter<'a, VoxelType>,
        position: Vec3,
    },
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = (VoxelType, Vec3);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ChunkIterator::SingleType {
                voxel_type,
                position,
            } => {
                position.z += 1.;
                if position.z >= CHUNK_SIZE_F {
                    position.z -= CHUNK_SIZE_F;
                    position.y += 1.;
                    if position.y >= CHUNK_SIZE_F {
                        position.y -= CHUNK_SIZE_F;
                        position.x += 1.;
                        if position.x >= CHUNK_SIZE_F {
                            return None;
                        }
                    }
                }
                Some((*voxel_type, *position))
            }
            ChunkIterator::MultiType { iter, position } => {
                position.z += 1.;
                if position.z >= CHUNK_SIZE_F {
                    position.z -= CHUNK_SIZE_F;
                    position.y += 1.;
                    if position.y >= CHUNK_SIZE_F {
                        position.y -= CHUNK_SIZE_F;
                        position.x += 1.;
                        if position.x >= CHUNK_SIZE_F {
                            return None;
                        }
                    }
                }
                iter.next().map(|vt| (*vt, *position))
            }
        }
    }
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = (VoxelType, Vec3);
    type IntoIter = ChunkIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Chunk::SingleType(voxel_type) => ChunkIterator::SingleType {
                voxel_type: *voxel_type,
                position: Vec3::new(0., 0., -1.),
            },
            Chunk::MultiType(array, _) => ChunkIterator::MultiType {
                iter: array.iter(),
                position: Vec3::new(0., 0., -1.),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator() {
        let mut chunk = Chunk::new();
        chunk.set_voxel_type_unchecked(ChunkLocation::new(0, 0, 1), VoxelType(1));
        chunk.set_voxel_type_unchecked(ChunkLocation::new(3, 2, 1), VoxelType(5));
        let mut iter = chunk.iter();
        assert_eq!(iter.next().unwrap(), (VoxelType(0), Vec3::new(0., 0., 0.)));
        assert_eq!(iter.next().unwrap(), (VoxelType(1), Vec3::new(0., 0., 1.)));
        for _ in 2..(3 * CHUNK_SIZE * CHUNK_SIZE + 2 * CHUNK_SIZE + 1) {
            assert_eq!(iter.next().unwrap().0, VoxelType(0));
        }
        assert_eq!(iter.next().unwrap(), (VoxelType(5), Vec3::new(3., 2., 1.)));
        for _ in (3 * CHUNK_SIZE * CHUNK_SIZE + 2 * CHUNK_SIZE + 2)..(CHUNK_LENGTH as u32) {
            assert_eq!(iter.next().unwrap().0, VoxelType(0));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn trace() {
        let mut chunk = Chunk::new();
        chunk.set_voxel_type_unchecked(ChunkLocation::new(0, 0, 2), VoxelType(1));

        let loc = Vec3::new(0.7398548, 1.6037731, 15.999945);
        let dir = Vec3::new(0.8108393, 0.19129802, -0.5531295);
        chunk.trace_ray(loc, dir);
    }
}
