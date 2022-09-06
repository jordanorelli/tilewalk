use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

#[derive(Component)]
struct Player;

const BACKGROUND_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle{
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn()
        .insert(Player)
        .insert_bundle(SpriteBundle{
            texture: asset_server.load("icon.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
}


fn map_key(now: &Res<Input<KeyCode>>, key: KeyCode, handler: &mut bool) {
    if now.just_pressed(key)  { *handler = true; }
    if now.just_released(key) { *handler = false; }
}

fn keyboard_input(now: Res<Input<KeyCode>>, mut input_state: ResMut<PlayerInput>) {
    map_key(&now, KeyCode::A, &mut input_state.left);
    map_key(&now, KeyCode::W, &mut input_state.up);
    map_key(&now, KeyCode::S, &mut input_state.down);
    map_key(&now, KeyCode::D, &mut input_state.right);
}

fn player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    input_state: Res<PlayerInput>
) {
    let mut player_t = query.single_mut();
    if input_state.up    { player_t.translation.y += 1.0; }
    if input_state.down  { player_t.translation.y -= 1.0; }
    if input_state.left  { player_t.translation.x -= 1.0; }
    if input_state.right { player_t.translation.x += 1.0; }
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
        .add_startup_system(setup)
        .insert_resource(PlayerInput::default())
        .add_system(keyboard_input)
        .add_system(player_movement)
        .add_system(bevy::window::close_on_esc)
        .run();
}
