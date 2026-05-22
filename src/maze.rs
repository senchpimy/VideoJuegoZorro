use crate::collision::Wall;
use crate::enemy::{Enemy, EnemyType};
use crate::platform::MovingPlatform;
use crate::powerup::{Chest, PowerUpItem, PowerUpType};
use crate::tutorial::PhysicsCube;
use bevy::prelude::*;
use avian3d::prelude::{RigidBody, Collider};

#[derive(Component)]
pub struct PuzzleBlock {
    pub color: PuzzleColor,
}

#[derive(Component)]
pub struct TargetZone {
    pub color: PuzzleColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PuzzleColor {
    Red,
    Blue,
}

// 1 = Wall, 0 = Empty, 2 = Player Start, 3 = Moving Platform Pit (Removed), 4 = Lava Static Pit, 5 = Enemy Spawn, 6 = Chest, 7 = Speed Gem, 8 = Shield Gem, 9 = Healing Gem, 11 = Phantom (Invisible Enemy)
pub const MAZE_DATA: [[u8; 30]; 30] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 2, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 7, 0, 0, 0, 0, 9, 1, 0, 0, 0, 5, 0, 0, 0, 7, 0, 1],
    [1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1],
    [1, 0, 4, 0, 0, 0, 1, 8, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 8, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1],
    [1, 0, 1, 0, 9, 0, 0, 0, 1, 0,11, 0, 0, 0, 1, 0, 7, 0, 0, 1, 0, 1, 9, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1],
    [1, 0, 0, 0, 1, 0, 7, 0, 0, 0, 0, 0, 1, 7, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 5, 0, 0, 0, 0, 1],
    [1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
    [1, 0, 8, 5, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 9, 1, 0, 0, 0, 0, 0, 0, 1, 0, 8, 1],
    [1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1],
    [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 9, 0, 0, 6, 1, 0, 7, 0, 0, 1, 0, 1, 7, 0, 1, 0, 0, 0, 0, 1],
    [1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1],
    [1, 7, 0, 0, 5, 0, 0, 8, 0, 0, 5, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 5, 0, 0, 1, 0, 1],
    [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
    [1, 0, 8, 0, 0, 9, 0, 0, 0, 5, 0, 0, 0, 7, 0,11, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
    [1, 0, 0, 0, 7, 0, 0, 0, 1, 0, 1, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1],
    [1, 9, 0, 5, 0, 0, 7, 0, 0,11, 0, 0, 5, 0, 0, 8, 0, 1, 9, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 7, 0, 0, 1, 0, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1],
    [1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 7, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1],
    [1, 5, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 9, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1],
    [1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 5, 0, 0, 0, 8, 1],
    [1, 9, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 9, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

#[derive(Component)]
pub struct MazeElement;

#[derive(Component)]
pub struct WorldTerrain;

#[derive(Component)]
pub struct WorldLight;

const WORLD_SIZE: i32 = 120;
pub const MAZE_OFFSET: Vec3 = Vec3::new(70.0, 0.1, 70.0);

pub fn terrain_height(_x: f32, _z: f32) -> f32 {
    0.0
}

pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Daylight Settings
    commands.spawn((
        AmbientLight {
            color: Color::WHITE,
            brightness: 1000.0,
            affects_lightmapped_meshes: true,
        },
        WorldLight,
    ));
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.9)));

    let ground_material = materials.add(Color::from(LinearRgba::from_f32_array([
        0.15, 0.25, 0.1, 1.0,
    ])));
    let mountain_material = materials.add(Color::from(LinearRgba::from_f32_array([
        0.3, 0.3, 0.35, 1.0,
    ])));

    for z in -WORLD_SIZE..WORLD_SIZE {
        for x in -WORLD_SIZE..WORLD_SIZE {
            let fx = x as f32 * 4.0;
            let fz = z as f32 * 4.0;
            let h = terrain_height(fx, fz);

            let mat = if h > 8.0 {
                mountain_material.clone()
            } else {
                ground_material.clone()
            };

            commands.spawn((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(4.0, 4.0))),
                MeshMaterial3d(mat),
                Transform::from_xyz(fx, h, fz),
                WorldTerrain,
            ));
        }
    }

    let pillar_mesh = meshes.add(Cylinder::new(0.4, 8.0));
    let pillar_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 1.0),
        emissive: LinearRgba::from_f32_array([0.0, 5.0, 5.0, 1.0]),
        ..default()
    });

    for i in 0..12 {
        let angle = i as f32 * std::f32::consts::PI / 6.0;
        let dist = 100.0;
        let pos = MAZE_OFFSET + Vec3::new(angle.cos() * dist, 0.0, angle.sin() * dist);
        let h = terrain_height(pos.x, pos.z);

        commands.spawn((
            Mesh3d(pillar_mesh.clone()),
            MeshMaterial3d(pillar_material.clone()),
            Transform::from_xyz(pos.x, h + 4.0, pos.z),
            WorldTerrain,
        ));

        commands.spawn((
            PointLight {
                color: Color::srgb(0.0, 1.0, 1.0),
                intensity: 150000.0,
                range: 80.0,
                ..default()
            },
            Transform::from_xyz(pos.x, h + 10.0, pos.z),
            WorldTerrain,
        ));
    }

    commands.spawn((
        DirectionalLight {
            illuminance: 32000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(100.0, 200.0, 100.0).looking_at(MAZE_OFFSET, Vec3::Y),
        MazeElement,
    ));

    spawn_maze_at(
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
        MAZE_OFFSET,
    );
}

