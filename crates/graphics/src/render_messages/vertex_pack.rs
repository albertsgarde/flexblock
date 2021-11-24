use std::fmt;
use utils::Vertex3D;

#[derive(Clone)]
pub struct VertexPack {
    pub vertices: Vec<Vertex3D>,
    pub elements: Vec<u32>,
}
impl VertexPack {
    ///TODO: Make this follow the contract
    pub fn new(vertices: Vec<Vertex3D>, elements: Option<Vec<u32>>) -> VertexPack {
        let elements = match elements {
            Some(e) => e,
            None => Vec::new(),
        };
        VertexPack { vertices, elements }
    }
}

impl fmt::Debug for VertexPack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("VertexPack")
    }
}
