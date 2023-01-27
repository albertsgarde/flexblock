use crate::View;
use world::Terrain;
pub struct GraphicsStateModel {
    pub terrain: Terrain,
    pub view: View,
}

impl GraphicsStateModel {
    pub fn new() -> GraphicsStateModel {
        GraphicsStateModel {
            terrain: Terrain::new(),
            view: View::default(),
        }
    }
}

impl Default for GraphicsStateModel {
    fn default() -> Self {
        Self::new()
    }
}
