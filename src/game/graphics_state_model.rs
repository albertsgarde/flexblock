use crate::game::{world::Terrain, Player};

pub struct GraphicsStateModel {
    pub terrain: Terrain,
    pub player: Player
}

impl GraphicsStateModel {
    pub fn new() -> GraphicsStateModel {
        GraphicsStateModel {
            terrain: Terrain::new(),
            player: Player::default(),
        }
    }
}
