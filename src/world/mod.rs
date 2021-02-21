mod chunk;
pub mod terrain;
pub mod voxel;

/// Defines a integer location in the world.
/// Specifies a voxel.
pub struct Location {
    x: i32,
    y: i32,
    z: i32,
}
