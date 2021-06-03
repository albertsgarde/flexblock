use crate::game::{
    physics::Aabb,
    world::{Location, Terrain},
    SECONDS_PER_TICK,
};
use glm::Vec3;
use serde::{Deserialize, Serialize};

/// Downwards acceleration in voxels per second squared.
const GRAVITY: f32 = 9.81;

#[derive(Serialize, Deserialize)]
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
    /// Returns distance moved or none if no collision occurs.
    pub fn collide_move(&mut self, vec: Vec3, terrain: &Terrain) -> Option<f32> {
        let move_distance = self.aabb.collide_distance(vec, terrain);
        // Correct for collision.
        let vec = vec * (move_distance.unwrap_or(1.) * 0.999);
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
