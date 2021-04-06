use super::{Framebuffer, FramebufferManager, Shader, ShaderIdentifier, ShaderManager, Texture, TextureManager, TextureMetadata, framebuffer::FramebufferIdentifier};
use crate::utils::read_png;
use strum::IntoEnumIterator;

pub unsafe fn load_shaders() -> ShaderManager {
    let folder = "graphics/shaders";

    let mut compute_shaders: Vec<String> = Vec::new();
    let mut fragment_shaders: Vec<String> = Vec::new();
    let mut vertex_shaders: Vec<String> = Vec::new();

    // First, we find every file in the folder we're loading from, and see if it's a shader file
    
    let entries = crate::utils::dir_entries(&std::path::Path::new(folder), folder);
    let entries = match entries {
        Ok(e) => e,
        Err(error) => { panic!("Could not load shaders! {:?}", error)}
    };

    for entry in entries {
        if entry.1.ends_with(".vert") {
            let name = &entry.1[0..(entry.1.len() - 5)];
            vertex_shaders.push(String::from(name));
        } else if entry.1.ends_with(".frag") {
            let name = &entry.1[0..(entry.1.len() - 5)];
            fragment_shaders.push(String::from(name));
        } else if entry.1.ends_with(".comp") {
            let name = &entry.1[0..(entry.1.len() - 5)];
            compute_shaders.push(String::from(name));
        } else {
            eprintln!("File {:?} does not contain a shader!", &entry.0);
        }
    }

    let mut shaders = Vec::new();
    for identifier in ShaderIdentifier::iter() {

        let shader = match Shader::new(
            identifier
        ) {
            Ok(s) => s,
            Err(s) => {
                eprintln!("Loading shader {:?} failed! Error: {}", identifier, s);
                continue;
            }
        };
        if identifier.is_compute() {
            compute_shaders.retain(|x| {x != identifier.extensionless_path()});
        } else {
            vertex_shaders.retain(|x| {x != identifier.extensionless_path()});
            fragment_shaders.retain(|x| {x != identifier.extensionless_path()});
        }

        shaders.push(shader);
    }

    for vs in vertex_shaders {
        eprintln!("Vertex shader {} doesn't exist in the shader identifier enum.", vs);
    }
    for fs in fragment_shaders {
        eprintln!("Vertex shader {} doesn't exist in the shader identifier enum.", fs);
    }

    ShaderManager::new(shaders)
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
            texture_manager.add_texture(t).unwrap();
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
                texture_manager.add_texture(t).unwrap();
            }

        }
    }
    //let mut t1 = Texture::new(800, 800, TextureFormat::RGB, "atlas");
    //t1.fill(crate::utils::read_png("textures/atlas.png"));
    //texture_manager.add_texture(t1);

    texture_manager
}

pub unsafe fn load_framebuffers(texture_manager : &TextureManager) -> FramebufferManager {
    let mut framebuffers = Vec::new();

    
    /*
    let framebuffer_files = Vec::new();

    let entries = crate::utils::dir_entries(&std::path::Path::new("./graphics/framebuffers"), "./graphics/framebuffers");
    let entries = match entries {
        Ok(e) => e,
        Err(error) => { panic!("Could not load textures! {:?}", error)}
    };

    for entry in entries {
        if entry.1.ends_with(".json") {
            framebuffer_files.push(entry.1);

            /*let metadatas : Vec<FramebufferMetadata> = match serde_json::from_str( match &std::fs::read_to_string(&entry.0.path()) {
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
                framebuffer_manager.add_framebuffer(Framebuffer::new(&metadata.name, ct, dt, metadata.width, metadata.height, metadata.has_depth).unwrap()).unwrap();
            }*/
        }
    }*/
    for identifier in FramebufferIdentifier::iter() {
        framebuffers.push(Framebuffer::new(identifier, texture_manager).unwrap());
    }

    FramebufferManager::new(framebuffers)
}