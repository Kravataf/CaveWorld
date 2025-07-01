use bevy::{
    prelude::*,
    text::{FontSmoothing, LineBreak, TextBounds},
};

pub const CLEAR_COLOR: Color = Color::srgb(1., 0., 0.);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cave World".to_string(),
                resizable: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run();
}