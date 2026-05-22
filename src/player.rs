use bevy::prelude::*;
use crate::collision::{check_collision, Wall};
use crate::maze::terrain_height;
use crate::platform::MovingPlatform;
use avian3d::prelude::{RigidBody, Collider, LinearVelocity, AngularVelocity};
use crate::tutorial::PhysicsCube;

#[derive(Component)]
pub struct Player {
    pub velocity_y: f32,
    pub is_grounded: bool,
    pub health: f32,
    pub max_health: f32,
    pub invulnerable_timer: f32,
    pub score: u32,
    pub speed_boost_timer: f32,
    pub shield_timer: f32,
    pub held_cube: Option<Entity>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            velocity_y: 0.0,
            is_grounded: true,
            health: 10.0,
            max_health: 10.0,
            invulnerable_timer: 0.0,
            score: 0,
            speed_boost_timer: 0.0,
            shield_timer: 0.0,
            held_cube: None,
        }
    }
}

#[derive(Component)]
pub struct PlayerAnimation {
    pub walk_node: AnimationNodeIndex,
    pub idle_node: AnimationNodeIndex,
}

#[derive(Component)]
pub struct AnimationPlayerLink(pub Entity);

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Start position in the tutorial zone
    let start_pos = crate::tutorial::TUTORIAL_OFFSET + Vec3::new(0.0, 1.0, 0.0);

    // Load animations
    let walk_anim = asset_server.load(GltfAssetLabel::Animation(1).from_asset("models/fox.glb"));
    let idle_anim = asset_server.load(GltfAssetLabel::Animation(0).from_asset("models/fox.glb"));
    
    let mut graph = AnimationGraph::new();
    let idle_node = graph.add_clip(idle_anim, 1.0, graph.root); 
    let walk_node = graph.add_clip(walk_anim, 1.0, graph.root);
    
    let graph_handle = graphs.add(graph);

    commands.spawn((
        SceneRoot(asset_server.load("models/fox.glb#Scene0")),
        Transform::from_translation(start_pos).with_scale(Vec3::splat(0.015)),
        Player::default(),
        PlayerAnimation {
            walk_node,
            idle_node,
        },
        AnimationGraphHandle(graph_handle),
        RigidBody::Kinematic,
        Collider::cuboid(0.8, 1.8, 0.8),
    ));
}

pub fn link_player_animations(
    mut commands: Commands,
    player_query: Query<(Entity, &AnimationGraphHandle), (With<Player>, Without<AnimationPlayerLink>)>,
    children_query: Query<&Children>,
    anim_player_query: Query<Entity, With<AnimationPlayer>>,
) {
    for (player_entity, graph_handle) in &player_query {
        for descendant in children_query.iter_descendants(player_entity) {
            if anim_player_query.get(descendant).is_ok() {
                commands.entity(descendant).insert((
                    graph_handle.clone(),
                    AnimationTransitions::default(),
                ));
                commands.entity(player_entity).insert(AnimationPlayerLink(descendant));
                break;
            }
        }
    }
}

