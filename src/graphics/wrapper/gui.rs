use crate::graphics::{RenderMessage, RenderMessages, UniformData};

use super::ShaderIdentifier;

/// Constructs a RenderMessage from widgets
/// Note that you place widgets in the rectangle [0, canvas_width], [0, canvas_height]
/// y=0 is the top of the screen, and y=canvas_height is the bottom.
pub struct Gui {
    widgets: Vec<LocatedWidget>,
    settings: GuiSettings,
}

struct GuiSettings {
    canvas_width: f32,
    canvas_height: f32,
    x_zero: f32,
    y_zero: f32,
}

impl GuiSettings {
    pub fn x_scale(&self) -> f32 {
        1.0 / self.canvas_width
    }
    pub fn y_scale(&self) -> f32 {
        1.0 / self.canvas_height
    }
}

pub struct LocatedWidget {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    widget: Box<dyn Widget>,
}

trait Widget {
    fn render_messages(
        &self,
        location: &LocatedWidget,
        settings: &GuiSettings,
    ) -> Vec<RenderMessage>;
}

impl Gui {
    pub fn new(canvas_dimensions: (f32, f32), zero_point: (f32, f32)) -> Self {
        Self {
            widgets: Vec::new(),
            settings: GuiSettings {
                canvas_width: canvas_dimensions.0,
                canvas_height: canvas_dimensions.1,
                x_zero: zero_point.0,
                y_zero: zero_point.1,
            },
        }
    }

    /// Removes all widgets from the GUI
    pub fn reset_gui(&mut self) {
        self.widgets.clear();
    }

    /// Adds a text widghet to the guy in the given location, with given scale, width, and height.
    pub fn add_text(
        &mut self,
        text: &str,
        location: (f32, f32),
        width: f32,
        height: f32,
        scale: f32,
    ) {
        self.widgets.push(LocatedWidget {
            x: location.0,
            y: location.1,
            width,
            height,
            widget: Box::new(widgets::Text::new(text.to_owned(), scale)),
        })
    }

    pub fn collect_render_messages(&self) -> RenderMessages {
        let mut result = RenderMessages::new();
        result.add_message(RenderMessage::SwitchTo2D {});
        result.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Gui,
        });
        let mut ud = UniformData::new();
        ud.texture(String::from("/gui.png"), "tex");
        ud.vec2(
            glm::vec2(self.settings.x_scale() * 2.0, self.settings.y_scale() * 2.0),
            "scale",
        );
        result.add_message(RenderMessage::Uniforms {
            uniforms: Box::new(ud),
        });
        for lw in &self.widgets {
            for message in lw.widget.render_messages(lw, &self.settings) {
                result.add_message(message)
            }
        }

        result
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
        scale: f32,
    }

    impl Text {
        pub fn new(text: String, scale: f32) -> Self {
            Self { text, scale }
        }

        fn char_to_index(char: char) -> u32 {
            let i = char as u32;
            match i {
                65..=96 => i - 39,
                97..=123 => i - 97,
                _ => match char {
                    'æ' => 52,
                    'ø' => 53,
                    'å' => 54,
                    'Æ' => 55,
                    'Ø' => 56,
                    'Å' => 57,
                    '.' => 58,
                    '!' => 59,
                    '?' => 60,
                    ',' => 61,
                    '-' => 62,
                    '+' => 63,
                    '(' => 64,
                    ')' => 65,
                    '[' => 66,
                    ']' => 67,
                    '{' => 68,
                    '}' => 69,
                    '/' => 70,
                    '\\' => 71,
                    '&' => 72,
                    '#' => 73,
                    '$' => 74,
                    '%' => 75,
                    '=' => 76,
                    '_' => 77,
                    '\'' => 78,
                    '"' => 79,
                    ':' => 80,
                    ';' => 81,
                    '>' => 82,
                    '<' => 83,
                    '|' => 84,
                    '~' => 85,
                    '0' => 86,
                    '1' => 87,
                    '2' => 88,
                    '3' => 89,
                    '4' => 90,
                    '5' => 91,
                    '6' => 92,
                    '7' => 93,
                    '8' => 94,
                    '9' => 95,
                    ' ' => 96,
                    _ => {
                        panic!(
                            "Invalid char {}, integer value {}, supplied for gui text object!",
                            char, char as u32
                        )
                    }
                },
            }
        }
    }

    impl Widget for Text {
        fn render_messages(
            &self,
            location: &LocatedWidget,
            _settings: &GuiSettings,
        ) -> Vec<RenderMessage> {
            let mut result = Vec::new();

            let mut vertices: Vec<Vertex3D> = Vec::with_capacity(self.text.len() * 4);
            let mut indices: Vec<u32> = Vec::with_capacity(self.text.len() * 6);

            let mut x = 0;
            let mut y = 0;
            let mut idx = 0;
            for char in self.text.chars() {
                let i = Text::char_to_index(char);

                let xl = location.x + x as f32 * self.scale;
                let yl = location.y + y as f32 * self.scale;
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
                    x: xl + self.scale,
                    y: yl,
                    z: 0.0,
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    u: u + 1. / 16.0,
                    v,
                });

                vertices.push(Vertex3D {
                    x: xl + self.scale,
                    y: yl + self.scale,
                    z: 0.0,
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    u: u + 1. / 16.0,
                    v: v + 1. / 16.0,
                });
                vertices.push(Vertex3D {
                    x: xl,
                    y: yl + self.scale,
                    z: 0.0,
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    u,
                    v: v + 1. / 16.0,
                });
                indices.push(idx + 0);
                indices.push(idx + 1);
                indices.push(idx + 2);
                indices.push(idx + 0);
                indices.push(idx + 2);
                indices.push(idx + 3);

                idx += 4;
                x += 1;
                if x as f32 * self.scale > location.width {
                    y += 1;
                    if y as f32 * self.scale > location.height {
                        break;
                    }
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
