use bevy::prelude::*;
use crate::collision::{check_collision, Wall};
use crate::maze::MAZE_DATA;
use crate::platform::MovingPlatform;

#[derive(Component)]
pub struct Player {
    pub velocity_y: f32,
    pub is_grounded: bool,
    pub health: u32,
    pub max_health: u32,
    pub invulnerable_timer: f32,
    pub score: u32,
    pub speed_boost_timer: f32,
    pub shield_timer: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            velocity_y: 0.0,
            is_grounded: true,
            health: 3,
            max_health: 3,
            invulnerable_timer: 0.0,
            score: 0,
            speed_boost_timer: 0.0,
            shield_timer: 0.0,
        }
    }
}

#[derive(Component)]
pub struct PlayerAnimation {
    pub walk_node: AnimationNodeIndex,
    pub idle_node: AnimationNodeIndex,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Find start position
    let mut start_pos = Vec3::ZERO;
    for (z, row) in MAZE_DATA.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 2 {
                start_pos = Vec3::new(x as f32, 0.0, z as f32);
            }
        }
    }

    // Load animations
    let walk_anim = asset_server.load(GltfAssetLabel::Animation(1).from_asset("models/fox.glb"));
    let idle_anim = asset_server.load(GltfAssetLabel::Animation(0).from_asset("models/fox.glb"));
    
    let mut graph = AnimationGraph::new();
    // In Bevy 0.18, add_clip might take (handle, weight, mask) or similar. 
    // The search said add_clip(handle, weight, mask).
    let idle_node = graph.add_clip(idle_anim, 1.0, graph.root); 
    // Wait, the search said graph.root is the target? No, it said add_clip(handle, 1.0, None) and then set_root.
    // Let's try to add nodes and connect them or just set one as root.
    // If I want to switch, I can use a blend node.
    
    let walk_node = graph.add_clip(walk_anim, 1.0, graph.root);
    // Actually, I'll just add them to the graph and use AnimationPlayer to play them.
    // In 0.18, AnimationPlayer::play(node_index) works if they are in the graph.
    
    let graph_handle = graphs.add(graph);

    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/fox.glb"))),
        Transform::from_translation(start_pos).with_scale(Vec3::splat(0.01)),
        Player::default(),
        PlayerAnimation {
            walk_node,
            idle_node,
        },
        AnimationGraphHandle(graph_handle),
    ));
}

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player, &PlayerAnimation, Entity)>,
    mut anim_player_query: Query<&mut AnimationPlayer>,
    children_query: Query<&Children>,
    wall_query: Query<&Transform, (With<Wall>, Without<Player>)>,
    platform_query: Query<(&Transform, &MovingPlatform), Without<Player>>,
) {
    let dt = time.delta_secs();
    for (mut player_transform, mut player, anim, entity) in &mut player_query {
        if player.invulnerable_timer > 0.0 {
            player.invulnerable_timer -= dt;
        }
        if player.speed_boost_timer > 0.0 {
            player.speed_boost_timer -= dt;
        }
        if player.shield_timer > 0.0 {
            player.shield_timer -= dt;
        }

        if player_transform.translation.y < -1.8 {
            if player.health > 0 {
                player.health -= 1;
            }
            player.invulnerable_timer = 1.5;
            player_transform.translation = Vec3::new(1.0, 1.0, 1.0);
            player.velocity_y = 0.0;
        }

        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) { direction.z -= 1.0; }
        if keyboard_input.pressed(KeyCode::KeyS) { direction.z += 1.0; }
        if keyboard_input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if keyboard_input.pressed(KeyCode::KeyD) { direction.x += 1.0; }

        let mut speed = 4.0;
        if player.speed_boost_timer > 0.0 {
            speed = 8.0;
        }

        // Check if player is on a moving platform
        let mut on_platform = false;
        let mut platform_delta = Vec3::ZERO;
        let player_pos = player_transform.translation;

        for (plat_transform, plat) in &platform_query {
            let plat_pos = plat_transform.translation;
            let dx = (player_pos.x - plat_pos.x).abs();
            let dz = (player_pos.z - plat_pos.z).abs();

            if dx < 0.7 && dz < 0.7 {
                let dy = player_pos.y - plat_pos.y;
                if dy >= -0.05 && dy <= 0.25 {
                    on_platform = true;
                    platform_delta = plat.delta;
                    player_transform.translation.y = plat_pos.y + 0.1;
                    break;
                }
            }
        }

        if on_platform {
            player.is_grounded = true;
            player.velocity_y = 0.0;
            player_transform.translation += platform_delta;
        } else if player_transform.translation.y <= 0.0 {
            player_transform.translation.y = 0.0;
            player.is_grounded = true;
            player.velocity_y = 0.0;
        } else {
            player.is_grounded = false;
            player.velocity_y -= 19.8 * dt;
            player_transform.translation.y += player.velocity_y * dt;

            if player_transform.translation.y < 0.0 {
                player_transform.translation.y = 0.0;
                player.is_grounded = true;
                player.velocity_y = 0.0;
            }
        }

        if keyboard_input.just_pressed(KeyCode::Space) && player.is_grounded {
            player.velocity_y = 7.0;
            player.is_grounded = false;
            player_transform.translation.y += 0.05;
        }

        // Find AnimationPlayer Entity
        let mut anim_player_entity = if anim_player_query.get(entity).is_ok() {
            Some(entity)
        } else {
            None
        };

        if anim_player_entity.is_none() {
            if let Ok(children) = children_query.get(entity) {
                for child in children.iter() {
                    if anim_player_query.get(child).is_ok() {
                        anim_player_entity = Some(child);
                        break;
                    }
                }
            }
        }

        let maybe_anim_player = anim_player_entity.and_then(|e| anim_player_query.get_mut(e).ok());

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            
            let target_rotation = Quat::from_rotation_y(f32::atan2(direction.x, direction.z));
            player_transform.rotation = player_transform.rotation.slerp(target_rotation, 0.2);

            let mut new_pos = player_transform.translation;
            let player_radius = 0.3;
            
            let test_pos_x = Vec3::new(new_pos.x + direction.x * speed * dt, new_pos.y, new_pos.z);
            if !check_collision(test_pos_x, player_radius, &wall_query) {
                new_pos.x = test_pos_x.x;
            }

            let test_pos_z = Vec3::new(new_pos.x, new_pos.y, new_pos.z + direction.z * speed * dt);
            if !check_collision(test_pos_z, player_radius, &wall_query) {
                new_pos.z = test_pos_z.z;
            }

            player_transform.translation = new_pos;
            
            if let Some(mut anim_player) = maybe_anim_player {
                anim_player.play(anim.walk_node).repeat();
            }
        } else {
            if let Some(mut anim_player) = maybe_anim_player {
                anim_player.play(anim.idle_node).repeat();
            }
        }
    }
}
