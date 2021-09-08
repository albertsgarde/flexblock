use crate::game::world::{raytrace, Location, Terrain, VoxelType};
use glm::Vec3;
use serde::{Deserialize, Serialize};
use std::mem::swap;

/// Represents a physical body that can collide with terrain and other physical bodies.
#[derive(Deserialize, Serialize)]
pub struct Aabb {
    location: Location,
    size: Vec3,
}

impl Aabb {
    /// Creates a new AABB with given location and size.
    ///
    /// # Arguments
    ///
    /// `location` - The location of the lower corner of the AABB.
    /// `size` - Vector from the lower corner of the AABB to the upper corner.
    pub fn new(location: Location, size: Vec3) -> Aabb {
        Aabb { location, size }
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn size(&self) -> Vec3 {
        self.size
    }

    /// Moves the AABB by the given vector.
    ///
    /// # Arguments
    ///
    /// `vec` - The vector to add to the body's location.
    pub fn translate(&mut self, vec: Vec3) {
        self.location += vec;
    }

    /// Returns true if self intersects with the given AABB.
    ///
    /// # Arguments
    ///
    /// `other` - The AABB to check for collisions with.
    pub fn collides_with(&self, other: &Aabb) -> bool {
        let self_upper = self.location + self.size;
        let other_upper = other.location + other.size;
        (0..3).all(|i| {
            self.location.chunk[i] < other_upper.chunk[i]
                || self.location.chunk[i] == other_upper.chunk[i]
                    && self.location.position[i] <= other_upper.position[i]
        }) && (0..3).all(|i| {
            other.location.chunk[i] < self_upper.chunk[i]
                || other.location.chunk[i] == self_upper.chunk[i]
                    && other.location.position[i] <= self_upper.position[i]
        })
    }

    /// Returns the distance measured in `vec` lengths the AABB can move in `vec` direction before it collides with an impassable voxel,
    /// and the dimension of the normal of the surface it hits.
    /// The dimension is 0 for the x-axis, 1 for the y-axis and 2 for the z-axis.
    /// Returns None if no collision happens and the AABB can move the entire distance.
    pub fn collide_distance(&self, vec: Vec3, terrain: &Terrain) -> Option<(f32, usize)> {
        let vec_norm = vec.norm();
        if vec_norm == 0. {
            return None;
        }
        let vec = vec.normalize();
        let round_vec = vec.map(|coord| if coord < 0. { -1e-6 } else { 0. });
        let mut orig_rear_vertex = self.location;
        let mut orig_front_vertex = self.location + self.size;

        for i in 0..3 {
            if vec[i] < 0. {
                swap(
                    &mut orig_rear_vertex.chunk[i],
                    &mut orig_front_vertex.chunk[i],
                );
                swap(
                    &mut orig_rear_vertex.position[i],
                    &mut orig_front_vertex.position[i],
                );
            }
        }

        let orig_rear_vertex = orig_rear_vertex;
        let orig_front_vertex = orig_front_vertex;
        let mut front_vertex = orig_front_vertex;

        for i in 0..3 {
            if (front_vertex.position[i] % 1.).abs() < 1e-6 {
                front_vertex.position[i] -= 1e-6 * vec[i].signum();
            }
        }

        let mut cur_t = 0.;
        while cur_t < vec_norm {
            // Find the side on which the ray exits the current voxel and how far before it does so.
            let (dim, t) = raytrace::voxel_exit_dimension_t(
                front_vertex.position,
                vec,
                raytrace::round(front_vertex.position + round_vec),
                1.,
            );
            cur_t += t;

            // If the ray has gone beyond the vectors length.
            if cur_t >= vec_norm {
                return None;
            }

            front_vertex = orig_front_vertex + vec * cur_t;

            // Create box within the frontier.
            let mut test_box_front = front_vertex;
            test_box_front.position[dim] += vec[dim] * 0.9;
            test_box_front
                .position
                .iter_mut()
                .enumerate()
                .for_each(|(i, coord)| {
                    *coord = if vec[i] < 0. {
                        coord.floor()
                    } else {
                        coord.ceil()
                    }
                });
            let mut test_box_rear = orig_rear_vertex + vec * cur_t;
            test_box_rear.chunk[dim] = front_vertex.chunk[dim];
            test_box_rear.position[dim] = front_vertex.position[dim] + vec[dim] * 0.1;
            // Test for non-passable blocks in frontier.
            // TODO: Test passableness in a better way.
            if terrain
                .voxel_type_iterator(test_box_rear, test_box_front)
                .any(|voxel_type| voxel_type != VoxelType(0))
            {
                return Some((cur_t / vec_norm, dim));
            }
        }
        panic!("Code should never get here.");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::world::{Location, Terrain, VoxelType};

    #[test]
    fn simple_x() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(Location::from_coords(3., 0., 0.), VoxelType(1));
        let aabb = Aabb::new(Location::origin(), Vec3::new(1., 1., 1.));
        let (collide_distance, collide_dim) = aabb
            .collide_distance(Vec3::new(4., 0., 0.), &terrain)
            .unwrap();
        assert!(collide_distance > 0.4999 && collide_distance < 0.5001);
        assert_eq!(collide_dim, 0);
    }

    #[test]
    fn simple_y() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(Location::from_coords(0., 3., 0.), VoxelType(1));
        let aabb = Aabb::new(Location::origin(), Vec3::new(1., 1., 1.));
        let (collide_distance, collide_dim) = aabb
            .collide_distance(Vec3::new(0., 40000., 0.), &terrain)
            .unwrap();
        assert!(collide_distance > 0.00004999 && collide_distance < 0.00005001);
        assert_eq!(collide_dim, 1);
    }

