use m_engine::prelude::*;

use bevy::prelude::*;
use bevy::sprite::ColorMaterial;
use bevy::utils::HashMap;

#[derive(Debug, Clone, Resource)]
pub(crate) struct GlobalMeshes{
    pub unit_circle : Option<Handle<Mesh>>,
}

impl GlobalMeshes{
    pub fn new() -> Self {
        Self {
            unit_circle: None,
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub(crate) struct GlobalMaterials{
    pub white_solid: Option<Handle<ColorMaterial>>,
}

impl GlobalMaterials{
    pub fn new() -> Self {
        Self {
            white_solid: None
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub(crate) struct SkinGraphics{
    pub materials : HashMap<ClassId, Handle<ColorMaterial>>,
    pub meshes : HashMap<ClassId, Handle<Mesh>>,
}

impl SkinGraphics{
    pub fn new() -> Self {
        Self {
            materials : HashMap::new(),
            meshes : HashMap::new(),
        }
    }
}