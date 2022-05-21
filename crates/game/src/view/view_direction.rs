use dyn_clone::DynClone;
use glm::Vec3;
use std::fmt::Debug;

#[typetag::serde(tag = "type")]
pub trait ViewDirection: DynClone + Send + Sync + Debug {
    fn forward(&self) -> Vec3;

    fn up(&self) -> Vec3;

    fn right(&self) -> Vec3;

    /// Rotate the view direction by the given delta.
    fn turn(&mut self, delta: (f32, f32));

    /// Given a vector in view coordinates, returns the same vector in world coordinates.
    fn view_to_world(&self, vec: Vec3) -> Vec3;

    /// Given a vector in world coordinates, returns the same vector in view coordinates.
    fn world_to_view(&self, vec: Vec3) -> Vec3;
}

dyn_clone::clone_trait_object!(ViewDirection);
