use serde::{Deserialize, Serialize};
use std::{fs, process::Command};

#[derive(Serialize, Deserialize)]
struct BuildConfig {
    pub fmt: bool,
}

impl BuildConfig {
    pub fn load_from_file() -> Self {
        if let Ok(config_string) = fs::read_to_string("build_config.toml") {
            if let Ok(config) = toml::from_str(&config_string) {
                config
            } else {
                eprintln!("Could not parse build config file.");
                BuildConfig::default()
            }
        } else {
            eprintln!("Could not read build config file.");
            BuildConfig::default()
        }
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        BuildConfig { fmt: false }
    }
}

fn main() {
    let build_config = BuildConfig::load_from_file();

    if build_config.fmt {
        Command::new("cargo")
            .arg("fmt")
            .status()
            .expect("Failed to run `cargo fmt`.");
    }
}
