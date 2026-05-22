use bevy::prelude::*;

#[derive(Component)]
pub struct Wall {
    pub half_size: Vec3,
}

pub fn check_collision<F: bevy::ecs::query::QueryFilter>(
    pos: Vec3, 
    player_radius: f32,
    wall_query: &Query<(&Transform, &Wall), F>
) -> bool {
    for (wall_transform, wall) in wall_query {
        let wall_pos = wall_transform.translation;
        // AABB vs Circle approximation (treating player as AABB for simplicity in this grid)
        let collision_x = (pos.x - wall_pos.x).abs() < (player_radius + wall.half_size.x);
        let collision_z = (pos.z - wall_pos.z).abs() < (player_radius + wall.half_size.z);
        
        if collision_x && collision_z {
            return true;
        }
    }
    false
}
