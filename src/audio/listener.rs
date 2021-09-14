use crate::game::{world::Location, Player};
use glm::Vec3;

/// All distances less than this will attentuate audio the same as this distance.
const MIN_ATTENUATION_DIST: f32 = 0.25;
/// Left and right will be placed this distance from the view location.
const HEAD_RADIUS: f32 = 0.1;

pub struct Listener {
    right_vec: Vec3,
    center: Location,
}

impl Listener {
    /// Create a listener from a player based on the player's position and view direction.
    pub fn from_player(player: &Player) -> Listener {
        let player_right_vec = player.view().right();
        let center = player.view().location();
        Listener {
            right_vec: player_right_vec,
            center: center,
        }
    }

    /// Takes a mono sample and transforms it into the stereo samples the listener hears using the samples
    /// generation location and information about the player's state.
    pub fn mono_to_stereo(&self, mono_sample: f32, location: Location) -> (f32, f32) {
        // Vector from the center of the listener to the location of the sound.
        let center_to_sound = location - self.center;
        let right_to_sound = center_to_sound - self.right_vec * HEAD_RADIUS;
        let left_to_sound = center_to_sound + self.right_vec * HEAD_RADIUS;
        let (left_sample, right_sample) = pan_sample(mono_sample, self.right_vec, center_to_sound);
        let left_sample = distance_attenuation(left_sample, left_to_sound);
        let right_sample = distance_attenuation(right_sample, right_to_sound);
        (left_sample, right_sample)
    }

    pub fn interpolate_to(self, destination_listener: &Listener) -> ListenerInterpolation {
        ListenerInterpolation::new(self, destination_listener)
    }
}

impl Default for Listener {
    fn default() -> Self {
        Listener {
            right_vec: Vec3::new(1., 0., 0.),
            center: Location::origin(),
        }
    }
}

/// Pans a sample depending on its source location.
fn pan_sample(mono_sample: f32, right_vec: Vec3, vec_to_sound: Vec3) -> (f32, f32) {
    // The cosine of the angle between the right direction of the listener and the vector from the center to the sound.
    // This represents how far to the left or right the sound should be panned.
    let x = vec_to_sound.dot(&right_vec) / (vec_to_sound.norm());
    // (x+1)/2 normalizes x to the interval [0;1].
    // The rest is an application of constant power panning to keep the signal power constant across all angles (assuming constant distance).
    let (sin, cos) = ((x + 1.) * std::f32::consts::FRAC_PI_4).sin_cos();
    (mono_sample * cos, mono_sample * sin)
}

/// Attenuate sample according to distance.
fn distance_attenuation(sample: f32, vec_to_sound: Vec3) -> f32 {
    let distance = vec_to_sound.norm();
    // Attenuate sound according to the inverse square law.
    // Set a minimum distance to avoid volumes approaching infinite very close to the ears.
    sample * f32::powi(MIN_ATTENUATION_DIST.max(distance), -2)
}

pub struct ListenerInterpolation {
    prev_listener: Listener,
    right_vec_prev_to_dest: Vec3,
    center_prev_to_dest: Vec3,
}

impl ListenerInterpolation {
    fn new(prev_listener: Listener, dest_listener: &Listener) -> Self {
        let right_vec_prev_to_dest = dest_listener.right_vec - prev_listener.right_vec;
        let center_prev_to_dest = dest_listener.center - prev_listener.center;
        ListenerInterpolation {
            prev_listener,
            right_vec_prev_to_dest,
            center_prev_to_dest,
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
        let listener = Listener {
            right_vec: self.prev_listener.right_vec + self.right_vec_prev_to_dest * tick_passed,
            center: self.prev_listener.center + self.center_prev_to_dest * tick_passed,
        };
        // Vector from the center of the listener to the location of the sound.
        let center_to_sound = location - listener.center;
        let right_to_sound = center_to_sound - listener.right_vec * HEAD_RADIUS;
        let left_to_sound = center_to_sound + listener.right_vec * HEAD_RADIUS;
        let (left_sample, right_sample) =
            pan_sample(mono_sample, listener.right_vec, center_to_sound);
        let left_sample = distance_attenuation(left_sample, left_to_sound);
        let right_sample = distance_attenuation(right_sample, right_to_sound);
        (left_sample, right_sample)
    }
}

impl Default for ListenerInterpolation {
    fn default() -> Self {
        ListenerInterpolation::new(Listener::default(), &Listener::default())
    }
}
