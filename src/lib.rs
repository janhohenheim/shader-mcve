use crate::grass::Grass;
use crate::plugin::GRASS_MESH_HANDLE;
use crate::render::DrawMeshInstanced;
use bevy::pbr::{SetMeshBindGroup, SetMeshViewBindGroup};
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResource;
use bevy::render::render_phase::SetItemPipeline;

pub mod generator;
pub mod grass;
pub mod plugin;

// Render stuff:
pub mod cache;
pub mod grass_pipeline; // Todo: Rename to pipeline

// Render stages:
pub mod extract;
pub mod prepare;
pub mod queue;
pub mod render;

/// A component bundle for a chunk of grass.
///
/// Note that each position of a [`GrassBlade`](crate::prelude::GrassBlade) is also relative to the [`Transform`] component of the entity
#[derive(Bundle)]
pub struct GrassBundle {
    /// The [`Grass`] to spawn in your world.
    ///
    /// ## Usage Detail
    /// Be aware that frustum culling is done using the minimal [Aabb](bevy::render::primitives::Aabb) box containing all elements in [Grass].
    /// Also since all elements in [Grass] are instanced together,
    /// it might be more performant to spawn multiple entities each containing locally seperate portions of the grass in the game.
    /// This however, will only be noticable at high number of grassblades.
    pub grass: Grass,
    /// The [`Mesh`] used to render each grassblade.
    ///
    /// The mesh can be changed to however needed,
    /// however note that the lowest vertex of the mesh should be around y=0
    /// in most cases.
    pub grass_mesh: Handle<Mesh>,
    #[bundle]
    pub spatial: SpatialBundle,
}

impl Default for GrassBundle {
    fn default() -> Self {
        Self {
            grass: Default::default(),
            grass_mesh: GRASS_MESH_HANDLE.typed(),
            spatial: Default::default(),
        }
    }
}

#[derive(Resource, Clone, Reflect)]
#[reflect(Resource)]
pub struct RegionConfig {
    pub color: Color,
}

impl FromWorld for RegionConfig {
    fn from_world(_world: &mut World) -> Self {
        RegionConfig {
            color: Color::rgb(0.2, 0.5, 0.0),
        }
    }
}

// Todo: Replace by ExtractResource derive
impl ExtractResource for RegionConfig {
    type Source = Self;

    fn extract_resource(source: &Self::Source) -> Self {
        source.clone()
    }
}

pub(crate) type GrassDrawCall = (
    // caches pipeline instead of reinit every call
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    DrawMeshInstanced,
);
