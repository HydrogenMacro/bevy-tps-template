use bevy::color::palettes::tailwind::*;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::reflect::array_apply;
use sickle_ui::prelude::*;
use crate::app_state::AppState;
use crate::ui::button::UiTextButtonExt;

#[derive(Component)]
pub struct StartButton;
pub fn loading_screen_plugin(app: &mut App) {
	app
		.add_systems(OnEnter(AppState::Loading), init_load_screen);
}
fn init_load_screen(world: &mut World) {
	let start_system_id = world.register_system(start_btn_cb);
	let mut commands = world.commands();
	commands.spawn((Camera2dBundle::default(), StateScoped(AppState::Loading)));
	let mut ui_builder = commands.ui_builder(UiRoot);
	let mut ui = ui_builder.column(|column| {
		column.label(LabelConfig {
				label: "Loading".to_string(),
				color: Color::srgb(0.5, 0.2, 0.3),
				..default()
			}).style().font_size(90.);
		let mut play_btn = column.text_btn("Play", TextStyle::default(), start_system_id);
		play_btn.style().padding(UiRect::all(Val::VMin(1.)));
		play_btn.insert(StartButton);
	});
	ui.insert(StateScoped(AppState::Loading));
	ui.style().width(Val::Vw(100.)).align_items(AlignItems::Center).justify_content(JustifyContent::Center);
	world.flush();
}
fn start_btn_cb(mut next_state: ResMut<NextState<AppState>>) {
	dbg!(0);
	next_state.set(AppState::MainMenu);
}