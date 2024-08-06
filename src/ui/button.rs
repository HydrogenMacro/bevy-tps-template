use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use sickle_ui::prelude::*;

pub fn text_btn_system(app: &mut App) {
	app
		.add_systems(Update, update_text_btn_interaction);
}
fn update_text_btn_interaction(query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<TextButton>)>) {
	for (interaction, background_color) in query.iter() {
		match interaction {
			Interaction::Pressed => {}
			Interaction::Hovered => {}
			Interaction::None => {}
		}
	}
}
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct TextButton;

impl TextButton {
    fn new() -> impl Bundle {
        (Name::new("Button"), ButtonBundle {
	        background_color: GRAY_300.into(),
	        ..default()
        })
    }
}

pub trait UiTextButtonExt {
    fn text_btn(
        &mut self,
        text: impl Into<String>,
    ) -> UiBuilder<Entity>;
	fn text_btn_ext(
        &mut self,
        text: impl Into<String>,
        text_style: TextStyle,
    ) -> UiBuilder<Entity>;
}

impl UiTextButtonExt for UiBuilder<'_, Entity> {
	fn text_btn(
        &mut self,
        text: impl Into<String>,
    ) -> UiBuilder<Entity> {
        let mut btn = self.spawn(TextButton::new());
	    btn.spawn(TextBundle {
		    text: Text::from_section(text, TextStyle::default()),
		    ..default()
	    });
	    btn
    }
	fn text_btn_ext(
        &mut self,
        text: impl Into<String>,
        text_style: TextStyle,
    ) -> UiBuilder<Entity> {
        let mut btn = self.spawn(TextButton::new());
	    btn.spawn(TextBundle {
		    text: Text::from_section(text, text_style),
		    ..default()
	    });
	    btn
    }
}