fn spawn_maze_at(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    offset: Vec3,
) {
    let wall_material = materials.add(Color::from(LinearRgba::from_f32_array([
        0.4, 0.4, 0.4, 1.0,
    ])));
    let ground_material = materials.add(Color::from(LinearRgba::from_f32_array([
        0.1, 0.1, 0.1, 1.0,
    ])));

    let pillar_mesh = meshes.add(Cuboid::new(0.8, 5.0, 0.8));
    let connector_mesh = meshes.add(Cuboid::new(3.2, 5.0, 0.8)); // For East/West

    for (z, row) in MAZE_DATA.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let pos = offset + Vec3::new(x as f32 * 4.0, 0.0, z as f32 * 4.0);

            // Ground
            if cell != 3 && cell != 4 {
                commands.spawn((
                    Mesh3d(meshes.add(Plane3d::default().mesh().size(4.0, 4.0))),
                    MeshMaterial3d(ground_material.clone()),
                    Transform::from_translation(pos),
                    MazeElement,
                ));
            }

            if cell == 1 {
                // Central Pillar
                commands.spawn((
                    Mesh3d(pillar_mesh.clone()),
                    MeshMaterial3d(wall_material.clone()),
                    Transform::from_xyz(pos.x, 2.5, pos.z),
                    Wall { half_size: Vec3::new(0.4, 2.5, 0.4) },
                    RigidBody::Static,
                    Collider::cuboid(0.8, 5.0, 0.8),
                    MazeElement,
                ));

                // Check neighbors for connectors
                // North (z-1)
                if z > 0 && MAZE_DATA[z-1][x] == 1 {
                    commands.spawn((
                        Mesh3d(connector_mesh.clone()),
                        MeshMaterial3d(wall_material.clone()),
                        Transform::from_xyz(pos.x, 2.5, pos.z - 2.0)
                            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
                        Wall { half_size: Vec3::new(0.4, 2.5, 1.6) },
                        RigidBody::Static,
                        Collider::cuboid(3.2, 5.0, 0.8),
                        MazeElement,
                    ));
                }
                // South (z+1)
                if z < 29 && MAZE_DATA[z+1][x] == 1 {
                    commands.spawn((
                        Mesh3d(connector_mesh.clone()),
                        MeshMaterial3d(wall_material.clone()),
                        Transform::from_xyz(pos.x, 2.5, pos.z + 2.0)
                            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
                        Wall { half_size: Vec3::new(0.4, 2.5, 1.6) },
                        RigidBody::Static,
                        Collider::cuboid(3.2, 5.0, 0.8),
                        MazeElement,
                    ));
                }
                // West (x-1)
                if x > 0 && MAZE_DATA[z][x-1] == 1 {
                    commands.spawn((
                        Mesh3d(connector_mesh.clone()),
                        MeshMaterial3d(wall_material.clone()),
                        Transform::from_xyz(pos.x - 2.0, 2.5, pos.z),
                        Wall { half_size: Vec3::new(1.6, 2.5, 0.4) },
                        RigidBody::Static,
                        Collider::cuboid(3.2, 5.0, 0.8),
                        MazeElement,
                    ));
                }
                // East (x+1)
                if x < 29 && MAZE_DATA[z][x+1] == 1 {
                    commands.spawn((
                        Mesh3d(connector_mesh.clone()),
                        MeshMaterial3d(wall_material.clone()),
                        Transform::from_xyz(pos.x + 2.0, 2.5, pos.z),
                        Wall { half_size: Vec3::new(1.6, 2.5, 0.4) },
                        RigidBody::Static,
                        Collider::cuboid(3.2, 5.0, 0.8),
                        MazeElement,
                    ));
                }
            }

            if cell == 3 {
                // Preserved logic
                let platform_mesh = meshes.add(Cuboid::new(4.0, 0.6, 4.0));
                let platform_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.0, 0.8, 1.0),
                    emissive: LinearRgba::from_f32_array([0.0, 0.4, 0.8, 1.0]),
                    ..default()
                });
                let start_pos = pos + Vec3::new(0.0, 0.0, -3.2);
                let end_pos = pos + Vec3::new(0.0, 0.0, 3.2);
                commands.spawn((
                    Mesh3d(platform_mesh),
                    MeshMaterial3d(platform_material),
                    Transform::from_translation(start_pos),
                    MovingPlatform {
                        start_pos,
                        end_pos,
                        speed: 1.2,
                        progress: 0.0,
                        forward: true,
                        delta: Vec3::ZERO,
                    },
                    MazeElement,
                ));
            }
            if cell == 4 {
                let lava_mesh = meshes.add(Plane3d::default().mesh().size(4.0, 4.0));
                let lava_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.8, 0.1, 0.0),
                    emissive: LinearRgba::from_f32_array([0.6, 0.05, 0.0, 1.0]),
                    ..default()
                });
                commands.spawn((
                    Mesh3d(lava_mesh),
                    MeshMaterial3d(lava_material),
                    Transform::from_xyz(pos.x, -2.0, pos.z),
                    MazeElement,
                ));
            }
            if cell == 5 {
                let patrol_points = vec![
                    Vec3::new(pos.x - 6.0, 2.5, pos.z),
                    Vec3::new(pos.x + 6.0, 2.5, pos.z),
                ];
                commands.spawn((
                    SceneRoot(asset_server.load("models/scorcher_enemy.glb#Scene0")),
                    Transform::from_translation(Vec3::new(pos.x, 2.5, pos.z))
                        .with_scale(Vec3::splat(0.024)),
                    Enemy {
                        enemy_type: EnemyType::Scorcher,
                        speed: 1.5,
                        patrol_points,
                        current_waypoint: 0,
                        health: 2.0,
                    },
                    MazeElement,
                ));
            }
            if cell == 11 {
                // Chasing Phantom enemy — represented as a glowing red cube
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(1.0, 0.0, 0.0),
                        emissive: LinearRgba::from_f32_array([5.0, 0.0, 0.0, 1.0]),
                        ..default()
                    })),
                    Transform::from_translation(Vec3::new(pos.x, 2.5, pos.z)),
                    Enemy {
                        enemy_type: EnemyType::Phantom,
                        speed: 3.5,
                        patrol_points: vec![],
                        current_waypoint: 0,
                        health: 4.0,
                    },
                    MazeElement,
                ));
            }

            if cell == 6 {
                let chest_mesh = meshes.add(Cuboid::new(2.4, 1.6, 1.6));
                let chest_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.6, 0.3, 0.0),
                    perceptual_roughness: 0.8,
                    ..default()
                });
                commands.spawn((
                    Mesh3d(chest_mesh),
                    MeshMaterial3d(chest_material),
                    Transform::from_translation(Vec3::new(pos.x, 0.8, pos.z)),
                    Chest { opened: false },
                    MazeElement,
                ));
            }
            if cell == 7 || cell == 8 || cell == 9 {
                let (color, effect) = match cell {
                    7 => (Color::srgb(0.0, 0.6, 1.0), PowerUpType::Speed),
                    8 => (Color::srgb(1.0, 0.8, 0.0), PowerUpType::Shield),
                    _ => (Color::srgb(0.0, 1.0, 0.2), PowerUpType::Healing),
                };
                if cell == 7 {
                    commands.spawn((
                        SceneRoot(asset_server.load("models/banana.glb#Scene0")),
                        Transform::from_translation(Vec3::new(pos.x, 1.0, pos.z))
                            .with_scale(Vec3::splat(0.5)),
                        PowerUpItem {
                            effect_type: effect,
                        },
                        MazeElement,
                    ));
                } else if cell == 9 {
                    commands.spawn((
                        SceneRoot(asset_server.load("models/apple.glb#Scene0")),
                        Transform::from_translation(Vec3::new(pos.x, 1.0, pos.z))
                            .with_scale(Vec3::splat(0.5)),
                        PowerUpItem {
                            effect_type: effect,
                        },
                        MazeElement,
                    ));
                } else {
                    let gem_mesh = meshes.add(Sphere::new(0.72).mesh());
                    let gem_material = materials.add(StandardMaterial {
                        base_color: color,
                        emissive: LinearRgba::from_f32_array(match cell {
                            7 => [0.0, 1.2, 2.0, 1.0],
                            8 => [2.0, 1.6, 0.0, 1.0],
                            _ => [0.0, 2.0, 0.4, 1.0],
                        }),
                        ..default()
                    });
                    commands.spawn((
                        Mesh3d(gem_mesh),
                        MeshMaterial3d(gem_material),
                        Transform::from_translation(Vec3::new(pos.x, 1.4, pos.z)),
                        PowerUpItem {
                            effect_type: effect,
                        },
                        MazeElement,
                    ));
                }
            }
        }
    }

    // Spawn Red Puzzle Block
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.2, 1.2, 1.2))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.1, 0.1),
            emissive: LinearRgba::from_f32_array([2.0, 0.2, 0.2, 1.0]),
            perceptual_roughness: 0.1,
            ..default()
        })),
        Transform::from_translation(offset + Vec3::new(5.0 * 4.0, 1.0, 1.0 * 4.0)),
        RigidBody::Dynamic,
        Collider::cuboid(1.2, 1.2, 1.2),
        PhysicsCube { is_held: false },
        PuzzleBlock {
            color: PuzzleColor::Red,
        },
        MazeElement,
    ));

    // Spawn Blue Puzzle Block
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.2, 1.2, 1.2))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.3, 1.0),
            emissive: LinearRgba::from_f32_array([0.2, 0.6, 5.0, 1.0]),
            perceptual_roughness: 0.1,
            ..default()
        })),
        Transform::from_translation(offset + Vec3::new(21.0 * 4.0, 1.0, 1.0 * 4.0)),
        RigidBody::Dynamic,
        Collider::cuboid(1.2, 1.2, 1.2),
        PhysicsCube { is_held: false },
        PuzzleBlock {
            color: PuzzleColor::Blue,
        },
        MazeElement,
    ));

    // Spawn Red Target Zone
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(3.0, 3.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.1, 0.1, 0.5),
            alpha_mode: AlphaMode::Blend,
            emissive: LinearRgba::from_f32_array([0.5, 0.05, 0.05, 1.0]),
            ..default()
        })),
        Transform::from_translation(offset + Vec3::new(3.0 * 4.0, 0.05, 28.0 * 4.0)),
        TargetZone {
            color: PuzzleColor::Red,
        },
        MazeElement,
    ));

    // Spawn Blue Target Zone
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(3.0, 3.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.1, 0.3, 1.0, 0.5),
            alpha_mode: AlphaMode::Blend,
            emissive: LinearRgba::from_f32_array([0.05, 0.15, 1.0, 1.0]),
            ..default()
        })),
        Transform::from_translation(offset + Vec3::new(15.0 * 4.0, 0.05, 28.0 * 4.0)),
        TargetZone {
            color: PuzzleColor::Blue,
        },
        MazeElement,
    ));

    // Spawning 3D In-World labels to guide the player
    commands.spawn((
        Text2d::new("ZONA ROJA\nDEPOSITAR BLOQUE ROJO"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.5, 0.5)),
        Transform::from_translation(offset + Vec3::new(3.0 * 4.0, 2.5, 28.0 * 4.0))
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
        MazeElement,
    ));

    commands.spawn((
        Text2d::new("ZONA AZUL\nDEPOSITAR BLOQUE AZUL"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.7, 1.0)),
        Transform::from_translation(offset + Vec3::new(15.0 * 4.0, 2.5, 28.0 * 4.0))
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
        MazeElement,
    ));

    commands.spawn((
        Text2d::new("BLOQUE ROJO"),
        TextFont { font_size: 20.0, ..default() },
        TextColor(Color::srgb(1.0, 0.5, 0.5)),
        Transform::from_translation(offset + Vec3::new(5.0 * 4.0, 2.5, 1.0 * 4.0)),
        MazeElement,
    ));

    commands.spawn((
        Text2d::new("BLOQUE AZUL"),
        TextFont { font_size: 20.0, ..default() },
        TextColor(Color::srgb(0.5, 0.7, 1.0)),
        Transform::from_translation(offset + Vec3::new(21.0 * 4.0, 2.5, 1.0 * 4.0)),
        MazeElement,
    ));
}

