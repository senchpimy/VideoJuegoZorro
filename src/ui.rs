use bevy::prelude::*;
use crate::player::Player;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct PowerUpText;

#[derive(Component)]
pub struct GameUi;

pub fn setup_ui(mut commands: Commands) {
    // Parent container for top-left UI
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(15.0),
            left: Val::Px(15.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        },
        GameUi,
    )).with_children(|parent| {
        // Health UI
        parent.spawn((
            Text::new("VIDA: ❤️❤️❤️"),
            TextFont { font_size: 26.0, ..default() },
            TextColor(Color::srgb(1.0, 0.2, 0.2)),
            HealthText,
        ));

        // Score UI
        parent.spawn((
            Text::new("PUNTOS: 0"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::srgb(1.0, 0.84, 0.0)),
            ScoreText,
        ));

        // PowerUp active state UI
        parent.spawn((
            Text::new(""),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.0, 0.8, 1.0)),
            PowerUpText,
        ));

        // Controls reminder
        parent.spawn((
            Text::new("WASD: Mover | ESPACIO: Saltar | J / Clic Izq: Fuego Espiritual | E: Abrir Cofres | ESC: Pausa"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
            Node {
                margin: UiRect::top(Val::Px(15.0)),
                ..default()
            }
        ));
    });
}

pub fn cleanup_ui(mut commands: Commands, query: Query<Entity, With<GameUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn update_ui(
    player_query: Query<&Player>,
    mut text_query: Query<(&mut Text, Option<&HealthText>, Option<&ScoreText>, Option<&PowerUpText>)>,
) {
    if let Some(player) = player_query.iter().next() {
        for (mut text, health, score, powerup) in &mut text_query {
            if health.is_some() {
                let mut hearts = String::new();
                for _ in 0..player.health {
                    hearts.push_str("❤️");
                }
                if hearts.is_empty() {
                    hearts.push_str("💀 DERROTADO");
                }
                text.0 = format!("VIDA: {}", hearts);
            } else if score.is_some() {
                text.0 = format!("PUNTOS: {}", player.score);
            } else if powerup.is_some() {
                let mut active_effects = Vec::new();
                if player.speed_boost_timer > 0.0 {
                    active_effects.push(format!("⚡ VELOCIDAD ({:.1}s)", player.speed_boost_timer));
                }
                if player.shield_timer > 0.0 {
                    active_effects.push(format!("🛡️ ESCUDO ({:.1}s)", player.shield_timer));
                }
                
                if active_effects.is_empty() {
                    text.0 = String::new();
                } else {
                    text.0 = active_effects.join(" | ");
                }
            }
        }
    }
}
