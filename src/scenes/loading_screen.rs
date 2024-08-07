use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use bevy::reflect::array_apply;
use sickle_ui::prelude::*;
use crate::scene_state::SceneState;
use crate::ui::button::UiTextButtonExt;

#[derive(Component)]
pub struct StartButton;
pub fn loading_screen_plugin(app: &mut App) {
	app
		.add_systems(OnEnter(SceneState::Loading), init_load_screen)
		.add_systems(Update, update_ui.run_if(in_state(SceneState::Loading)));
}
fn init_load_screen(mut commands: Commands) {
	commands.spawn((Camera2dBundle::default(), StateScoped(SceneState::Loading)));
	let mut ui_builder = commands.ui_builder(UiRoot);
	let mut ui = ui_builder.column(|column| {
		column.label(LabelConfig {
				label: "Loading".to_string(),
				color: Color::srgb(0.5, 0.2, 0.3),
				..default()
			}).style().font_size(90.);
		let mut play_btn = column.text_btn("Play");
		play_btn.style().padding(UiRect::all(Val::VMin(1.)));
		play_btn.insert(StartButton);
	});
	ui.insert(StateScoped(SceneState::Loading));
	ui.style().width(Val::Vw(100.)).align_items(AlignItems::Center).justify_content(JustifyContent::Center);
}
fn update_ui(start_btn_interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>, mut next_state: ResMut<NextState<SceneState>>) {
	if start_btn_interaction_query.is_empty() { return }
	let start_btn_interaction = start_btn_interaction_query.single();
	match start_btn_interaction {
		Interaction::Pressed => {
			next_state.set(SceneState::MainMenu);
		}
		_ => {}
	}
}