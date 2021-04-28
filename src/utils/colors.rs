use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorFormat {
    RGB,
    RGBA,
}

impl ColorFormat {
    /// Returns the number of bytes that each texel in the texture takes
    pub fn bytes(&self) -> usize {
        match self {
            &ColorFormat::RGB => 3,
            &ColorFormat::RGBA => 4,
        }
    }

    pub fn gl_format(&self) -> u32 {
        match self {
            &ColorFormat::RGB => gl::RGB,
            &ColorFormat::RGBA => gl::RGBA,
        }
    }
}
