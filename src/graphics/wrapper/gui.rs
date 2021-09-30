use crate::graphics::RenderMessage;

/// Constructs a RenderMessage from widgets
/// Locations are in the [0,1] square
pub struct Gui {
    widgets: Vec<LocatedWidget>,
}

pub struct LocatedWidget {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    widget: Box<dyn Widget>,
}

pub trait Widget {
    fn render_messages(&self, location: &LocatedWidget) -> Vec<RenderMessage>;
}

impl Gui {
    pub fn add_text(&mut self, text: &str, location: glm::Vec2) {
        self.widgets.push(
            LocatedWidget {
                x : location.x, y : location.y, width : 100.0, height : 100.0, widget : Box::new(widgets::Text::new(text.to_owned()))
            }
        )
    }
}

mod widgets {
    use crate::{
        graphics::{wrapper::BufferTarget, VertexPack},
        utils::Vertex3D,
    };

    use super::*;
    pub struct Text {
        text: String,
    }

    impl Text {
        pub fn new(text : String) -> Self {
            Self { text }
        }
    }

    impl Widget for Text {
        fn render_messages(&self, location: &LocatedWidget) -> Vec<RenderMessage> {
            let mut result = Vec::new();

            let mut vertices: Vec<Vertex3D> = Vec::with_capacity(self.text.len() * 4);
            let mut indices: Vec<u32> = Vec::with_capacity(self.text.len() * 6);

            let mut x = 0;
            let mut y = 0;
            let mut idx = 0;
            for char in self.text.chars() {
                let i = char as u32;
                println!("{} = {}", char, i);
                if i >= 256 {
                    println!("{} is not ascii!", char);
                }

                let xl = location.x + x as f32;
                let yl = location.y + y as f32;
                let u = (i % 16) as f32 / 16.0;
                let v = (i / 16) as f32 / 16.0;
                vertices.push(Vertex3D {
                    x: xl,
                    y: yl,
                    z: 0.0,
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    u,
                    v,
                });
                vertices.push(Vertex3D {
                    x: xl + 1.0,
                    y: yl,
                    z: 0.0,
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    u: u + 1. / 16.0,
                    v,
                });

                vertices.push(Vertex3D {
                    x: xl + 1.0,
                    y: yl + 1.0,
                    z: 0.0,
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    u: u + 1. / 16.0,
                    v: v + 1. / 16.0,
                });
                vertices.push(Vertex3D {
                    x: xl,
                    y: yl + 1.0,
                    z: 0.0,
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    u,
                    v: v + 1. / 16.0,
                });
                indices.push(idx+0);
                indices.push(idx+1);
                indices.push(idx+2);
                indices.push(idx+0);
                indices.push(idx+2);
                indices.push(idx+3);

                idx += 4;
                x += 1;
                if (x * 16) as f32 > location.width {
                    y += 1;
                    x = 0;
                }
            }

            result.push(RenderMessage::Pack {
                buffer: BufferTarget::GuiBuffer,
                pack: VertexPack {
                    vertices,
                    elements: indices,
                },
            });
            result.push(RenderMessage::Draw {
                buffer: BufferTarget::GuiBuffer,
            });

            result
        }
    }
}
