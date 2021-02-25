use cgmath::Vector3;

/// Represents a half-line with a start point and a direction vector.
#[derive(Debug)]
pub struct Ray{
    origin: Vector3<f32>,
    direction: Vector3<f32>,
    pub min: f32,
    pub max: f32,
}

impl Ray {
    /// Creates a new ray with given origin, direction vector, and minimum distance.
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>, min: f32) -> Ray {
        Ray {
            origin,
            direction,
            min,
            max: f32::INFINITY,
        }
    }

    /// The origin or start point of the ray.
    pub fn origin(&self) -> Vector3<f32> {
        self.origin
    }

    /// The direction vector.
    /// Defines where the ray points and how fast it moves.
    pub fn direction(&self) -> Vector3<f32> {
        self.direction
    }

    /// Returns the point `t` along the ray.
    pub fn point_at(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction*t
    }
}
