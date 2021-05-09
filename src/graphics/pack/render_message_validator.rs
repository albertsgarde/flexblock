use super::RenderState;
use crate::graphics::{
    wrapper::ShaderMetadata, GraphicsCapabilities, RenderMessage, RenderMessages, UniformData,
};
use std::fmt;

///Types of validator errors.
pub enum ValidationErrorType {
    InvalidShader { shader: String },
    VBOOutOfBounds { vbo: usize },
    ClearEmptyVBO { vbo: usize },
    DrawEmptyVBO { vbo: usize },
    PackFullVBO { vbo: usize },
    NoClearedBuffers,
    NoRenderTarget,
    NoShaderChosen,
    InvalidFramebuffer { framebuffer: String },
    WrongTriangleCount,
    EmptyVertexPack,
    InvalidTexture { texture: String },
    UnwantedUniform { uniform: String },
    UnfilledUniform { uniforms: Vec<String> },
    NoGraphicsCapabilities,
    NonComputeShader { shader: String },
    NonGraphicsShader { shader: String },
}
/// Context for an error from the validator (What are the render messages, which is the current message, yadayadayada)
pub struct ValidationContext<'a> {
    pub render_messages: &'a RenderMessages,
    pub message_index: usize,
}
/// An error from the validator.
/// Contains both its type and a context for the error.
///
pub struct ValidationError<'a> {
    pub error_type: ValidationErrorType,
    pub context: ValidationContext<'a>,
}

impl<'a> fmt::Debug for ValidationError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = match &self.error_type {
            ValidationErrorType::InvalidShader { shader } => format!("Trying to choose shader {}, that does not exist!", shader),
            ValidationErrorType::VBOOutOfBounds {vbo} => format!("Trying to access VBO {}, which is out of bounds.", vbo),
            ValidationErrorType::ClearEmptyVBO {vbo} => format!("Trying to clear VBO {}, which is already empty!", vbo),
            ValidationErrorType::NoClearedBuffers => format!("A clear buffers render message is sent, but no buffers are cleared!"),
            ValidationErrorType::NoRenderTarget => format!("Trying to draw without first picking a render target!"),
            ValidationErrorType::UnfilledUniform {uniforms } => format!("Trying to draw without supplying all needed uniforms! Missing uniforms: {:?}", uniforms),
            ValidationErrorType::NoShaderChosen => format!("Trying to draw without picking a shader first!"),
            ValidationErrorType::DrawEmptyVBO {vbo} => format!("Trying to draw VBO {}, which is empty!", vbo),
            ValidationErrorType::WrongTriangleCount => format!("Received a vertex pack that does not contain a whole number of triangles!"),
            ValidationErrorType::EmptyVertexPack => format!("Trying to fill VBO with empty vertex pack! A VBO is cleared by sending a RenderMessage::ClearArray message!"),
            ValidationErrorType::PackFullVBO {vbo}=> format!("Trying to pack VBO {}, which is already full!", vbo),
            ValidationErrorType::InvalidFramebuffer {framebuffer} => format!("Trying to bind framebuffer {}, which does not exist", framebuffer),
            ValidationErrorType::InvalidTexture {texture} => format!("Trying to pass texture {} as shader uniform, but texture does not exist in the graphics capabilities object!", texture),
            ValidationErrorType::UnwantedUniform {uniform} => format!("Trying to pass uniform {} to a shader that does not want it! (This is non-critical, should maybe be a warning instead)", uniform),
            ValidationErrorType::NoGraphicsCapabilities => format!("Trying to send RenderMessages with no graphics capabilities!"),
            ValidationErrorType::NonComputeShader {shader}=> format!("Trying to run a compute dispatch with graphics shader {}!", shader),
            ValidationErrorType::NonGraphicsShader {shader}=> format!("Trying to render with compute shader {}!", shader),
        };
        res.push_str("\n");
        let mut counter = 0;
        for message in self.context.render_messages.iter() {
            if counter == self.context.message_index {
                res.push_str(&format!("\n{:?} -- ERROR HERE\n\n", message));
                break;
            } else {
                res.push_str(&format!("{:?}|", message));
            }
            counter += 1;
        }
        res.push_str(&format!("{:?}", self.context.render_messages));
        f.write_str(&res)
    }
}

