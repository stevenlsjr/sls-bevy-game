use bevy::prelude::*;
use sls_bevy_game;
use sls_bevy_game::systems::{camera_systems, setup_3d_scene};
use bevy_inspector_egui::WorldInspectorPlugin;
use sls_bevy_game::components::FpsCameraState;

fn main() {
  let mut app = App::new();
  app
    .insert_resource(WindowDescriptor {
      title: "3d Demo App!".to_owned(),
      ..Default::default()
    })
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .register_type::<FpsCameraState>()
    .add_startup_system(setup_3d_scene)
    .add_system(camera_systems::camera_translate)
    .add_system(camera_systems::camera_rotate)
    .add_system(camera_systems::update_camera_rotation)
    .add_plugin(WorldInspectorPlugin::new())
  ;
  app.run();
}
