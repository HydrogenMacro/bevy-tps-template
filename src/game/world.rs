use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

pub fn init_ground(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>
) {
	commands.spawn(PbrBundle {
		mesh: meshes.add(Circle::new(3.)),
		material: materials.add(Color::from(RED_500)),
		transform: Transform::from_xyz(0., 0., -10.),
		..default()
	});
}