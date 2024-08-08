use bevy::app::DynEq;
use bevy::prelude::*;
use bevy::utils::HashMap;


pub fn load_assets(server: Res<AssetServer>) {
	let _ = server.load_folder(".");
}