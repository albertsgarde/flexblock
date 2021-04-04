use super::{ShaderManager, TextureManager, Texture, Framebuffer, FramebufferManager, TextureMetadata, FramebufferMetadata};
use crate::utils::read_png;

pub unsafe fn load_shaders() -> ShaderManager{

    let mut shader_manager = ShaderManager::new();

    //TODO: This should maybe not be called from the RenderCaller new. Some decision has to be made.
    shader_manager.load_shaders("graphics/shaders");

    shader_manager
}

pub unsafe fn load_textures() -> TextureManager {
    let mut texture_manager = TextureManager::new();

    let entries = crate::utils::dir_entries(&std::path::Path::new("./graphics/textures"), "");
    let entries = match entries {
        Ok(e) => e,
        Err(error) => { panic!("Could not load textures! {:?}", error)}
    };

    //TODO: Maybe panicking when failing to load a texture is a bit melodramatic.
    for entry in entries {
        if entry.1.ends_with(".png") {
            let data = match read_png(&entry.0.path()) {
                Ok(d) => d,
                Err(error) => {
                    panic!("Failed to load texture in png file {:?}! Error {:?}",entry.0.path(), error);
                }
            };


            let mut t = Texture::new(data.width, data.height, data.format, &entry.1);
            t.fill(data.data);
            println!("Loaded texture {}!", &t.metadata.name);
            texture_manager.add_texture(t);
        } else if entry.1.ends_with(".json") {
            let metadatas : Vec<TextureMetadata> = match serde_json::from_str( match &std::fs::read_to_string(&entry.0.path()) {
                Ok(s) => s,
                Err(e) => panic!("Failed reading file {:?}! Error {:?}",&entry.0,&e),
            }) {
                Ok(v) => v,
                Err(e) => panic!("Json error in file {:?}! Error {:?}", &entry.0, &e),
            };

            for metadata in metadatas {
                let t = Texture::new(metadata.width, metadata.height, metadata.format, &metadata.name);
                texture_manager.add_texture(t);
            }

        }
    }
    //let mut t1 = Texture::new(800, 800, TextureFormat::RGB, "atlas");
    //t1.fill(crate::utils::read_png("textures/atlas.png"));
    //texture_manager.add_texture(t1);

    texture_manager
}

pub unsafe fn load_framebuffers(texture_manager : &TextureManager) -> FramebufferManager {
    let mut framebuffer_manager = FramebufferManager::new();

    
    let entries = crate::utils::dir_entries(&std::path::Path::new("./graphics/framebuffers"), "");
    let entries = match entries {
        Ok(e) => e,
        Err(error) => { panic!("Could not load textures! {:?}", error)}
    };
    for entry in entries {
        if entry.1.ends_with(".json") {
            let metadatas : Vec<FramebufferMetadata> = match serde_json::from_str( match &std::fs::read_to_string(&entry.0.path()) {
                Ok(s) => s,
                Err(e) => panic!("Failed reading file {:?}! Error {:?}",&entry.0,&e),
            }) {
                Ok(v) => v,
                Err(e) => panic!("Json error in file {:?}! Error {:?}", &entry.0, &e),
            };

            for metadata in metadatas {
                let ct = match &metadata.color_texture {
                    Some(t) => {
                        if !texture_manager.contains_texture(t) {
                            panic!("Color texture \"{}\" referenced by framebuffer {} does not exist in texture manager!", t, metadata.name);
                        }
                        Some(texture_manager.get_texture(t))
                    }
                    None => None
                };
                let dt = match &metadata.depth_texture {
                    Some(t) => {
                        if !texture_manager.contains_texture(t) {
                            panic!("Depth texture \"{}\" referenced by framebuffer {} does not exist in texture manager!", t, metadata.name);
                        }
                        Some(texture_manager.get_texture(t))
                    }
                    None => None
                };
                println!("{:?}",metadata);
                framebuffer_manager.add_framebuffer(Framebuffer::new(&metadata.name, ct, dt, metadata.width, metadata.height, metadata.has_depth).unwrap());
            }
        }
    }

    framebuffer_manager
}

#[cfg(test)]
mod tests {

    #[test]
    fn serialize_json() {

    }
}