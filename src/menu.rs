use bevy::prelude::*;
use crate::GameState;

#[derive(Component)]
pub struct OnMenuScreen;

pub fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.8)),
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("LABERINTO DEL ZORRO"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.84, 0.0)), // GOLD
                Node {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("JUGAR"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn menu_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<OnMenuScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
