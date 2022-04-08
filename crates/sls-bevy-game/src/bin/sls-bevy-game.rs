use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use sls_bevy_game;
use sls_bevy_game::components::FpsCameraState;
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
    .add_plugin(sls_bevy_game::infinite_scroll_game::InfiniteScrollGamePlugin::default())
    .add_plugin(WorldInspectorPlugin::new());
  app.run();
}
