use bevy::prelude::*;

#[derive(Component)]
pub struct Wall;

pub fn check_collision(
    pos: Vec3, 
    player_radius: f32,
    wall_query: &Query<&Transform, (With<Wall>, Without<super::player::Player>)>
) -> bool {
    let wall_size = 1.0;

    for wall_transform in wall_query {
        let wall_pos = wall_transform.translation;
        let collision_x = (pos.x - wall_pos.x).abs() < (player_radius + wall_size);
        let collision_z = (pos.z - wall_pos.z).abs() < (player_radius + wall_size);
        if collision_x && collision_z {
            return true;
        }
    }
    false
}
