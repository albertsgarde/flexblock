use std::collections::HashMap;

///TODO
///Contains the capabilities that the Graphics wrapper makes available to the packer.
pub struct GraphicsCapabilities {
    /// The number of avaliable VBOS in the 3d vertex array
    pub vbo_count: usize,
    /// A hashmap of strings and texture ids
    pub texture_names: HashMap<String, usize>,
}
