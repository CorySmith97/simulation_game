use bevy::prelude::*;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub enum AntType {
    Queen_Ant,
    Worker_Ant,
    Warrior_Ant,
}

pub struct AntDefinition {
    pub texture: Handle<TextureAtlus>,
}

