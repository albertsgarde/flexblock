use crate::game::{world::{Location, Terrain}, physics::AABB, View};
use glm::Vec3;
use serde::{Deserialize, Serialize};

macro_rules! PLAYER_SIZE {
    () => {Vec3::new(0.6, 1.8, 0.6)}
}
macro_rules! PLAYER_VIEW_LOC {
    () => {Vec3::new(0.3, 1.5, 0.3)}
}


#[derive(Deserialize, Serialize)]
pub struct Player {
    aabb: AABB,
    view: View,
}

impl Player {
    pub fn new(location: Location, view_direction: Vec3) -> Player {
        Player{aabb: AABB::new(location, PLAYER_SIZE!()), view: View::new(location + PLAYER_VIEW_LOC!(), view_direction, Vec3::new(0., 1., 0.))}
    }

    pub fn default () -> Player {
        Player::new(Location::origin(), Vec3::new(0., 0., -1.))
    }

    /// Moves the player not considering collision.
    pub fn translate(&mut self, vec: Vec3) {
        self.aabb.translate(vec);
        self.view.translate(vec);
    }

    /// Moves the player as far along the vector as possible without colliding with the terrain.
    pub fn collide_move(&mut self, vec: Vec3, terrain: &Terrain) {
        let move_distance = self.aabb.collide_distance(vec, terrain);
        self.translate(vec * move_distance.unwrap_or(1.));
    }

    /// Converts vec from view coordinates to world coordinates and moves the player as far along
    /// the resulting vector as possible without colliding with the terrain.
    pub fn collide_move_relative(&mut self, vec: Vec3, terrain: &Terrain) {
        let move_vec = self.view.view_to_world(vec);
        let move_distance = self.aabb.collide_distance(move_vec, terrain);
        let move_vec = move_vec * (move_distance.unwrap_or(1.) * 0.999);
        let move_vec = move_vec.map(|coord| if coord.abs() < 1e-5 {0.} else {coord});
        self.translate(move_vec);
    }

    /// Rotates the view direction along the great circle in the delta direction by |delta| radians.
    pub fn turn(&mut self, delta: (f32, f32)) {
        self.view.turn(delta);
    }

    pub fn view(&self) -> View {
        self.view.clone()
    }
}
