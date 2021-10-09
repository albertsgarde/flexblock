use crate::view::view_direction::ViewDirection;
use glm::Vec3;
use serde::{Deserialize, Serialize};
use std::f32::consts::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct PrincipalAxes {
    yaw: f32,
    pitch: f32,
}

impl PrincipalAxes {
    pub fn new(yaw: f32, pitch: f32) -> PrincipalAxes {
        PrincipalAxes { yaw, pitch }
    }
}

#[typetag::serde]
impl ViewDirection for PrincipalAxes {
    fn forward(&self) -> Vec3 {
        let (yaw_sine, yaw_cosine) = self.yaw.sin_cos();
        let (pitch_sine, pitch_cosine) = self.pitch.sin_cos();
        Vec3::new(
            yaw_sine * pitch_sine,
            -pitch_cosine,
            -yaw_cosine * pitch_sine,
        )
    }

    fn up(&self) -> Vec3 {
        let (yaw_sine, yaw_cosine) = self.yaw.sin_cos();
        let (pitch_sine, pitch_cosine) = self.pitch.sin_cos();
        Vec3::new(
            yaw_sine * pitch_cosine,
            pitch_sine,
            -yaw_cosine * pitch_cosine,
        )
    }

    fn right(&self) -> Vec3 {
        let (yaw_sine, yaw_cosine) = self.yaw.sin_cos();
        Vec3::new(yaw_cosine, 0., yaw_sine)
    }

    /// Rotate the view direction by the given delta.
    fn turn(&mut self, delta: (f32, f32)) {
        self.yaw += delta.0;
        self.pitch -= delta.1;
    }

    /// Given a vector in view coordinates, returns the same vector in world coordinates.
    fn view_to_world(&self, vec: Vec3) -> Vec3 {
        self.right() * vec.x - self.up() * vec.y - self.forward() * vec.z
    }

    /// Given a vector in world coordinates, returns the same vector in view coordinates.
    fn world_to_view(&self, vec: Vec3) -> Vec3 {
        let (yaw_sine, yaw_cosine) = self.yaw.sin_cos();
        let (pitch_sine, pitch_cosine) = self.pitch.sin_cos();
        Vec3::new(
            yaw_cosine * vec.x + yaw_sine * vec.z,
            yaw_sine * pitch_sine * vec.x + pitch_cosine * vec.y - yaw_cosine * pitch_sine * vec.z,
            -yaw_sine * pitch_cosine * vec.x
                + pitch_sine * vec.y
                + yaw_cosine * pitch_cosine * vec.z,
        )
    }
}

impl Default for PrincipalAxes {
    fn default() -> Self {
        PrincipalAxes::new(0., PI * 0.5)
    }
}