pub fn cleanup_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn can_move_cube(
    cube_entity: Entity,
    direction: Vec3,
    cube_query: &Query<(Entity, &Transform, &PhysicsCube), Without<Player>>,
    wall_query: &Query<(&Transform, &Wall), (With<Wall>, Without<Player>)>,
) -> bool {
    let Ok((_, cube_transform, _)) = cube_query.get(cube_entity) else {
        return false;
    };
    let test_delta = direction * 0.2;
    let next_pos = cube_transform.translation + test_delta;

    // Check collision with walls
    if check_collision(next_pos, 0.5, wall_query) {
        return false;
    }

    // Check collision with other cubes
    for (other_entity, other_transform, other_cube) in cube_query {
        if other_entity == cube_entity || other_cube.is_held {
            continue;
        }
        let other_pos = other_transform.translation;
        let collision_x = (next_pos.x - other_pos.x).abs() < 1.0;
        let collision_z = (next_pos.z - other_pos.z).abs() < 1.0;
        let collision_y = (next_pos.y - other_pos.y).abs() < 1.0;

        if collision_x && collision_z && collision_y {
            // Recursively check if the other cube can move in the same direction
            if !can_move_cube(other_entity, direction, cube_query, wall_query) {
                return false;
            }
        }
    }

    true
}

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player, &PlayerAnimation, Option<&AnimationPlayerLink>)>,
    mut anim_player_query: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    wall_query: Query<(&Transform, &Wall), (With<Wall>, Without<Player>)>,
    platform_query: Query<(&Transform, &MovingPlatform), Without<Player>>,
    cube_query: Query<(Entity, &Transform, &PhysicsCube), Without<Player>>,
    mut cube_velocity_query: Query<&mut LinearVelocity, With<PhysicsCube>>,
) {
    let dt = time.delta_secs();
    for (mut player_transform, mut player, anim, anim_link) in &mut player_query {
        if player.invulnerable_timer > 0.0 {
            player.invulnerable_timer -= dt;
        }
        if player.speed_boost_timer > 0.0 {
            player.speed_boost_timer -= dt;
        }
        if player.shield_timer > 0.0 {
            player.shield_timer -= dt;
        }

        // Falling off the world (though with terrain it's harder)
        if player_transform.translation.y < -10.0 {
            if player.health > 0.0 {
                player.health -= 1.5;
            }
            player.invulnerable_timer = 1.5;
            player_transform.translation = Vec3::new(0.0, 10.0, 0.0); 
            player.velocity_y = 0.0;
        }

        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) { direction.z -= 1.0; }
        if keyboard_input.pressed(KeyCode::KeyS) { direction.z += 1.0; }
        if keyboard_input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if keyboard_input.pressed(KeyCode::KeyD) { direction.x += 1.0; }

        let mut speed = 8.0;
        if player.speed_boost_timer > 0.0 {
            speed = 16.0;
        }

        // Check terrain height at current position
        let current_x = player_transform.translation.x;
        let current_z = player_transform.translation.z;
        let floor_h = terrain_height(current_x, current_z);

        // Check if player is on a moving platform
        let mut on_platform = false;
        let mut platform_delta = Vec3::ZERO;
        let player_pos = player_transform.translation;

        for (plat_transform, plat) in &platform_query {
            let plat_pos = plat_transform.translation;
            let dx = (player_pos.x - plat_pos.x).abs();
            let dz = (player_pos.z - plat_pos.z).abs();

            if dx < 1.4 && dz < 1.4 {
                let dy = player_pos.y - plat_pos.y;
                if dy >= -0.1 && dy <= 0.5 {
                    on_platform = true;
                    platform_delta = plat.delta;
                    player_transform.translation.y = plat_pos.y + 0.15;
                    break;
                }
            }
        }

        if on_platform {
            player.is_grounded = true;
            player.velocity_y = 0.0;
            player_transform.translation += platform_delta;
        } else if player_transform.translation.y <= floor_h {
            player_transform.translation.y = floor_h;
            player.is_grounded = true;
            player.velocity_y = 0.0;
        } else {
            player.is_grounded = false;
            player.velocity_y -= 19.8 * dt;
            player_transform.translation.y += player.velocity_y * dt;

            if player_transform.translation.y < floor_h {
                player_transform.translation.y = floor_h;
                player.is_grounded = true;
                player.velocity_y = 0.0;
            }
        }

        if keyboard_input.just_pressed(KeyCode::Space) && player.is_grounded {
            player.velocity_y = 10.0;
            player.is_grounded = false;
            player_transform.translation.y += 0.1;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            
            let target_rotation = Quat::from_rotation_y(f32::atan2(direction.x, direction.z));
            player_transform.rotation = player_transform.rotation.slerp(target_rotation, 0.2);

            let mut new_pos = player_transform.translation;
            let player_radius = 0.4;
            
            let mut test_pos_x = Vec3::new(new_pos.x + direction.x * speed * dt, new_pos.y, new_pos.z);
            if !check_collision(test_pos_x, player_radius, &wall_query) {
                let dir_x = Vec3::new(direction.x, 0.0, 0.0).normalize_or_zero();
                if dir_x != Vec3::ZERO {
                    // Check collision with physics cubes
                    for (cube_entity, cube_transform, cube) in &cube_query {
                        if cube.is_held {
                            continue;
                        }
                        let cube_pos = cube_transform.translation;
                        let dx = test_pos_x.x - cube_pos.x;
                        let dz = test_pos_x.z - cube_pos.z;
                        let dy = test_pos_x.y - cube_pos.y;
                        
                        let limit_x = player_radius + 0.5;
                        let limit_z = player_radius + 0.5;
                        let limit_y = 0.9 + 0.5;

                        if dx.abs() < limit_x && dz.abs() < limit_z && dy.abs() < limit_y {
                            // Check if this cube can move in dir_x
                            let can_push = can_move_cube(cube_entity, dir_x, &cube_query, &wall_query);
                            if !can_push {
                                // Blocked! Stop the player completely (0.01 max penetration)
                                let allowed_dist = limit_x - 0.01;
                                if dx < 0.0 {
                                    test_pos_x.x = cube_pos.x - allowed_dist;
                                } else {
                                    test_pos_x.x = cube_pos.x + allowed_dist;
                                }
                            } else {
                                if let Ok(mut vel) = cube_velocity_query.get_mut(cube_entity) {
                                    vel.x = dir_x.x * speed;
                                }
                            }
                        }
                    }
                }
                new_pos.x = test_pos_x.x;
            }

            let mut test_pos_z = Vec3::new(new_pos.x, new_pos.y, new_pos.z + direction.z * speed * dt);
            if !check_collision(test_pos_z, player_radius, &wall_query) {
                let dir_z = Vec3::new(0.0, 0.0, direction.z).normalize_or_zero();
                if dir_z != Vec3::ZERO {
                    // Check collision with physics cubes
                    for (cube_entity, cube_transform, cube) in &cube_query {
                        if cube.is_held {
                            continue;
                        }
                        let cube_pos = cube_transform.translation;
                        let dx = test_pos_z.x - cube_pos.x;
                        let dz = test_pos_z.z - cube_pos.z;
                        let dy = test_pos_z.y - cube_pos.y;

                        let limit_x = player_radius + 0.5;
                        let limit_z = player_radius + 0.5;
                        let limit_y = 0.9 + 0.5;

                        if dx.abs() < limit_x && dz.abs() < limit_z && dy.abs() < limit_y {
                            // Check if this cube can move in dir_z
                            let can_push = can_move_cube(cube_entity, dir_z, &cube_query, &wall_query);
                            if !can_push {
                                // Blocked! Stop the player completely (0.01 max penetration)
                                let allowed_dist = limit_z - 0.01;
                                if dz < 0.0 {
                                    test_pos_z.z = cube_pos.z - allowed_dist;
                                } else {
                                    test_pos_z.z = cube_pos.z + allowed_dist;
                                }
                            } else {
                                if let Ok(mut vel) = cube_velocity_query.get_mut(cube_entity) {
                                    vel.z = dir_z.z * speed;
                                }
                            }
                        }
                    }
                }
                new_pos.z = test_pos_z.z;
            }

            player_transform.translation = new_pos;
            
            if let Some(link) = anim_link {
                if let Ok((mut anim_player, mut transitions)) = anim_player_query.get_mut(link.0) {
                    transitions.play(&mut anim_player, anim.walk_node, std::time::Duration::from_millis(200)).repeat();
                }
            }
        } else {
            if let Some(link) = anim_link {
                if let Ok((mut anim_player, mut transitions)) = anim_player_query.get_mut(link.0) {
                    transitions.play(&mut anim_player, anim.idle_node, std::time::Duration::from_millis(200)).repeat();
                }
            }
        }
    }
}

