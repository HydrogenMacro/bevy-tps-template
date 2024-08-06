use bevy::prelude::*;
use sickle_ui::prelude::*;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct TextButton;

impl TextButton {
    fn new() -> impl Bundle {
        (Name::new("Button"), ButtonBundle::default())
    }
}

pub trait UiTextButtonExt {
    fn my_widget(
        &mut self,
        text: Option<String>,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl UiTextButtonExt for UiBuilder<'_, Entity> {
    fn my_widget(
        &mut self,
        text: impl Into<String>,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        let mut btn = self.container(TextButton::new(), spawn_children);
	    btn.spawn(TextBundle {
		    text: Text::from_section(text, TextStyle {
			    font: Default::default(),
			    font_size: 0.0,
			    color: Default::default(),
		    }),
		    ..default()
	    });
	    btn

    }
}