use::bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (500., 500.).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                prevent_default_event_handling: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    cmd.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });

    cmd.spawn(Camera2dBundle::default());
}
