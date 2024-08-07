mod scenes;
mod game;
mod ui;
mod assets;
mod app_state;

use bevy::prelude::*;
use sickle_ui::SickleUiPlugin;
use crate::app_state::AppState;
use crate::game::game_state::GameState;
use crate::scenes::loading_screen::loading_screen_plugin;
use crate::scenes::main_menu::main_menu_plugin;
use crate::ui::button::text_btn_plugin;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(SickleUiPlugin)
		.init_state::<AppState>()
		.add_sub_state::<GameState>()
		.enable_state_scoped_entities::<AppState>()
		.add_plugins(text_btn_plugin)
		.add_plugins((
			loading_screen_plugin,
			main_menu_plugin
		))
		.run();
}
