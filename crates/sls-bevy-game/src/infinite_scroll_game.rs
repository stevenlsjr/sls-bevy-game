use bevy::prelude::*;

#[derive(Default)]
pub struct InfiniteScrollGamePlugin {}

impl Plugin for InfiniteScrollGamePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(PlayerScore { distance: 0 })
      .add_startup_system(game_setup_system);
  }
  fn name(&self) -> &str {
    "InfiniteScrollGamePlugin"
  }
}

#[derive(Default, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct PlayerScore {
  pub distance: u64,
}


pub fn game_setup_system(mut commands: Commands) {}
