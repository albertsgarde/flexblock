use crate::game::{view::view_direction::ViewDirection, world::Location};
use glm::Vec3;
use serde::{Deserialize, Serialize};

use super::principal_axes::PrincipalAxes;

/// Represents a view point in the world.
#[derive(Serialize, Deserialize, Clone)]
pub struct View {
    location: Location,
    view_direction: Box<dyn ViewDirection>,
}

impl View {
    /// Creates a new view with the given location, view direction, and up direction.
    pub fn new<V: 'static + ViewDirection>(location: Location, view_direction: V) -> Self {
        View {
            location,
            view_direction: Box::new(view_direction),
        }
    }

    /// Returns the location of the eye of this View.
    pub fn location(&self) -> Location {
        self.location
    }

    /// Returns a vector in the view direction of this View.
    pub fn view_direction(&self) -> Vec3 {
        self.view_direction.forward()
    }

    /// Returns a vector in the up direction for this View.
    pub fn up(&self) -> Vec3 {
        self.view_direction.up()
    }

    pub fn right(&self) -> Vec3 {
        self.view_direction.right()
    }

    /// Translates the location of the view.
    pub fn translate(&mut self, vec: Vec3) {
        self.location = self.location + vec;
    }

    /// Moves the view to the given location.
    pub fn teleport(&mut self, loc: Location) {
        self.location = loc;
    }

    /// Rotates the view direction along the great circle in the delta direction by |delta| radians.
    pub fn turn(&mut self, delta: (f32, f32)) {
        self.view_direction.turn(delta);
    }

    /// Given a vector in view coordinates, returns the same vector in world coordinates.
    pub fn view_to_world(&self, vec: Vec3) -> Vec3 {
        self.view_direction.view_to_world(vec)
    }
}

impl Default for View {
    fn default() -> View {
        View::new(Location::origin(), PrincipalAxes::default())
    }
}
