use crate::cache::GrassCache;
use crate::pipeline::GrassPipeline;
use crate::RegionConfig;
use bevy::prelude::*;
use bevy::render::render_resource::{
    BindGroupDescriptor, BindGroupEntry, BindingResource, BufferBinding, BufferInitDescriptor,
    BufferUsages,
};
use bevy::render::renderer::RenderDevice;

pub fn prepare_instance_buffers(
    pipeline: Res<GrassPipeline>,
    mut cache: ResMut<GrassCache>,
    region_config: Res<RegionConfig>,
    render_device: Res<RenderDevice>,
) {
    for instance_data in cache.values_mut() {
        let entity_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("Instance entity buffer"),
            contents: bytemuck::cast_slice(&instance_data.grass.instances.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        let color_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("Color buffer"),
            contents: bytemuck::cast_slice(&region_config.color.as_rgba_f32()),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let layout = pipeline.region_outline.clone();
        let bind_group_descriptor = BindGroupDescriptor {
            label: Some("Grass uniform bind group"),
            layout: &layout,
            entries: &[
                // color
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
        instance_data.buffer = Some(entity_buffer); // Todo: Do we have to override this each time?
        instance_data.uniform_bind_ground = Some(bind_group);
    }
}
