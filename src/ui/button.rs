use bevy::color::palettes::tailwind::*;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use sickle_ui::prelude::*;

pub fn text_btn_system(app: &mut App) {
	app
		.add_systems(Update, update_text_btn_interaction);
}

#[derive(Component, PartialEq, Eq)]
pub enum ClickMode {
	PointerUp,
	PointerDown
}
#[derive(Component)]
pub enum ButtonClickCallback<I: 'static, O: 'static, M, S: IntoSystem<I, O, M> + 'static> {
	Unregistered(S),
	SystemId(SystemId)
}

fn update_text_btn_interaction(mut commands: Commands, mut query: Query<(&Interaction, &mut ClickMode, &ButtonClickCallback), (Changed<Interaction>, With<TextButton>)>) {
	for (interaction, click_mode, button_click_callback) in query.iter_mut() {
		match interaction {
			Interaction::Pressed => {
				*click_mode = ClickMode::PointerDown;
			}
			Interaction::None => {
				*click_mode = ClickMode::PointerUp;
			}
			Interaction::Hovered => {
				if *click_mode == ClickMode::PointerDown {
					commands.run_system(button_click_callback.0);
				}
				*click_mode = ClickMode::PointerUp;
			}
		}
	}
}
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct TextButton;

impl TextButton {
    fn new(cb_system_id: SystemId) -> impl Bundle {
        (Name::new("Button"), ButtonBundle {
	        background_color: GRAY_300.into(),
	        ..default()
        }, ClickMode::PointerUp, ButtonClickCallback(cb_system_id))
    }
}

pub trait UiTextButtonExt {
	fn text_btn(
        &mut self,
        text: impl Into<String>,
        text_style: TextStyle,
        callback_system_id: SystemId
	) -> UiBuilder<Entity>;
}

impl UiTextButtonExt for UiBuilder<'_, Entity> {
	fn text_btn(
        &mut self,
        text: impl Into<String>,
        text_style: TextStyle,
        callback_system_id: SystemId
    ) -> UiBuilder<Entity> {
        let mut btn = self.spawn(TextButton::new(callback_system_id));
	    btn.spawn(TextBundle {
		    text: Text::from_section(text, text_style),
		    ..default()
	    });
	    btn
    }
}