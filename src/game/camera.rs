use bevy::core_pipeline::Skybox;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraAnchor;

#[derive(Reflect, Component)]
pub enum CameraTransform {
	Relative(Vec3, Quat),
	Absolute(Vec3, Quat)
}
pub fn init_camera(mut commands: Commands, mut asset_server: Res<AssetServer>) {
	commands.spawn(Camera3dBundle::default())
		.insert(MainCamera)
		.insert(CameraTransform::Absolute(Vec3::new(0., 10., 10.), Quat::from_rotation_x(90.)))
		.insert(Skybox {
			image: asset_server.load("skybox.png"),
			brightness: 1000.0,
		});
}
pub fn update_camera_position(
	mut camera_query: Query<(&mut Transform, &CameraTransform), With<(MainCamera)>>,
	anchor_query: Query<&Transform, (With<CameraAnchor>, Without<MainCamera>)>,
	time: Res<Time<Virtual>>
) {
	let time_since_last_update = time.delta_seconds();
	let (mut current_transform, target_transform) = camera_query.single_mut();
	let (absolute_target_transform, target_rotation) = match target_transform {
		CameraTransform::Relative(delta_transform, rotation) => {
			let anchor_transform = anchor_query.single();
			(anchor_transform.translation + delta_transform.clone(), *rotation)
		}
		CameraTransform::Absolute(transform, rotation) => {
			(*transform, *rotation)
		}
	};
	current_transform.translation = current_transform.translation.lerp(absolute_target_transform, time_since_last_update.min(1.));
	current_transform.rotation = current_transform.rotation.slerp(target_rotation, time_since_last_update.min(1.));
}