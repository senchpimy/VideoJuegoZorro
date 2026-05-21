use bevy::prelude::*;
use crate::GameState;
use bevy::app::AppExit;

#[derive(Component)]
pub struct OnPauseScreen;

#[derive(Component)]
pub enum PauseButtonAction {
    Resume,
    Quit,
}

pub fn setup_pause(mut commands: Commands) {
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
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
            OnPauseScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSA"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::all(Val::Px(30.0)),
                    ..default()
                },
            ));

            // Resume Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    PauseButtonAction::Resume,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("RESUMIR"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Quit Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    PauseButtonAction::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("SALIR"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn pause_action(
    interaction_query: Query<(&Interaction, &PauseButtonAction), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                PauseButtonAction::Resume => next_state.set(GameState::Playing),
                PauseButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
            }
        }
    }
}

pub fn cleanup_pause(mut commands: Commands, query: Query<Entity, With<OnPauseScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
