use crate::game::{world, GraphicsStateModel, InputEvent};

pub struct State {
    terrain: world::Terrain,
    cur_tick: u64,
}

impl State {
    pub fn new() -> State {
        State {
            terrain: world::Terrain::new(),
            cur_tick: 0,
        }
    }

    pub fn tick(&mut self, _: &[InputEvent]) {
        self.cur_tick += 1;
    }

    pub fn update_graphics_state_model(&self, graphics_state_model: &mut GraphicsStateModel) {
        graphics_state_model.terrain = self.terrain.clone();
    }
}
