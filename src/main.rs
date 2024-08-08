mod scenes;
mod game;
mod ui;
mod assets;
mod app_state;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use sickle_ui::SickleUiPlugin;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::app_state::AppState;
use crate::assets::{load_assets};
use crate::game::game_plugin;
use crate::game::game_state::GameState;
use crate::scenes::loading_screen::loading_screen_plugin;
use crate::scenes::main_menu::main_menu_plugin;
use crate::ui::button::text_btn_plugin;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(SickleUiPlugin)
		.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
		.add_plugins(RapierDebugRenderPlugin::default())
		.add_plugins(WorldInspectorPlugin::default())
		.init_state::<AppState>()
		.add_sub_state::<GameState>()
		.enable_state_scoped_entities::<AppState>()
		.add_plugins(text_btn_plugin)
		.add_plugins((
			loading_screen_plugin,
			main_menu_plugin,
			game_plugin
		))
		.add_systems(Startup, load_assets)
		.run();
}
