extern crate nalgebra_glm as glm;

pub mod pack;
mod wrapper;


//TODO: Move these things under here out of wrapper.
pub use wrapper::{ShaderIdentifier, ShaderMetadata, BufferTarget, ProgramType, TextureMetadata, InternalFormat, FramebufferIdentifier, FramebufferMetadata, RenderCaller};
pub use wrapper::gui::Gui;

mod external_event;
pub use external_event::ExternalEvent;

mod render_messages;
pub use render_messages::{RenderMessage, RenderMessages, UniformData, VertexPack};

mod capabilities;
pub use capabilities::GraphicsCapabilities;