use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::camera::ScalingMode,
};
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Update, close_on_esc)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 6.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn(SceneRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("kenney_td/Models/glb/tile-tree.glb"),
    )));
    commands.spawn((
        SceneRoot(
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("kenney_td/Models/glb/tile-end.glb")),
        ),
        Transform::from_xyz(1.0, 0.0, 0.0),
    ));

    ambient_light.brightness = 1500.0;
}

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
