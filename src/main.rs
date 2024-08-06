mod scene_state;
mod scenes;
mod game;
mod ui;

use bevy::prelude::*;
use sickle_ui::SickleUiPlugin;
use crate::scene_state::SceneState;
use crate::scenes::loading_screen_plugin;
use crate::ui::button::text_btn_system;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(SickleUiPlugin)
		.init_state::<SceneState>()
		.enable_state_scoped_entities::<SceneState>()
		.add_plugins(text_btn_system)
		.add_plugins(loading_screen_plugin)
		.run();
}
