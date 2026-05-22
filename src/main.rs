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
use bevy::asset::AssetMetaCheck;
use bevy::window::{CursorGrabMode, CursorOptions};
use avian3d::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
    GameWon,
}

#[derive(Resource)]
pub struct UiAudioAssets {
    pub click: Handle<AudioSource>,
    pub death: Handle<AudioSource>,
    pub revive: Handle<AudioSource>,
    pub damage: Handle<AudioSource>,
    pub enemy_death: Handle<AudioSource>,
    pub steps: Handle<AudioSource>,
    pub music: Handle<AudioSource>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy".to_string()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }).set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .init_state::<GameState>()
        .add_systems(Startup, setup_ui_audio)
        // Global Startup
        .add_systems(Startup, camera::spawn_camera)
        
        // Menu State
        .add_systems(OnEnter(GameState::Menu), menu::setup_menu)
        .add_systems(Update, menu::menu_action.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), menu::cleanup_menu)

        // Playing State
        .add_systems(OnEnter(GameState::Playing), (
            maze::spawn_world, 
            tutorial::spawn_tutorial, 
            player::spawn_player, 
            ui::setup_ui,
            enemy::setup_enemy_assets,
        ))
        .add_systems(Update, (
            player::link_player_animations.run_if(in_state(GameState::Playing)),
            player::player_movement.run_if(in_state(GameState::Playing)),
            player::player_grab_block.run_if(in_state(GameState::Playing)),
            platform::move_platforms.run_if(in_state(GameState::Playing)),
            projectile::player_fire.run_if(in_state(GameState::Playing)),
            projectile::update_projectiles.run_if(in_state(GameState::Playing)),
            enemy::process_enemy_assets.run_if(in_state(GameState::Playing)),
            enemy::init_enemy_animations.run_if(in_state(GameState::Playing)),
            enemy::play_enemy_animations.run_if(in_state(GameState::Playing)),
            enemy::move_enemies.run_if(in_state(GameState::Playing)),
        ))
        .add_systems(Update, (
            enemy::check_enemy_projectile_collision.run_if(in_state(GameState::Playing)),
            enemy::check_enemy_player_collision.run_if(in_state(GameState::Playing)),
            enemy::check_player_death.run_if(in_state(GameState::Playing)),
            powerup::animate_items.run_if(in_state(GameState::Playing)),
            powerup::check_powerup_collisions.run_if(in_state(GameState::Playing)),
            powerup::check_chest_interactions.run_if(in_state(GameState::Playing)),
            tutorial::spawn_physics_cubes.run_if(in_state(GameState::Playing)),
            tutorial::update_physics_cubes.run_if(in_state(GameState::Playing)),
            tutorial::check_portal_teleport.run_if(in_state(GameState::Playing)),
        ))
        .add_systems(Update, (
            ui::update_ui.run_if(in_state(GameState::Playing)),
            maze::check_puzzle_completion.run_if(in_state(GameState::Playing)),
            camera::camera_follow.run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
            toggle_pause.run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
            handle_cursor_grab,
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
        
        // GameOver State
        .add_systems(OnEnter(GameState::GameOver), ui::setup_death_screen)
        .add_systems(Update, ui::death_screen_action.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), ui::cleanup_death_screen)

        // GameWon State
        .add_systems(OnEnter(GameState::GameWon), ui::setup_win_screen)
        .add_systems(Update, ui::win_screen_action.run_if(in_state(GameState::GameWon)))
        .add_systems(OnExit(GameState::GameWon), ui::cleanup_win_screen)

        .run();
}

fn handle_cursor_grab(
    mut cursor_options: Query<&mut CursorOptions>,
    state: Res<State<GameState>>,
) {
    if let Some(mut options) = cursor_options.iter_mut().next() {
        if *state.get() == GameState::Playing {
            options.grab_mode = CursorGrabMode::Locked;
            options.visible = false;
        } else {
            options.grab_mode = CursorGrabMode::None;
            options.visible = true;
        }
    }
}

fn toggle_pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    audio_assets: Res<UiAudioAssets>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        commands.spawn(AudioPlayer(audio_assets.click.clone()));
        match state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

fn setup_ui_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let music = asset_server.load("audio/fondo.mp3");
    
    // Spawn background music: Looping, lower volume (0.2)
    commands.spawn((
        AudioPlayer(music.clone()),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: bevy::audio::Volume::Linear(0.2),
            ..default()
        },
    ));

    commands.insert_resource(UiAudioAssets {
        click: asset_server.load("audio/click.mp3"),
        death: asset_server.load("audio/muerte.mp3"),
        revive: asset_server.load("audio/revive.mp3"),
        damage: asset_server.load("audio/dano.mp3"),
        enemy_death: asset_server.load("audio/enemigo_muerte.mp3"),
        steps: asset_server.load("audio/pasos.mp3"),
        music,
    });
}

pub fn exit_game(mut app_exit_events: MessageWriter<AppExit>) {
    app_exit_events.write(AppExit::Success);
}
