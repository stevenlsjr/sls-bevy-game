use crate::components::{FpsCameraState, MainCamera};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;

pub fn camera_translate(
  mut query: Query<(&FpsCameraState, &mut Transform)>,
  key_input: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  let mut movement_input = IVec2::new(0, 0);

  if key_input.pressed(KeyCode::W) {
    movement_input.y += 1;
  }
  if key_input.pressed(KeyCode::S) {
    movement_input.y -= 1;
  }
  if key_input.pressed(KeyCode::A) {
    movement_input.x -= 1;
  }
  if key_input.pressed(KeyCode::D) {
    movement_input.x += 1;
  }
  for (camera, mut transform) in query.iter_mut() {
    if (movement_input.x.abs() + movement_input.y.abs()) > 0 {
      let forward = transform.forward() * (movement_input.y as f32);
      let right = transform.right() * (movement_input.x as f32);
      let movement_step_3d =
        (forward + right).normalize_or_zero() * camera.movement_speed * time.delta_seconds();
      transform.translation += movement_step_3d;
    }
  }
}

pub fn camera_rotate(
  mut query: Query<(&mut FpsCameraState, &Camera), With<MainCamera>>,
  mouse_button: Res<Input<MouseButton>>,
  mut mouse_motion: EventReader<MouseMotion>,
  key: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  let motion = mouse_motion.iter().fold(vec2(0.0, 0.0), |a, b| a + b.delta);
  match query.get_single_mut() {
    Ok((mut camera_state, camera)) => {
      if key.pressed(KeyCode::LAlt) || mouse_button.pressed(MouseButton::Middle) {
        if motion.length() >= 0.01 {
          let rotation_delta = motion * time.delta_seconds() * camera_state.movement_sensitivity;
          camera_state.yaw += rotation_delta.x;
          camera_state.pitch = f32::clamp(camera_state.pitch + rotation_delta.y, -89.0, 89.0);
        }

      }
    }
    Err(e) => {
      // log::warn!("can't select valid main camera {:?}", e)
    }
  }
}

pub fn update_camera_rotation(mut query: Query<(&FpsCameraState, &mut Transform), With<MainCamera>>) {
  match query.get_single_mut() {
    Ok((camera_state, mut transform)) => {
      let pitch_r = camera_state.pitch.to_radians();
      let yaw_r = camera_state.yaw.to_radians();
      let rotation = Quat::from_euler(EulerRot::YXZ,  yaw_r, pitch_r, 0.0);
      *transform = transform.with_rotation(rotation);
    }
    Err(e) => {
      // log::warn!("can't select valid main camera {:?}", e)
    }
  }
}
