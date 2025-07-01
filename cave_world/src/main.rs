use std::f32::consts::PI;

use bevy::{
    color::palettes::css::*,
    pbr::CascadeShadowConfigBuilder,
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

    for i in 0..10 {
        for j in 0..10 {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(material_handle.clone()),
                Transform::from_xyz(i as f32, 0.0, j as f32),
            ));
        }
    }

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5., 5., 5.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 1.0,
        ..default()
    });

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));
}