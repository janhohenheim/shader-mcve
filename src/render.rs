use crate::cache::GrassCache;
use bevy::ecs::system::lifetimeless::{Read, SQuery, SRes};
use bevy::ecs::system::SystemParamItem;
use bevy::prelude::*;
use bevy::render::mesh::GpuBufferInfo;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::{EntityRenderCommand, RenderCommandResult, TrackedRenderPass};

pub struct DrawMeshInstanced;

impl EntityRenderCommand for DrawMeshInstanced {
    type Param = (
        SRes<RenderAssets<Mesh>>,
        SRes<GrassCache>,
        SQuery<Read<Handle<Mesh>>>,
    );

    #[inline]
    fn render<'w>(
        _view: Entity,
        item: Entity,
        (meshes, cache, mesh_query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let mesh_handle = mesh_query.get(item).unwrap();
        let gpu_mesh = match meshes.into_inner().get(mesh_handle) {
            Some(mesh) => mesh,
            None => return RenderCommandResult::Failure,
        };
        if !cache.contains_key(&item) {
            return RenderCommandResult::Failure;
        }
        let chunk = cache.into_inner().get(&item).unwrap();
        // set uniform
        pass.set_bind_group(2, chunk.uniform_bind_ground.as_ref().unwrap(), &[]);
        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, chunk.buffer.as_ref().unwrap().slice(..));
        let grass_blade_count = chunk.grass.instances.len() as u32;
        match &gpu_mesh.buffer_info {
            GpuBufferInfo::Indexed {
                buffer,
                index_format,
                count,
            } => {
                pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                pass.draw_indexed(0..*count, 0, 0..grass_blade_count)
            }
            GpuBufferInfo::NonIndexed { vertex_count } => {
                pass.draw(0..*vertex_count, 0..grass_blade_count)
            }
        }
        RenderCommandResult::Success
    }
}
