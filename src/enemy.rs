use bevy::prelude::*;
use crate::player::Player;
use crate::projectile::Projectile;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub patrol_points: Vec<Vec3>,
    pub current_waypoint: usize,
    pub health: f32,
}

pub fn move_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut enemy) in &mut enemy_query {
        if enemy.patrol_points.is_empty() {
            continue;
        }
        let target = enemy.patrol_points[enemy.current_waypoint];
        let diff = target - transform.translation;
        let dist = diff.length();

        if dist < 0.2 {
            enemy.current_waypoint = (enemy.current_waypoint + 1) % enemy.patrol_points.len();
        } else {
            let dir = diff.normalize();
            transform.translation += dir * enemy.speed * 2.0 * dt; // Scaled speed
            
            let target_rotation = Quat::from_rotation_y(f32::atan2(dir.x, dir.z));
            transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
        }
    }
}

pub fn check_enemy_projectile_collision(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
) {
    let enemy_radius = 0.4;
    let proj_radius = 0.3;

    for (enemy_entity, enemy_transform, mut enemy) in &mut enemy_query {
        let enemy_pos = enemy_transform.translation;
        for (proj_entity, proj_transform) in &projectile_query {
            let proj_pos = proj_transform.translation;
            let dist = enemy_pos.distance(proj_pos);

            if dist < (enemy_radius + proj_radius) {
                commands.entity(proj_entity).despawn();

                enemy.health -= 1.0;
                if enemy.health <= 0.0 {
                    commands.entity(enemy_entity).despawn();
                }
                break;
            }
        }
    }
}

pub fn check_enemy_player_collision(
    mut player_query: Query<(&mut Transform, &mut Player), Without<Enemy>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    let player_radius = 0.6;
    let enemy_radius = 0.4;
    
    for (mut player_transform, mut player) in &mut player_query {
        if player.invulnerable_timer > 0.0 || player.shield_timer > 0.0 {
            continue;
        }
        
        let player_pos = player_transform.translation;
        for enemy_transform in &enemy_query {
            let enemy_pos = enemy_transform.translation;
            let dist = player_pos.distance(enemy_pos);
            
            if dist < (player_radius + enemy_radius) {
                if player.health > 0 {
                    player.health -= 1;
                }
                player.invulnerable_timer = 1.5;
                
                // Knockback
                let push_dir = (player_pos - enemy_pos).normalize();
                player_transform.translation += Vec3::new(push_dir.x, 0.0, push_dir.z) * 1.6;
                break;
            }
        }
    }
}