/// Validates that a set of render messages are legal.
/// Only used for debug
pub struct RenderMessageValidator {
    /// Which vbos are currently packed as far as the validator knows (note that this doesn't update until validate is called)
    packed_vbos: Vec<bool>,
}

impl RenderMessageValidator {
    pub fn new() -> RenderMessageValidator {
        RenderMessageValidator {
            packed_vbos: Vec::new(),
        }
    }

    /// Captures context for a ValidationError so it can give a more detailed report.
    pub fn capture_context<'a>(
        &self,
        _state: &RenderState,
        messages: &'a RenderMessages,
        _chosen_shader: Option<&ShaderMetadata>,
        _has_render_target: bool,
        _bound_uniforms: Vec<String>,
        message_index: usize,
    ) -> ValidationContext<'a> {
        ValidationContext {
            render_messages: messages,
            message_index,
        }
    }

    /// Validates that this contains only allowed render messages in an allowed order
    /// Note that this cannot be turned on in the middle of the program; it is stateful (since vbos are packed and unpacked.)
    pub fn validate<'a>(
        &mut self,
        state: &RenderState,
        messages: &'a RenderMessages,
    ) -> Result<(), ValidationError<'a>> {
        let verbose = false;
        if self.packed_vbos.len() < state.packed_chunks.len() {
            self.packed_vbos.append(&mut vec![
                false;
                state.packed_chunks.len() - self.packed_vbos.len()
            ]);
        }

        if verbose {
            println!(
                "Validating render message pack with {} messages!",
                messages.size()
            );
        }

        if let Some(capabilities) = &state.capabilities {
            //What shader is currently chosen
            let mut chosen_shader = None;
            // Whether a render target(framebuffer) has currently been chosen
            let mut has_render_target = false;
            // What uniforms are currently bound for the current shader
            let mut bound_uniforms = Vec::new();
            // A hashmap of shader metadata
            let shader_metadata = &capabilities.shader_metadata;

            let mut message_index = 0;

            for message in messages.iter() {
                match message {
                    RenderMessage::ChooseShader { shader } => {
                        chosen_shader = Some(&shader_metadata[*shader as usize]);
                        bound_uniforms = Vec::new();

                        if verbose {
                            println!("Choosing shader {}", shader.name());
                        }
                    }
                    RenderMessage::ClearArray { buffer } => {
                        if *buffer >= capabilities.vbo_count {
                            return Err(ValidationError {
                                error_type: ValidationErrorType::VBOOutOfBounds { vbo: *buffer },
                                context: self.capture_context(
                                    state,
                                    messages,
                                    chosen_shader,
                                    has_render_target,
                                    bound_uniforms,
                                    message_index,
                                ),
                            });
                        }

                        if message_index >= messages.old_new_split_index() {
                            if !self.packed_vbos[*buffer] {
                                return Err(ValidationError {
                                    error_type: ValidationErrorType::ClearEmptyVBO { vbo: *buffer },
                                    context: self.capture_context(
                                        state,
                                        messages,
                                        chosen_shader,
                                        has_render_target,
                                        bound_uniforms,
                                        message_index,
                                    ),
                                });
                            }
                            self.packed_vbos[*buffer] = false;
                        }

                        if verbose {
                            println!("Clearing VBO {}", buffer);
                        }
                    }
                    RenderMessage::ClearBuffers {
                        color_buffer,
                        depth_buffer,
                    } => {
                        if !color_buffer && !depth_buffer {
                            return Err(ValidationError {
                                error_type: ValidationErrorType::NoClearedBuffers,
                                context: self.capture_context(
                                    state,
                                    messages,
                                    chosen_shader,
                                    has_render_target,
                                    bound_uniforms,
                                    message_index,
                                ),
                            });
                        }
                        if verbose {
                            println!(
                                "Clearing color? {} and depth? {}",
                                color_buffer, depth_buffer
                            );
                        }
                    }
                    RenderMessage::Draw { buffer } => {
                        if !has_render_target {
                            return Err(ValidationError {
                                error_type: ValidationErrorType::NoRenderTarget,
                                context: self.capture_context(
                                    state,
                                    messages,
                                    chosen_shader,
                                    has_render_target,
                                    bound_uniforms,
                                    message_index,
                                ),
                            });
                        }

                        if let Some(s) = chosen_shader {
                            if s.identifier.is_compute() {
                                return Err(ValidationError {
                                    error_type: ValidationErrorType::NonGraphicsShader {
                                        shader: String::from(s.identifier.name()),
                                    },
                                    context: self.capture_context(
                                        state,
                                        messages,
                                        chosen_shader,
                                        has_render_target,
                                        bound_uniforms,
                                        message_index,
                                    ),
                                });
                            }

                            for uniform in &s.required_uniforms {
                                if !bound_uniforms.contains(&uniform.0) {
                                    return Err(ValidationError {
                                        error_type: ValidationErrorType::UnfilledUniform {
                                            uniforms: (&s.required_uniforms)
                                                .into_iter()
                                                .map(|x| String::from(&x.0))
                                                .filter(|x| !bound_uniforms.contains(&x))
                                                .collect::<Vec<String>>(),
                                        },
                                        context: self.capture_context(
                                            state,
                                            messages,
                                            chosen_shader,
                                            has_render_target,
                                            bound_uniforms,
                                            message_index,
                                        ),
                                    });
                                }
                            }
                        } else {
                            return Err(ValidationError {
                                error_type: ValidationErrorType::NoShaderChosen,
                                context: self.capture_context(
                                    state,
                                    messages,
                                    chosen_shader,
                                    has_render_target,
                                    bound_uniforms,
                                    message_index,
                                ),
                            });
                        }

                        // This one doesn't need to check whether it is above the old/new split, since draw is not a persistent render message.
                        // So it will always be in the new part.
                        if !self.packed_vbos[*buffer] {
                            return Err(ValidationError {
                                error_type: ValidationErrorType::DrawEmptyVBO { vbo: *buffer },
                                context: self.capture_context(
                                    state,
                                    messages,
                                    chosen_shader,
                                    has_render_target,
                                    bound_uniforms,
                                    message_index,
                                ),
                            });
                        }

                        if verbose {
                            println!("Drawing buffer {}", buffer);
                        }
                    }
                    RenderMessage::Pack { buffer, pack } => {
                        if *buffer >= capabilities.vbo_count {
                            return Err(ValidationError {
                                error_type: ValidationErrorType::VBOOutOfBounds { vbo: *buffer },
                                context: self.capture_context(
                                    state,
                                    messages,
                                    chosen_shader,
                                    has_render_target,
                                    bound_uniforms,
                                    message_index,
                                ),
                            });
                            // Trying to pack a VBO out of bounds
                        }

                        if pack.elements.len() % 3 != 0
                            || (pack.elements.len() == 0 && pack.vertices.len() % 3 != 0)
                        {
                            return Err(ValidationError {
                                error_type: ValidationErrorType::WrongTriangleCount,
                                context: self.capture_context(
                                    state,
                                    messages,
                                    chosen_shader,
                                    has_render_target,
                                    bound_uniforms,
                                    message_index,
                                ),
                            });
                        }
                        if pack.vertices.len() == 0 {
                            return Err(ValidationError {
                                error_type: ValidationErrorType::EmptyVertexPack,
                                context: self.capture_context(
                                    state,
                                    messages,
                                    chosen_shader,
                                    has_render_target,
                                    bound_uniforms,
                                    message_index,
                                ),
                            });
                        }

                        if message_index >= messages.old_new_split_index() {
                            if self.packed_vbos[*buffer] {
                                return Err(ValidationError {
                                    error_type: ValidationErrorType::PackFullVBO { vbo: *buffer },
                                    context: self.capture_context(
                                        state,
                                        messages,
                                        chosen_shader,
                                        has_render_target,
                                        bound_uniforms,
                                        message_index,
                                    ),
                                });
                            }
                            self.packed_vbos[*buffer] = true;
                        }

                        if verbose {
                            println!("filling VBO {}", buffer);
                        }
                    }
                    RenderMessage::Uniforms { uniforms } => {
                        match RenderMessageValidator::validate_uniforms(
                            uniforms,
                            &chosen_shader,
                            &mut bound_uniforms,
                            capabilities,
                        ) {
                            Ok(()) => {}
                            Err(e) => {
                                return Err(ValidationError {
                                    error_type: e,
                                    context: self.capture_context(
                                        state,
                                        messages,
                                        chosen_shader,
                                        has_render_target,
                                        bound_uniforms,
                                        message_index,
                                    ),
                                })
                            }
                        }
                        if verbose {
                            println!("Filling in uniforms");
                        }
                    }
                    RenderMessage::ChooseFramebuffer { framebuffer: _ } => {
                        /*if let Some(target) = framebuffer {
                            if !capabilities.framebuffer_metadata.contains_key(target) {

                            return Err(ValidationError {
                                error_type : ValidationErrorType::InvalidFramebuffer {framebuffer : String::from(target)},
                                context : self.capture_context(state, messages, chosen_shader, has_render_target, bound_uniforms, message_index)
                            })
                            }
                        }*/
                        has_render_target = true;
                    }
                    RenderMessage::Compute {
                        output_texture,
                        dimensions: _,
                    } => {
                        //TODO: ENFORCE TEXTURE FORMAT FIT

                        if !capabilities.texture_metadata.contains_key(output_texture) {
                            return Err(ValidationError {
                                error_type: ValidationErrorType::InvalidTexture {
                                    texture: String::from(output_texture),
                                },
                                context: self.capture_context(
                                    state,
                                    messages,
                                    chosen_shader,
                                    has_render_target,
                                    bound_uniforms,
                                    message_index,
                                ),
                            });
                        }

                        if let Some(s) = chosen_shader {
                            if !s.identifier.is_compute() {
                                return Err(ValidationError {
                                    error_type: ValidationErrorType::NonComputeShader {
                                        shader: String::from(s.identifier.name()),
                                    },
                                    context: self.capture_context(
                                        state,
                                        messages,
                                        chosen_shader,
                                        has_render_target,
                                        bound_uniforms,
                                        message_index,
                                    ),
                                });
                            }

                            for uniform in &s.required_uniforms {
                                if !bound_uniforms.contains(&uniform.0) {
                                    return Err(ValidationError {
                                        error_type: ValidationErrorType::UnfilledUniform {
                                            uniforms: (&s.required_uniforms)
                                                .into_iter()
                                                .map(|x| String::from(&x.0))
                                                .filter(|x| !bound_uniforms.contains(&x))
                                                .collect::<Vec<String>>(),
                                        },
                                        context: self.capture_context(
                                            state,
                                            messages,
                                            chosen_shader,
                                            has_render_target,
                                            bound_uniforms,
                                            message_index,
                                        ),
                                    });
                                }
                            }
                        }
                    }
                }
                message_index += 1;
            }

            Ok(())
        } else {
            if messages.size() > 0 {
                return Err(ValidationError {
                    error_type: ValidationErrorType::NoGraphicsCapabilities,
                    context: ValidationContext {
                        render_messages: messages,
                        message_index: 0,
                    },
                });
                //Err("Trying to send render messages when no graphics capabilities object is available!")
            } else {
                Ok(())
            }
        }
    }

    /// Validate a single RenderMessage::Uniforms
    fn validate_uniforms(
        uniforms: &UniformData,
        chosen_shader: &Option<&ShaderMetadata>,
        bound_uniforms: &mut Vec<String>,
        capabilities: &GraphicsCapabilities,
    ) -> Result<(), ValidationErrorType> {
        //TODO: Enforce uniform type matching to shader known type

        if let Some(s) = &chosen_shader {
            // Test if every passed texture exists in the graphics capabilities.
            for entry in &uniforms.texture {
                if !capabilities.texture_metadata.contains_key(&entry.0) {
                    return Err(ValidationErrorType::InvalidTexture {
                        texture: String::from(&entry.0),
                    });
                }
            }

            // Test if the shader wants every uniform passed to it
            // (TODO: This may be overzealous, maybe you should be let off with a warning.)
            for uniform in uniforms.get_uniform_locations() {
                let req_uniforms = &s.required_uniforms;
                let mut contained = false;
                for req_uni in req_uniforms {
                    if req_uni.0 == *uniform {
                        contained = true
                    }
                }

                if !contained {
                    return Err(ValidationErrorType::UnwantedUniform {
                        uniform: String::from(uniform),
                    });
                }

                if !bound_uniforms.contains(&uniform) {
                    bound_uniforms.push(String::from(uniform));
                }
            }
        } else {
            return Err(ValidationErrorType::NoShaderChosen {});
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{RenderMessageValidator, RenderState};
    use crate::graphics::wrapper::{
        ProgramType, ShaderIdentifier, ShaderMetadata, TextureMetadata,
    };
    use crate::graphics::{wrapper::InternalFormat, GraphicsCapabilities};
    use crate::graphics::{RenderMessage, RenderMessages, UniformData, VertexPack};
    use crate::utils::ColorFormat;
    use std::collections::HashMap;

    fn create_shader_metadata(extra_uniform: bool) -> Vec<ShaderMetadata> {
        let mut res = Vec::new();
        let mut required_uniforms = vec![(String::from("test_texture"), String::from(""))];
        if extra_uniform {
            required_uniforms.push((String::from("vector"), String::from("")));
        }
        let s1 = ShaderMetadata {
            identifier: ShaderIdentifier::Default,
            required_uniforms,
            shader_type: ProgramType::Graphics,
        };
        res.push(s1);

        res
    }

    /// Creates a basic render state that has a capabilities object with:
    ///  - one shader (s1) that needs one uniform (test_texture)
    ///  - one texture (atlas)
    ///  - 100 vbos
    /// If extra_uniform is supplied
    ///  - another uniform, of type vec4, with name "vector"
    fn create_render_state(extra_uniform: bool) -> RenderState {
        let mut rs = RenderState::new();

        let shader_metadata = create_shader_metadata(extra_uniform);
        let mut texture_metadata = HashMap::new();
        texture_metadata.insert(
            String::from("atlas"),
            TextureMetadata {
                format: ColorFormat::RGB,
                internal_format: InternalFormat::RGB8,
                width: 2,
                height: 2,
                name: String::from("atlas"),
                screen_dependant_dimensions: false,
            },
        );
        let framebuffer_metadata = Vec::new();
        rs.update_capabilities(GraphicsCapabilities {
            vbo_count: 100,
            texture_metadata,
            shader_metadata,
            framebuffer_metadata,
            screen_dimensions: (2, 2),
        });

        rs
    }

    ///Creates a VertexPack for a basic quad
    fn create_quad_pack() -> VertexPack {
        let mut vertices = Vec::new();
        let mut elements = Vec::new();
        let x0 = 0.;
        let x1 = x0 + 1.;
        let y0 = 0.;
        let y1 = y0 + 1.;
        let z0 = 0.;

        // Back face
        let (mut vadd, mut eadd) =
            super::super::cube_faces::back(z0, x0, y0, x1, y1, 1., 0., 0., 0);
        vertices.append(&mut vadd);
        elements.append(&mut eadd);
        let vertex_pack = VertexPack::new(vertices, Some(elements));
        vertex_pack
    }

    #[test]
    fn basic_validation() {
        // Does extremely basic validation
        let mut rs = create_render_state(false);

        let mut render_messages = RenderMessages::new();
        let mut validator = RenderMessageValidator::new();

        render_messages.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Default,
        });
        let mut ud = UniformData::new();
        ud.texture(String::from("atlas"), String::from("test_texture"));
        render_messages.add_message(RenderMessage::Uniforms { uniforms: ud });
        render_messages.add_message(RenderMessage::Pack {
            buffer: 0,
            pack: create_quad_pack(),
        });

        assert!(validator.validate(&mut rs, &render_messages).unwrap() == ());
    }

    #[test]
    fn texture_name_validation() {
        // Ensures that we can only use textures that exist in the graphics capabilities object
        let mut rs = create_render_state(false);

        let mut render_messages = RenderMessages::new();
        let mut validator = RenderMessageValidator::new();
        render_messages.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Default,
        });
        let mut ud = UniformData::new();
        ud.texture(String::from("atlas"), String::from("test"));
        render_messages.add_message(RenderMessage::Uniforms { uniforms: ud });
        render_messages.add_message(RenderMessage::Pack {
            buffer: 0,
            pack: create_quad_pack(),
        });

        assert!(
            validator.validate(&mut rs, &render_messages).is_err(),
            "Validate wrongfully accepts non-existent uniform name!"
        );
        let mut render_messages = RenderMessages::new();
        let mut validator = RenderMessageValidator::new();
        render_messages.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Default,
        });
        let mut ud = UniformData::new();
        ud.texture(String::from("atlass"), String::from("test_texture"));
        render_messages.add_message(RenderMessage::Uniforms { uniforms: ud });
        render_messages.add_message(RenderMessage::Pack {
            buffer: 0,
            pack: create_quad_pack(),
        });

        let res = validator.validate(&mut rs, &render_messages);
        assert!(
            res.is_err(),
            "Validate wrongfully accepts non-existent texture name!"
        );

        let mut render_messages = RenderMessages::new();
        let mut validator = RenderMessageValidator::new();
        render_messages.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Default,
        });
        let mut ud = UniformData::new();
        ud.texture(String::from("atlas"), String::from("test_texture"));
        render_messages.add_message(RenderMessage::Uniforms { uniforms: ud });
        render_messages.add_message(RenderMessage::Pack {
            buffer: 0,
            pack: create_quad_pack(),
        });

        assert!(
            validator.validate(&mut rs, &render_messages).is_ok(),
            "Validate wrongfully doesn't accept correct texture and uniform name!"
        );
    }

    #[test]
    fn uniform_validation() {
        // Ensures that the uniform validation method works correctly
        let mut rs = create_render_state(true);

        let mut render_messages = RenderMessages::new();
        let mut validator = RenderMessageValidator::new();
        render_messages.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Default,
        });
        render_messages.add_message(RenderMessage::Pack {
            buffer: 0,
            pack: create_quad_pack(),
        });
        render_messages.add_message(RenderMessage::ChooseFramebuffer { framebuffer: None });
        render_messages.add_message(RenderMessage::Draw { buffer: 0 });

        assert!(
            validator.validate(&mut rs, &render_messages).is_err(),
            "Validate wrongfully accepts non-filled uniforms!"
        );

        let mut render_messages = RenderMessages::new();
        let mut validator = RenderMessageValidator::new();
        render_messages.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Default,
        });
        render_messages.add_message(RenderMessage::Pack {
            buffer: 0,
            pack: create_quad_pack(),
        });
        let mut ud = UniformData::new();
        ud.texture(String::from("atlas"), String::from("test_texture"));
        render_messages.add_message(RenderMessage::Uniforms { uniforms: ud });
        render_messages.add_message(RenderMessage::ChooseFramebuffer { framebuffer: None });
        render_messages.add_message(RenderMessage::Draw { buffer: 0 });

        assert!(
            validator.validate(&mut rs, &render_messages).is_err(),
            "Validate wrongfully accepts non-filled uniforms!"
        );

        let mut render_messages = RenderMessages::new();
        let mut validator = RenderMessageValidator::new();
        render_messages.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Default,
        });
        render_messages.add_message(RenderMessage::Pack {
            buffer: 0,
            pack: create_quad_pack(),
        });
        let mut ud = UniformData::new();
        ud.texture(String::from("atlas"), String::from("test_texture"));
        ud.vec3(glm::vec3(0., 0., 0.), String::from("vector"));
        render_messages.add_message(RenderMessage::Uniforms { uniforms: ud });
        render_messages.add_message(RenderMessage::ChooseFramebuffer { framebuffer: None });
        render_messages.add_message(RenderMessage::Draw { buffer: 0 });
        assert!(
            validator.validate(&mut rs, &render_messages).is_ok(),
            "Validate doesn't accept filled out uniforms!"
        );

        let mut render_messages = RenderMessages::new();
        let mut validator = RenderMessageValidator::new();
        render_messages.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Default,
        });
        render_messages.add_message(RenderMessage::Pack {
            buffer: 0,
            pack: create_quad_pack(),
        });
        let mut ud = UniformData::new();
        ud.texture(String::from("atlas"), String::from("test_texture"));
        ud.vec3(glm::vec3(0., 0., 0.), String::from("vector"));
        render_messages.add_message(RenderMessage::Uniforms { uniforms: ud });
        render_messages.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Default,
        });
        render_messages.add_message(RenderMessage::ChooseFramebuffer { framebuffer: None });
        render_messages.add_message(RenderMessage::Draw { buffer: 0 });

        assert!(
            validator.validate(&mut rs, &render_messages).is_err(),
            "Validate wrongfully accepts non-filled uniforms after shader swap!"
        );

        let mut render_messages = RenderMessages::new();
        let mut validator = RenderMessageValidator::new();
        render_messages.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Default,
        });
        render_messages.add_message(RenderMessage::Pack {
            buffer: 0,
            pack: create_quad_pack(),
        });
        let mut ud = UniformData::new();
        ud.texture(String::from("atlas"), String::from("test_texture"));
        ud.vec3(glm::vec3(0., 0., 0.), String::from("vector"));
        render_messages.add_message(RenderMessage::Uniforms { uniforms: ud });
        render_messages.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Default,
        });
        let mut ud = UniformData::new();
        ud.texture(String::from("atlas"), String::from("test_texture"));
        render_messages.add_message(RenderMessage::Uniforms { uniforms: ud });
        let mut ud = UniformData::new();
        ud.texture(String::from("atlas"), String::from("test_texture"));
        ud.vec3(glm::vec3(0., 0., 0.), String::from("vector"));
        render_messages.add_message(RenderMessage::Uniforms { uniforms: ud });
        render_messages.add_message(RenderMessage::ChooseFramebuffer { framebuffer: None });
        render_messages.add_message(RenderMessage::Draw { buffer: 0 });

        assert!(
            validator.validate(&mut rs, &render_messages).is_ok(),
            "Validate doesn't accept filled out uniforms after shader swap!"
        );
    }

    #[test]
    fn framebuffer_validation() {
        let mut rs = create_render_state(false);

        let mut render_messages = RenderMessages::new();
        let mut validator = RenderMessageValidator::new();
        render_messages.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Default,
        });
        let mut ud = UniformData::new();
        ud.texture(String::from("atlas"), String::from("test_texture"));
        render_messages.add_message(RenderMessage::Uniforms { uniforms: ud });
        render_messages.add_message(RenderMessage::Pack {
            buffer: 0,
            pack: create_quad_pack(),
        });
        render_messages.add_message(RenderMessage::Draw { buffer: 0 });

        assert!(
            validator.validate(&mut rs, &render_messages).is_err(),
            "Validate wrongfully accepts no render target!"
        );

        let mut render_messages = RenderMessages::new();
        let mut validator = RenderMessageValidator::new();
        render_messages.add_message(RenderMessage::ChooseShader {
            shader: ShaderIdentifier::Default,
        });
        render_messages.add_message(RenderMessage::ChooseFramebuffer { framebuffer: None });
        let mut ud = UniformData::new();
        ud.texture(String::from("atlas"), String::from("test_texture"));
        render_messages.add_message(RenderMessage::Uniforms { uniforms: ud });
        render_messages.add_message(RenderMessage::Pack {
            buffer: 0,
            pack: create_quad_pack(),
        });
        render_messages.add_message(RenderMessage::Draw { buffer: 0 });

        let res = validator.validate(&mut rs, &render_messages);
        assert!(
            res.is_ok(),
            "Validate wrongfully doesn't accept a render target! {:?}",
            res
        );
    }
}
