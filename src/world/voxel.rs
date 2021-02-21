/// Represents a type of voxel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VoxelType(pub u16);

pub const DEFAULT_TYPE: VoxelType = VoxelType(0);

/// Defines functionality and extra information for a voxel.
pub trait Voxel {}
