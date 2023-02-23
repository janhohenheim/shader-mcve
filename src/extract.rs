use crate::cache::GrassCache;
use crate::grass::Grass;
use bevy::prelude::*;
use bevy::render::Extract;

pub fn extract_grass(
    grass_query: Extract<
        Query<(Entity, &Grass, &GlobalTransform, &ComputedVisibility), Changed<Grass>>,
    >,
    mut grass_cache: ResMut<GrassCache>,
) {
    for (entity, grass, transform, visibility) in grass_query.iter() {
        if !visibility.is_visible() {
            continue;
        }
        let cache_value = grass_cache.entry(entity).or_default();
        cache_value.grass = grass.clone();
        cache_value.transform = *transform;
    }
}
