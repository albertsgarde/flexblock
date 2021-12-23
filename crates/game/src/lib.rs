mod state_input_event;
pub use state_input_event::InputEventHistory;
pub use state_input_event::StateInputEvent;

pub mod physics;

mod state;
pub use state::State;

pub mod view;
pub use view::View;

mod player;
pub use player::Player;

mod graphics_state_model;
pub use graphics_state_model::GraphicsStateModel;

extern crate nalgebra_glm as glm;

pub const TPS: u32 = 24;
pub const SECONDS_PER_TICK: f32 = 1. / (TPS as f32);
