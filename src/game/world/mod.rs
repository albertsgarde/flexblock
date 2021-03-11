mod chunk;
mod terrain;
mod voxel;

pub use terrain::Terrain;
pub use voxel::Voxel;

/// Defines a integer location in the world.
/// Specifies a voxel.
#[derive(Clone, Copy, Debug)]
pub struct Location {
    x: i32,
    y: i32,
    z: i32,
}

impl Location {
    /// Creates a new Location from specified coordinates.
    pub fn new(x: i32, y: i32, z: i32) -> Location {
        Location { x, y, z }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
