use cgmath::Vector3;
use std::mem::swap;

pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray { origin, direction }
    }

    pub fn voxel_exit(&self, voxel_corner: Vector3<f32>, voxel_size: f32) -> Option<Vector3<f32>> {
        let mut t_min_x = (voxel_corner.x - self.origin.x) / self.direction.x;
        let mut t_max_x = (voxel_corner.x + voxel_size - self.origin.x) / self.direction.x;
        if t_min_x > t_max_x {
            swap(&mut t_min_x, &mut t_max_x);
        }
        let mut max_min = t_min_x;
        let mut t_min_y = (voxel_corner.y - self.origin.y) / self.direction.y;
        let mut t_max_y = (voxel_corner.y + voxel_size - self.origin.y) / self.direction.y;
        if t_min_y > t_max_y {
            swap(&mut t_min_y, &mut t_max_y);
        }
        if t_min_y > max_min {
            max_min = t_min_y;
        }
        let mut t_min_z = (voxel_corner.z - self.origin.z) / self.direction.z;
        let mut t_max_z = (voxel_corner.z + voxel_size - self.origin.z) / self.direction.z;
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
        if t_max_x > t_max_y {
            if t_max_x > t_max_z {
                result.x += voxel_size.copysign(self.direction.x);
            } else {
                result.z += voxel_size.copysign(self.direction.z);
            }
        } else {
            result.y += voxel_size.copysign(self.direction.y);
        }
        Some(result)
    }
}
