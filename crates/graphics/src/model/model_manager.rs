use std::collections::{HashMap, HashSet, hash_map::Keys};

use itertools::Itertools;

use crate::{BufferTarget, RenderMessages};
use super::{model::Model, PlacedModel};


pub struct ModelManager {
    models : HashMap<String, Model>,
    packed_models : HashSet<String>
}

impl ModelManager {
    pub fn new() -> Self {
        Self {
            models : HashMap::new(),
            packed_models : HashSet::new()
        }
    }

    pub fn add_model(&mut self, name : String, model : Model) -> Result<(), String>{
        if self.models.contains_key(&name) {
            Err(format!("Trying to add model {} to ModelManager, but there's already a model by that name!", name))
        } else {
            self.models.insert(name, model);
            Ok(())
        }
    }

    pub fn draw_models<'a>(&self, drawn_models : Vec<PlacedModel>, view_projection_matrix : &glm::Mat4) -> RenderMessages {
        let models_to_pack : HashSet<String> = drawn_models.iter()
            .map(|x| (x.model_name()))
            .unique()
            .cloned()
            .collect();
        
        if models_to_pack.difference(&self.packed_models).count() > 0 {
            panic!("Trying to draw models that aren't packed!")
            // TODO: You're only allowed to change packed models through the specific pack_models function
        }

        let mut render_messages = RenderMessages::new();

        for model in drawn_models {
            render_messages.merge_current(self.models[model.model_name()].render(&model, view_projection_matrix))
        }
        

        render_messages
    }

    ///
    /// Before any model is drawn, it must be packed
    /// This should be called at some point like level load or something. Like it should be called as little as possible, and only when the set of models changes.
    /// 
    pub fn pack_models(&mut self, models : HashSet<String>) -> RenderMessages {
        let mut messages = RenderMessages::new();

        for model in &self.packed_models {
            messages.add_message(self.models.get_mut(model).unwrap().unpack());
        }

        for (i, model) in models.iter().enumerate() {
            messages.add_message(self.models.get_mut(model).unwrap().pack(BufferTarget::ModelBuffer(i)));
        }

        self.packed_models = models;

        messages
    }

    pub fn get_model_names<'a>(&'a self) -> Keys<'a, String, Model> {
        self.models.keys()
    }

    pub fn load_models() -> Self {
        let mut model_manager = ModelManager::new();

        model_manager.add_model("test".into(), Model::default()).unwrap();

        model_manager
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let mut model_manager = ModelManager::new();
        model_manager.add_model("bob".into(), Model::default()).unwrap();

        let models = ["bob"];
        model_manager.pack_models(models.iter().map(|x| x.to_string()).collect());

        let mut drawn_models = Vec::new();
        drawn_models.push(PlacedModel::new("bob".into(), glm::vec3(0.0,0.0,0.0), glm::vec3(1.0,1.0,1.0)));

        model_manager.draw_models(drawn_models, &glm::identity());
    }

    #[test]
    #[should_panic]
    fn test_no_model() {
        let mut model_manager = ModelManager::new();
        model_manager.add_model("bob1".into(), Model::default()).unwrap();

        let models = ["bob"];
        model_manager.pack_models(models.iter().map(|x| x.to_string()).collect());

        let mut drawn_models = Vec::new();
        drawn_models.push(PlacedModel::new("bob".into(), glm::vec3(0.0,0.0,0.0), glm::vec3(1.0,1.0,1.0)));

        model_manager.draw_models(drawn_models, &glm::identity());
    }

    
    #[test]
    #[should_panic]
    fn test_not_packed() {
        let mut model_manager = ModelManager::new();
        model_manager.add_model("bob".into(), Model::default()).unwrap();

        let mut drawn_models = Vec::new();
        drawn_models.push(PlacedModel::new("bob".into(), glm::vec3(0.0,0.0,0.0), glm::vec3(1.0,1.0,1.0)));

        model_manager.draw_models(drawn_models, &glm::identity());
    }

    
    #[test]
    #[should_panic]
    fn test_pack_unpack_fail() {
        let mut model_manager = ModelManager::new();
        model_manager.add_model("bob".into(), Model::default()).unwrap();
        model_manager.add_model("david".into(), Model::default()).unwrap();

        let models = ["bob"];
        model_manager.pack_models(models.iter().map(|x| x.to_string()).collect());

        let mut drawn_models = Vec::new();
        drawn_models.push(PlacedModel::new("bob".into(), glm::vec3(0.0,0.0,0.0), glm::vec3(1.0,1.0,1.0)));
        model_manager.draw_models(drawn_models, &glm::identity());

        
        let models = ["david"];
        model_manager.pack_models(models.iter().map(|x| x.to_string()).collect());
        
        let mut drawn_models = Vec::new();
        drawn_models.push(PlacedModel::new("bob".into(), glm::vec3(0.0,0.0,0.0), glm::vec3(1.0,1.0,1.0)));
        model_manager.draw_models(drawn_models, &glm::identity());
    }

    #[test]
    fn test_pack_unpack() {
        let mut model_manager = ModelManager::new();
        model_manager.add_model("bob".into(), Model::default()).unwrap();
        model_manager.add_model("david".into(), Model::default()).unwrap();

        let models = ["bob"];
        model_manager.pack_models(models.iter().map(|x| x.to_string()).collect());

        let mut drawn_models = Vec::new();
        drawn_models.push(PlacedModel::new("bob".into(), glm::vec3(0.0,0.0,0.0), glm::vec3(1.0,1.0,1.0)));
        model_manager.draw_models(drawn_models, &glm::identity());

        
        let models = ["david"];
        model_manager.pack_models(models.iter().map(|x| x.to_string()).collect());
        
        let mut drawn_models = Vec::new();
        drawn_models.push(PlacedModel::new("david".into(), glm::vec3(0.0,0.0,0.0), glm::vec3(1.0,1.0,1.0)));
        model_manager.draw_models(drawn_models, &glm::identity());
    }
}