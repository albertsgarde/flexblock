use crate::{
    audio::{AudioMessage, AudioMessageHandle},
    game::{world, GraphicsStateModel, Player, StateInputEvent},
};
use glm::Vec3;
use serde::{Deserialize, Serialize};

use super::world::VoxelType;

/// Holds the entire world state.
/// Everything that is part of the game is held within.
#[derive(Deserialize, Serialize)]
pub struct State {
    terrain: world::Terrain,
    player: Player,
    cur_tick: u64,
}

impl State {
    /// Initializes a state with no terrain and a default placed player.
    pub fn new() -> State {
        let mut state = State {
            terrain: world::Terrain::new(),
            player: Player::default(),
            cur_tick: 0,
        };
        state.terrain.set_voxel_type(
            world::Location::from_coords(3., 3., -8.),
            world::VoxelType(1),
        );
        state.terrain.set_voxel_type(
            world::Location::from_coords(19., 1., -8.),
            world::VoxelType(1),
        );
        state
    }

    /// Runs one game tick reacting to the given input events.
    ///
    /// # Arguments
    ///
    /// `_` - The input events received this tick.
    pub fn tick(&mut self, events: &[StateInputEvent], audio_message_handle: &AudioMessageHandle) {
        self.cur_tick += 1;
        for event in events {
            match *event {
                StateInputEvent::RotateView { delta } => self.player.turn(delta),
                StateInputEvent::MovePlayerRelative { delta } => self
                    .player
                    .collide_move_relative(delta * 0.05, &self.terrain),
                StateInputEvent::PlayerInteract1 => {
                    let point_at = self.terrain.trace_ray(
                        self.player.view().location(),
                        self.player.view().view_direction(),
                    );
                    if let Some(target) = point_at {
                        self.terrain.set_voxel_type(target, VoxelType(0));
                    }
                }
                StateInputEvent::PlayerInteract2 => {
                    let point_at = self.terrain.trace_ray(
                        self.player.view().location(),
                        self.player.view().view_direction(),
                    );
                    if let Some(target) = point_at {
                        let place_target = target + Vec3::new(1., 0., 0.);
                        if self.terrain.voxel_type(place_target) == VoxelType(0) {
                            self.terrain.set_voxel_type(place_target, VoxelType(1));
                            audio_message_handle.send_message(AudioMessage::StartSound(0));
                        }
                    }
                }
            }
        }
    }

    /// Updates the graphics model with any changes in the state.
    ///
    /// # Arguments
    ///
    /// `graphics_state_model` - A mutable reference to the model to update.
    pub fn update_graphics_state_model(&self, graphics_state_model: &mut GraphicsStateModel) {
        graphics_state_model.terrain = self.terrain.clone();
        graphics_state_model.view = self.player.view();
    }
}
