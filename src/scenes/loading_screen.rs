use bevy::prelude::*;
use sickle_ui::prelude::*;
use crate::scene_state::SceneState;

#[derive(Component)]
pub struct StartButton;
pub fn loading_screen_plugin(app: &mut App) {
	app
		.add_systems(Startup, init_load_screen.run_if(in_state(SceneState::Loading)))
		.add_systems(Update, update_ui.run_if(in_state(SceneState::Loading)));
}
fn init_load_screen(mut commands: Commands) {
	let mut ui_builder = commands.ui_builder(UiRoot);
	let mut ui = ui_builder.column(|column| {
		column.label(LabelConfig {
				label: "Loading".to_string(),
				color: Color::srgb(0.5, 0.2, 0.3),
				..default()
			}).style().font_size(90.);
		column.checkbox(Some("ABC".into()), false).insert(StartButton);
	});
	ui.insert(StateScoped(SceneState::Loading));
	ui.style().width(Val::Vw(100.)).align_items(AlignItems::Center).justify_content(JustifyContent::Center);
}
fn update_ui(checkbox_query: Query<&Checkbox, With<StartButton>>, mut next_state: ResMut<NextState<SceneState>>) {
	let checkbox = checkbox_query.single();
	if checkbox.checked {
		next_state.set(SceneState::MainMenu);
	}
}