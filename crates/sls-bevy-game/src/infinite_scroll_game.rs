use bevy::prelude::*;
use bevy::render::render_phase::batch_phase_system;
use bevy_inspector_egui::{prelude::*, Inspectable};
use std::default::default;

use serde::{Deserialize, Serialize};

const PALLET_BEIGE: &'static str = "a08a40";
const PALLET_GREEN: &'static str = "86a040";
const PALLET_OCHRE: &'static str = "a05940";

#[derive(Default)]
pub struct PlayerScore {
  distance: u64,
}

#[derive(Debug, Component, Reflect)]
pub struct PlayerControlled;

#[derive(Inspectable, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum DinoState {
  Running,
  Jumping,
  Ducking,
  Dead,
}

impl Default for DinoState {
  fn default() -> Self {
    Self::Running
  }
}



#[derive(Inspectable, Debug, Clone, PartialEq)]
pub struct PhysicsSettings {
  // acceleration of gravity in the z axis
  g_factor: f32,
  // z coordinate for the ground level
  ground_y: f32,
}
impl Default for PhysicsSettings {
  fn default() -> Self {
    Self {
      g_factor: 9.0,
      ground_y: 0.0,
    }
  }
}

#[derive(Inspectable, Component, Clone, PartialEq, Serialize, Deserialize)]
pub struct DinoMotion {
  initial_jump_velocity: f32,
  velocity_y: f32,
}

impl Default for DinoMotion {
  fn default() -> Self {
    Self {
      initial_jump_velocity: 10.0,
      velocity_y: 0.0,
    }
  }
}

pub fn game_setup_system(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let dino_mesh = meshes.add(Mesh::from(shape::Capsule {
    radius: 1.0,
    depth: 2.0,
    ..Default::default()
  }));
  let dino_material = materials.add(StandardMaterial {
    base_color: Color::hex(PALLET_GREEN).unwrap(),
    // vary key PBR parameters on a grid of spheres to show the effect
    metallic: 0.9,
    perceptual_roughness: 0.1,
    ..default()
  });
  commands
    .spawn_bundle((
      PlayerControlled,
      DinoState::Running,
      DinoMotion::default(),
      GlobalTransform::default(),
      Transform {
        translation: [0.0, 0.0, 0.0].into(),
        ..default()
      },
      Name::new("Dino"),
    ))
    .with_children(|builder| {
      builder
        .spawn_bundle(PbrBundle {
          mesh: dino_mesh,
          material: dino_material,
          transform: Transform {
            translation: [0.0, 2.0, 0.0].into(),
            ..default()
          },
          global_transform: Default::default(),
          visibility: Default::default(),
          computed_visibility: Default::default(),
        })
        .insert(Name::new("Dino Mesh"));
    });

  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Plane { size: 30.0 })),
    material: materials.add(StandardMaterial {
      base_color: Color::hex(PALLET_OCHRE).unwrap(),

      ..default()
    }),
    transform: Transform {
      translation: [0.0, 0.0, 0.0].into(),
      ..default()
    },
    ..default()
  });

  const HALF_SIZE: f32 = 10.0;
  commands.spawn_bundle(DirectionalLightBundle {
    directional_light: DirectionalLight {
      shadow_projection: OrthographicProjection {
        left: -HALF_SIZE,
        right: HALF_SIZE,
        bottom: -HALF_SIZE,
        top: HALF_SIZE,
        near: -10.0 * HALF_SIZE,
        far: 10.0 * HALF_SIZE,
        ..default()
      },
      shadows_enabled: true,
      ..default()
    },
    transform: Transform {
      translation: Vec3::new(0.0, 2.0, 0.0),
      rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
      ..Default::default()
    },
    ..default()
  });

  commands.spawn_bundle(PerspectiveCameraBundle {
    transform: Transform::from_xyz(-0.0, 5.0, 10.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
    ..default()
  });
}
pub fn update_dino_state_system(
  mut query: Query<(&mut DinoState, &mut DinoMotion, &Transform)>,
  keyboard: Res<Input<KeyCode>>,
  physics: Res<PhysicsSettings>,
) {
  let is_space_pressed = keyboard.pressed(KeyCode::Space);
  let is_down_pressed = keyboard.pressed(KeyCode::Down);
  for entity_data in query.iter_mut() {
    let (mut dino_state, mut motion, transform) = entity_data;
    let current_state: DinoState = *dino_state;
    match current_state {
      DinoState::Running => {
        if is_space_pressed {
          *dino_state = DinoState::Jumping;
          motion.velocity_y = motion.initial_jump_velocity;
        } else if is_down_pressed {
          *dino_state = DinoState::Ducking
        }
      }
      DinoState::Jumping => {
        // stop jumping when z position is at or below ground and is moving downwards
        if (transform.translation.y <= physics.ground_y) && motion.velocity_y < 0.0 {
          *dino_state = DinoState::Running;
        }
      }
      DinoState::Ducking => {
        if is_space_pressed {
          *dino_state = DinoState::Jumping;
          motion.velocity_y = motion.initial_jump_velocity;
        } else if !is_down_pressed {
          *dino_state = DinoState::Running;
        }
      }
      _ => {}
    }
  }
}

pub fn update_mesh_shape_system(mut query: Query<(&DinoState, &mut Transform), Changed<DinoState>>) {

  for (state, mut transform) in query.iter_mut(){
    match state {
      DinoState::Ducking => {
        transform.scale.y = 0.5;
      }
      _ => {
        transform.scale.y = 1.0;
      }
    }
  }
}


pub(crate) fn move_dino_system(
  mut query: Query<(&DinoState, &mut DinoMotion, &mut Transform)>,
  physics: Res<PhysicsSettings>,
  time: Res<Time>,
) {
  for item in query.iter_mut() {
    let (state, mut motion, mut transform): (&DinoState, Mut<DinoMotion>, Mut<Transform>) = item;
    match state {
      DinoState::Ducking | DinoState::Running => {
        transform.translation.y = physics.ground_y;
      }
      DinoState::Jumping => {
        let delta_y = motion.velocity_y * time.delta_seconds();
        let delta_vy = physics.g_factor * time.delta_seconds();
        transform.translation.y += delta_y;
        motion.velocity_y -= delta_vy;
      }
      _ => {}
    }
  }
}

#[derive(Inspectable, PartialOrd, PartialEq, Eq, Debug, Hash, Clone, SystemLabel)]
pub enum DinoSystemLabels {
  UpdateDinoState,
  MoveDino,
}

#[derive(Default)]
pub struct InfiniteScrollGamePlugin {}
impl Plugin for InfiniteScrollGamePlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<PlayerControlled>();
    app.register_inspectable::<DinoState>();
    app.register_inspectable::<DinoMotion>();
    app.register_inspectable::<PhysicsSettings>();
    app
      .insert_resource(PlayerScore { distance: 0 })
      .insert_resource(PhysicsSettings { ..default() })
      .add_startup_system(game_setup_system)
      .add_system(update_dino_state_system.label(DinoSystemLabels::UpdateDinoState))
      .add_system(update_mesh_shape_system.after(DinoSystemLabels::UpdateDinoState))
      .add_system(
        move_dino_system
          .label(DinoSystemLabels::MoveDino)
          .after(DinoSystemLabels::UpdateDinoState),
      );
  }
  fn name(&self) -> &str {
    "InfiniteScrollGamePlugin"
  }
}
