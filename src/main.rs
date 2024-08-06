mod scene_state;
mod scenes;
mod game;

use std::sync::{LazyLock, Mutex};
use bevy::prelude::*;
use sickle_ui::SickleUiPlugin;
use crate::scene_state::SceneState;
use crate::scenes::loading_screen_plugin;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(SickleUiPlugin)
		.init_state::<SceneState>()
		.enable_state_scoped_entities::<SceneState>()
		.add_plugins(loading_screen_plugin)
		.add_systems(Startup, add_camera)
		.run();
}
fn add_camera(mut commands: Commands) {
	commands.spawn(Camera3dBundle::default());
}
