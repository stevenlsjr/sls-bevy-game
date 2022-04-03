use crate::components::FpsCameraState;
use bevy::math::vec3;
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
