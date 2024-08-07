use bevy::prelude::States;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
	#[default]
	Loading,
	MainMenu,
	InGame
}