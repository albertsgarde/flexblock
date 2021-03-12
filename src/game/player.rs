use crate::game::world::Location;
use cgmath::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    location: Location,
    view_direction: Vector3<f32>,
}

impl Player {
    pub fn new(location: Location, view_direction: Vector3<f32>) -> Player {
        Player {location, view_direction}
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn view_direction(&self) -> Vector3<f32> {
        self.view_direction
    }
}
