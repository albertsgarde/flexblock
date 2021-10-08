use std::{fs, path::Path};

use glutin::event::{MouseButton, VirtualKeyCode};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
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
    #[serde(default = "jump_default")]
    pub jump: Control,
    #[serde(default = "save_default")]
    pub save: Control,
    #[serde(default = "load_default")]
    pub load: Control,
    #[serde(default = "player_interact_1_default")]
    pub player_interact_1: Control,
    #[serde(default = "player_interact_2_default")]
    pub player_interact_2: Control,
}

impl Default for ControlConfig {
    fn default() -> Self {
        ControlConfig {
            move_forward: move_forward_default(),
            move_back: move_back_default(),
            strafe_right: strafe_right_default(),
            strafe_left: strafe_left_default(),
            jump: jump_default(),
            save: save_default(),
            load: load_default(),
            player_interact_1: player_interact_1_default(),
            player_interact_2: player_interact_2_default(),
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

fn jump_default() -> Control {
    Control::Keyboard {
        key_code: VirtualKeyCode::Space,
    }
}

fn save_default() -> Control {
    Control::Keyboard {
        key_code: VirtualKeyCode::S,
    }
}
fn load_default() -> Control {
    Control::Keyboard {
        key_code: VirtualKeyCode::L,
    }
}
fn player_interact_1_default() -> Control {
    Control::Mouse {
        mouse_button: MouseButton::Left,
    }
}
fn player_interact_2_default() -> Control {
    Control::Mouse {
        mouse_button: MouseButton::Right,
    }
}

pub fn save_control_config<P>(path: P, control_config: &ControlConfig)
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if let Err(error) = std::fs::create_dir_all(path.parent().unwrap()) {
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

pub fn load_control_config<P>(path: P) -> ControlConfig
where
    P: AsRef<Path>,
{
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