pub fn cleanup_world(
    mut commands: Commands,
    q_maze: Query<Entity, With<MazeElement>>,
    q_world: Query<Entity, With<WorldTerrain>>,
    q_light: Query<Entity, With<WorldLight>>,
) {
    for e in q_maze.iter() {
        commands.entity(e).despawn();
    }
    for e in q_world.iter() {
        commands.entity(e).despawn();
    }
    for e in q_light.iter() {
        commands.entity(e).despawn();
    }
    commands.insert_resource(ClearColor::default());
}

pub fn check_puzzle_completion(
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_blocks: Query<(&Transform, &PuzzleBlock)>,
    q_targets: Query<(&Transform, &TargetZone, &MeshMaterial3d<StandardMaterial>)>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    let mut red_satisfied = false;
    let mut blue_satisfied = false;

    // Check each block against each target zone
    for (block_transform, block) in q_blocks.iter() {
        for (target_transform, target, _) in q_targets.iter() {
            if block.color == target.color {
                let dist_xz = Vec2::new(block_transform.translation.x, block_transform.translation.z)
                    .distance(Vec2::new(target_transform.translation.x, target_transform.translation.z));
                
                if dist_xz < 2.0 {
                    match block.color {
                        PuzzleColor::Red => red_satisfied = true,
                        PuzzleColor::Blue => blue_satisfied = true,
                    }
                }
            }
        }
    }

    // Now update target materials based on satisfaction!
    for (_, target, mat_handle) in q_targets.iter() {
        let is_satisfied = match target.color {
            PuzzleColor::Red => red_satisfied,
            PuzzleColor::Blue => blue_satisfied,
        };

        if let Some(material) = materials.get_mut(mat_handle) {
            if is_satisfied {
                // Bright glow when satisfied
                material.emissive = match target.color {
                    PuzzleColor::Red => LinearRgba::from_f32_array([10.0, 1.0, 1.0, 1.0]),
                    PuzzleColor::Blue => LinearRgba::from_f32_array([1.0, 3.0, 15.0, 1.0]),
                };
            } else {
                // Soft glow when not satisfied
                material.emissive = match target.color {
                    PuzzleColor::Red => LinearRgba::from_f32_array([0.5, 0.05, 0.05, 1.0]),
                    PuzzleColor::Blue => LinearRgba::from_f32_array([0.05, 0.15, 1.0, 1.0]),
                };
            }
        }
    }

    if red_satisfied && blue_satisfied {
        info!("Puzzle complete! Victory!");
        next_state.set(crate::GameState::GameWon);
    }
}
