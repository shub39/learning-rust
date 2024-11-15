use bevy::prelude::*;

pub mod alien;
pub mod game;
pub mod resolution;

fn main() {
    App::new()
        .add_plugins((
            // list of plugins
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Space Invaders"),
                        position: WindowPosition::Automatic,
                        resolution: Vec2::new(512., 512.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            game::GamePlugin,
        ))
        .run();
}
