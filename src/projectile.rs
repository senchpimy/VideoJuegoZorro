use bevy::prelude::*;
use crate::player::Player;
use crate::collision::Wall;

#[derive(Component)]
pub struct Projectile {
    pub direction: Vec3,
    pub speed: f32,
    pub lifetime: Timer,
}

pub fn player_fire(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    _mouse_input: Res<ButtonInput<MouseButton>>,
    player_query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keyboard_input.just_pressed(KeyCode::ShiftLeft) || keyboard_input.just_pressed(KeyCode::ShiftRight) {
        if let Some(player_transform) = player_query.iter().next() {
            // Get forward vector based on player's current rotation
            let forward = player_transform.rotation * Vec3::new(0.0, 0.0, 1.0);
            let spawn_pos = player_transform.translation + forward * 1.0 + Vec3::new(0.0, 0.6, 0.0);

            let proj_mesh = meshes.add(Sphere::new(0.3).mesh());
            let proj_material = materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.5, 0.0),
                emissive: LinearRgba::from_f32_array([2.0, 1.0, 0.0, 1.0]),
                ..default()
            });

            commands.spawn((
                Mesh3d(proj_mesh),
                MeshMaterial3d(proj_material),
                Transform::from_translation(spawn_pos),
                Projectile {
                    direction: forward.normalize(),
                    speed: 16.0,
                    lifetime: Timer::from_seconds(2.0, TimerMode::Once),
                },
            ));
        }
    }
}

pub fn cleanup_projectiles(mut commands: Commands, query: Query<Entity, With<Projectile>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut Transform, &mut Projectile)>,
    wall_query: Query<(&Transform, &Wall), (With<Wall>, Without<Projectile>)>,
) {
    let dt = time.delta_secs();
    let proj_radius = 0.3;

    for (entity, mut transform, mut projectile) in &mut projectile_query {
        projectile.lifetime.tick(time.delta());
        if projectile.lifetime.is_finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // Move projectile
        transform.translation += projectile.direction * projectile.speed * dt;

        // Collision with walls
        let pos = transform.translation;
        let mut hit = false;
        for (wall_transform, wall) in &wall_query {
            let wall_pos = wall_transform.translation;
            let collision_x = (pos.x - wall_pos.x).abs() < (proj_radius + wall.half_size.x);
            let collision_z = (pos.z - wall_pos.z).abs() < (proj_radius + wall.half_size.z);
            let collision_y = pos.y >= 0.0 && pos.y <= 4.0;

            if collision_x && collision_z && collision_y {
                hit = true;
                break;
            }
        }

        if hit {
            commands.entity(entity).despawn();
        }
    }
}
