use crate::collision::Wall;
use crate::enemy::{Enemy, EnemyType};
use crate::powerup::{PowerUpItem, PowerUpType};
use avian3d::prelude::*;
use bevy::camera::visibility::NoFrustumCulling;
use bevy::prelude::*;

#[derive(Component)]
pub struct TutorialElement;

#[derive(Component)]
pub struct PhysicsCube {
    pub is_held: bool,
}

#[derive(Component)]
pub struct MagicPortal;

#[derive(Resource)]
pub struct CubeSpawnTimer(pub Timer);

// Move closer to the maze
pub const TUTORIAL_OFFSET: Vec3 = Vec3::new(-30.0, 0.1, -30.0);

pub fn spawn_tutorial(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let wall_material = materials.add(Color::srgb(0.3, 0.3, 0.4));
    let floor_material = materials.add(Color::srgb(0.15, 0.15, 0.2));

    commands.insert_resource(CubeSpawnTimer(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    )));

    // Phantom enemy in tutorial area — worm model to test chasing logic
    let room3_pos = TUTORIAL_OFFSET + Vec3::new(0.0, 0.0, -40.0);
    commands.spawn((
        SceneRoot(asset_server.load("models/worm_enemy.glb#Scene0")),
        Transform::from_translation(room3_pos + Vec3::new(0.0, 0.5, 0.0))
            .with_scale(Vec3::splat(3.0)),
        Enemy {
            enemy_type: EnemyType::Phantom,
            speed: 3.5,
            patrol_points: vec![],
            current_waypoint: 0,
            health: 4.0,
        },
        TutorialElement,
        NoFrustumCulling,
        Visibility::default(),
        InheritedVisibility::default(),
    ));

    // ROOM 1: MOVEMENT
    spawn_room(
        &mut commands,
        &mut meshes,
        floor_material.clone(),
        wall_material.clone(),
        TUTORIAL_OFFSET,
        "BIENVENIDO\nUSA WASD PARA MOVERTE",
        true,
        false,
        false,
        false,
    );

    // ROOM 2: JUMPING
    let room2_pos = TUTORIAL_OFFSET + Vec3::new(0.0, 0.0, -20.0);
    spawn_room(
        &mut commands,
        &mut meshes,
        floor_material.clone(),
        wall_material.clone(),
        room2_pos,
        "SALTO\nESPACIO PARA SALTAR",
        true,
        true,
        false,
        false,
    );

    // Obstacle
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(8.0, 0.8, 2.0))),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_translation(room2_pos + Vec3::new(0.0, 0.4, 0.0)),
        Wall {
            half_size: Vec3::new(4.0, 0.4, 1.0),
        },
        TutorialElement,
    ));

    // ROOM 3: COMBAT
    let room3_pos = room2_pos + Vec3::new(0.0, 0.0, -20.0);
    spawn_room(
        &mut commands,
        &mut meshes,
        floor_material.clone(),
        wall_material.clone(),
        room3_pos,
        "COMBATE\nSHIFT PARA ATACAR",
        true,
        true,
        false,
        false,
    );

    commands.spawn((
        SceneRoot(asset_server.load("models/scorcher_enemy.glb#Scene0")),
        Transform::from_translation(room3_pos + Vec3::new(0.0, 2.5, -4.0))
            .with_scale(Vec3::splat(0.012)),
        Enemy {
            enemy_type: EnemyType::Scorcher,
            speed: 0.0,
            patrol_points: vec![],
            current_waypoint: 0,
            health: 1.0,
        },
        TutorialElement,
    ));

    // ROOM 4: POWERUPS
    let room4_pos = room3_pos + Vec3::new(0.0, 0.0, -20.0);
    spawn_room(
        &mut commands,
        &mut meshes,
        floor_material.clone(),
        wall_material.clone(),
        room4_pos,
        "POWER-UPS\nMEJORA TUS HABILIDADES",
        true,
        true,
        true,
        false,
    );

    // Powerups
    commands.spawn((
        SceneRoot(asset_server.load("models/banana.glb#Scene0")),
        Transform::from_translation(room4_pos + Vec3::new(-3.0, 0.5, 0.0))
            .with_scale(Vec3::splat(0.25)),
        PowerUpItem {
            effect_type: PowerUpType::Speed,
        },
        TutorialElement,
    ));

    commands.spawn((
        SceneRoot(asset_server.load("models/apple.glb#Scene0")),
        Transform::from_translation(room4_pos + Vec3::new(3.0, 0.5, 0.0))
            .with_scale(Vec3::splat(0.25)),
        PowerUpItem {
            effect_type: PowerUpType::Healing,
        },
        TutorialElement,
    ));

    // ROOM 5: PHYSICS (East of Room 4)
    let room5_pos = room4_pos + Vec3::new(20.0, 0.0, 0.0);
    spawn_room(
        &mut commands,
        &mut meshes,
        floor_material.clone(),
        wall_material.clone(),
        room5_pos,
        "FISICAS\nCTRL PARA AGARRAR\nCAMINA PARA EMPUJAR",
        false,
        false,
        false,
        true,
    );

    // Magic Portal at the end of Room 5 (where the physics blocks drop)
    commands.spawn((
        SceneRoot(asset_server.load("models/magic_portal.glb#Scene0")),
        Transform::from_translation(room5_pos + Vec3::new(6.0, 0.1, 0.0))
            .with_scale(Vec3::splat(1.5)),
        MagicPortal,
        TutorialElement,
    ));

    // Magical glowing cyan light for the portal
    commands.spawn((
        PointLight {
            color: Color::srgb(0.0, 0.8, 1.0),
            intensity: 80000.0,
            range: 15.0,
            ..default()
        },
        Transform::from_translation(room5_pos + Vec3::new(6.0, 2.5, 0.0)),
        TutorialElement,
    ));

    // Lights
    for pos in [TUTORIAL_OFFSET, room2_pos, room3_pos, room4_pos, room5_pos] {
        commands.spawn((
            PointLight {
                color: Color::WHITE,
                intensity: 600000.0,
                range: 40.0,
                ..default()
            },
            Transform::from_translation(pos + Vec3::new(0.0, 8.0, 0.0)),
            TutorialElement,
        ));
    }
}

