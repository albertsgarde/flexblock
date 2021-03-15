mod chunk;
mod raytrace;
mod terrain;
mod voxel;

pub use terrain::Terrain;
pub use voxel::Voxel;

use crate::utils::maths;
use cgmath::Vector3;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

/// Defines a integer location in the world.
/// Specifies a voxel.
#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub struct Location {
    pub chunk: Vector3<i32>,
    pub position: Vector3<f32>,
}

impl Location {
    /// Creates a new Location from specified coordinates.
    /// Asserts in debug mode that the position is within 0 and chunk size in all dimensions.
    /// In release no such checks are made and if the position is out of bounds undefined behaviour occurs.
    ///
    /// # Arguments
    ///
    /// `chunk` - The index of the chunk.
    /// `position` - The floating point position in this chunk.
    pub fn new(chunk: Vector3<i32>, position: Vector3<f32>) -> Location {
        debug_assert!(
            position.x >= 0.
                && position.y >= 0.
                && position.z >= 0.
                && position.x <= chunk::CHUNK_SIZE as f32
                && position.y <= chunk::CHUNK_SIZE as f32
                && position.z <= chunk::CHUNK_SIZE as f32,
            "Position must be between 0 and chunk size on all coordinates. Position: ({}, {}, {})",
            position.x,
            position.y,
            position.z
        );
        Location { chunk, position }
    }

    /// 0 on all coordinates.
    pub fn origin() -> Location {
        Location::new(Vector3::new(0, 0, 0), Vector3::new(0., 0., 0.))
    }

    /// If the position is out of bounds, the chunk will be moved to correct for it.
    pub fn coerce(&mut self) {
        while self.position.x < 0. {
            self.chunk.x -= 1;
            self.position.x += chunk::CHUNK_SIZE_F;
        }
        while self.position.y < 0. {
            self.chunk.y -= 1;
            self.position.y += chunk::CHUNK_SIZE_F;
        }
        while self.position.z < 0. {
            self.chunk.z -= 1;
            self.position.z += chunk::CHUNK_SIZE_F;
        }
        while self.position.x >= chunk::CHUNK_SIZE_F {
            self.chunk.x += 1;
            self.position.x -= chunk::CHUNK_SIZE_F;
        }
        while self.position.y >= chunk::CHUNK_SIZE_F {
            self.chunk.y += 1;
            self.position.y -= chunk::CHUNK_SIZE_F;
        }
        while self.position.z >= chunk::CHUNK_SIZE_F {
            self.chunk.z += 1;
            self.position.z -= chunk::CHUNK_SIZE_F;
        }
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
        Location::new(self.chunk, raytrace::round(self.position))
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

impl Add<Vector3<f32>> for Location {
    type Output = Location;

    fn add(mut self, rhs: Vector3<f32>) -> Location {
        self.position += rhs;
        self.coerce();
        self
    }
}

impl Sub<Location> for Location {
    type Output = Vector3<f32>;

    fn sub(self, rhs: Location) -> Vector3<f32> {
        self.position-rhs.position+(self.chunk-rhs.chunk).map(|x| x as f32 *16.)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_coords() {
        let loc = Location::from_coords(2., 3., 4.);
        assert_eq!(loc.chunk, Vector3::new(0, 0, 0));
        assert!(loc.position.x > 1.999 && loc.position.x < 2.001);
        assert!(loc.position.y > 2.999 && loc.position.y < 3.001);
        assert!(loc.position.z > 3.999 && loc.position.z < 4.001);

        let loc = Location::from_coords(16. + 2., 16. + 3., 16. + 4.);
        assert_eq!(loc.chunk, Vector3::new(1, 1, 1));
        assert!(loc.position.x > 1.999 && loc.position.x < 2.001);
        assert!(loc.position.y > 2.999 && loc.position.y < 3.001);
        assert!(loc.position.z > 3.999 && loc.position.z < 4.001);
    }

    #[test]
    fn from_coords_negative() {
        let loc = Location::from_coords(2., -3., 4.);
        assert_eq!(loc.chunk, Vector3::new(0, -1, 0));
        assert!(loc.position.x > 1.999 && loc.position.x < 2.001);
        assert!(loc.position.y > 12.999 && loc.position.y < 13.001);
        assert!(loc.position.z > 3.999 && loc.position.z < 4.001);
    }

    #[test]
    fn subtract() {
        let loc1 = Location::from_coords(1321., -231., 21.);
        let loc2 = Location::from_coords(-21., -32.13, 42.);
        let result = loc1-loc2;
        assert!(result.x > 1341.999 && result.x < 1342.001);
        assert!(result.y > -198.8701 && result.y < -198.8699);
        assert!(result.z > -21.001 && result.z < -20.999);
    }
}
