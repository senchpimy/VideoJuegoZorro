use bevy::prelude::*;
use crate::collision::Wall;
use crate::enemy::Enemy;
use crate::powerup::{PowerUpItem, PowerUpType};

#[derive(Component)]
pub struct TutorialElement;

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

    // ROOM 1: MOVEMENT
    spawn_room(&mut commands, &mut meshes, floor_material.clone(), wall_material.clone(), 
        TUTORIAL_OFFSET, "BIENVENIDO\nUSA WASD PARA MOVERTE", true, false);

    // ROOM 2: JUMPING
    let room2_pos = TUTORIAL_OFFSET + Vec3::new(0.0, 0.0, -20.0);
    spawn_room(&mut commands, &mut meshes, floor_material.clone(), wall_material.clone(), 
        room2_pos, "SALTO\nESPACIO PARA SALTAR", true, true);
    
    // Obstacle
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(8.0, 0.8, 2.0))),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_translation(room2_pos + Vec3::new(0.0, 0.4, 0.0)),
        Wall { half_size: Vec3::new(4.0, 0.4, 1.0) },
        TutorialElement,
    ));

    // ROOM 3: COMBAT
    let room3_pos = room2_pos + Vec3::new(0.0, 0.0, -20.0);
    spawn_room(&mut commands, &mut meshes, floor_material.clone(), wall_material.clone(), 
        room3_pos, "COMBATE\nSHIFT PARA ATACAR", true, true);
    
    commands.spawn((
        SceneRoot(asset_server.load("models/scorcher_enemy.glb#Scene0")),
        Transform::from_translation(room3_pos + Vec3::new(0.0, 5.0, -4.0))
            .with_scale(Vec3::splat(0.012)),
        Enemy {
            speed: 0.0,
            patrol_points: vec![],
            current_waypoint: 0,
            health: 1.0,
        },
        TutorialElement,
    ));

    // ROOM 4: POWERUPS
    let room4_pos = room3_pos + Vec3::new(0.0, 0.0, -20.0);
    spawn_room(&mut commands, &mut meshes, floor_material.clone(), wall_material.clone(), 
        room4_pos, "POWER-UPS\nMEJORA TUS HABILIDADES", false, true);
    
    // Powerups
    commands.spawn((
        SceneRoot(asset_server.load("models/banana.glb#Scene0")),
        Transform::from_translation(room4_pos + Vec3::new(-3.0, 0.5, 0.0))
            .with_scale(Vec3::splat(0.25)),
        PowerUpItem { effect_type: PowerUpType::Speed },
        TutorialElement,
    ));
    
    commands.spawn((
        SceneRoot(asset_server.load("models/apple.glb#Scene0")),
        Transform::from_translation(room4_pos + Vec3::new(3.0, 0.5, 0.0))
            .with_scale(Vec3::splat(0.25)),
        PowerUpItem { effect_type: PowerUpType::Healing },
        TutorialElement,
    ));

    // Lights
    for pos in [TUTORIAL_OFFSET, room2_pos, room3_pos, room4_pos] {
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

fn spawn_room(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    floor_mat: Handle<StandardMaterial>,
    wall_mat: Handle<StandardMaterial>,
    pos: Vec3,
    text: &str,
    open_n: bool,
    open_s: bool,
) {
    let size = 20.0;
    let h = 5.0;
    let thick = 0.5;

    // Floor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(size, size))),
        MeshMaterial3d(floor_mat),
        Transform::from_translation(pos),
        TutorialElement,
    ));

    // WALLS
    // North
    if open_n {
        spawn_wall(commands, meshes, wall_mat.clone(), pos + Vec3::new(-7.0, h/2.0, -size/2.0), Vec3::new(6.0, h, thick));
        spawn_wall(commands, meshes, wall_mat.clone(), pos + Vec3::new(7.0, h/2.0, -size/2.0), Vec3::new(6.0, h, thick));
    } else {
        spawn_wall(commands, meshes, wall_mat.clone(), pos + Vec3::new(0.0, h/2.0, -size/2.0), Vec3::new(size, h, thick));
    }

    // South
    if open_s {
        spawn_wall(commands, meshes, wall_mat.clone(), pos + Vec3::new(-7.0, h/2.0, size/2.0), Vec3::new(6.0, h, thick));
        spawn_wall(commands, meshes, wall_mat.clone(), pos + Vec3::new(7.0, h/2.0, size/2.0), Vec3::new(6.0, h, thick));
    } else {
        spawn_wall(commands, meshes, wall_mat.clone(), pos + Vec3::new(0.0, h/2.0, size/2.0), Vec3::new(size, h, thick));
    }

    // West & East
    spawn_wall(commands, meshes, wall_mat.clone(), pos + Vec3::new(-size/2.0, h/2.0, 0.0), Vec3::new(thick, h, size));
    spawn_wall(commands, meshes, wall_mat.clone(), pos + Vec3::new(size/2.0, h/2.0, 0.0), Vec3::new(thick, h, size));

    // IN-WORLD 3D TEXT (Actual Text2d in 3D Space)
    // We attach it to the wall
    commands.spawn((
        Text2d::new(text),
        TextFont { font_size: 60.0, ..default() },
        TextColor(Color::WHITE),
        // Positioned eye-level on the north wall, slightly in front
        Transform::from_translation(pos + Vec3::new(0.0, 3.0, -size/2.0 + 0.3)),
        TutorialElement,
    ));
}

fn spawn_wall(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, mat: Handle<StandardMaterial>, pos: Vec3, size: Vec3) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(size))),
        MeshMaterial3d(mat),
        Transform::from_translation(pos),
        Wall { half_size: size / 2.0 },
        TutorialElement,
    ));
}

pub fn cleanup_tutorial(mut commands: Commands, query: Query<Entity, With<TutorialElement>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
