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

pub const SIMPLE_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(67682536051574991107);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SimpleMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl From<Color> for SimpleMaterial {
    fn from(color: Color) -> Self {
        SimpleMaterial { color }
    }
}

impl Material for SimpleMaterial {
    fn vertex_shader() -> ShaderRef {
        //SIMPLE_SHADER_HANDLE.into()
        "shaders/simplest_shader.spv".into()
    }

    fn fragment_shader() -> ShaderRef {
        //SIMPLE_SHADER_HANDLE.into()
        "shaders/simplest_shader.spv".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        // if let Some(fragment) = descriptor.fragment.as_mut() {
        //     fragment.entry_point = "main_fs".into()
        // }
        //descriptor.fragment.entry_point = "main_fs".into();
        descriptor.vertex.entry_point = "main_vs".into();
        Ok(())
    }
}

use bevy::{asset::load_internal_asset, pbr::{MaterialPipeline, MaterialPipelineKey}, prelude::*, render::{mesh::MeshVertexBufferLayout, render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError}}};
// use rust_gpu_shaders::simplest_shader::SimpleMaterial;
// use rust_gpu_shaders::simplest_shader::SIMPLE_SHADER_HANDLE;

fn main() {
    let mut app = App::new();



    app
    .add_plugins(DefaultPlugins)
    .add_plugins(MaterialPlugin::<SimpleMaterial>::default())

    .add_systems(Startup, setup)
    .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut simple_mat: ResMut<Assets<SimpleMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
    //     material: materials.add(Color::rgb_u8(124, 144, 255)),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: simple_mat.add(Color::rgb_u8(124, 144, 255)),
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

