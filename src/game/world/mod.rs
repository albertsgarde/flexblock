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
    pub fn new(chunk: Vector3<i32>, position: Vector3<f32>) -> Location {
        Location { chunk, position }
    }

    pub fn from_coords(x: f32, y: f32, z: f32) -> Location {
        Vector3::new(x, y, z).into()
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
