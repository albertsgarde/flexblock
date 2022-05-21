use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Represents a type of voxel.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct VoxelType(pub u16);

pub const DEFAULT_TYPE: VoxelType = VoxelType(0);

/// Defines functionality and extra information for a voxel.
#[typetag::serde(tag = "type")]
pub trait Voxel: VoxelClone + Send + Sync + Debug {}

pub trait VoxelClone {
    fn clone_box(&self) -> Box<dyn Voxel>;
}

impl<T> VoxelClone for T
where
    T: 'static + Voxel + Clone,
{
    fn clone_box(&self) -> Box<dyn Voxel> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Voxel> {
    fn clone(&self) -> Box<dyn Voxel> {
        self.clone_box()
    }
}
