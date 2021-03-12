use crate::game::{world, InputEvent, GraphicsStateModel};

pub struct State {
    terrain: world::Terrain,
    cur_tick: u64,
}

impl State {
    pub fn new() -> State {
        State { terrain: world::Terrain::new() }
    }

    pub fn tick(&mut self, input_events: &[InputEvent]) {
        cur_tick += 1;
    }

    pub fn update_graphics_state_model(&self, graphics_state_model: &mut GraphicsStateModel) {
        graphics_state_model.terrain = terrain.clone();
    }
}
