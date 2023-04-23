use::bevy::prelude::*;

mod snake;

const WINDOW_HEIGHT: f32 = 550.;
const WINDOW_WIDTH: f32 = 550.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_HEIGHT, WINDOW_WIDTH).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                prevent_default_event_handling: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(snake::SnakeBoard::new(WINDOW_WIDTH as usize, WINDOW_HEIGHT as usize))
        .add_startup_system(snake::setup)
        .run();
}
