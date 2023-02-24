use crate::cache::GrassCache;
use crate::pipeline::GrassPipeline;
use crate::RegionConfig;
use bevy::prelude::*;
use bevy::render::render_resource::{
    BindGroupDescriptor, BindGroupEntry, BindingResource, BufferBinding, BufferInitDescriptor,
    BufferUsages, ShaderType,
};
use bevy::render::renderer::RenderDevice;
use bytemuck::{Pod, Zeroable};

pub fn prepare_instance_buffer(mut cache: ResMut<GrassCache>, render_device: Res<RenderDevice>) {
    if !cache.is_changed() {
        return;
    }
    for instance_data in cache.values_mut() {
        let entity_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("Instance entity buffer"),
            contents: bytemuck::cast_slice(&instance_data.grass.instances.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        instance_data.buffer = Some(entity_buffer);
    }
}

pub fn prepare_uniform_buffers(
    pipeline: Res<GrassPipeline>,
    mut cache: ResMut<GrassCache>,
    region_config: Res<RegionConfig>,
    render_device: Res<RenderDevice>,
) {
    if !region_config.is_changed() {
        return;
    }

    let shader_config = ShaderRegionConfig::from(region_config.as_ref());
    let color_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("Config"),
        contents: bytemuck::bytes_of(&shader_config),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let layout = pipeline.region_outline.clone();
    let bind_group_descriptor = BindGroupDescriptor {
        label: Some("Grass uniform bind group"),
        layout: &layout,
        entries: &[
            // config
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &color_buffer,
                    offset: 0,
                    size: None,
                }),
            },
        ],
    };
    let bind_group = render_device.create_bind_group(&bind_group_descriptor);

    for instance_data in cache.values_mut() {
        instance_data.uniform_bind_ground = Some(bind_group.clone());
    }
}

#[derive(Debug, Clone, Copy, Pod, Zeroable, ShaderType)]
#[repr(C)]
struct ShaderRegionConfig {
    main_color: Vec4,
    bottom_color: Vec4,
}

impl From<&RegionConfig> for ShaderRegionConfig {
    fn from(config: &RegionConfig) -> Self {
        Self {
            main_color: config.main_color.into(),
            bottom_color: config.bottom_color.into(),
        }
    }
}
