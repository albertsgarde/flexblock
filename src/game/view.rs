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
        View::new(
            Location::origin(),
            Vec3::new(0., 0., -1.),
            Vec3::new(0., 1., 0.),
        )
    }

    /// Returns the location of the eye of this View.
    pub fn location(&self) -> Location {
        self.location
    }

    /// Returns a vector in the view direction of this View.
    pub fn view_direction(&self) -> Vec3 {
        self.view_direction
    }

    /// Returns a vector in the up direction for this View.
    pub fn up(&self) -> Vec3 {
        self.up
    }

    /// Translates the location of the view.
    pub fn translate(&mut self, vec: Vec3) {
        self.location = self.location + vec;
    }

    /// Applies the given matrix to the view and up directions.
    pub fn rotate(&mut self, matrix: glm::Mat3x3) {
        self.view_direction = matrix * self.view_direction;
        self.up = matrix * self.up;
    }

    /// Rotates the view direction along the great circle in the delta direction by |delta| radians.
    pub fn turn(&mut self, delta: (f32, f32)) {
        let length = (delta.0.powi(2) + delta.1.powi(2)).sqrt();
        let normal: Vec3 =
            (-delta.1 * glm::cross(&self.view_direction, &self.up) - delta.0 * self.up) / length;
        // See https://en.wikipedia.org/wiki/Rodrigues%27_rotation_formula#Matrix_notation.
        let k_matrix = nalgebra_glm::make_mat3x3(&[
            0., normal.z, -normal.y, -normal.z, 0., normal.x, normal.y, -normal.x, 0.,
        ]);
        let radians = length;
        let rotation_matrix: glm::Mat3x3 = nalgebra_glm::identity::<f32, nalgebra_glm::U3>()
            + radians.sin() * k_matrix
            + (1. - radians.cos()) * k_matrix * k_matrix;
        self.rotate(rotation_matrix);
    }

    /// Given a vector in view coordinates, returns the same vector in world coordinates.
    pub fn view_to_world(&self, vec: Vec3) -> Vec3 {
        -vec.z * self.view_direction
            + vec.y * self.up
            + vec.x * glm::cross(&self.view_direction, &self.up)
    }
}
