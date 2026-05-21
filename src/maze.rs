use crate::collision::Wall;
use crate::enemy::Enemy;
use crate::platform::MovingPlatform;
use crate::powerup::{Chest, PowerUpItem, PowerUpType};
use bevy::prelude::*;

// 1 = Wall, 0 = Empty, 2 = Player Start, 3 = Moving Platform Pit (Removed), 4 = Lava Static Pit, 5 = Enemy Spawn, 6 = Chest, 7 = Speed Gem, 8 = Shield Gem, 9 = Healing Gem
pub const MAZE_DATA: [[u8; 20]; 20] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 2, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 7, 0, 0, 0, 0, 9, 1],
    [1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1],
    [1, 0, 4, 0, 0, 0, 1, 8, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 8, 1],
    [1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1],
    [1, 0, 1, 0, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 7, 0, 0, 1],
    [1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1],
    [1, 0, 0, 0, 1, 0, 7, 0, 0, 0, 0, 0, 1, 7, 1, 0, 0, 0, 0, 1],
    [1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1],
    [1, 0, 8, 5, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 9, 1],
    [1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1],
    [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 9, 0, 0, 6, 1, 0, 7, 0, 0, 1],
    [1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1],
    [1, 7, 0, 0, 5, 0, 0, 8, 0, 0, 5, 0, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
    [1, 0, 8, 0, 0, 9, 0, 0, 0, 5, 0, 0, 0, 7, 0, 0, 0, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
    [1, 0, 0, 0, 7, 0, 0, 0, 1, 0, 1, 0, 0, 0, 8, 0, 0, 0, 0, 1],
    [1, 9, 0, 5, 0, 0, 7, 0, 0, 0, 0, 0, 5, 0, 0, 8, 0, 0, 9, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

#[derive(Component)]
pub struct MazeElement;

#[derive(Component)]
pub struct WorldTerrain;

#[derive(Component)]
pub struct WorldLight;

const WORLD_SIZE: i32 = 80; // Larger terrain for larger maze
pub const MAZE_OFFSET: Vec3 = Vec3::new(70.0, 0.0, 70.0); // Repositioned

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
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.9))); // Sky Blue

    let ground_material = materials.add(Color::from(LinearRgba::from_f32_array([
        0.15, 0.25, 0.1, 1.0,
    ])));
    let mountain_material = materials.add(Color::from(LinearRgba::from_f32_array([
        0.3, 0.3, 0.35, 1.0,
    ])));

    // Spawn Terrain Grid
    for z in -WORLD_SIZE..WORLD_SIZE {
        for x in -WORLD_SIZE..WORLD_SIZE {
            let fx = x as f32 * 2.0;
            let fz = z as f32 * 2.0;
            let h = terrain_height(fx, fz);

            let mat = if h > 8.0 {
                mountain_material.clone()
            } else {
                ground_material.clone()
            };

            commands.spawn((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(2.0, 2.0))),
                MeshMaterial3d(mat),
                Transform::from_xyz(fx, h, fz),
                WorldTerrain,
            ));
        }
    }

    // Spawn Clues (Spirit Pillars) pointing to the maze
    let pillar_mesh = meshes.add(Cylinder::new(0.2, 4.0));
    let pillar_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 1.0),
        emissive: LinearRgba::from_f32_array([0.0, 5.0, 5.0, 1.0]),
        ..default()
    });

    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::PI / 4.0;
        let dist = 60.0;
        let pos = MAZE_OFFSET + Vec3::new(angle.cos() * dist, 0.0, angle.sin() * dist);
        let h = terrain_height(pos.x, pos.z);

        commands.spawn((
            Mesh3d(pillar_mesh.clone()),
            MeshMaterial3d(pillar_material.clone()),
            Transform::from_xyz(pos.x, h + 2.0, pos.z),
            WorldTerrain,
        ));

        // Point light at each pillar to make it visible from afar
        commands.spawn((
            PointLight {
                color: Color::srgb(0.0, 1.0, 1.0),
                intensity: 100000.0,
                range: 50.0,
                ..default()
            },
            Transform::from_xyz(pos.x, h + 5.0, pos.z),
            WorldTerrain,
        ));
    }

    // Light (Sun)
    commands.spawn((
        DirectionalLight {
            illuminance: 32000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(50.0, 100.0, 50.0).looking_at(MAZE_OFFSET, Vec3::Y),
        MazeElement,
    ));

    // Spawn the Maze at the center
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

    for (z, row) in MAZE_DATA.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let pos = offset + Vec3::new(x as f32 * 2.0, 0.0, z as f32 * 2.0);

            // Ground
            if cell != 3 && cell != 4 {
                commands.spawn((
                    Mesh3d(meshes.add(Plane3d::default().mesh().size(2.0, 2.0))),
                    MeshMaterial3d(ground_material.clone()),
                    Transform::from_translation(pos),
                    MazeElement,
                ));
            }

            if cell == 1 {
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.5, 2.0, 0.5))),
                    MeshMaterial3d(wall_material.clone()),
                    Transform::from_xyz(pos.x, 1.0, pos.z),
                    Wall,
                    MazeElement,
                ));
            }

            // Moving Platform Logic (Preserved but currently unused by MAZE_DATA)
            if cell == 3 {
                let platform_mesh = meshes.add(Cuboid::new(2.0, 0.3, 2.0));
                let platform_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.0, 0.8, 1.0),
                    emissive: LinearRgba::from_f32_array([0.0, 0.4, 0.8, 1.0]),
                    ..default()
                });
                let start_pos = pos + Vec3::new(0.0, 0.0, -1.6);
                let end_pos = pos + Vec3::new(0.0, 0.0, 1.6);
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
                let lava_mesh = meshes.add(Plane3d::default().mesh().size(2.0, 2.0));
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
            if cell == 4 {
                let lava_mesh = meshes.add(Plane3d::default().mesh().size(2.0, 2.0));
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
                    Vec3::new(pos.x - 3.0, 3.2, pos.z),
                    Vec3::new(pos.x + 3.0, 3.2, pos.z),
                ];
                commands.spawn((
                    SceneRoot(asset_server.load("models/scorcher_enemy.glb#Scene0")),
                    Transform::from_translation(Vec3::new(pos.x, 6., pos.z))
                        .with_scale(Vec3::splat(0.012)),
                    Enemy {
                        speed: 1.5,
                        patrol_points,
                        current_waypoint: 0,
                        health: 2.0,
                    },
                    MazeElement,
                ));
            }
            if cell == 6 {
                let chest_mesh = meshes.add(Cuboid::new(1.2, 0.8, 0.8));
                let chest_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.6, 0.3, 0.0),
                    perceptual_roughness: 0.8,
                    ..default()
                });
                commands.spawn((
                    Mesh3d(chest_mesh),
                    MeshMaterial3d(chest_material),
                    Transform::from_translation(Vec3::new(pos.x, 0.4, pos.z)),
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
                let emissive = match cell {
                    7 => [0.0, 1.2, 2.0, 1.0],
                    8 => [2.0, 1.6, 0.0, 1.0],
                    _ => [0.0, 2.0, 0.4, 1.0],
                };
                if cell == 7 {
                    commands.spawn((
                        SceneRoot(asset_server.load("models/banana.glb#Scene0")),
                        Transform::from_translation(Vec3::new(pos.x, 0.5, pos.z))
                            .with_scale(Vec3::splat(0.25)),
                        PowerUpItem {
                            effect_type: effect,
                        },
                        MazeElement,
                    ));
                } else if cell == 9 {
                    commands.spawn((
                        SceneRoot(asset_server.load("models/apple.glb#Scene0")),
                        Transform::from_translation(Vec3::new(pos.x, 0.5, pos.z))
                            .with_scale(Vec3::splat(0.25)),
                        PowerUpItem {
                            effect_type: effect,
                        },
                        MazeElement,
                    ));
                } else {
                    let gem_mesh = meshes.add(Sphere::new(0.36).mesh());
                    let gem_material = materials.add(StandardMaterial {
                        base_color: color,
                        emissive: LinearRgba::from_f32_array(emissive),
                        ..default()
                    });
                    commands.spawn((
                        Mesh3d(gem_mesh),
                        MeshMaterial3d(gem_material),
                        Transform::from_translation(Vec3::new(pos.x, 0.7, pos.z)),
                        PowerUpItem {
                            effect_type: effect,
                        },
                        MazeElement,
                    ));
                }
            }
        }
    }
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
