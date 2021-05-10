mod chunk;
mod generation;
pub mod raytrace;
mod terrain;
mod voxel;

pub use chunk::chunk_index_to_position;
pub use chunk::Chunk;
pub use terrain::Terrain;
pub use voxel::Voxel;
pub use voxel::VoxelType;

use crate::utils::maths;
use glm::{IVec3, Vec3};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub};

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct LocationCoordinate {
    pub chunk: i32,
    pub position: f32,
}

impl Sub<LocationCoordinate> for LocationCoordinate {
    type Output = f32;

    fn sub(self, rhs: LocationCoordinate) -> f32 {
        self.position - rhs.position + self.chunk as f32 - rhs.chunk as f32
    }
}

/// Defines a integer location in the world.
/// Specifies a voxel.
#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub struct Location {
    pub chunk: IVec3,
    pub position: Vec3,
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
    pub fn new(chunk: IVec3, position: Vec3) -> Location {
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
        Location::new(IVec3::new(0, 0, 0), Vec3::new(0., 0., 0.))
    }

    pub fn x(&self) -> LocationCoordinate {
        LocationCoordinate {
            chunk: self.chunk.x,
            position: self.position.x,
        }
    }

    pub fn y(&self) -> LocationCoordinate {
        LocationCoordinate {
            chunk: self.chunk.y,
            position: self.position.y,
        }
    }

    pub fn z(&self) -> LocationCoordinate {
        LocationCoordinate {
            chunk: self.chunk.z,
            position: self.position.z,
        }
    }

    pub fn coord(&self, index: usize) -> LocationCoordinate {
        LocationCoordinate {
            chunk: self.chunk[index],
            position: self.position[index],
        }
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
        Vec3::new(x, y, z).into()
    }

    /// Rounds the location to the nearest whole voxel.
    pub fn round(&self) -> Location {
        Location::new(self.chunk, raytrace::round(self.position))
    }
}

impl From<Vec3> for Location {
    fn from(position: Vec3) -> Location {
        let chunk =
            position.map(|x| maths::integer_division(x.floor() as i32, chunk::CHUNK_SIZE as i32));
        let position = position.map(|x| maths::modulus(x, chunk::CHUNK_SIZE as f32));
        Location::new(chunk, position)
    }
}

impl Add<Vec3> for Location {
    type Output = Location;

    fn add(mut self, rhs: Vec3) -> Location {
        self.position += rhs;
        self.coerce();
        self
    }
}

impl AddAssign<Vec3> for Location {
    fn add_assign(&mut self, other: Vec3) {
        *self = *self + other;
    }
}

impl Sub<Location> for Location {
    type Output = Vec3;

    fn sub(self, rhs: Location) -> Vec3 {
        self.position - rhs.position + (self.chunk - rhs.chunk).map(|x| x as f32 * 16.)
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
        assert_eq!(loc.chunk, IVec3::new(0, 0, 0));
        assert!(loc.position.x > 1.999 && loc.position.x < 2.001);
        assert!(loc.position.y > 2.999 && loc.position.y < 3.001);
        assert!(loc.position.z > 3.999 && loc.position.z < 4.001);

        let loc = Location::from_coords(16. + 2., 16. + 3., 16. + 4.);
        assert_eq!(loc.chunk, IVec3::new(1, 1, 1));
        assert!(loc.position.x > 1.999 && loc.position.x < 2.001);
        assert!(loc.position.y > 2.999 && loc.position.y < 3.001);
        assert!(loc.position.z > 3.999 && loc.position.z < 4.001);
    }

    #[test]
    fn from_coords_negative() {
        let loc = Location::from_coords(2., -3., 4.);
        assert_eq!(loc.chunk, IVec3::new(0, -1, 0));
        assert!(loc.position.x > 1.999 && loc.position.x < 2.001);
        assert!(loc.position.y > 12.999 && loc.position.y < 13.001);
        assert!(loc.position.z > 3.999 && loc.position.z < 4.001);
    }

    #[test]
    fn subtract() {
        let loc1 = Location::from_coords(1321., -231., 21.);
        let loc2 = Location::from_coords(-21., -32.13, 42.);
        let result = loc1 - loc2;
        assert!(result.x > 1341.999 && result.x < 1342.001);
        assert!(result.y > -198.8701 && result.y < -198.8699);
        assert!(result.z > -21.001 && result.z < -20.999);
    }

    #[test]
    fn location_coordinate_comparison() {
        assert!(
            LocationCoordinate {
                chunk: 2,
                position: 5.
            } > LocationCoordinate {
                chunk: 1,
                position: 6.
            }
        );
    }
}
