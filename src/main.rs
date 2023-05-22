use std::{f32::consts::PI, time::Duration};

use basic_camera::{CameraController, CameraControllerPlugin};
use bevy::{
    asset::ChangeWatcher,
    core_pipeline::{
        experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
        tonemapping::Tonemapping,
    },
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
};
use plugin::{VoxelBundle, VoxelMaterial, VoxelPlugin};

mod basic_camera;
mod plugin;
mod vox;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..Default::default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::Mailbox,
                        ..default()
                    }),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    wgpu_settings: WgpuSettings {
                        ..Default::default()
                    },
                }),
        )
        .insert_resource(AmbientLight {
            brightness: 1.0,
            ..Default::default()
        })
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(CameraControllerPlugin)
        .add_plugin(VoxelPlugin)
        .add_plugin(TemporalAntiAliasPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut vox_materials: ResMut<Assets<VoxelMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(VoxelBundle {
        material: vox_materials.add(VoxelMaterial {
            vox: asset_server.load(r#"C:\Users\dylan\dev\lastattempt\assets\vox\3x3x3.vox"#),
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0)
            .with_rotation(Quat::from_rotation_y(PI / 4.0)),
        ..Default::default()
    });

    commands.spawn(VoxelBundle {
        material: vox_materials.add(VoxelMaterial {
            vox: asset_server.load(r#"C:\Users\dylan\dev\lastattempt\assets\vox\castle.vox"#),
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, -10.0),
        ..Default::default()
    });

    commands.spawn(VoxelBundle {
        material: vox_materials.add(VoxelMaterial {
            vox: asset_server.load(r#"C:\Users\dylan\dev\lastattempt\assets\vox\monu3.vox"#),
            ..Default::default()
        }),
        transform: Transform::from_xyz(-20.0, 0.5, -10.0),
        ..Default::default()
    });

    let plane = asset_server.load(r#"C:\Users\dylan\dev\lastattempt\assets\vox\basic-tile.vox"#);

    for x in 0..10 {
        for z in 0..10 {
            commands.spawn(VoxelBundle {
                material: vox_materials.add(VoxelMaterial {
                    vox: plane.clone(),
                    ..Default::default()
                }),
                transform: Transform::from_xyz(-x as f32 * 50.0, -5.0, -z as f32 * 50.0),
                ..Default::default()
            });
        }
    }

    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(1.0, 0.4, 0.4).into()),
        transform: Transform::from_xyz(-1.5, 0.5, 0.0)
            .with_rotation(Quat::from_rotation_y(PI / 4.0)),
        ..Default::default()
    });

    // camera
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                tonemapping: Tonemapping::TonyMcMapface,
                ..default()
            },
            TemporalAntiAliasBundle::default(),
        ))
        .insert(CameraController {
            orbit_mode: true,
            orbit_focus: Vec3::new(0.0, 0.5, 0.0),
            ..default()
        });
}
