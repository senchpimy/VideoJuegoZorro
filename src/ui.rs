use bevy::prelude::*;
use crate::player::Player;

#[derive(Component)]
pub struct HealthBar;

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
            padding: UiRect::all(Val::Px(10.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(12.0),
            ..default()
        },
        GameUi,
    ))
    .insert(BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.4)))
    .with_children(|parent| {
        // Health UI (Bar)
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(12.0),
                ..default()
            },
        )).with_children(|health_parent| {
            health_parent.spawn((
                Text::new("VIDA"),
                TextFont { font_size: 22.0, ..default() },
                TextColor(Color::srgb(1.0, 0.3, 0.3)),
                Node::default(),
            ));
            
            // Health Bar Background
            health_parent.spawn((
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(22.0),
                    ..default()
                },
            ))
            .insert(BackgroundColor(Color::srgb(0.1, 0.1, 0.1)))
            .with_children(|bar_bg| {
                // Health Bar Foreground
                bar_bg.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    HealthBar,
                ))
                .insert(BackgroundColor(Color::srgb(1.0, 0.1, 0.1)));
            });
        });

        // Score UI
        parent.spawn((
            Text::new("PUNTOS: 0"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::srgb(1.0, 0.84, 0.0)),
            Node::default(),
            ScoreText,
        ));

        // PowerUp active state UI
        parent.spawn((
            Text::new(""),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.0, 0.8, 1.0)),
            Node::default(),
            PowerUpText,
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
    mut text_query: Query<(&mut Text, Option<&ScoreText>, Option<&PowerUpText>)>,
    mut bar_query: Query<&mut Node, With<HealthBar>>,
) {
    if let Some(player) = player_query.iter().next() {
        // Update texts
        for (mut text, score, powerup) in &mut text_query {
            if score.is_some() {
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

        // Update health bar
        if let Some(mut bar_node) = bar_query.iter_mut().next() {
            let percentage = (player.health as f32 / player.max_health as f32) * 100.0;
            bar_node.width = Val::Percent(percentage.max(0.0));
        }
    }
}
