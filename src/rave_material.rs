use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;

use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError};

pub const SIMPLE_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(67682536051574991107);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct RaveMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl From<Color> for RaveMaterial {
    fn from(color: Color) -> Self {
        RaveMaterial { color }
    }
}

impl Material for RaveMaterial {
    // fn vertex_shader() -> ShaderRef {
    //     "shaders/rave_shader.spv".into()
    // }

    fn fragment_shader() -> ShaderRef {
        "shaders/rave_shader.spv".into()
    }

    // fn alpha_mode(&self) -> AlphaMode {
    //     AlphaMode::Opaque
    // }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        //descriptor.primitive.cull_mode = None;
        // if let Some(fragment) = descriptor.fragment.as_mut() {
        //     println!("rave fragment shader found, party time!");
        //     fragment.entry_point = "main_fs".into();
        //     println!("fragement state: {:#?}", &fragment);

        // }
        descriptor.fragment.as_mut().unwrap().entry_point = "main_fs".into();
        //descriptor.vertex.entry_point = "main_vs".into();
        Ok(())
    }
}