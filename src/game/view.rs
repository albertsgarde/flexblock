use crate::game::world::Location;
use glm::Vec3;
use serde::{Deserialize, Serialize};

/// Represents a view point in the world.
#[derive(Serialize, Deserialize, Clone)]
pub struct View {
    location: Location,
    view_direction: Vec3,
    up: Vec3,
}

impl View {
    /// Creates a new view with the given location, view direction, and up direction.
    ///
    /// # Arguments
    ///
    /// `location` - The initial location of the view.
    /// `view_direction` - The initial view direction.
    /// `up` - The initial up direction of the view.
    pub fn new(location: Location, view_direction: Vec3, up: Vec3) -> View {
        View {
            location,
            view_direction,
            up,
        }
    }

    /// Creates a new view at origin facing in the negative z-direction.
    pub fn default() -> View {
        View::new(Location::origin(), Vec3::new(0., 0., -1.), Vec3::new(0., 1., 0.))
    }

    /// Returns the location of the eye of this View.
    pub fn location(&self) -> Location {
        self.location
    }

    /// Returns a vector in the view direction of this View.
    pub fn view_direction(&self) -> Vec3 {
        self.view_direction
    }

    // Returns a vector in the up direction for this View.
    pub fn up(&self) -> Vec3 {
        self.up
    }
}
