use crate::game::world::voxel;
use glm::Vec3;
use std::mem::swap;

/// Whether to ignore the specified voxel type when tracing.
pub fn ignore_voxel_type(voxel_type: voxel::VoxelType) -> bool {
    voxel_type == voxel::DEFAULT_TYPE
}

pub fn round(position: Vec3) -> Vec3 {
    position.map(|x| x.floor())
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    /// After entering the voxel with the specified corner, returns the voxel the ray exits into or None if the ray never hits the voxel.
    ///
    /// # Arguments
    ///
    /// `voxel_corner` - The corner of the voxel with lowest coordinates.
    /// `voxel_size` - The size of voxels. Should usually be 1.
    pub fn voxel_exit(&self, voxel_corner: Vec3, voxel_size: f32) -> Option<Vec3> {
        println!("Origin: {:?}  Direction: {:?}", self.origin, self.direction);
        println!("Helleflynder: {:?}", voxel_corner);
        let (mut t_min_x, mut t_max_x) = if self.direction.x == 0. {
            (-std::f32::INFINITY, std::f32::INFINITY)
        } else {
            (
                (voxel_corner.x - self.origin.x) / self.direction.x,
                (voxel_corner.x + voxel_size - self.origin.x) / self.direction.x,
            )
        };
        if t_min_x > t_max_x {
            swap(&mut t_min_x, &mut t_max_x);
        }
        let mut max_min = t_min_x;

        let (mut t_min_y, mut t_max_y) = if self.direction.y == 0. {
            (-std::f32::INFINITY, std::f32::INFINITY)
        } else {
            (
                (voxel_corner.y - self.origin.y) / self.direction.y,
                (voxel_corner.y + voxel_size - self.origin.y) / self.direction.y,
            )
        };
        if t_min_y > t_max_y {
            swap(&mut t_min_y, &mut t_max_y);
        }
        if t_min_y > max_min {
            max_min = t_min_y;
        }

        let (mut t_min_z, mut t_max_z) = if self.direction.z == 0. {
            (-std::f32::INFINITY, std::f32::INFINITY)
        } else {
            (
                (voxel_corner.z - self.origin.z) / self.direction.z,
                (voxel_corner.z + voxel_size - self.origin.z) / self.direction.z,
            )
        };
        if t_min_z > t_max_z {
            swap(&mut t_min_z, &mut t_max_z);
        }
        if t_min_z > max_min {
            max_min = t_min_z;
        }
        if max_min > t_max_x || max_min > t_max_y || max_min > t_max_z {
            return None;
        }
        let mut result = voxel_corner;
        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                result.x += voxel_size.copysign(self.direction.x);
            } else {
                result.z += voxel_size.copysign(self.direction.z);
            }
        } else if t_max_y < t_max_z {
            result.y += voxel_size.copysign(self.direction.y);
        } else {
            result.z += voxel_size.copysign(self.direction.z);
        }
        Some(result)
    }
}

/// After entering the voxel with the specified corner, returns the voxel the ray exits into or None if the ray never hits the voxel.
///
/// # Arguments
///
/// `voxel_corner` - The corner of the voxel with lowest coordinates.
/// `voxel_size` - The size of voxels. Should usually be 1.
pub fn voxel_exit(origin: Vec3, direction: Vec3, voxel_corner: Vec3, voxel_size: f32) -> f32 {
    let (mut t_min_x, mut t_max_x) = if direction.x == 0. {
        (-std::f32::INFINITY, std::f32::INFINITY)
    } else {
        (
            (voxel_corner.x - origin.x) / direction.x,
            (voxel_corner.x + voxel_size - origin.x) / direction.x,
        )
    };
    if t_min_x > t_max_x {
        swap(&mut t_min_x, &mut t_max_x);
    }
    let mut min_max = t_max_x;

    let (mut t_min_y, mut t_max_y) = if direction.y == 0. {
        (-std::f32::INFINITY, std::f32::INFINITY)
    } else {
        (
            (voxel_corner.y - origin.y) / direction.y,
            (voxel_corner.y + voxel_size - origin.y) / direction.y,
        )
    };
    if t_min_y > t_max_y {
        swap(&mut t_min_y, &mut t_max_y);
    }
    if t_max_y < min_max {
        min_max = t_max_y;
    }

    let (mut t_min_z, mut t_max_z) = if direction.z == 0. {
        (-std::f32::INFINITY, std::f32::INFINITY)
    } else {
        (
            (voxel_corner.z - origin.z) / direction.z,
            (voxel_corner.z + voxel_size - origin.z) / direction.z,
        )
    };
    if t_min_z > t_max_z {
        swap(&mut t_min_z, &mut t_max_z);
    }
    if t_max_z < min_max {
        min_max = t_max_z;
    }
    debug_assert!(!(min_max < t_min_x || min_max < t_min_y || min_max < t_min_z));
    min_max
}
