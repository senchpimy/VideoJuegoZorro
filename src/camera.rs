use crate::player::Player;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera {
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for MainCamera {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: -0.5,
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(7.0, 15.0, 20.0).looking_at(Vec3::new(7.0, 0.0, 7.0), Vec3::Y),
        MainCamera::default(),
    ));
}

pub fn camera_follow(
    mut motion_evr: MessageReader<MouseMotion>, // ← EventReader → MessageReader
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<(&mut Transform, &mut MainCamera)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok((mut camera_transform, mut camera_data)) = camera_query.single_mut() {
            let mut mouse_delta = Vec2::ZERO;
            for ev in motion_evr.read() {
                mouse_delta += ev.delta;
            }
            let sensitivity = 0.005;
            camera_data.yaw -= mouse_delta.x * sensitivity;
            camera_data.pitch = (camera_data.pitch - mouse_delta.y * sensitivity).clamp(-1.5, -0.1);
            let distance = 20.0;
            let rotation = Quat::from_euler(EulerRot::YXZ, camera_data.yaw, camera_data.pitch, 0.0);
            let offset = rotation * Vec3::new(0.0, 0.0, distance);
            let target = player_transform.translation + offset;
            camera_transform.translation = camera_transform.translation.lerp(target, 0.1);
            camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}
