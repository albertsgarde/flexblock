use std::{
    fs::{self},
    path::Path,
};

use glutin::event::{MouseButton, VirtualKeyCode};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Control {
    Mouse { mouse_button: MouseButton },
    Keyboard { key_code: VirtualKeyCode },
}

#[derive(Serialize, Deserialize)]
pub struct ControlConfig {
    #[serde(default = "move_forward_default")]
    pub move_forward: Control,
    #[serde(default = "move_back_default")]
    pub move_back: Control,
    #[serde(default = "strafe_right_default")]
    pub strafe_right: Control,
    #[serde(default = "strafe_left_default")]
    pub strafe_left: Control,
}

impl Default for ControlConfig {
    fn default() -> Self {
        ControlConfig {
            move_forward: move_forward_default(),
            move_back: move_back_default(),
            strafe_right: strafe_right_default(),
            strafe_left: strafe_left_default(),
        }
    }
}

fn move_forward_default() -> Control {
    Control::Keyboard {
        key_code: VirtualKeyCode::W,
    }
}

fn move_back_default() -> Control {
    Control::Keyboard {
        key_code: VirtualKeyCode::S,
    }
}

fn strafe_right_default() -> Control {
    Control::Keyboard {
        key_code: VirtualKeyCode::D,
    }
}

fn strafe_left_default() -> Control {
    Control::Keyboard {
        key_code: VirtualKeyCode::A,
    }
}

pub fn save_control_config(path: &str, control_config: &ControlConfig) {
    let config_path = Path::new(path);
    if let Err(error) = std::fs::create_dir_all(config_path.parent().unwrap()) {
        error!(
            "Control config save failed. Could not create directory. Error: {:?}",
            error
        );
        return;
    }

    let config_string = match toml::to_string(&control_config) {
        Ok(config_string) => config_string,
        Err(error) => {
            error!("Could not serialize controls config. Error: {:?}", error);
            return;
        }
    };

    if let Err(error) = fs::write(path, &config_string) {
        error!(
            "Could not write controls config to file. Error: {:?}",
            error
        )
    }
}

pub fn load_control_config(path: &str) -> ControlConfig {
    match fs::read_to_string(path) {
        Ok(config_string) => toml::from_str(&config_string).unwrap_or_else(|error| {
            error!(
                "Could not parse control configs. Using default. Error: {:?}",
                error
            );
            ControlConfig::default()
        }),
        Err(error) => {
            error!(
                "Could not read control config file. Using default. Error: {:?}",
                error
            );
            ControlConfig::default()
        }
    }
}
