use bevy::prelude::*;
use sickle_ui::prelude::*;
use crate::scene_state::SceneState;

pub fn loading_screen_plugin(app: &mut App) {
	app
		.add_systems(Startup, init_load_screen.run_if(in_state(SceneState::Loading)));
}
fn init_load_screen(mut commands: Commands) {
	commands.ui_builder(UiRoot).column(|column| {
		column.label(LabelConfig {
				label: "Loading".to_string(),
				color: Color::srgb(0.5, 0.2, 0.3),
				..default()
			}).style().font_size(90.);
		column.checkbox(Some("ABC".into()), false);
	}).style().width(Val::Vw(100.)).align_items(AlignItems::Center).justify_content(JustifyContent::Center);
}