use glm::Vec3;

/// Represents a half-line with a start point and a direction vector.
#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    pub min: f32,
    pub max: f32,
}

impl Ray {
    /// Creates a new ray with given origin, direction vector, and minimum distance.
    pub fn new(origin: Vec3, direction: Vec3, min: f32) -> Ray {
        Ray {
            origin,
            direction,
            min,
            max: f32::INFINITY,
        }
    }

    /// The origin or start point of the ray.
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    /// The direction vector.
    /// Defines where the ray points and how fast it moves.
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// Returns the point `t` along the ray.
    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
