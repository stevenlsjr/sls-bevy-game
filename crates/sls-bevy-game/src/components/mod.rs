use bevy::math::Quat;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MainCamera;

#[derive(Bundle, Debug, Default)]
pub struct FpsCameraBundle {
  pub fps_camera: FpsCameraState,
}

#[derive(Reflect, Component, Debug, Clone)]
#[reflect(Component)]
pub struct FpsCameraState {

  pub yaw: f32,
  pub pitch: f32,
  pub movement_speed: f32,
  pub movement_sensitivity: f32,
  pub zoom: f32,
}

impl Default for FpsCameraState {
  fn default() -> Self {
    let camera = Self {
      yaw: -90.0,
      pitch: 0.0,
      movement_speed: 2.5,
      movement_sensitivity: 5.0,
      zoom: 45.0,
    };
    camera
  }

}

impl FpsCameraState {
  pub fn set_from_rotation(&mut self, rotation: &Quat){
    let (yaw, pitch, _) = rotation.to_euler(EulerRot::YXZ);
    self.yaw = yaw.to_degrees();
    self.pitch = pitch.to_degrees();
  }

}
