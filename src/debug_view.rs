use bevy::prelude::*;

const DEBUG_PINK: Color = Color::rgb(0.8, 0.2, 0.2);

#[derive(Component)]
struct DebugView;

pub struct DebugViewPlugin;

impl Plugin for DebugViewPlugin {
    fn build(&self, app: &mut App) {
        info!("setting up the DebugView plugin");
        app
            .add_startup_system(init_view)
            .add_system(debug_input);
    }
}

fn init_view(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut root = commands.spawn();

    root.insert(DebugView);
    root.insert_bundle(NodeBundle{
        visibility: Visibility{is_visible: false},
        style: Style{
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    });

    root.with_children(|parent| {
        let text_style = TextStyle{
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 20.0,
            color: DEBUG_PINK,
        };
        let text_bundle = TextBundle::from_section("fart", text_style)
            .with_text_alignment(TextAlignment::TOP_RIGHT)
            .with_style(Style{
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..default()
                },
                ..default()
            });
        parent.spawn_bundle(text_bundle);
    });
}

fn debug_input(now: Res<Input<KeyCode>>, mut query: Query<&mut Visibility, With<DebugView>>) {
    let mut visibility = query.single_mut();

    if now.just_pressed(KeyCode::F10) {
        visibility.is_visible = !visibility.is_visible;
    }
}
