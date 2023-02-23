use crate::cache::EntityCache;
use crate::grass_pipeline::GrassPipeline;
use crate::GrassDrawCall;
use bevy::core_pipeline::core_3d::Opaque3d;
use bevy::pbr::{MeshPipelineKey, MeshUniform};
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::{DrawFunctions, RenderPhase};
use bevy::render::render_resource::{PipelineCache, SpecializedMeshPipelines};
use bevy::render::view::ExtractedView;

pub fn queue_grass_buffers(
    opaque_3d_draw_functions: Res<DrawFunctions<Opaque3d>>,
    grass_pipeline: Res<GrassPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedMeshPipelines<GrassPipeline>>,
    mut pipeline_cache: ResMut<PipelineCache>,
    cacher: Res<EntityCache>,
    meshes: Res<RenderAssets<Mesh>>,
    material_meshes: Query<(Entity, &MeshUniform, &Handle<Mesh>)>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Opaque3d>)>,
) {
    let draw_custom = opaque_3d_draw_functions
        .read()
        .get_id::<GrassDrawCall>()
        .unwrap();
    let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples);

    for (view, mut transparent_phase) in views.iter_mut() {
        let view_key = msaa_key | MeshPipelineKey::from_hdr(view.hdr);
        let rangefinder = view.rangefinder3d();
        for (entity, mesh_uniform, mesh_handle) in material_meshes.iter() {
            if !cacher.entities.contains(&entity) {
                continue;
            }
            if let Some(mesh) = meshes.get(mesh_handle) {
                // Todo: Should this emit an error on None?
                let key =
                    view_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
                let pipeline = pipelines
                    .specialize(&mut pipeline_cache, &grass_pipeline, key, &mesh.layout)
                    .unwrap();
                transparent_phase.add(Opaque3d {
                    distance: rangefinder.distance(&mesh_uniform.transform),
                    pipeline,
                    entity,
                    draw_function: draw_custom,
                })
            }
        }
    }
}
