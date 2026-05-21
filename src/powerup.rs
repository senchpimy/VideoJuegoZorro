use bevy::prelude::*;
use crate::player::Player;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PowerUpType {
    Healing,
    Speed,
    Shield,
}

#[derive(Component)]
pub struct PowerUpItem {
    pub effect_type: PowerUpType,
}

#[derive(Component)]
pub struct Chest {
    pub opened: bool,
}

pub fn animate_items(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<PowerUpItem>, Without<Player>)>,
    mut chest_query: Query<&mut Transform, (With<Chest>, Without<Player>, Without<PowerUpItem>)>,
) {
    let elapsed = time.elapsed_secs();
    let dt = time.delta_secs();

    // Spin and bob power-ups
    for mut transform in &mut query {
        transform.rotate_y(1.5 * dt);
        transform.translation.y = 0.35 + (elapsed * 4.0).sin() * 0.08;
    }

    // Slightly bob chests if not opened (just a micro-animation)
    for mut transform in &mut chest_query {
        transform.rotate_y(0.2 * dt);
    }
}

pub fn check_powerup_collisions(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player)>,
    powerup_query: Query<(Entity, &Transform, &PowerUpItem)>,
) {
    let player_radius = 0.3;
    let powerup_radius = 0.25;

    for (player_transform, mut player) in &mut player_query {
        let player_pos = player_transform.translation;
        for (entity, transform, powerup) in &powerup_query {
            let dist = player_pos.distance(transform.translation);
            if dist < (player_radius + powerup_radius) {
                // Apply effect
                match powerup.effect_type {
                    PowerUpType::Healing => {
                        player.health = (player.health + 1).min(player.max_health);
                    }
                    PowerUpType::Speed => {
                        player.speed_boost_timer = 8.0;
                    }
                    PowerUpType::Shield => {
                        player.shield_timer = 5.0;
                    }
                }
                player.score += 50; // Extra points for power-up
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn check_chest_interactions(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &mut Player)>,
    mut chest_query: Query<(Entity, &Transform, &mut Chest)>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyE) {
        return;
    }

    for (player_transform, mut player) in &mut player_query {
        let player_pos = player_transform.translation;
        for (entity, transform, mut chest) in &mut chest_query {
            if chest.opened {
                continue;
            }

            let dist = player_pos.distance(transform.translation);
            if dist < 1.2 {
                chest.opened = true;
                player.score += 200; // Large reward!
                
                // Visual change: despawn or change color of the chest!
                // To keep it simple and visually premium, let's spawn a gold burst of light or just delete the chest and spawn gold cubes!
                commands.entity(entity).despawn();
            }
        }
    }
}
