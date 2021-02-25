use serde::{Deserialize, Serialize};

use crate::world::{
    chunk::{Chunk, ChunkLocation},
    voxel::{Voxel, VoxelType},
    Location, *,
};
use hashbrown::hash_map::HashMap;

const CHUNK_SIZE: i32 = crate::world::chunk::CHUNK_SIZE as i32;

fn modulus(lhs: i32, rhs: i32) -> u32 {
    if lhs < 0 {
        (lhs % rhs + rhs) as u32
    } else {
        (lhs % rhs) as u32
    }
}

fn integer_division(lhs: i32, rhs: i32) -> i32 {
    if lhs < 0 {
        (lhs + 1) / rhs - 1
    } else {
        lhs / rhs
    }
}

/// Struct that stores all voxels in the world.
#[derive(Serialize, Deserialize)]
pub struct Terrain {
    chunks: HashMap<(i32, i32, i32), Chunk>,
}

/// Transforms a world location into a chunck index and a chunk location.
/// In other words, given a world location, this function finds which chunk that location is in
/// and the location within that chunk.
fn location_to_chunk_index_and_location(loc: Location) -> ((i32, i32, i32), ChunkLocation) {
    (
        (
            integer_division(loc.x, CHUNK_SIZE),
            integer_division(loc.y, CHUNK_SIZE),
            integer_division(loc.z, CHUNK_SIZE),
        ),
        ChunkLocation::new(
            modulus(loc.x, CHUNK_SIZE),
            modulus(loc.y, CHUNK_SIZE),
            modulus(loc.z, CHUNK_SIZE),
        ),
    )
}

impl Terrain {
    /// Creates a new Terrain with all voxels set to default type.
    pub fn new() -> Terrain {
        Terrain {
            chunks: HashMap::new(),
        }
    }

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
    pub fn voxel(&self, loc: Location) -> (VoxelType, Option<&dyn Voxel>) {
        let (chunk_index, chunk_location) = location_to_chunk_index_and_location(loc);
        match self.chunks.get(&chunk_index) {
            Some(chunk) => chunk.voxel_unchecked(chunk_location),
            _ => (voxel::DEFAULT_TYPE, None),
        }
    }

    /// Sets the voxel type for the voxel at the given location.
    /// If the location is outside of all chunks, a new chunk is created.
    pub fn set_voxel_type(&mut self, loc: Location, voxel_type: VoxelType) {
        let (chunk_index, chunk_location) = location_to_chunk_index_and_location(loc);
        if let Some(chunk) = self.chunks.get_mut(&chunk_index) {
            chunk.set_voxel_type_unchecked(chunk_location, voxel_type);
        } else {
            let mut chunk = Chunk::new();
            chunk.set_voxel_type_unchecked(chunk_location, voxel_type);
            self.chunks.insert(chunk_index, chunk);
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
}

#[cfg(test)]
mod tests {
    use crate::world::*;
    use std::fs::File;
    use std::io::{BufWriter, BufReader};
    #[test]
    fn negative_numbers() {
        // Create a terrain and add two voxels.
        let mut terrain = terrain::Terrain::new();
        terrain.set_voxel_type(Location::new(0,-3,0), voxel::VoxelType(1));
        terrain.set_voxel_type(Location::new(25,0,0), voxel::VoxelType(1));
        
        assert_eq!(terrain.voxel_type(Location::new(0,-3,0)), voxel::VoxelType(1));
        assert_eq!(terrain.voxel_type(Location::new(0,-2,0)), voxel::VoxelType(0));
        assert_eq!(terrain.voxel_type(Location::new(0,-4,0)), voxel::VoxelType(0));
        assert_eq!(terrain.voxel_type(Location::new(0,1,0)), voxel::VoxelType(0));
        assert_eq!(terrain.voxel_type(Location::new(25,0,0)), voxel::VoxelType(1));
        assert_eq!(terrain.voxel_type(Location::new(25,1,0)), voxel::VoxelType(0));
    }

    #[test]
    fn write_read() {
        // Create a terrain and add two voxels.
        let mut terrain = terrain::Terrain::new();
        terrain.set_voxel_type(Location::new(0,0,0), voxel::VoxelType(1));
        terrain.set_voxel_type(Location::new(25,0,0), voxel::VoxelType(1));
        
        // Write terrain to file in bincode format.
        {
            let mut file = BufWriter::new(File::create("save.flex").unwrap());
            bincode::serialize_into(&mut file, &terrain).unwrap();
        }
        // Read terrain back from file.
        let file = BufReader::new(File::open("save.flex").unwrap());
        let terrain: terrain::Terrain = bincode::deserialize_from(file).unwrap();
        std::fs::remove_file("save.flex").unwrap();
        assert_eq!(terrain.voxel_type(Location::new(0,0,0)), voxel::VoxelType(1));
        assert_eq!(terrain.voxel_type(Location::new(0,1,0)), voxel::VoxelType(0));
        assert_eq!(terrain.voxel_type(Location::new(25,0,0)), voxel::VoxelType(1));
        assert_eq!(terrain.voxel_type(Location::new(25,1,0)), voxel::VoxelType(0));
    }
}
