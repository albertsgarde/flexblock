use crate::game::world::{Location, Terrain};
use cgmath::Vector3;

/// Represents a physical body that can collide with terrain and other physical bodies.
pub struct RigidBody {
    location: Location,
    size: Vector3<f32>,
}

impl RigidBody {
    /// Creates a new RigidBody with given location and size.
    ///
    /// # Arguments
    ///
    /// `location` - The location of the lower corner of the RigidBody.
    /// `size` - Vector from the lower corner of the RigidBody to the upper corner.
    pub fn new(location: Location, size: Vector3<f32>) -> RigidBody {
        RigidBody {location, size}
    }

    /// Moves the RigidBody by the given vector.
    ///
    /// # Arguments
    ///
    /// `vec` - The vector to add to the body's location.
    pub fn translate(&mut self, vec: Vector3<f32>) {
        self.location = self.location + vec;
    }

    /// Returns true if self intersects with the given RigidBody.
    ///
    /// # Arguments
    ///
    /// `other` - The RigidBody to check for collisions with.
    pub fn collides_with(&self, other: &RigidBody) -> bool {
        let self_upper = self.location+self.size;
        let other_upper = other.location+other.size;
        (0..3).all(|i| self.location.chunk[i] < other_upper.chunk[i] || self.location.chunk[i] == other_upper.chunk[i] && self.location.position[i] <= other_upper.position[i]) &&
        (0..3).all(|i| other.location.chunk[i] < self_upper.chunk[i] || other.location.chunk[i] == self_upper.chunk[i] && other.location.position[i] <= self_upper.position[i])
    }
}