pub fn player_grab_block(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &mut Player)>,
    mut cube_query: Query<(Entity, &mut Transform, &mut PhysicsCube), Without<Player>>,
) {
    let ctrl_pressed = keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight);

    for (player_transform, mut player) in &mut player_query {
        if let Some(held_entity) = player.held_cube {
            // Check if we should release it
            if !ctrl_pressed {
                // Release the cube
                if let Ok((_, _, mut cube)) = cube_query.get_mut(held_entity) {
                    cube.is_held = false;
                    commands.entity(held_entity).insert((
                        RigidBody::Dynamic,
                        LinearVelocity::ZERO,
                        AngularVelocity::ZERO,
                    ));
                }
                player.held_cube = None;
            } else {
                // Update held cube position/rotation
                if let Ok((_, mut cube_transform, _)) = cube_query.get_mut(held_entity) {
                    // Position 1.5 units in front of player, 1.0 unit up
                    let target_pos = player_transform.translation + player_transform.rotation * Vec3::new(0.0, 1.0, 1.5);
                    cube_transform.translation = target_pos;
                    cube_transform.rotation = player_transform.rotation;
                } else {
                    // Held entity might have been despawned
                    player.held_cube = None;
                }
            }
        } else if ctrl_pressed {
            // Try to grab the closest cube
            let mut closest_cube: Option<(Entity, f32)> = None;
            for (cube_entity, cube_transform, _) in cube_query.iter() {
                let dist = player_transform.translation.distance(cube_transform.translation);
                // Grab range: 3.0 units
                if dist < 3.0 {
                    if let Some((_, closest_dist)) = closest_cube {
                        if dist < closest_dist {
                            closest_cube = Some((cube_entity, dist));
                        }
                    } else {
                        closest_cube = Some((cube_entity, dist));
                    }
                }
            }

            if let Some((cube_entity, _)) = closest_cube {
                player.held_cube = Some(cube_entity);
                if let Ok((_, _, mut cube)) = cube_query.get_mut(cube_entity) {
                    cube.is_held = true;
                }
                commands.entity(cube_entity).insert((
                    RigidBody::Kinematic,
                    LinearVelocity::ZERO,
                    AngularVelocity::ZERO,
                ));
            }
        }
    }
}
