use std::mem::swap;

use crate::{
    chunk::{self, Chunk},
    raytrace,
    voxel::{self, Voxel, VoxelType},
    Location,
};
use glm::{IVec3, Vec3};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utils::mesh_iterator::MeshIterator;

pub struct VoxelTypeBoxIterator<'a> {
    lower_bound: Location,
    mesh_iterator: MeshIterator<f32>,
    terrain: &'a Terrain,
}

impl Iterator for VoxelTypeBoxIterator<'_> {
    type Item = VoxelType;

    fn next(&mut self) -> Option<VoxelType> {
        let cur_voxel = self.lower_bound + self.mesh_iterator.next()?;
        Some(self.terrain.voxel_type(cur_voxel))
    }
}

/// Struct that stores all voxels in the world.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Terrain {
    chunks: HashMap<IVec3, Chunk>,
}

impl Terrain {
    /// Creates a new Terrain with all voxels set to default type.
    pub fn new() -> Terrain {
        Terrain {
            chunks: HashMap::new(),
        }
    }

    /// Returns the chunk with the specified index or None if no such chunk exists.
    pub fn chunk(&self, chunk: IVec3) -> Option<&Chunk> {
        self.chunks.get(&chunk)
    }

    /// Returns the type of voxel at the specified location.
    /// If a location outside of all chunks is specified, the default voxel type is returned.
    ///
    /// # Arguments
    ///
    /// * `loc` - Will find the type of the voxel at this location.
    pub fn voxel_type(&self, loc: Location) -> VoxelType {
        match self.chunks.get(&loc.chunk) {
            Some(chunk) => unsafe { chunk.voxel_type_unchecked(loc.position.into()) },
            _ => voxel::DEFAULT_TYPE,
        }
    }

