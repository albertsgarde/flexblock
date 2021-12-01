use utils::vertex::V3C3UV;

use crate::{BufferTarget, RenderMessage, RenderMessages, VertexPack, UniformData};



///
/// Will contain model data, right now all models are just squares.
/// Watch out, models are stateful. They keep track of which buffer they're bound to.
/// 
pub struct Model {
    vertex_pack : VertexPack,
    /// Which OpenGl buffer the Model is currently packed in
    bound_target : Option<BufferTarget>,
}

impl Model {

    pub fn from_str(model_string : &str) -> Self {
        let mut vertices = Vec::new();
        let mut elements = Vec::new();

        for line in model_string.split("\n") {
            let line = line.trim();

            if line.starts_with("v") {
                // VERTEX

                let mut vertex_parts = line.split(" ");

                vertex_parts.next().unwrap();
                let x = vertex_parts.next().unwrap().parse::<f32>().unwrap();
                let y = vertex_parts.next().unwrap().parse::<f32>().unwrap();
                let z = vertex_parts.next().unwrap().parse::<f32>().unwrap();
                let r = vertex_parts.next().unwrap().parse::<f32>().unwrap();
                let g = vertex_parts.next().unwrap().parse::<f32>().unwrap();
                let b = vertex_parts.next().unwrap().parse::<f32>().unwrap();
                let u = vertex_parts.next().unwrap().parse::<f32>().unwrap();
                let v = vertex_parts.next().unwrap().parse::<f32>().unwrap();
                if vertex_parts.next().is_some() {
                    panic!("Model string {} contains a vertex with too many parts!", model_string);
                }
                
                vertices.push(V3C3UV {x,y,z,r,g,b,u,v});
            }
            else if line.starts_with("f") {
                // FACE

                let mut face_parts = line.split(" ");
                face_parts.next().unwrap();

                for _ in 0..3 {
                    elements.push(face_parts.next().unwrap().parse::<u32>().unwrap());
                }
                if face_parts.next().is_some() {
                    panic!("Model string {} contains a face with more than three sides!", model_string);
                }
            } else if !line.starts_with("#") && !line.starts_with("g") && line.contains("^\\s") {
                panic!("Model string contains line {} that contains illegal model code", line);
            }
        }

        Self { vertex_pack : VertexPack::new(vertices, Some(elements)), bound_target : None}
    }

    pub fn pack(&mut self, buffer_target : BufferTarget) -> RenderMessage {
        if self.bound_target.is_some() {
            panic!("Trying to pack an already packed model!")
        }
        self.bound_target = Some(buffer_target);
        RenderMessage::Pack {
            buffer : buffer_target,
            pack : self.vertex_pack.clone()
        }
    }

    pub fn unpack(&mut self) -> RenderMessage {
        if let Some(buffer_target) = self.bound_target.take() {
            RenderMessage::ClearArray {
                buffer : buffer_target
            }
        }
        else {
            panic!("Trying to unpack a model that isn't packed!")
        }
    }

    pub fn is_packed(&self) -> bool {
        self.bound_target.is_some()
    }

    pub fn render(&self, placed_model : &PlacedModel, view_projection_matrix : &glm::Mat4) -> RenderMessages {
        let mut render_messages = RenderMessages::new();
        let mut uniforms = UniformData::new();
        let model_matrix = glm::mat4(placed_model.scale.x, 0.0, 0.0, placed_model.position.x, 0.0, placed_model.scale.y, 0.0, placed_model.position.y, 0.0, 0.0, placed_model.scale.z, placed_model.position.z, 0.0 ,0.0, 0.0, 1.0);
        uniforms.mat4(view_projection_matrix * model_matrix /* * model_matrix*/, "MVP");
        // TODO: ROTATION UNIFORM
        render_messages.add_message(RenderMessage::Uniforms {
            uniforms: Box::new( uniforms),
        });
        render_messages.add_message(RenderMessage::Draw{ buffer : self.bound_target.unwrap() });

        render_messages
    }
}

impl Default for Model {
    fn default() -> Model {
        let mut vertices = Vec::new();
        let mut elements = Vec::new();
        let mut index = 0;

        let (x0,y0,z0,x1,y1,z1) = (-0.5,-0.5,-0.5,0.5,0.5,0.5);

        // Back face
        let (mut vadd, mut eadd) =
            crate::pack::cube_faces::back(z0,x0, y0, x1, y1, 1., 1.,1., index);
        index += vadd.len() as u32;
        vertices.append(&mut vadd);
        elements.append(&mut eadd);

        //Front face
        let (mut vadd, mut eadd) =
        crate::pack::cube_faces::front(z1, x0, y0, x1, y1, 1., 1.,1., index);
        index += vadd.len() as u32;
        vertices.append(&mut vadd);
        elements.append(&mut eadd);

        //Left face
        let (mut vadd, mut eadd) =
        crate::pack::cube_faces::left(x0, y0, z0, y1, z1, 1., 1.,1., index);
        index += vadd.len() as u32;
        vertices.append(&mut vadd);
        elements.append(&mut eadd);

        //Right face
        let (mut vadd, mut eadd) =
        crate::pack::cube_faces::right(x1, y0, z0, y1, z1, 1., 1.,1., index);
        index += vadd.len() as u32;
        vertices.append(&mut vadd);
        elements.append(&mut eadd);

        //Bottom face
        let (mut vadd, mut eadd) =
        crate::pack::cube_faces::bottom(y0, x0, z0, x1, z1, 1., 1.,1., index);
        index += vadd.len() as u32;
        vertices.append(&mut vadd);
        elements.append(&mut eadd);

        //Top face
        let (mut vadd, mut eadd) =
        crate::pack::cube_faces::top(y1, x0, z0, x1, z1, 1., 1.,1., index);
        vertices.append(&mut vadd);
        elements.append(&mut eadd);

        Self { vertex_pack : VertexPack::new(vertices, Some(elements)), bound_target : None}
    }

}

///
/// TODO: BETTER NAME!!!
/// A model in the world
/// 
pub struct PlacedModel {
    model_name : String,
    position : glm::Vec3,
    scale : glm::Vec3
}

impl PlacedModel {
    pub fn new(model_name : String, position : glm::Vec3, scale : glm::Vec3) -> Self {
        Self {
            model_name,
            position,
            scale
        }
    }

    pub fn model_name(&self) -> &String {
        &self.model_name
    }
}
