use bevy::prelude::*;
use super::{AntHealth, Ant};

#[derive(Component)]
pub struct Queen {}

#[derive(Component)]
pub struct RepdouctionRate {
    pub speed: i32,
}


#[derive(Bundle)]
pub struct QueenBundle {
    pub name: Name,
    pub queen: Queen,
    pub rr: RepdouctionRate,
    pub health: AntHealth,
    pub sprite: SpriteSheetBundle,
}
   

impl QueenBundle {
    pub fn new(name: Name, rr: i32, health: u32, sprite: SpriteSheetBundle ) -> QueenBundle {
        QueenBundle { name: name, queen: Queen {}, rr: RepdouctionRate { speed: rr}, health: AntHealth::new(health), sprite: sprite }
    }
}
