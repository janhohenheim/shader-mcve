use crate::grass::GrassBlade;
use crate::plugin::GRASS_SHADER_HANDLE;
use bevy::pbr::{MeshPipeline, MeshPipelineKey};
use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, RenderPipelineDescriptor, ShaderStages, SpecializedMeshPipeline,
    SpecializedMeshPipelineError, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexStepMode,
};
use bevy::render::renderer::RenderDevice;

#[derive(Resource)]
pub struct GrassPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
    pub region_outline: BindGroupLayout,
}

impl FromWorld for GrassPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();
        let region_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Grass configuration layout"),
            entries: &[
                // color
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let shader = GRASS_SHADER_HANDLE.typed::<Shader>();
        let mesh_pipeline = world.resource::<MeshPipeline>();
        GrassPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
            region_outline: region_layout,
        }
    }
}

impl SpecializedMeshPipeline for GrassPipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;
        descriptor.label = Some("Grass render pipeline".into());
        descriptor.vertex.shader = self.shader.clone();
        let layouts = descriptor.layout.get_or_insert(Vec::new());
        layouts.push(self.region_outline.clone());
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<GrassBlade>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                // position of the mesh as instance
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 1, // Todo: can this be 0?
                },
                // height scale
                VertexAttribute {
                    format: VertexFormat::Float32,
                    offset: VertexFormat::Float32x3.size(),
                    shader_location: 2,
                },
            ],
        });
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        Ok(descriptor)
    }
}
