mod arm;
mod look_at;
mod position;
mod rotation;
mod smooth;
mod yaw_pitch;
use bevy::prelude::*;

pub use {
    arm::*,
    look_at::*,
    position::*,
    rotation::*,
    smooth::*,
    yaw_pitch::*
};

pub trait RigDriver: std::any::Any + Sync + Send {
    /// Calculates the transform of this driver component based on the parent
    /// provided in `params`.
    fn update(&mut self, params: RigUpdateParams) -> Transform;

    /// Returns `self` as `&mut dyn Any`
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// Prevents user calls to `RigDriver::update`. All updates must come from `CameraRig::update`.
pub struct RigUpdateParams<'a> {
    pub parent: &'a Transform,
    pub delta_time_seconds: f32,
}

pub(crate) trait Interpolate {
    fn interpolate(self, other: Self, t: f32) -> Self;
}

impl Interpolate for Vec3 {
    fn interpolate(self, other: Self, t: f32) -> Self {
        Vec3::lerp(self, other, t)
    }
}

impl Interpolate for Quat {
    fn interpolate(self, other: Self, t: f32) -> Self {
        // Technically should be a `slerp` for framerate independence, but the latter
        // will rotate in the negative direction when interpolating a 180..360 degree rotation
        // to the 0..180 range. See the comment about `yaw_degrees` in `YawPitch` for more details.
        Quat::lerp(self.normalize(), other.normalize(), t).normalize()
    }
}

pub(crate) struct ExpSmoothingParams {
    pub smoothness: f32,
    pub output_offset_scale: f32,
    pub delta_time_seconds: f32,
}

#[derive(Default, Debug)]
pub(crate) struct ExpSmoothed<T: Interpolate + Copy + std::fmt::Debug>(Option<T>);

impl<T: Interpolate + Copy + std::fmt::Debug> ExpSmoothed<T> {
    pub(crate) fn exp_smooth_towards(&mut self, other: &T, params: ExpSmoothingParams) -> T {
        // An ad-hoc multiplier to make default smoothness parameters
        // produce good-looking results.
        const SMOOTHNESS_MULT: f32 = 8.0;

        // Calculate the exponential blending based on frame time
        let interp_t = 1.0
            - (-SMOOTHNESS_MULT * params.delta_time_seconds / params.smoothness.max(1e-5)).exp();

        let prev = self.0.unwrap_or(*other);
        let smooth = prev.interpolate(*other, interp_t);

        self.0 = Some(smooth);

        #[allow(clippy::float_cmp)]
        if params.output_offset_scale != 1.0 {
            Interpolate::interpolate(*other, smooth, params.output_offset_scale)
        } else {
            smooth
        }
    }
}
