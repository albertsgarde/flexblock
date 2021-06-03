use crate::{
    audio::{AudioMessage, AudioMessageHandle, Listener},
    game::{
        world::{self, Location, Terrain},
        GraphicsStateModel, Player, StateInputEvent,
    },
};
use glm::Vec3;
use serde::{Deserialize, Serialize};

use super::world::VoxelType;

/// Holds the entire world state.
/// Everything that is part of the game is held within.
#[derive(Deserialize, Serialize)]
pub struct State {
    terrain: Terrain,
    player: Player,
    cur_tick: u64,
}

impl State {
    /// Initializes a state with no terrain and a default placed player.
    pub fn new() -> State {
        let mut state = State {
            terrain: Terrain::new(),
            player: Player::default(),
            cur_tick: 0,
        };
        for x in -128..128 {
            for z in -128..128 {
                state.terrain.set_voxel_type(
                    Location::from_coords(x as f32, -1., z as f32),
                    world::VoxelType(1),
                );
            }
        }
        state
            .terrain
            .set_voxel_type(Location::from_coords(4., 0., -4.), world::VoxelType(1));
        state
    }

    pub fn terrain(&self) -> &Terrain {
        &self.terrain
    }

    /// Runs one game tick reacting to the given input events.
    ///
    /// # Arguments
    ///
    /// `_` - The input events received this tick.
    pub fn tick(&mut self, events: &[StateInputEvent], audio_message_handle: &AudioMessageHandle) {
        self.cur_tick += 1;
        self.handle_events(events, audio_message_handle);

        self.player.tick(&self.terrain);

        audio_message_handle
            .send_message(AudioMessage::Listener(Listener::from_player(&self.player)));
    }

    fn handle_events(
        &mut self,
        events: &[StateInputEvent],
        audio_message_handle: &AudioMessageHandle,
    ) {
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
                    if let Some((distance, point_at)) = self.terrain.trace_ray_with_position(
                        self.player.view().location(),
                        self.player.view().view_direction(),
                    ) {
                        let target = point_at
                            + (self.player.view().location()
                                + self.player.view().view_direction() * (distance + 1e-4))
                                .vec_to_nearest_other_voxel();
                        if self.terrain.voxel_type(target) == VoxelType(0) {
                            self.terrain.set_voxel_type(target, VoxelType(1));
                            audio_message_handle
                                .send_message(AudioMessage::StartSound(0, Some(target)));
                        }
                    }
                }
                StateInputEvent::Jump => self.player.add_velocity(Vec3::new(0., 2., 0.)),
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
