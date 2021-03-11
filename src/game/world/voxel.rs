use serde::{Deserialize, Serialize};

/// Represents a type of voxel.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct VoxelType(pub u16);

pub const DEFAULT_TYPE: VoxelType = VoxelType(0);

/// Defines functionality and extra information for a voxel.
#[typetag::serde(tag = "type")]
pub trait Voxel {}
