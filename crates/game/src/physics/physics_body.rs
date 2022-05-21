use crate::{physics::Aabb, SECONDS_PER_TICK};
use glm::Vec3;
use serde::{Deserialize, Serialize};
use world::{Location, Terrain};

/// Downwards acceleration in voxels per second squared.
const GRAVITY: f32 = 1.8;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhysicsBody {
    aabb: Aabb,
    velocity: Vec3,
}

impl PhysicsBody {
    pub fn new(aabb: Aabb) -> PhysicsBody {
        PhysicsBody {
            aabb,
            velocity: Vec3::new(0., 0., 0.),
        }
    }

    pub fn aabb(&self) -> &Aabb {
        &self.aabb
    }

    pub fn location(&self) -> Location {
        self.aabb.location()
    }

    pub fn size(&self) -> Vec3 {
        self.aabb.size()
    }

    /// Moves the AABB by the given vector.
    ///
    /// # Arguments
    ///
    /// `vec` - The vector to add to the body's location.
    pub fn translate(&mut self, vec: Vec3) {
        self.aabb.translate(vec)
    }

    /// Moves the body as far along the vector as possible without colliding with the terrain.
    /// Returns distance moved in vector units and the dimension of the normal of the surface hit, or none if no collision occurs.
    /// The dimension is 0 for the x-axis, 1 for the y-axis and 2 for the z-axis.
    pub fn collide_move(&mut self, vec: Vec3, terrain: &Terrain) -> Option<(f32, usize)> {
        // Correct for collision.
        let move_distance = self.aabb.collide_distance(vec, terrain);
        let vec = vec * (move_distance.map(|(distance, _)| distance).unwrap_or(1.) * 0.999);
        // Remove very small movement, as it is probably an error...
        let vec = vec.map(|coord| if coord.abs() < 1e-5 { 0. } else { coord });
        self.translate(vec);
        move_distance
    }

    pub fn add_velocity(&mut self, vec: Vec3) {
        self.velocity += vec;
    }

    pub fn tick(&mut self, terrain: &Terrain) {
        // Add gravity.
        self.velocity += Vec3::new(0., -GRAVITY * SECONDS_PER_TICK, 0.);

        // If the movement is stopped, set velocity to 0.
        if self.collide_move(self.velocity, terrain).is_some() {
            self.velocity = Vec3::zeros();
        }
    }
}
