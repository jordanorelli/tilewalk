mod input;
mod debug_view;

use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

use input::PlayerInput;
use debug_view::DebugViewPlugin;

#[derive(Component)]
struct Player;

const TILE_WIDTH: i32 = 32;
const TILE_HEIGHT: i32 = 32;
const BACKGROUND_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

struct TileSize {
    width: i32,
    height: i32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle{
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn()
        .insert(Player)
        .insert_bundle(SpriteBundle{
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load("icon.png"),
            sprite: Sprite{
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            ..default()
        });

    for x in 0..10 {
        for y in 0..10 {
            commands.spawn_bundle(SpriteBundle{
                transform: Transform::from_xyz(32.0 * x as f32, 32.0 * y as f32, 0.0),
                texture: asset_server.load("grass-tile.png"),
                sprite: Sprite{
                    color: Color::rgb(1.0, 1.0, 1.0),
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                ..default()
            });
        }
    }

}

/// player_keyboard_input is a system that maps keyboard key presses to the player's input state
fn player_keyboard_input(now: Res<Input<KeyCode>>, mut input_state: ResMut<PlayerInput>) {
    input_state.update(&now);
}

fn player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    input_state: Res<PlayerInput>,
    tile: Res<TileSize>,
) {
    let mut player_t = query.single_mut();
    if input_state.up.pressed    { player_t.translation.y += tile.height as f32; }
    if input_state.down.pressed  { player_t.translation.y -= tile.height as f32; }
    if input_state.left.pressed  { player_t.translation.x -= tile.width as f32; }
    if input_state.right.pressed { player_t.translation.x += tile.width as f32; }
}

fn main() {
    App::new()
        .insert_resource(bevy::log::LogSettings{
            // set to TRACE|DEBUG|INFO|WARN|ERROR to change log verbosity.
            // INFO is the default level.
            level: bevy::log::Level::INFO,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugViewPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(PlayerInput::default())
        .insert_resource(TileSize{width: TILE_WIDTH, height: TILE_HEIGHT})
        .add_startup_system(setup)
        .add_system(player_keyboard_input)
        .add_system(player_movement)
        .add_system(bevy::window::close_on_esc)
        .run();
}
