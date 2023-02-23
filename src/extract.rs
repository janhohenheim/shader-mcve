use crate::cache::{EntityCache, GrassCache};
use crate::grass::Grass;
use bevy::prelude::*;
use bevy::render::Extract;

pub fn extract_grass(
    grass_query: Extract<
        Query<
            (Entity, &Grass, &GlobalTransform, &ComputedVisibility),
            // Todo: Only use Changed
            Or<(Added<Grass>, Changed<Grass>)>,
        >,
    >,
    mut grass_cache: ResMut<GrassCache>,
    mut entity_cache: ResMut<EntityCache>,
) {
    for (entity, grass, transform, visibility) in grass_query.iter() {
        if !visibility.is_visible() {
            continue;
        }
        let cache_value = grass_cache.entry(entity).or_default();
        cache_value.grass = grass.clone();
        cache_value.transform = *transform;
        if !entity_cache.entities.contains(&entity) {
            entity_cache.entities.push(entity);
        }
    }
}
