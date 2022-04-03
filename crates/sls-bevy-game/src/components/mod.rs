use bevy::math::Vec3A;
use bevy::prelude::*;

#[derive(Bundle, Debug, Default)]
pub struct FpsCameraBundle {
  pub fps_camera: FpsCameraState,
}

#[derive(Component, Debug)]
pub struct FpsCameraState {
  pub position: Vec3A,
  front: Vec3A,
  right: Vec3A,
  up: Vec3A,
  pub world_up: Vec3A,

  pub yaw: f32,
  pub pitch: f32,
  pub movement_speed: f32,
  pub movement_sensitivity: f32,
  pub zoom: f32,
}

impl Default for FpsCameraState {
  fn default() -> Self {
    let up = [0.0, 1.0, 0.0].into();
    let mut camera = Self {
      position: [0.0, 0.0, 0.0].into(),
      front: Default::default(),
      right: Default::default(),
      world_up: up,
      up,
      yaw: -90.0,
      pitch: 0.0,
      movement_speed: 2.5,
      movement_sensitivity: 0.1,
      zoom: 45.0,
    };

    camera.update_camera_vectors();

    camera
  }
}

impl FpsCameraState {
  pub fn view(&self) -> Mat4 {
    Mat4::look_at_rh(
      self.position.into(),
      (self.position + self.front).into(),
      self.up.into(),
    )
  }

  pub fn update_camera_vectors(&mut self) {
    let pitch_radians = f32::to_radians(self.pitch);
    let yaw_radians = f32::to_radians(self.yaw);
    let front_unnormalized = Vec3A::new(
      f32::cos(yaw_radians) * f32::cos(pitch_radians),
      f32::sin(pitch_radians),
      f32::sin(yaw_radians) * f32::cos(pitch_radians),
    );
    self.front = front_unnormalized.normalize();
    self.right = Vec3A::cross(self.front, self.world_up).normalize();
    self.up = Vec3A::cross(self.right, self.front).normalize();
  }
}
