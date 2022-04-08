use crate::components::{FpsCameraBundle, FpsCameraState, MainCamera};
use bevy::gltf::Gltf;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::render_graph::NodeLabel;
use std::default::default;

#[allow(unused_mut, unused_variables)]
pub fn setup_system(mut commands: Commands) {}

pub fn setup_3d_scene(
  mut commands: Commands,
  assets: Res<AssetServer>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let gltf_scene = assets.load("BoomBox.glb#Scene0");
  commands
    .spawn_bundle((
      Transform::from_xyz(0.0, 0.5, 0.0).with_scale(
        Vec3::ONE * 20.0),
      GlobalTransform::default(),
      Name::new("My GLTF scene"),
    ))
    .with_children(|parent| {
      parent.spawn_scene(gltf_scene);
    });
  // plane
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });
  // // cube
  // commands.spawn_bundle(PbrBundle {
  //   mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
  //   material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
  //   transform: Transform::from_xyz(0.0, 0.5, 0.0),
  //   ..default()
  // });
  // light
  commands.spawn_bundle(PointLightBundle {
    point_light: PointLight {
      intensity: 1500.0,
      shadows_enabled: true,
      ..default()
    },
    transform: Transform::from_xyz(0.0, 5.0, 3.0),
    ..default()
  });
  // camera
  let transform = Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
  let mut fps_camera = FpsCameraState {
    yaw: 0.0,
    pitch: -10.0,
    ..default()
  };
  fps_camera.set_from_rotation(&transform.rotation);
  commands
    .spawn_bundle(PerspectiveCameraBundle {
      transform,
      ..default()
    })
    .insert_bundle(FpsCameraBundle { fps_camera })
    .insert(MainCamera);
}