    /// Returns the object representing the voxel at the specified location,
    /// or none if the voxel has no object.
    /// If a location outside of all chunks is specified, None is returned.
    ///
    /// # Arguments
    ///
    /// * `loc` - Will find the object of the voxel at this location.
    pub fn voxel_object(&self, loc: Location) -> Option<&dyn Voxel> {
        match self.chunks.get(&loc.chunk) {
            Some(chunk) => chunk.voxel_object(loc.position.into()),
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
    pub fn voxel(&self, loc: Location) -> (VoxelType, Option<&dyn Voxel>) {
        match self.chunks.get(&loc.chunk) {
            Some(chunk) => unsafe { chunk.voxel_unchecked(loc.position.into()) },
            _ => (voxel::DEFAULT_TYPE, None),
        }
    }

    /// Sets the voxel type for the voxel at the given location.
    /// If the location is outside of all chunks, a new chunk is created.
    pub fn set_voxel_type(&mut self, loc: Location, voxel_type: VoxelType) {
        if let Some(chunk) = self.chunks.get_mut(&loc.chunk) {
            chunk.set_voxel_type_unchecked(loc.position.into(), voxel_type);
        } else {
            let mut chunk = Chunk::new();
            chunk.set_voxel_type_unchecked(loc.position.into(), voxel_type);
            self.chunks.insert(loc.chunk, chunk);
        }
    }

    /// Returns the current number of chunks in the Terrain.
    pub fn num_chunks(&self) -> usize {
        self.chunks.len()
    }

    /// Removes chunks that contain only the default voxel type and no voxel objects.
    pub fn clean(&mut self) {
        for chunk in self.chunks.values_mut() {
            chunk.single_type();
        }
        self.chunks.retain(|_, chunk| !matches!(chunk, Chunk::SingleType(voxel_type) if *voxel_type == voxel::DEFAULT_TYPE));
    }

    /// Traces the given ray and returns both the voxel it hits and the location where it hits the voxel.
    pub fn trace_ray_with_position(
        &self,
        origin: Location,
        direction: Vec3,
    ) -> Option<(f32, Location)> {
        let mut t = 0.;
        let mut loc = origin;
        let mut chunks = 0;
        while chunks < 100 {
            loc.coerce();
            if let Some(chunk) = self.chunks.get(&loc.chunk) {
                if t > 0. && !unsafe { chunk.ignore_voxel(loc.position.into()) } {
                    return Some((t, loc.round()));
                }
                let new_origin = loc.position;
                if let Some((chunk_t, position)) = chunk.trace_ray(new_origin, direction) {
                    loc.position = position;
                    t += chunk_t;
                } else {
                    let delta_t = raytrace::voxel_exit_t(
                        loc.position,
                        direction,
                        Vec3::new(0., 0., 0.),
                        chunk::CHUNK_SIZE_F,
                    ) + 1e-4;
                    loc.position += direction * delta_t;
                    t += delta_t;
                }
            } else {
                let delta_t = raytrace::voxel_exit_t(
                    loc.position,
                    direction,
                    Vec3::new(0., 0., 0.),
                    chunk::CHUNK_SIZE_F,
                ) + 1e-4;
                loc.position += direction * delta_t;
                t += delta_t
            }
            if Chunk::within_chunk(loc.position) {
                return Some((t, loc.round()));
            }
            chunks += 1;
        }
        None
    }

    pub fn trace_ray(&self, origin: Location, direction: Vec3) -> Option<Location> {
        self.trace_ray_with_position(origin, direction)
            .map(|(_, voxel)| voxel)
    }

    pub fn voxel_type_iterator(
        &self,
        mut vec1: Location,
        mut vec2: Location,
    ) -> VoxelTypeBoxIterator {
        for i in 0..3 {
            if vec1.coord(i) > vec2.coord(i) {
                swap(&mut vec1.chunk[i], &mut vec2.chunk[i]);
                swap(&mut vec1.position[i], &mut vec2.position[i]);
            }
        }
        VoxelTypeBoxIterator {
            lower_bound: vec1,
            mesh_iterator: MeshIterator::create(vec2 - vec1),
            terrain: &self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, BufWriter};
    #[test]
    fn negative_numbers() {
        // Create a terrain and add two voxels.
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(Location::from_coords(0., -3., 0.), voxel::VoxelType(1));
        terrain.set_voxel_type(Location::from_coords(25., 0., 0.), voxel::VoxelType(1));

        assert_eq!(
            terrain.voxel_type(Location::from_coords(0., -3., 0.)),
            voxel::VoxelType(1)
        );
        assert_eq!(
            terrain.voxel_type(Location::from_coords(0., -2., 0.)),
            voxel::VoxelType(0)
        );
        assert_eq!(
            terrain.voxel_type(Location::from_coords(0., -4., 0.)),
            voxel::VoxelType(0)
        );
        assert_eq!(
            terrain.voxel_type(Location::from_coords(0., 1., 0.)),
            voxel::VoxelType(0)
        );
        assert_eq!(
            terrain.voxel_type(Location::from_coords(25., 0., 0.)),
            voxel::VoxelType(1)
        );
        assert_eq!(
            terrain.voxel_type(Location::from_coords(25., 1., 0.)),
            voxel::VoxelType(0)
        );
    }

    #[test]
    fn write_read() {
        // Create a terrain and add two voxels.
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(Location::from_coords(0., 0., 0.), voxel::VoxelType(1));
        terrain.set_voxel_type(Location::from_coords(25., 0., 0.), voxel::VoxelType(1));

        // Write terrain to file in bincode format.
        {
            let mut file = BufWriter::new(File::create("save.flex").unwrap());
            bincode::serialize_into(&mut file, &terrain).unwrap();
        }
        // Read terrain back from file.
        let file = BufReader::new(File::open("save.flex").unwrap());
        let terrain: Terrain = bincode::deserialize_from(file).unwrap();
        std::fs::remove_file("save.flex").unwrap();
        assert_eq!(
            terrain.voxel_type(Location::from_coords(0., 0., 0.)),
            voxel::VoxelType(1)
        );
        assert_eq!(
            terrain.voxel_type(Location::from_coords(0., 1., 0.)),
            voxel::VoxelType(0)
        );
        assert_eq!(
            terrain.voxel_type(Location::from_coords(25., 0., 0.)),
            voxel::VoxelType(1)
        );
        assert_eq!(
            terrain.voxel_type(Location::from_coords(25., 1., 0.)),
            voxel::VoxelType(0)
        );
    }

    #[test]
    fn ray_trace_2d() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(Location::from_coords(-6.48, 10.63, 0.), voxel::VoxelType(3));
        terrain.set_voxel_type(Location::from_coords(-5.73, 10.21, 0.), voxel::VoxelType(4));

        let dir = Vec3::new(-4.18, 2.34, 0.);
        let loc = Location::from_coords(18.22, -3.2, 0.);
        let hit = terrain.trace_ray(loc, dir).unwrap();
        let hit_type = terrain.voxel_type(hit);
        assert_eq!(hit.chunk, IVec3::new(-1, 0, 0));
        assert_eq!(hit_type, voxel::VoxelType(4));
    }

    #[test]
    fn ray_trace_3d() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(
            Location::from_coords(19.01, -3.64, -7.58),
            voxel::VoxelType(1),
        );
        terrain.set_voxel_type(
            Location::from_coords(12.62, -0.07, 7.58),
            voxel::VoxelType(2),
        );
        terrain.set_voxel_type(Location::from_coords(11.61, 0.5, 9.97), voxel::VoxelType(3));

        let dir = Vec3::new(-4.18, 2.34, 9.92);
        let loc = Location::from_coords(18.22, -3.2, -5.71);
        let hit = terrain.trace_ray(loc, dir).unwrap();
        let hit_type = terrain.voxel_type(hit);
        assert_eq!(hit.chunk, IVec3::new(0, -1, 0));
        assert!(hit.position.x > 11.999 && hit.position.x < 12.0001);
        assert_eq!(hit_type, voxel::VoxelType(2));
    }

    #[test]
    fn ray_trace_3d_long_distance() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(
            Location::from_coords(19.01, -3.64, -7.58),
            voxel::VoxelType(1),
        );
        terrain.set_voxel_type(
            Location::from_coords(-671.76, 258.52, 84.34),
            voxel::VoxelType(2),
        );

        let dir = Vec3::new(-2.51, -1.84, 0.02);
        let loc = Location::from_coords(-310.81, 523.12, 81.9);
        let hit = terrain.trace_ray(loc, dir).unwrap();
        let hit_type = terrain.voxel_type(hit);
        assert_eq!(hit_type, voxel::VoxelType(2));
    }

    #[test]
    fn ray_trace_chunk_border() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(
            Location::from_coords(chunk::CHUNK_SIZE_F, 0., 0.),
            voxel::VoxelType(1),
        );
        let dir = Vec3::new(1., 0., 0.);
        let loc = Location::from_coords(0.5, 0.5, 0.5);
        terrain.trace_ray(loc, dir).unwrap();
    }
}
