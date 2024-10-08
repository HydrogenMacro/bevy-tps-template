use bevy::prelude::*;
use crate::app_state::AppState;

#[derive(SubStates, Default, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Clone)]
#[source(AppState = AppState::InGame)]
pub enum GameState {
	#[default]
	Loading
}
