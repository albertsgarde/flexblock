mod chunk;
mod raytrace;
pub mod terrain;
pub mod voxel;

use crate::utils::maths;
use cgmath::Vector3;

/// Defines a integer location in the world.
/// Specifies a voxel.
#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub chunk: Vector3<i32>,
    pub position: Vector3<f32>,
}

impl Location {
    /// Creates a new Location from specified coordinates.
    /// 
    /// # Arguments
    /// 
    /// `chunk` - The index of the chunk.
    /// `position` - The floating point position in this chunk.
    pub fn new(chunk: Vector3<i32>, position: Vector3<f32>) -> Location {
        Location { chunk, position }
    }

    /// Creates a Location from a set of absolute floating point coordinates.
    /// 
    /// # Arguments
    /// 
    /// `x` - The absolute x-coordinate of the location.
    /// `y` - The absolute y-coordinate of the location.
    /// `z` - The absolute z-coordinate of the location.
    pub fn from_coords(x: f32, y: f32, z: f32) -> Location {
        Vector3::new(x, y, z).into()
    }

    /// Rounds the location to the nearest whole voxel.
    pub fn round(&self) -> Location {
        Location::new(self.chunk, self.position.map(|x| maths::modulus(x, chunk::CHUNK_SIZE as f32)))
    }
}

impl From<Vector3<f32>> for Location {
    fn from(position: Vector3<f32>) -> Location {
        let chunk =
            position.map(|x| maths::integer_division(x.floor() as i32, chunk::CHUNK_SIZE as i32));
        let position = position.map(|x| maths::modulus(x, chunk::CHUNK_SIZE as f32));
        Location::new(chunk, position)
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Chunk: ({}, {}, {}), Position: ({}, {}, {})",
            self.chunk.x,
            self.chunk.y,
            self.chunk.z,
            self.position.x,
            self.position.y,
            self.position.z
        )
    }
}
