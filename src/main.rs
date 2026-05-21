mod collision;
mod maze;
mod player;
mod camera;
mod ui;
mod menu;
mod pause;
mod platform;
mod enemy;
mod projectile;
mod powerup;
mod tutorial;

use bevy::prelude::*;
use bevy::app::AppExit;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        // Global Startup
        .add_systems(Startup, camera::spawn_camera)
        
        // Menu State
        .add_systems(OnEnter(GameState::Menu), menu::setup_menu)
        .add_systems(Update, menu::menu_action.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), menu::cleanup_menu)

        // Playing State
        .add_systems(OnEnter(GameState::Playing), (maze::spawn_world, tutorial::spawn_tutorial, player::spawn_player, ui::setup_ui))
        .add_systems(Update, (
            player::link_player_animations.run_if(in_state(GameState::Playing)),
            player::player_movement.run_if(in_state(GameState::Playing)),
            platform::move_platforms.run_if(in_state(GameState::Playing)),
            projectile::player_fire.run_if(in_state(GameState::Playing)),
            projectile::update_projectiles.run_if(in_state(GameState::Playing)),
            enemy::move_enemies.run_if(in_state(GameState::Playing)),
            enemy::check_enemy_projectile_collision.run_if(in_state(GameState::Playing)),
            enemy::check_enemy_player_collision.run_if(in_state(GameState::Playing)),
            powerup::animate_items.run_if(in_state(GameState::Playing)),
            powerup::check_powerup_collisions.run_if(in_state(GameState::Playing)),
            powerup::check_chest_interactions.run_if(in_state(GameState::Playing)),
            ui::update_ui.run_if(in_state(GameState::Playing)),
            camera::camera_follow.run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
            toggle_pause.run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
        ))
        .add_systems(OnExit(GameState::Playing), (
            maze::cleanup_world,
            tutorial::cleanup_tutorial,
            player::cleanup_player,
            ui::cleanup_ui,
            projectile::cleanup_projectiles,
        ))

        // Paused State
        .add_systems(OnEnter(GameState::Paused), pause::setup_pause)
        .add_systems(Update, pause::pause_action.run_if(in_state(GameState::Paused)))
        .add_systems(OnExit(GameState::Paused), pause::cleanup_pause)
        
        .run();
}

fn toggle_pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

pub fn exit_game(mut app_exit_events: MessageWriter<AppExit>) {
    app_exit_events.write(AppExit::Success);
}
