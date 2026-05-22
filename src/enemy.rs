use bevy::prelude::*;
use crate::player::Player;
use crate::projectile::Projectile;

use bevy::gltf::Gltf;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub patrol_points: Vec<Vec3>,
    pub current_waypoint: usize,
    pub health: f32,
}

#[derive(Component)]
pub struct EnemyAnimation {
    pub start_node: AnimationNodeIndex,
}

#[derive(Component)]
pub struct EnemyAnimationPlayerLink(pub Entity);

#[derive(Resource, Default)]
pub struct EnemyAnimationAssets {
    pub gltf_handle: Handle<Gltf>,
    pub graph_handle: Option<Handle<AnimationGraph>>,
    pub start_node: Option<AnimationNodeIndex>,
}

pub fn setup_enemy_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let gltf_handle = asset_server.load("models/scorcher_enemy.glb");
    commands.insert_resource(EnemyAnimationAssets {
        gltf_handle,
        graph_handle: None,
        start_node: None,
    });
}

pub fn process_enemy_assets(
    mut assets: ResMut<EnemyAnimationAssets>,
    gltfs: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    if assets.graph_handle.is_none() {
        if let Some(gltf) = gltfs.get(&assets.gltf_handle) {
            // Log all available animations to help debug
            let names: Vec<_> = gltf.named_animations.keys().collect();
            info!("Available animations in scorcher_enemy.glb: {:?}", names);

            let start_clip = if let Some(clip) = gltf.named_animations.get("start") {
                info!("Found 'start' animation");
                Some(clip.clone())
            } else if let Some(clip) = gltf.named_animations.get("Start") {
                info!("Found 'Start' animation");
                Some(clip.clone())
            } else if let Some(clip) = gltf.animations.first() {
                warn!("No 'start' or 'Start' animation found, using first available");
                Some(clip.clone())
            } else {
                error!("No animations found in scorcher_enemy.glb");
                None
            };

            if let Some(clip) = start_clip {
                let mut graph = AnimationGraph::new();
                let start_node = graph.add_clip(clip, 1.0, graph.root);
                assets.graph_handle = Some(graphs.add(graph));
                assets.start_node = Some(start_node);
            }
        }
    }
}

pub fn init_enemy_animations(
    mut commands: Commands,
    assets: Res<EnemyAnimationAssets>,
    enemy_query: Query<(Entity, &Enemy), Without<EnemyAnimationPlayerLink>>,
    children_query: Query<&Children>,
    anim_player_query: Query<Entity, With<AnimationPlayer>>,
) {
    let (Some(graph_handle), Some(start_node)) = (assets.graph_handle.as_ref(), assets.start_node) else {
        return;
    };
    
    for (enemy_entity, _) in &enemy_query {
        for descendant in children_query.iter_descendants(enemy_entity) {
            if anim_player_query.get(descendant).is_ok() {
                info!("Linking animation to enemy entity {:?}", enemy_entity);
                
                commands.entity(descendant).insert((
                    AnimationGraphHandle(graph_handle.clone()),
                    AnimationTransitions::default(),
                ));
                
                commands.entity(enemy_entity).insert((
                    EnemyAnimationPlayerLink(descendant),
                    EnemyAnimation {
                        start_node,
                    },
                ));
                
                info!("Enemy animation components assigned");
                break;
            }
        }
    }
}

pub fn play_enemy_animations(
    mut anim_player_query: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    enemy_query: Query<(&EnemyAnimationPlayerLink, &EnemyAnimation)>,
) {
    for (link, anim) in &enemy_query {
        if let Ok((mut player, mut transitions)) = anim_player_query.get_mut(link.0) {
            // Only call play if we are not already playing/transitioning to this node
            if !player.is_playing_animation(anim.start_node) {
                transitions.play(&mut player, anim.start_node, std::time::Duration::from_millis(200)).repeat();
            }
        }
    }
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
    let player_radius = 0.4;
    let enemy_radius = 0.4;
    
    for (mut player_transform, mut player) in &mut player_query {
        if player.invulnerable_timer > 0.0 || player.shield_timer > 0.0 {
            continue;
        }
        
        let player_pos = player_transform.translation;
        for enemy_transform in &enemy_query {
            let enemy_pos = enemy_transform.translation;
            
            // Check horizontal distance (More generous radius: 1.2 total)
            let xz_dist = Vec2::new(player_pos.x, player_pos.z).distance(Vec2::new(enemy_pos.x, enemy_pos.z));
            // Check vertical overlap (Much more generous: 4.0 units)
            let y_diff = (player_pos.y - enemy_pos.y).abs();
            
            if xz_dist < 1.2 && y_diff < 4.0 {
                if player.health > 0 {
                    player.health -= 1;
                    info!("!!! PLAYER DAMAGED !!! Health: {}", player.health);
                }
                player.invulnerable_timer = 1.5;
                
                // Knockback
                let diff = player_pos - enemy_pos;
                let push_dir = Vec3::new(diff.x, 0.0, diff.z).normalize_or_zero();
                player_transform.translation += push_dir * 2.0;
                break;
            }
        }
    }
}
