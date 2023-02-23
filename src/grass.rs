use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::render::render_resource::ShaderType;
use bytemuck::{Pod, Zeroable};

/// A collection of grassblades to be extracted later into the render world
#[derive(Clone, Debug, Component, Default)]
pub struct Grass {
    pub instances: Vec<GrassBlade>,
}

impl Grass {
    pub fn new(instances: Vec<GrassBlade>) -> Self {
        Grass { instances }
    }
    /// Calculates an [`Aabb`] box which contains all grass blades in self.
    ///
    /// This can be used to check if the grass is in the camera view
    pub fn calculate_aabb(&self) -> Aabb {
        let mut outer = Vec3::new(f32::MIN, f32::MIN, f32::MIN);
        let mut inner = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        self.instances
            .iter()
            .map(|blade| (blade.position, blade.height))
            .for_each(|(blade_pos, height)| {
                inner = inner.min(blade_pos);
                outer = outer.max(blade_pos + Vec3::Y * height);
            });
        Aabb::from_min_max(inner, outer)
    }
}

/// Representation of a single grassblade
#[derive(Copy, Clone, Debug, Pod, Zeroable, ShaderType)]
#[repr(C)]
pub struct GrassBlade {
    pub position: Vec3,
    pub height: f32,
}

/// To calculate frustum culling we need the [Aabb] box of the entity
///
/// Note that it is in the responsabilty of the user to minimize the [Aabb] boxes of the chunks if high performance is needed
pub(crate) fn add_aabb_box_to_grass(
    mut commands: Commands,
    grasses: Query<(Entity, &Grass), Added<Grass>>,
) {
    for (e, grass) in grasses.iter() {
        let aabb = grass.calculate_aabb();
        commands.entity(e).insert(aabb);
    }
}
