use bevy::prelude::*;
use sickle_ui::prelude::*;
use crate::scene_state::SceneState;
use crate::ui::button::UiTextButtonExt;


#[derive(Component)]
pub struct PlayBtn;

pub fn main_menu_plugin(app: &mut App) {
	app.add_systems(OnEnter(SceneState::MainMenu), init_main_menu)
		.add_systems(Update, update_ui.run_if(in_state(SceneState::MainMenu)));
}
fn init_main_menu(mut commands: Commands) {
	commands.spawn((Camera2dBundle::default(), StateScoped(SceneState::MainMenu)));
	commands.ui_builder(UiRoot).column(|column| {
		column.label(LabelConfig {
			label: "Main Menu".to_string(),
			..default()
		});
		column.text_btn("Abc", Default::default(), ).insert(PlayBtn);
	}).insert(StateScoped(SceneState::MainMenu));
}
fn update_ui(query: Query<&Interaction, (Changed<Interaction>, With<PlayBtn>)>, mut next_state: ResMut<NextState<SceneState>>) {
	if query.is_empty() { return; }
	let interaction = query.single();
	match interaction {
		Interaction::Pressed => {
			next_state.set(SceneState::Loading);
		}
		_ => {}
	}
}