use bevy::prelude::*;
use crate::player::Player;
use crate::UiAudioAssets;

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
    // In Bevy 0.15+, UI is rendered by the main camera by default.
    // No extra camera needed unless we want a separate layer.
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
        BackgroundColor(Color::BLACK.with_alpha(0.8)),
        GameUi,
        GlobalZIndex(10),
    )).with_children(|parent| {
        // Health UI (Bar)
        parent.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        }).with_children(|health_parent| {
            health_parent.spawn((
                Text::new("VIDA"),
                TextFont { font_size: 22.0, ..default() },
                TextColor(Color::srgb(1.0, 0.3, 0.3)),
            ));
            
            // Health Bar Background
            health_parent.spawn((
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(22.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            )).with_children(|bar_bg| {
                // Health Bar Foreground
                bar_bg.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.1, 0.1)),
                    HealthBar,
                ));
            });
        });

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
            let percentage = (player.health / player.max_health) * 100.0;
            bar_node.width = Val::Percent(percentage.max(0.0));
        }
    }
}

#[derive(Component)]
pub struct DeathScreen;

pub fn setup_death_screen(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.0, 0.0).with_alpha(0.9)),
        DeathScreen,
        GlobalZIndex(100),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("HAS MUERTO"),
            TextFont { font_size: 80.0, ..default() },
            TextColor(Color::srgb(1.0, 0.0, 0.0)),
        ));

        parent.spawn((
            Text::new("PRESIONA ESPACIO PARA REINTENTAR"),
            TextFont { font_size: 30.0, ..default() },
            TextColor(Color::WHITE),
        ));
    });
}

pub fn cleanup_death_screen(mut commands: Commands, query: Query<Entity, With<DeathScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn death_screen_action(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<crate::GameState>>,
    mut commands: Commands,
    audio_assets: Res<UiAudioAssets>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn(AudioPlayer(audio_assets.revive.clone()));
        next_state.set(crate::GameState::Playing);
    }
}

#[derive(Component)]
pub struct WinScreen;

pub fn setup_win_screen(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(25.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.0, 0.2, 0.0).with_alpha(0.9)),
        WinScreen,
        GlobalZIndex(100),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("¡VICTORIA!"),
            TextFont { font_size: 80.0, ..default() },
            TextColor(Color::srgb(0.0, 1.0, 0.3)),
        ));

        parent.spawn((
            Text::new("¡Has colocado ambos bloques en sus zonas correspondientes!"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::srgb(0.8, 1.0, 0.8)),
        ));

        parent.spawn((
            Text::new("PRESIONA ESPACIO PARA REINICIAR"),
            TextFont { font_size: 30.0, ..default() },
            TextColor(Color::WHITE),
        ));

        parent.spawn((
            Text::new("PRESIONA ESC PARA IR AL MENÚ"),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));
    });
}

pub fn cleanup_win_screen(mut commands: Commands, query: Query<Entity, With<WinScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn win_screen_action(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<crate::GameState>>,
    mut commands: Commands,
    audio_assets: Res<UiAudioAssets>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn(AudioPlayer(audio_assets.revive.clone()));
        next_state.set(crate::GameState::Playing);
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        commands.spawn(AudioPlayer(audio_assets.click.clone()));
        next_state.set(crate::GameState::Menu);
    }
}
