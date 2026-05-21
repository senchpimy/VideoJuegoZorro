use bevy::prelude::*;
use crate::collision::Wall;
use crate::platform::MovingPlatform;
use crate::enemy::Enemy;
use crate::powerup::{PowerUpItem, PowerUpType, Chest};

// 1 = Wall, 0 = Empty, 2 = Player Start, 3 = Moving Platform Pit, 4 = Lava Static Pit, 5 = Enemy Spawn, 6 = Chest, 7 = Speed Gem, 8 = Shield Gem, 9 = Healing Gem
pub const MAZE_DATA: [[u8; 15]; 15] = [
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,2,0,0,1,0,0,0,0,0,1,0,0,9,1],
    [1,1,1,0,1,0,1,1,1,0,1,0,1,0,1],
    [1,0,4,0,3,0,1,0,0,0,0,0,1,0,1],
    [1,0,1,1,1,1,1,0,1,1,1,1,1,0,1],
    [1,0,1,0,3,0,0,0,1,0,0,0,0,0,1],
    [1,0,1,0,1,1,1,1,1,0,1,1,1,0,1],
    [1,0,0,0,1,0,0,0,0,0,0,0,1,7,1],
    [1,1,1,0,1,1,1,0,1,1,1,0,1,0,1],
    [1,0,0,5,0,0,1,0,1,0,0,0,1,0,1],
    [1,0,1,1,1,0,1,0,1,0,1,1,1,0,1],
    [1,0,0,0,1,0,0,0,1,0,0,0,0,6,1],
    [1,1,1,0,1,1,1,1,1,1,1,1,1,0,1],
    [1,0,0,0,5,0,0,8,0,0,5,0,0,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
];

pub fn spawn_maze(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_mesh = meshes.add(Cuboid::new(1.0, 2.0, 1.0));
    let wall_material = materials.add(Color::from(LinearRgba::from_f32_array([0.4, 0.4, 0.4, 1.0])));
    let ground_material = materials.add(Color::from(LinearRgba::from_f32_array([0.1, 0.1, 0.1, 1.0])));

    for (z, row) in MAZE_DATA.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let pos = Vec3::new(x as f32, 0.0, z as f32);
            
            // Ground: spawn only if it's not a pit/platform hole
            if cell != 3 && cell != 4 {
                commands.spawn((
                    Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
                    MeshMaterial3d(ground_material.clone()),
                    Transform::from_translation(pos),
                ));
            }

            if cell == 1 {
                commands.spawn((
                    Mesh3d(wall_mesh.clone()),
                    MeshMaterial3d(wall_material.clone()),
                    Transform::from_xyz(pos.x, 1.0, pos.z),
                    Wall,
                ));
            }

            // Spawn moving platform
            if cell == 3 {
                let platform_mesh = meshes.add(Cuboid::new(1.0, 0.15, 1.0));
                let platform_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.0, 0.8, 1.0),
                    emissive: LinearRgba::from_f32_array([0.0, 0.4, 0.8, 1.0]),
                    ..default()
                });
                
                // Platforms oscillate along Z axis
                let start_pos = Vec3::new(pos.x, 0.0, pos.z - 0.8);
                let end_pos = Vec3::new(pos.x, 0.0, pos.z + 0.8);
                
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
                ));

                // Red lava glow far below
                let lava_mesh = meshes.add(Plane3d::default().mesh().size(1.0, 1.0));
                let lava_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.8, 0.1, 0.0),
                    emissive: LinearRgba::from_f32_array([0.6, 0.05, 0.0, 1.0]),
                    ..default()
                });
                commands.spawn((
                    Mesh3d(lava_mesh),
                    MeshMaterial3d(lava_material),
                    Transform::from_xyz(pos.x, -2.0, pos.z),
                ));
            }

            // Spawn static lava pit
            if cell == 4 {
                let lava_mesh = meshes.add(Plane3d::default().mesh().size(1.0, 1.0));
                let lava_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.8, 0.1, 0.0),
                    emissive: LinearRgba::from_f32_array([0.6, 0.05, 0.0, 1.0]),
                    ..default()
                });
                commands.spawn((
                    Mesh3d(lava_mesh),
                    MeshMaterial3d(lava_material),
                    Transform::from_xyz(pos.x, -2.0, pos.z),
                ));
            }

            // Spawn patrolling enemy
            if cell == 5 {
                let enemy_mesh = meshes.add(Cuboid::new(0.6, 0.6, 0.6));
                let enemy_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.5, 0.0, 1.0),
                    emissive: LinearRgba::from_f32_array([1.5, 0.0, 3.0, 1.0]),
                    ..default()
                });

                // Let enemy patrol horizontally (X-axis) back and forth
                let patrol_points = vec![
                    Vec3::new(pos.x - 1.5, 0.3, pos.z),
                    Vec3::new(pos.x + 1.5, 0.3, pos.z),
                ];

                commands.spawn((
                    Mesh3d(enemy_mesh),
                    MeshMaterial3d(enemy_material),
                    Transform::from_translation(Vec3::new(pos.x, 0.3, pos.z)),
                    Enemy {
                        speed: 1.5,
                        patrol_points,
                        current_waypoint: 0,
                        health: 2.0,
                    },
                ));
            }

            // Spawn treasure chest
            if cell == 6 {
                let chest_mesh = meshes.add(Cuboid::new(0.6, 0.4, 0.4));
                let chest_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.6, 0.3, 0.0),
                    perceptual_roughness: 0.8,
                    ..default()
                });
                
                commands.spawn((
                    Mesh3d(chest_mesh),
                    MeshMaterial3d(chest_material),
                    Transform::from_translation(Vec3::new(pos.x, 0.2, pos.z)),
                    Chest { opened: false },
                ));
            }

            // Spawn speed gem
            if cell == 7 {
                let gem_mesh = meshes.add(Sphere::new(0.18).mesh());
                let gem_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.0, 0.6, 1.0),
                    emissive: LinearRgba::from_f32_array([0.0, 1.2, 2.0, 1.0]),
                    ..default()
                });
                
                commands.spawn((
                    Mesh3d(gem_mesh),
                    MeshMaterial3d(gem_material),
                    Transform::from_translation(Vec3::new(pos.x, 0.35, pos.z)),
                    PowerUpItem { effect_type: PowerUpType::Speed },
                ));
            }

            // Spawn shield gem
            if cell == 8 {
                let gem_mesh = meshes.add(Sphere::new(0.18).mesh());
                let gem_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(1.0, 0.8, 0.0),
                    emissive: LinearRgba::from_f32_array([2.0, 1.6, 0.0, 1.0]),
                    ..default()
                });
                
                commands.spawn((
                    Mesh3d(gem_mesh),
                    MeshMaterial3d(gem_material),
                    Transform::from_translation(Vec3::new(pos.x, 0.35, pos.z)),
                    PowerUpItem { effect_type: PowerUpType::Shield },
                ));
            }

            // Spawn healing gem
            if cell == 9 {
                let gem_mesh = meshes.add(Sphere::new(0.18).mesh());
                let gem_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.0, 1.0, 0.2),
                    emissive: LinearRgba::from_f32_array([0.0, 2.0, 0.4, 1.0]),
                    ..default()
                });
                
                commands.spawn((
                    Mesh3d(gem_mesh),
                    MeshMaterial3d(gem_material),
                    Transform::from_translation(Vec3::new(pos.x, 0.35, pos.z)),
                    PowerUpItem { effect_type: PowerUpType::Healing },
                ));
            }
        }
    }

    // Light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::new(7.0, 0.0, 7.0), Vec3::Y),
    ));
}
