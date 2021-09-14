use crate::game::{world::Location, Player};
use glm::Vec3;

/// All distances less than this will attentuate audio the same as this distance.
const MIN_ATTENUATION_DIST: f32 = 0.25;
/// Left and right will be placed this distance from the view location.
const HEAD_RADIUS: f32 = 0.1;

pub struct Listener {
    left: Location,
    right: Location,
    center: Location,
}

impl Listener {
    pub fn from_player(player: &Player) -> Listener {
        let player_right_vec = player.view().right();
        Listener {
            left: player.view().location() - player_right_vec * HEAD_RADIUS,
            right: player.view().location() + player_right_vec * HEAD_RADIUS,
            center: player.view().location(),
        }
    }

    fn pan_sample(&self, mono_sample: f32, location: Location) -> (f32, f32) {
        let vector = location - self.center;
        let x = vector.dot(&(self.right - self.left)) / (vector.norm() * HEAD_RADIUS * 2.);
        let (sin, cos) = ((x + 1.) * std::f32::consts::FRAC_PI_4).sin_cos();
        (mono_sample * cos, mono_sample * sin)
    }

    /// Takes a mono sample and transforms it into the stereo samples the listener hears using the samples
    /// generation location and information about the player's state.
    pub fn mono_to_stereo(&self, mono_sample: f32, location: Location) -> (f32, f32) {
        let (left_sample, right_sample) = self.pan_sample(mono_sample, location);
        let left_sample = distance_attenuation(left_sample, location, self.left);
        let right_sample = distance_attenuation(right_sample, location, self.right);
        (left_sample, right_sample)
    }
}

impl Default for Listener {
    fn default() -> Self {
        Listener {
            left: Location::origin() - Vec3::new(HEAD_RADIUS, 0., 0.),
            right: Location::origin() + Vec3::new(HEAD_RADIUS, 0., 0.),
            center: Location::origin(),
        }
    }
}

/// Attenuate sample according to distance.
fn distance_attenuation(sample: f32, transmit_loc: Location, receive_loc: Location) -> f32 {
    let distance = (receive_loc - transmit_loc).norm();
    // Attenuate sound according to the inverse square law.
    // Set a minimum distance to avoid volumes approaching infinite very close to the ears.
    sample * f32::powi(MIN_ATTENUATION_DIST.max(distance), -2)
}
