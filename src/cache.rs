use crate::grass::Grass;
use bevy::prelude::*;
use bevy::render::render_resource::{BindGroup, Buffer};
use bevy::utils::HashMap;

#[derive(Resource, DerefMut, Deref, Debug, Default)]
pub struct GrassCache {
    pub data: HashMap<Entity, CachedGrassChunk>,
}

#[derive(Debug, Default)]
pub struct CachedGrassChunk {
    pub grass: Grass,
    pub uniform_bind_ground: Option<BindGroup>,
    // Todo: Rename to buffer
    pub grass_buffer: Option<Buffer>,
    pub transform: GlobalTransform,
}

// Todo: Remove this type
#[derive(Resource, DerefMut, Deref, Debug, Default)]
pub struct EntityCache {
    // Todo: Use HashSet
    pub entities: Vec<Entity>,
}
