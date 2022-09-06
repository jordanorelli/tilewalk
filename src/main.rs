use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

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


fn map_key(now: &Res<Input<KeyCode>>, key: KeyCode, handler: &mut bool) {
    *handler = now.just_pressed(key);
    // if now.just_pressed(key)  { *handler = true; }
    // if now.just_released(key) { *handler = false; }
}

fn keyboard_input(now: Res<Input<KeyCode>>, mut input_state: ResMut<PlayerInput>) {
    map_key(&now, KeyCode::W, &mut input_state.up);
    map_key(&now, KeyCode::Up, &mut input_state.up);

    map_key(&now, KeyCode::A, &mut input_state.left);
    map_key(&now, KeyCode::Left, &mut input_state.left);

    map_key(&now, KeyCode::S, &mut input_state.down);
    map_key(&now, KeyCode::Down, &mut input_state.down);

    map_key(&now, KeyCode::D, &mut input_state.right);
    map_key(&now, KeyCode::Right, &mut input_state.right);
}

fn player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    input_state: Res<PlayerInput>
) {
    let mut player_t = query.single_mut();
    if input_state.up    { player_t.translation.y += 32.0; }
    if input_state.down  { player_t.translation.y -= 32.0; }
    if input_state.left  { player_t.translation.x -= 32.0; }
    if input_state.right { player_t.translation.x += 32.0; }
}

#[derive(Default, Debug)]
struct PlayerInput {
    up:    bool,
    down:  bool,
    left:  bool,
    right: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(PlayerInput::default())
        .insert_resource(TileSize{width: TILE_WIDTH, height: TILE_HEIGHT})
        .add_startup_system(setup)
        .add_system(keyboard_input)
        .add_system(player_movement)
        .add_system(bevy::window::close_on_esc)
        .run();
}
