use bevy::prelude::*;
use sls_bevy_game;
use sls_bevy_game::systems::{camera_systems, setup_3d_scene};

fn main() {
  let mut app = App::new();
  app
    .insert_resource(WindowDescriptor {
      title: "3d Demo App!".to_owned(),
      ..Default::default()
    })
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup_3d_scene)
    .add_system(camera_systems::camera_translate);
  app.run();
}
