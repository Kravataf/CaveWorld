use bevy::{
    prelude::*,
    window::PresentMode,
};

pub const CLEAR_COLOR: Color = Color::srgb(0., 0., 0.);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cave World".to_string(),
                resizable: true,
                present_mode: PresentMode::Fifo, // toto je vsync vraj
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("texture.png");
    
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..Default::default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(material_handle),
        Transform::from_xyz(0., 0., 0.),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5., 5., 5.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}