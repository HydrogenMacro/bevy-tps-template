use bevy::prelude::*;

pub fn init_camera(mut commands: Commands) {
	commands.spawn(Camera3dBundle::default());
}