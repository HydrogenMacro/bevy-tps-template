use bevy::prelude::*;
use sickle_ui::prelude::*;
use crate::app_state::AppState;
use crate::ui::button::UiTextButtonExt;


#[derive(Component)]
pub struct PlayBtn;

pub fn main_menu_plugin(app: &mut App) {
	app.add_systems(OnEnter(AppState::MainMenu), init_main_menu);
}
fn init_main_menu(world: &mut World) {
	let play_system_id = world.register_system(play_btn_cb);
	let mut commands = world.commands();
	commands.spawn((Camera2dBundle::default(), StateScoped(AppState::MainMenu)));
	commands.ui_builder(UiRoot).column(|column| {
		column.label(LabelConfig {
			label: "Main Menu".to_string(),
			..default()
		});
		column.text_btn("Abc", Default::default(), play_system_id).insert(PlayBtn);
	}).insert(StateScoped(AppState::MainMenu));
	world.flush();
}
fn play_btn_cb(mut next_state: ResMut<NextState<AppState>>) {
	next_state.set(AppState::InGame);
}