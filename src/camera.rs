use bevy::prelude::*;
use crate::player::Player;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(7.0, 15.0, 20.0).looking_at(Vec3::new(7.0, 0.0, 7.0), Vec3::Y),
        MainCamera,
    ));
}

pub fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    for player_transform in &player_query {
        for mut camera_transform in &mut camera_query {
            let target = player_transform.translation + Vec3::new(0.0, 8.0, 6.0);
            camera_transform.translation = camera_transform.translation.lerp(target, 0.1);
            camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}
