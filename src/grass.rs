use bevy::prelude::*;
use bevy::render::render_resource::ShaderType;
use bytemuck::{Pod, Zeroable};

/// A collection of grassblades to be extracted later into the render world
#[derive(Clone, Debug, Component, Default)]
pub struct Grass {
    pub instances: Vec<GrassBlade>,
}

/// Representation of a single grassblade
#[derive(Copy, Clone, Debug, Pod, Zeroable, ShaderType)]
#[repr(C)]
pub struct GrassBlade {
    pub position: Vec3,
    pub height: f32,
}
