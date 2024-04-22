//! bevy material shader example

// pub struct SimpleMaterial;

// pub struct SimpleMaterialPlugin;

// impl Plugin for SimpleMaterialPlugin {
//     fn build(&self, app: &mut App) {
//         load_internal_asset!(
//             app,
//             SimpleMaterial::GIZMO_SHADER_HANDLE,
//             "simplest_shader.spv",
//             Shader::from_wgsl
//         );
//     }
// }



use bevy::{asset::load_internal_asset, pbr::{MaterialPipeline, MaterialPipelineKey}, prelude::*, render::{mesh::MeshVertexBufferLayout, render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError}}};
use bevy_mod_rust_gpu::rave_material::RaveMaterial;
// use rust_gpu_shaders::simplest_shader::SimpleMaterial;
// use rust_gpu_shaders::simplest_shader::SIMPLE_SHADER_HANDLE;

fn main() {
    let mut app = App::new();



    app
    .add_plugins(DefaultPlugins)
    .add_plugins(MaterialPlugin::<RaveMaterial> {
        prepass_enabled: false,
        ..default()
    })

    .add_systems(Startup, setup)
    .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rave_material: ResMut<Assets<RaveMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // rave cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: rave_material.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

