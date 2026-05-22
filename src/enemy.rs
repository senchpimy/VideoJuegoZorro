use bevy::prelude::*;
use crate::player::Player;
use crate::projectile::Projectile;
use crate::GameState;

use bevy::gltf::Gltf;

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyType {
    Scorcher,
    Worm,
    /// Invisible enemy — no mesh, no model. Silently chases the player.
    Phantom,
}

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub speed: f32,
    pub patrol_points: Vec<Vec3>,
    pub current_waypoint: usize,
    pub health: f32,
}

#[derive(Component)]
pub struct EnemyAnimation {
    pub anim_node: AnimationNodeIndex,
}

#[derive(Component)]
pub struct EnemyAnimationPlayerLink(pub Entity);

#[derive(Resource, Default)]
pub struct EnemyAnimationAssets {
    pub scorcher_gltf: Handle<Gltf>,
    pub worm_gltf: Handle<Gltf>,
    pub scorcher_graph: Option<Handle<AnimationGraph>>,
    pub scorcher_node: Option<AnimationNodeIndex>,
    pub worm_graph: Option<Handle<AnimationGraph>>,
    pub worm_node: Option<AnimationNodeIndex>,
}

pub fn setup_enemy_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let scorcher_gltf = asset_server.load("models/scorcher_enemy.glb");
    let worm_gltf = asset_server.load("models/sign_enemy.glb");
    commands.insert_resource(EnemyAnimationAssets {
        scorcher_gltf,
        worm_gltf,
        ..default()
    });
}

pub fn process_enemy_assets(
    mut assets: ResMut<EnemyAnimationAssets>,
    gltfs: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Process Scorcher
    if assets.scorcher_graph.is_none() {
        if let Some(gltf) = gltfs.get(&assets.scorcher_gltf) {
            let clip = gltf.named_animations.get("start")
                .or_else(|| gltf.named_animations.get("Start"))
                .or_else(|| gltf.animations.first())
                .cloned();

            if let Some(clip) = clip {
                let mut graph = AnimationGraph::new();
                let node = graph.add_clip(clip, 1.0, graph.root);
                assets.scorcher_graph = Some(graphs.add(graph));
                assets.scorcher_node = Some(node);
            }
        }
    }

    // Process Worm (using sign_enemy.glb now)
    if assets.worm_graph.is_none() {
        if let Some(gltf) = gltfs.get(&assets.worm_gltf) {
            info!("Sign Enemy GLTF loaded. Animations: {}, Scenes: {}", gltf.animations.len(), gltf.scenes.len());
            // Trying animation index 2 if available, fallback to first
            let clip = gltf.animations.get(2).cloned().or_else(|| gltf.animations.first().cloned());

            if let Some(clip) = clip {
                let mut graph = AnimationGraph::new();
                let node = graph.add_clip(clip, 1.0, graph.root);
                assets.worm_graph = Some(graphs.add(graph));
                assets.worm_node = Some(node);
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
    for (enemy_entity, enemy) in &enemy_query {
        // Phantoms have no model/animations — skip them entirely.
        if enemy.enemy_type == EnemyType::Phantom {
            continue;
        }

        let (graph_handle, node) = match enemy.enemy_type {
            EnemyType::Scorcher => (assets.scorcher_graph.as_ref(), assets.scorcher_node),
            EnemyType::Worm => (assets.worm_graph.as_ref(), assets.worm_node),
            EnemyType::Phantom => unreachable!(),
        };

        if let (Some(graph_handle), Some(node)) = (graph_handle, node) {
            for descendant in children_query.iter_descendants(enemy_entity) {
                if anim_player_query.get(descendant).is_ok() {
                    commands.entity(descendant).insert((
                        AnimationGraphHandle(graph_handle.clone()),
                        AnimationTransitions::default(),
                    ));
                    
                    commands.entity(enemy_entity).insert((
                        EnemyAnimationPlayerLink(descendant),
                        EnemyAnimation {
                            anim_node: node,
                        },
                    ));
                    break;
                }
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
            if !player.is_playing_animation(anim.anim_node) {
                transitions.play(&mut player, anim.anim_node, std::time::Duration::from_millis(200)).repeat();
            }
        }
    }
}

pub fn move_enemies(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy), Without<Player>>,
) {
    let dt = time.delta_secs();
    let player_pos = player_query.iter().next().map(|t| t.translation).unwrap_or(Vec3::ZERO);

    for (mut transform, mut enemy) in &mut enemy_query {
        let target_pos = match enemy.enemy_type {
            EnemyType::Scorcher => {
                if enemy.patrol_points.is_empty() {
                    continue;
                }
                enemy.patrol_points[enemy.current_waypoint]
            }
            // Worm and Phantom both chase the player.
            EnemyType::Worm | EnemyType::Phantom => player_pos,
        };

        let diff = target_pos - transform.translation;
        let dist = diff.length();

        match enemy.enemy_type {
            EnemyType::Scorcher => {
                if dist < 0.2 {
                    enemy.current_waypoint = (enemy.current_waypoint + 1) % enemy.patrol_points.len();
                } else {
                    let dir = diff.normalize();
                    transform.translation += dir * enemy.speed * 2.0 * dt;
                    let target_rotation = Quat::from_rotation_y(f32::atan2(dir.x, dir.z));
                    transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
                }
            }
            // Worm and Phantom share the same XZ-only chase logic.
            // Phantom is faster (speed 3.5 vs 2.0) but otherwise identical.
            EnemyType::Worm | EnemyType::Phantom => {
                if dist > 0.5 {
                    let diff_xz = Vec3::new(
                        target_pos.x - transform.translation.x,
                        0.0,
                        target_pos.z - transform.translation.z,
                    );
                    if diff_xz.length() > 0.1 {
                        let dir = diff_xz.normalize();
                        transform.translation += dir * enemy.speed * dt;
                        let target_rotation = Quat::from_rotation_y(f32::atan2(dir.x, dir.z));
                        transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
                    }
                }
            }
        }
    }
}

pub fn check_enemy_projectile_collision(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
) {
    let enemy_radius = 1.0; // Generous radius so hits feel responsive
    let proj_radius = 0.3;

    for (enemy_entity, enemy_transform, _enemy) in &mut enemy_query {
        let enemy_pos = enemy_transform.translation;
        for (proj_entity, proj_transform) in &projectile_query {
            let proj_pos = proj_transform.translation;
            
            // Check XZ horizontal distance to ignore floating/vertical offsets
            let xz_dist = Vec2::new(enemy_pos.x, enemy_pos.z).distance(Vec2::new(proj_pos.x, proj_pos.z));

            if xz_dist < (enemy_radius + proj_radius) {
                // Destroy the projectile
                commands.entity(proj_entity).despawn();
                // Destroy the enemy
                commands.entity(enemy_entity).despawn();
                break;
            }
        }
    }
}

pub fn check_enemy_player_collision(
    mut player_query: Query<(&mut Transform, &mut Player), Without<Enemy>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
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
                if player.health > 0.0 {
                    player.health -= 1.5;
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

pub fn check_player_death(
    player_query: Query<&Player>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(player) = player_query.iter().next() {
        if player.health <= 0.0 {
            next_state.set(GameState::GameOver);
        }
    }
}
