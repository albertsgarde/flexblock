use crate::game::world::Location;
use cgmath::Vector3;
use serde::{Deserialize, Serialize};

/// Represents the player character in the world.
/// Holds information about the position and view direction.
#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    location: Location,
    view_direction: Vector3<f32>,
}

impl Player {
    /// Creates a new player with the given location and view direction.
    /// 
    /// # Arguments
    /// 
    /// `location` - The initial location of the player.
    /// `view_direction` - The initial view direction of the player.
    pub fn new(location: Location, view_direction: Vector3<f32>) -> Player {
        Player {location, view_direction}
    }

    /// Creates a new player at origin facing in the positive x-direction.
    pub fn default() -> Player {
        Player::new(Location::origin(), Vector3::new(1., 0., 0.))
    }

    /// Returns the player's location.
    pub fn location(&self) -> Location {
        self.location
    }

    /// Returns the player's view direction.
    pub fn view_direction(&self) -> Vector3<f32> {
        self.view_direction
    }
}
