use crate::game::{
    physics::{Aabb, PhysicsBody},
    world::{Location, Terrain},
    View,
};
use glm::Vec3;
use serde::{Deserialize, Serialize};

macro_rules! PLAYER_SIZE {
    () => {
        Vec3::new(0.6, 1.8, 0.6)
    };
}
macro_rules! PLAYER_VIEW_LOC {
    () => {
        Vec3::new(0.3, 1.5, 0.3)
    };
}

#[derive(Deserialize, Serialize)]
pub struct Player {
    physics_body: PhysicsBody,
    view: View,
}

impl Player {
    pub fn new(location: Location, view_direction: Vec3) -> Player {
        Player {
            physics_body: PhysicsBody::new(Aabb::new(location, PLAYER_SIZE!())),
            view: View::new(
                location + PLAYER_VIEW_LOC!(),
                view_direction,
                Vec3::new(0., 1., 0.),
            ),
        }
    }

    pub fn default() -> Player {
        Player::new(Location::origin(), Vec3::new(0., 0., -1.))
    }

    /// Moves the player not considering collision.
    pub fn translate(&mut self, vec: Vec3) {
        self.physics_body.translate(vec);
        self.view.translate(vec);
    }

    /// Converts vec from view coordinates to world coordinates and moves the player as far along
    /// the resulting vector as possible without colliding with the terrain.
    pub fn collide_move_relative(&mut self, vec: Vec3, terrain: &Terrain) {
        let move_vec = self.view.view_to_world(vec);
        self.physics_body.collide_move(move_vec, terrain);
        self.view
            .teleport(self.physics_body.location() + PLAYER_VIEW_LOC!());
    }

    /// Converts vec from view coordinates to world coordinates, removes the y-component
    /// normalizes to the same length as the original vec and moves the player as far
    /// along the resulting vector as possible without colliding with terrain.
    pub fn collide_move_relative_horizontal(&mut self, vec: Vec3, terrain: &Terrain) {
        let mut move_vec = self.view.view_to_world(vec);
        move_vec.y = 0.;
        let move_vec = move_vec
            .normalize()
            .map(|coord| if coord.is_nan() { 0. } else { coord })
            * vec.norm();
        self.physics_body.collide_move(move_vec, terrain);
        self.view
            .teleport(self.physics_body.location() + PLAYER_VIEW_LOC!());
    }

    pub fn add_velocity(&mut self, vec: Vec3) {
        self.physics_body.add_velocity(vec)
    }

    /// Rotates the view direction along the great circle in the delta direction by |delta| radians.
    pub fn turn(&mut self, delta: (f32, f32)) {
        self.view.turn(delta);
    }

    pub fn view(&self) -> View {
        self.view.clone()
    }

    pub fn tick(&mut self, terrain: &Terrain) {
        self.physics_body.tick(terrain);
        self.view
            .teleport(self.physics_body.location() + PLAYER_VIEW_LOC!());
    }
}
