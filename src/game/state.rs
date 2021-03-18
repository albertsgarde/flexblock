use crate::game::{world, GraphicsStateModel, InputEvent, View};
use serde::{Deserialize, Serialize};

/// Holds the entire world state.
/// Everything that is part of the game is held within.
#[derive(Deserialize, Serialize)]
pub struct State {
    terrain: world::Terrain,
    view: View,
    cur_tick: u64,
}

impl State {
    /// Initializes a state with no terrain and a default placed player.
    pub fn new() -> State {
        let mut state = State {
            terrain: world::Terrain::new(),
            view: View::new(world::Location::origin(), glm::Vec3::new(0., 0., -1.)),
            cur_tick: 0,
        };
        state.terrain.set_voxel_type(world::Location::from_coords(3., 3., -8.), world::VoxelType(1));
        state
    }

    /// Runs one game tick reacting to the given input events.
    ///
    /// # Arguments
    ///
    /// `_` - The input events received this tick.
    pub fn tick(&mut self, _: &[InputEvent]) {
        self.cur_tick += 1;
    }

    /// Updates the graphics model with any changes in the state.
    ///
    /// # Arguments
    ///
    /// `graphics_state_model` - A mutable reference to the model to update.
    pub fn update_graphics_state_model(&self, graphics_state_model: &mut GraphicsStateModel) {
        graphics_state_model.terrain = self.terrain.clone();
        graphics_state_model.view = self.view.clone();
    }
}