pub fn spawn_physics_cubes(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<CubeSpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube_query: Query<&PhysicsCube>,
    portal_query: Query<Entity, With<MagicPortal>>,
) {
    if portal_query.is_empty() {
        return;
    }

    if timer.0.tick(time.delta()).just_finished() {
        let count = cube_query.iter().count();
        if count < 3 {
            let room5_pos = TUTORIAL_OFFSET + Vec3::new(20.0, 0.0, -60.0); // Room 5 is East of Room 4

            let offset = match count {
                0 => Vec3::new(-1.5, 8.0, -1.5),
                1 => Vec3::new(1.5, 8.0, 1.5),
                _ => Vec3::new(0.0, 8.0, 0.0),
            };

            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
                Transform::from_translation(room5_pos + offset),
                RigidBody::Dynamic,
                Collider::cuboid(1.0, 1.0, 1.0),
                PhysicsCube { is_held: false },
                TutorialElement,
            ));
        }
    }
}

pub fn update_physics_cubes() {}

fn spawn_room(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    floor_mat: Handle<StandardMaterial>,
    wall_mat: Handle<StandardMaterial>,
    pos: Vec3,
    text: &str,
    open_n: bool,
    open_s: bool,
    open_e: bool,
    open_w: bool,
) {
    let size = 20.0;
    let h = 5.0;
    let thick = 0.5;

    // Floor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(size, size))),
        MeshMaterial3d(floor_mat),
        Transform::from_translation(pos),
        RigidBody::Static,
        Collider::cuboid(size, 0.1, size),
        TutorialElement,
    ));

    // WALLS
    // North
    if open_n {
        spawn_wall(
            commands,
            meshes,
            wall_mat.clone(),
            pos + Vec3::new(-7.0, h / 2.0, -size / 2.0),
            Vec3::new(6.0, h, thick),
        );
        spawn_wall(
            commands,
            meshes,
            wall_mat.clone(),
            pos + Vec3::new(7.0, h / 2.0, -size / 2.0),
            Vec3::new(6.0, h, thick),
        );
    } else {
        spawn_wall(
            commands,
            meshes,
            wall_mat.clone(),
            pos + Vec3::new(0.0, h / 2.0, -size / 2.0),
            Vec3::new(size, h, thick),
        );
    }

    // South
    if open_s {
        spawn_wall(
            commands,
            meshes,
            wall_mat.clone(),
            pos + Vec3::new(-7.0, h / 2.0, size / 2.0),
            Vec3::new(6.0, h, thick),
        );
        spawn_wall(
            commands,
            meshes,
            wall_mat.clone(),
            pos + Vec3::new(7.0, h / 2.0, size / 2.0),
            Vec3::new(6.0, h, thick),
        );
    } else {
        spawn_wall(
            commands,
            meshes,
            wall_mat.clone(),
            pos + Vec3::new(0.0, h / 2.0, size / 2.0),
            Vec3::new(size, h, thick),
        );
    }

    // West
    if open_w {
        spawn_wall(
            commands,
            meshes,
            wall_mat.clone(),
            pos + Vec3::new(-size / 2.0, h / 2.0, -7.0),
            Vec3::new(thick, h, 6.0),
        );
        spawn_wall(
            commands,
            meshes,
            wall_mat.clone(),
            pos + Vec3::new(-size / 2.0, h / 2.0, 7.0),
            Vec3::new(thick, h, 6.0),
        );
    } else {
        spawn_wall(
            commands,
            meshes,
            wall_mat.clone(),
            pos + Vec3::new(-size / 2.0, h / 2.0, 0.0),
            Vec3::new(thick, h, size),
        );
    }

    // East
    if open_e {
        spawn_wall(
            commands,
            meshes,
            wall_mat.clone(),
            pos + Vec3::new(size / 2.0, h / 2.0, -7.0),
            Vec3::new(thick, h, 6.0),
        );
        spawn_wall(
            commands,
            meshes,
            wall_mat.clone(),
            pos + Vec3::new(size / 2.0, h / 2.0, 7.0),
            Vec3::new(thick, h, 6.0),
        );
    } else {
        spawn_wall(
            commands,
            meshes,
            wall_mat.clone(),
            pos + Vec3::new(size / 2.0, h / 2.0, 0.0),
            Vec3::new(thick, h, size),
        );
    }

    // IN-WORLD 3D TEXT
    commands.spawn((
        Text2d::new(text),
        TextFont {
            font_size: 50.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(pos + Vec3::new(0.0, 2.5, -size / 2.0 + 0.1)),
        TutorialElement,
    ));
}

