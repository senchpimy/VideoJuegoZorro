use bevy::prelude::*;

#[derive(Component)]
pub struct MovingPlatform {
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    pub speed: f32,
    pub progress: f32,
    pub forward: bool,
    pub delta: Vec3,
}

pub fn move_platforms(
    time: Res<Time>,
    mut platform_query: Query<(&mut Transform, &mut MovingPlatform)>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut platform) in &mut platform_query {
        let direction = if platform.forward { 1.0 } else { -1.0 };
        platform.progress += direction * platform.speed * dt;

        if platform.progress >= 1.0 {
            platform.progress = 1.0;
            platform.forward = false;
        } else if platform.progress <= 0.0 {
            platform.progress = 0.0;
            platform.forward = true;
        }

        let target_pos = Vec3::lerp(platform.start_pos, platform.end_pos, platform.progress);
        platform.delta = target_pos - transform.translation;
        transform.translation = target_pos;
    }
}
