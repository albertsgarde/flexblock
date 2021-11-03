pub mod maths;
pub mod mesh_iterator;
pub mod vertex;
pub use vertex::Locatedf32;
pub use vertex::Vertex3D;

pub mod png_reader;
pub use png_reader::read_png;

mod file_utilities;
pub use file_utilities::dir_entries;

mod colors;
pub use colors::ColorFormat;

mod csv_reader;
pub mod ply;
pub use csv_reader::{read_csv, CsvGrid};

extern crate nalgebra_glm as glm;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref ASSETS_PATH: Box<std::path::Path> = {
        let path = std::env::current_exe().map_or_else(
            |_| {
                log::info!("Executable path unavailable. Using working directory instead");
                std::env::current_dir()
                    .expect("Both executable path and working directory are unavailable.")
            },
            |exe_path| exe_path.join(".."),
        );
        let root_path = path.join("../..");
        let result = if root_path.join("Cargo.toml").is_file() {
            if root_path.join("assets").is_dir() {
                root_path.join("assets")
            } else {
                panic!("No assets directory at project root.");
            }
        } else {
            if path.join("../assets").is_dir() {
                path.join("../assets")
            } else if path.join("assets").is_dir() {
                path.join("assets")
            } else {
                panic!("Either the assets directory is missing or it is inaccessable.")
            }
        };
        result.into_boxed_path()
    };
}