fn spawn_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    mat: Handle<StandardMaterial>,
    pos: Vec3,
    size: Vec3,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(size))),
        MeshMaterial3d(mat),
        Transform::from_translation(pos),
        Wall {
            half_size: size / 2.0,
        },
        RigidBody::Static,
        Collider::cuboid(size.x, size.y, size.z),
        TutorialElement,
    ));
}

pub fn cleanup_tutorial(mut commands: Commands, query: Query<Entity, With<TutorialElement>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn check_portal_teleport(
    mut commands: Commands,
    mut player_query: Query<&mut Transform, With<crate::player::Player>>,
    portal_query: Query<&Transform, (With<MagicPortal>, Without<crate::player::Player>)>,
    elements_query: Query<Entity, With<TutorialElement>>,
) {
    if let Ok(mut player_transform) = player_query.single_mut() {
        for portal_transform in &portal_query {
            // Check flat XZ horizontal distance to make triggering portal feel smooth and responsive
            let player_pos = player_transform.translation;
            let portal_pos = portal_transform.translation;
            let xz_dist = Vec2::new(player_pos.x, player_pos.z)
                .distance(Vec2::new(portal_pos.x, portal_pos.z));

            if xz_dist < 1.8 {
                info!("Teleporting player from tutorial to the maze!");
                // Teleport to the maze starting position (index 1, 1)
                // Each cell is 4.0 units, so index (1,1) is at offset + (4, 1, 4)
                player_transform.translation = crate::maze::MAZE_OFFSET + Vec3::new(4.0, 1.0, 4.0);

                // Cleanup tutorial elements including the controls UI
                for entity in &elements_query {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
