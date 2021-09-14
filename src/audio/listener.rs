use crate::game::{world::Location, Player};
use glm::Vec3;

/// All distances less than this will attentuate audio the same as this distance.
const MIN_ATTENUATION_DIST: f32 = 0.25;
/// Left and right will be placed this distance from the view location.
const HEAD_RADIUS: f32 = 0.1;

pub struct Listener {
    dest_right_vec: Vec3,
    dest_center: Location,
    prev_right_vec: Vec3,
    prev_center: Location,
}

impl Listener {
    /// Create a listener from a player based on the player's position and view direction.
    pub fn from_player(player: &Player) -> Listener {
        let player_right_vec = player.view().right();
        let center = player.view().location();
        Listener {
            dest_right_vec: player_right_vec,
            dest_center: center,
            prev_right_vec: player_right_vec,
            prev_center: center,
        }
    }

    pub fn update(prev_listener: Listener, player: &Player) -> Listener {
        let player_right_vec = player.view().right();
        let center = player.view().location();
        Listener {
            dest_right_vec: player_right_vec,
            dest_center: center,
            ..prev_listener
        }
    }

    /// Takes a mono sample and transforms it into the stereo samples the listener hears using the samples
    /// generation location and information about the player's state.
    pub fn mono_to_stereo(
        &self,
        mono_sample: f32,
        location: Location,
        tick_passed: f32,
    ) -> (f32, f32) {
        // Calculates the current right direction vector by interpolating between the destination
        let cur_right_vec = interpolate(self.prev_right_vec, self.dest_right_vec, tick_passed);
        let center_to_sound = interpolate(
            location - self.prev_center,
            location - self.dest_center,
            tick_passed,
        );
        let right_to_sound = center_to_sound - cur_right_vec;
        let left_to_sound = center_to_sound + cur_right_vec;
        let (left_sample, right_sample) = pan_sample(mono_sample, center_to_sound, cur_right_vec);
        let left_sample = distance_attenuation(left_sample, left_to_sound);
        let right_sample = distance_attenuation(right_sample, right_to_sound);
        (left_sample, right_sample)
    }
}

impl Default for Listener {
    fn default() -> Self {
        Listener {
            dest_center: Location::origin(),
            dest_right_vec: Vec3::new(1., 0., 0.),
            prev_center: Location::origin(),
            prev_right_vec: Vec3::new(1., 0., 0.),
        }
    }
}

fn interpolate(prev: Vec3, dest: Vec3, interpolation_value: f32) -> Vec3 {
    prev * (1. - interpolation_value) + dest * interpolation_value
}

/// Pans a sample depending on its source location.
fn pan_sample(mono_sample: f32, sound_vec: Vec3, right_vec: Vec3) -> (f32, f32) {
    // The cosine of the angle between the right direction of the listener and the vector from the center to the sound.
    // This represents how far to the left or right the sound should be panned.
    let x = sound_vec.dot(&(right_vec)) / (sound_vec.norm());
    // (x+1)/2 normalizes x to the interval [0;1].
    // The rest is an application of constant power panning to keep the signal power constant across all angles (assuming constant distance).
    let (sin, cos) = ((x + 1.) * std::f32::consts::FRAC_PI_4).sin_cos();
    (mono_sample * cos, mono_sample * sin)
}

/// Attenuate sample according to distance.
fn distance_attenuation(sample: f32, sound_vec: Vec3) -> f32 {
    let distance = sound_vec.norm();
    // Attenuate sound according to the inverse square law.
    // Set a minimum distance to avoid volumes approaching infinite very close to the ears.
    sample * f32::powi(MIN_ATTENUATION_DIST.max(distance), -2)
}