    #[test]
    fn simple_z() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(Location::from_coords(0., 0., 3.), VoxelType(1));
        let aabb = Aabb::new(Location::origin(), Vec3::new(1., 1., 1.));
        let (collide_distance, collide_dim) = aabb
            .collide_distance(Vec3::new(0., 0., 400.), &terrain)
            .unwrap();
        assert!(collide_distance > 0.004999 && collide_distance < 0.005001);
        assert_eq!(collide_dim, 2);
    }

    #[test]
    fn diagonal_positive_2d() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(Location::from_coords(4., 8., 0.), VoxelType(1));
        terrain.set_voxel_type(Location::from_coords(1., 5., 0.), VoxelType(1));
        let aabb = Aabb::new(Location::from_coords(1., 1., 0.), Vec3::new(1., 1., 1.));
        let move_vector = Vec3::new(4., 11., 0.);
        let (collide_distance, _) = aabb.collide_distance(move_vector, &terrain).unwrap();
        assert!(collide_distance > 6.37 / move_vector.norm());
        assert!(collide_distance < 6.39 / move_vector.norm());
    }

    #[test]
    fn diagonal_positive_2d_non_whole() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(Location::from_coords(4., 8., 0.), VoxelType(1));
        terrain.set_voxel_type(Location::from_coords(1., 5., 0.), VoxelType(1));
        let aabb = Aabb::new(
            Location::from_coords(1.18, 1.18, 0.),
            Vec3::new(0.64, 0.62, 1.),
        );
        let move_vector = Vec3::new(3.7, 9.52, 0.);
        let (collide_distance, _) = aabb.collide_distance(move_vector, &terrain).unwrap();
        assert!(collide_distance > 6.64 / move_vector.norm());
        assert!(collide_distance < 6.66 / move_vector.norm());
    }

    #[test]
    fn diagonal_2d_negative() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(Location::from_coords(11., 0., -3.), VoxelType(1));
        terrain.set_voxel_type(Location::from_coords(9., 0., -2.), VoxelType(1));
        let aabb = Aabb::new(
            Location::from_coords(6.35, 0., -6.08),
            Vec3::new(3.62, 1., 1.36),
        );
        let move_vector = Vec3::new(2.62, 0., 5.49);
        let (collide_distance, _) = aabb.collide_distance(move_vector, &terrain).unwrap();
        println!(
            "Collide distance * move_vector norm: {}",
            collide_distance * move_vector.norm()
        );
        assert!(collide_distance > 2.38 / move_vector.norm());
        assert!(collide_distance < 2.40 / move_vector.norm());
    }

    #[test]
    fn diagonal_2d_negative_move() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(Location::from_coords(8., 0., -11.), VoxelType(1));
        terrain.set_voxel_type(Location::from_coords(4., 0., -13.), VoxelType(1));
        let aabb = Aabb::new(
            Location::from_coords(3.8, 0., -8.59),
            Vec3::new(3.62, 1., 1.36),
        );
        let move_vector = Vec3::new(1.13, 0., -6.27);
        let (collide_distance, _) = aabb.collide_distance(move_vector, &terrain).unwrap();
        println!(
            "Collide distance * move_vector norm: {}",
            collide_distance * move_vector.norm()
        );
        assert!(collide_distance > 3.26 / move_vector.norm());
        assert!(collide_distance < 3.28 / move_vector.norm());
    }

    #[test]
    fn diagonal_2d_small_move() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(Location::from_coords(0., 6., -6.), VoxelType(1));
        let aabb = Aabb::new(
            Location::from_coords(0., 6.88, -4.95),
            Vec3::new(1., 1.24, 1.24),
        );
        let move_vector = Vec3::new(0., -0.1, -0.09);
        let (collide_distance, _) = aabb.collide_distance(move_vector, &terrain).unwrap();
        println!(
            "Collide distance * move_vector norm: {}",
            collide_distance * move_vector.norm()
        );
        assert!(collide_distance > 0.06 / move_vector.norm());
        assert!(collide_distance < 0.08 / move_vector.norm());
    }

    #[test]
    fn negative_2d_small_move() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(Location::from_coords(2., 0., 1.), VoxelType(1));
        let aabb = Aabb::new(
            Location::from_coords(3.84, 0., 1.2),
            Vec3::new(1., 1.24, 1.24),
        );
        let move_vector = Vec3::new(-0.08, 0., 0.02);
        assert_eq!(aabb.collide_distance(move_vector, &terrain), None);
    }

    #[test]
    fn adjacent_2d_small_negative_move() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(Location::from_coords(2., 0., 1.), VoxelType(1));
        let aabb = Aabb::new(
            Location::from_coords(3., 0., 1.27),
            Vec3::new(1., 1.24, 1.24),
        );
        let move_vector = Vec3::new(-0.05, 0., -0.01);
        let (collide_distance, _) = aabb.collide_distance(move_vector, &terrain).unwrap();
        assert!(collide_distance < 0.02 / move_vector.norm());
    }

    #[test]
    fn adjacent_2d_small_positive_move() {
        let mut terrain = Terrain::new();
        terrain.set_voxel_type(Location::from_coords(2., 0., 1.), VoxelType(1));
        let aabb = Aabb::new(
            Location::from_coords(1., 0., 1.27),
            Vec3::new(1., 1.24, 1.24),
        );
        let move_vector = Vec3::new(0.05, 0., -0.01);
        let (collide_distance, _) = aabb.collide_distance(move_vector, &terrain).unwrap();
        assert!(collide_distance < 0.02 / move_vector.norm());
    }

    #[test]
    fn no_movement() {
        let terrain = Terrain::new();
        let aabb = Aabb::new(
            Location::from_coords(1., 0., 1.27),
            Vec3::new(1., 1.24, 1.24),
        );
        let move_vector = Vec3::new(0., 0., 0.);
        assert_eq!(aabb.collide_distance(move_vector, &terrain), None);
    }
}
