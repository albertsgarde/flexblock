use super::{
    framebuffer::FramebufferIdentifier, Framebuffer, FramebufferManager, Shader, ShaderIdentifier,
    ShaderManager, Texture, TextureManager, TextureMetadata,
};
use log::{debug, error};
use strum::IntoEnumIterator;
use utils::read_png;

pub unsafe fn load_shaders() -> ShaderManager {
    let folder = utils::ASSETS_PATH.join("graphics/shaders");
    debug!("{}", folder.to_str().unwrap());
    debug!("{}", folder.canonicalize().unwrap().to_str().unwrap());

    let mut compute_shaders: Vec<String> = Vec::new();
    let mut fragment_shaders: Vec<String> = Vec::new();
    let mut vertex_shaders: Vec<String> = Vec::new();

    // First, we find every file in the folder we're loading from, and see if it's a shader file

    let entries = utils::dir_entries(&folder, folder.to_str().unwrap());
    let entries = match entries {
        Ok(e) => e,
        Err(error) => {
            panic!("Could not load shaders! {:?}", error)
        }
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
            error!("File {:?} does not contain a shader!", &entry.0);
        }
    }

    let mut shaders = Vec::new();
    for identifier in ShaderIdentifier::iter() {
        let shader = match Shader::new(identifier) {
            Ok(s) => s,
            Err(s) => {
                panic!("Loading shader {:?} failed! Error: {}", identifier, s);
            }
        };
        if identifier.is_compute() {
            compute_shaders.retain(|x| x != identifier.extensionless_path());
        } else {
            vertex_shaders.retain(|x| x != identifier.extensionless_path());
            fragment_shaders.retain(|x| x != identifier.extensionless_path());
        }

        shaders.push(shader);
    }

    for vs in vertex_shaders {
        error!(
            "Vertex shader {} doesn't exist in the shader identifier enum.",
            vs
        );
    }
    for fs in fragment_shaders {
        error!(
            "Fragment shader {} doesn't exist in the shader identifier enum.",
            fs
        );
    }

    ShaderManager::new(shaders)
}

pub unsafe fn load_textures(screen_dimensions: (u32, u32)) -> TextureManager {
    let mut texture_manager = TextureManager::new();
    let entries = utils::dir_entries(&utils::ASSETS_PATH.join("graphics/textures"), "");
    let entries = match entries {
        Ok(e) => e,
        Err(error) => {
            panic!("Could not load textures! {:?}", error)
        }
    };

    //TODO: Maybe panicking when failing to load a texture is a bit melodramatic.
    for entry in entries {
        if entry.1.ends_with(".png") {
            let data = match read_png(&entry.0.path()) {
                Ok(d) => d,
                Err(error) => {
                    panic!(
                        "Failed to load texture in png file {:?}! Error {:?}",
                        entry.0.path(),
                        error
                    );
                }
            };

            use super::InternalFormat;
            use utils::ColorFormat;
            let int_format = match data.format {
                ColorFormat::RGB => InternalFormat::RGB8,
                ColorFormat::RGBA => InternalFormat::RGBA8,
                _ => unreachable!(), // read_png handles the case where the color format isn't one of these two with an error.
            };

            let mut t = Texture::new(
                Some((data.width, data.height)),
                data.format,
                int_format,
                &entry.1,
                screen_dimensions,
            );
            t.fill(data.data);
            debug!("Loaded texture {}!", &t.metadata.name);
            texture_manager.add_texture(t).unwrap();
        } else if entry.1.ends_with(".json") {
            let metadatas: Vec<TextureMetadata> =
                match serde_json::from_str(match &std::fs::read_to_string(&entry.0.path()) {
                    Ok(s) => s,
                    Err(e) => panic!("Failed reading file {:?}! Error {:?}", &entry.0, &e),
                }) {
                    Ok(v) => v,
                    Err(e) => panic!("Json error in file {:?}! Error {:?}", &entry.0, &e),
                };

            for metadata in metadatas {
                let t = Texture::new(
                    match metadata.screen_dependant_dimensions {
                        false => Some((metadata.width, metadata.height)),
                        true => None,
                    },
                    metadata.format,
                    metadata.internal_format,
                    &metadata.name,
                    screen_dimensions,
                );
                texture_manager.add_texture(t).unwrap();
            }
        }
    }
    //let mut t1 = Texture::new(800, 800, TextureFormat::RGB, "atlas");
    //t1.fill(utils::read_png("textures/atlas.png"));
    //texture_manager.add_texture(t1);

    texture_manager
}

pub unsafe fn load_framebuffers(
    texture_manager: &TextureManager,
    screen_dimensions: (u32, u32),
) -> FramebufferManager {
    let mut framebuffers = Vec::new();

    for identifier in FramebufferIdentifier::iter() {
        framebuffers
            .push(Framebuffer::new(identifier, texture_manager, screen_dimensions).unwrap());
    }

    FramebufferManager::new(framebuffers)
}
