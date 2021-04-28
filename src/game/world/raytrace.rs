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

/// Returns the entry and exit t's for all dimensions of the given ray through the given voxel.
///
/// # Panics
///
/// Panics if `direction` is the zero vector.
fn voxel_entry_exit(
    origin: Vec3,
    direction: Vec3,
    voxel_corner: Vec3,
    voxel_size: f32,
) -> (Vec3, Vec3) {
    let mut t_min: Vec3 = Vec3::new(0., 0., 0.);
    let mut t_max: Vec3 = Vec3::new(0., 0., 0.);
    // Compare with t_min to not create a new vector.
    if direction == t_min {
        panic!("Direction may not be the 0 vector.")
    }
    for i in 0..3 {
        if direction[i] == 0. {
            t_min[i] = -std::f32::INFINITY;
            t_max[i] = std::f32::INFINITY;
        } else {
            t_min[i] = (voxel_corner[i] - origin[i]) / direction[i];
            t_max[i] = (voxel_corner[i] + voxel_size - origin[i]) / direction[i];
            if t_min[i] > t_max[i] {
                swap(&mut t_min[i], &mut t_max[i]);
            }
        }
    }
    (t_min, t_max)
}

/// After entering the voxel with the specified corner, returns the voxel the ray exits into or None if the ray never hits the voxel.
/// Returns None if the ray never hits the voxel.
///
/// # Panics
///
/// Panics if `direction` is the zero vector.
pub fn voxel_exit_voxel(
    origin: Vec3,
    direction: Vec3,
    voxel_corner: Vec3,
    voxel_size: f32,
) -> Option<Vec3> {
    let (t_min, t_max): (Vec3, Vec3) =
        voxel_entry_exit(origin, direction, voxel_corner, voxel_size);
    // Find t for entry and exit on the x-axis.

    // Define variable for the latest entry point,
    // which is the t the ray actually enters the voxel.
    let max_min = t_min.iter().fold(-f32::INFINITY, |a, &b| a.max(b));

    // If the entry point is later than any of the exit points, the ray never enters the voxel.
    if t_max.iter().any(|t| max_min > *t) {
        return None;
    }

    let mut result = voxel_corner;
    if t_max.x < t_max.y {
        if t_max.x < t_max.z {
            result.x += voxel_size.copysign(direction.x);
        } else {
            result.z += voxel_size.copysign(direction.z);
        }
    } else if t_max.y < t_max.z {
        result.y += voxel_size.copysign(direction.y);
    } else {
        result.z += voxel_size.copysign(direction.z);
    }
    Some(result)
}

/// Returns how far from the ray origin the ray exits the voxel with the given corner.
///
/// # Panics
///
/// Panics if `direction` is the zero vector or the ray never enters the voxel.
pub fn voxel_exit_t(origin: Vec3, direction: Vec3, voxel_corner: Vec3, voxel_size: f32) -> f32 {
    let (t_min, t_max): (Vec3, Vec3) =
        voxel_entry_exit(origin, direction, voxel_corner, voxel_size);

    // Define variable for the earliest exit point,
    // which is the t the ray actually exits the voxel.
    let min_max = t_max.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    // If the exit point is earlier than any of the entry points, the ray never enters the voxel.
    debug_assert!(!t_min.iter().any(|t| min_max < *t));
    min_max
}

pub fn voxel_exit_dimension_t(
    origin: Vec3,
    direction: Vec3,
    voxel_corner: Vec3,
    voxel_size: f32,
) -> (usize, f32) {
    let (t_min, t_max): (Vec3, Vec3) =
        voxel_entry_exit(origin, direction, voxel_corner, voxel_size);

    // Define variable for the earliest exit point,
    // which is the t the ray actually exits the voxel.
    let min_max = t_max.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    // If the exit point is earlier than any of the entry points, the ray never enters the voxel.
    debug_assert!(!t_min.iter().any(|t| min_max < *t));

    if t_max.x < t_max.y {
        if t_max.x < t_max.z {
            (0, min_max)
        } else {
            (2, min_max)
        }
    } else if t_max.y < t_max.z {
        (1, min_max)
    } else {
        (2, min_max)
    }
}

/// If the ray enters the voxel, returns both how far from the ray origin the ray exits the voxel with the given corner,
/// and the position og the voxel exited into.
///
/// # Panics
///
/// Panics if `directions` is the zero vector.
pub fn voxel_exit(
    origin: Vec3,
    direction: Vec3,
    voxel_corner: Vec3,
    voxel_size: f32,
) -> Option<(f32, Vec3)> {
    let (t_min, t_max): (Vec3, Vec3) =
        voxel_entry_exit(origin, direction, voxel_corner, voxel_size);

    // Define variable for the earliest exit point,
    // which is the t the ray actually exits the voxel.
    let min_max = t_max.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    if t_min.iter().any(|t| min_max < *t) {
        None
    } else {
        let mut result = voxel_corner;
        if t_max.x < t_max.y {
            if t_max.x < t_max.z {
                result.x += voxel_size.copysign(direction.x);
            } else {
                result.z += voxel_size.copysign(direction.z);
            }
        } else if t_max.y < t_max.z {
            result.y += voxel_size.copysign(direction.y);
        } else {
            result.z += voxel_size.copysign(direction.z);
        }
        Some((min_max, result))
    }
}
