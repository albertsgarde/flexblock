use crate::game::world::Terrain;

pub struct GraphicsStateModel {
    pub terrain: Terrain,
}

impl GraphicsStateModel {
    pub fn new() -> GraphicsStateModel {
        GraphicsStateModel {
            terrain: Terrain::new(),
        }
    }
}
