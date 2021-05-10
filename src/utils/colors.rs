use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ColorFormat {
    RGB,
    RGBA,
    /// 16 bit depth format
    D,
}

impl ColorFormat {
    /// Returns the GL texture type, but not the internal format.
    pub fn gl_format(&self) -> u32 {
        match self {
            ColorFormat::RGB => gl::RGB,
            ColorFormat::RGBA => gl::RGBA,
            ColorFormat::D => gl::DEPTH_COMPONENT,
        }
    }
}
