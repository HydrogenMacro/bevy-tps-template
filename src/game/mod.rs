pub mod game_state;
pub mod player;
pub mod world;
mod camera;

use bevy::prelude::*;
use crate::app_state::AppState;
use crate::game::camera::init_camera;
use crate::game::world::init_ground;

pub fn game_plugin(app: &mut App) {
	app.add_systems(OnEnter(AppState::InGame), (init_ground, init_camera));
}