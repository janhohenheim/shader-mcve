use crate::cache::GrassCache;
use crate::grass::add_aabb_box_to_grass;
use crate::pipeline::GrassPipeline;
use crate::GrassDrawCall;
use crate::{extract, prepare, queue, RegionConfig};
use bevy::asset::load_internal_asset;
use bevy::core_pipeline::core_3d::Opaque3d;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_phase::AddRenderCommand;
use bevy::render::render_resource::SpecializedMeshPipelines;
use bevy::render::texture::FallbackImage;
use bevy::render::{RenderApp, RenderStage};

pub struct GrassPlugin;

/// A raw handle which points to the shader used to render the grass.
pub(crate) const GRASS_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 2263343952151597128);

pub const GRASS_MESH_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 9357128457583957922);

impl Plugin for GrassPlugin {
    fn build(&self, app: &mut App) {
        // Load grass shader into cache
        load_internal_asset!(app, GRASS_SHADER_HANDLE, "grass.wgsl", Shader::from_wgsl);

        // Load default grass mesh
        let mut meshes = app.world.resource_mut::<Assets<Mesh>>();
        meshes.set_untracked(GRASS_MESH_HANDLE, default_grass_mesh());
        // Init resources
        app.init_resource::<RegionConfig>()
            .register_type::<RegionConfig>()
            .add_system(add_aabb_box_to_grass);
        // Add extraction
        app.add_plugin(ExtractResourcePlugin::<RegionConfig>::default());
        // Init render app
        app.sub_app_mut(RenderApp)
            .add_render_command::<Opaque3d, GrassDrawCall>()
            .init_resource::<FallbackImage>()
            .init_resource::<GrassPipeline>()
            .init_resource::<GrassCache>()
            .init_resource::<SpecializedMeshPipelines<GrassPipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract::extract_grass)
            .add_system_to_stage(RenderStage::Prepare, prepare::prepare_uniform_buffers)
            .add_system_to_stage(RenderStage::Prepare, prepare::prepare_instance_buffer)
            .add_system_to_stage(RenderStage::Queue, queue::queue_grass_buffers);
    }
}

/// Constructs the default look of the grass, as shown in the examples
fn default_grass_mesh() -> Mesh {
    let mut grass_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    grass_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0., 0., 0.],
            [0.5, 0., 0.],
            [0.25, 0., 0.4],
            [0.25, 1., 0.15],
        ],
    );
    grass_mesh.set_indices(Some(Indices::U32(vec![1, 0, 3, 2, 1, 3, 0, 2, 3])));
    grass_mesh
}
