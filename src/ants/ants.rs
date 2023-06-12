use bevy::{prelude::*, utils::HashMap};
use rand::Rng;
use super::{AntHealth, RizzPoints};

#[derive(Component)]
pub struct Ant;

#[derive(Bundle)]
pub struct AntBundle {
    pub ant: Ant,
    pub health : AntHealth,
    pub rizz: RizzPoints,

    #[bundle]
    pub sprite : SpriteSheetBundle,
}

impl AntBundle {
    pub fn new( health: u32, rizz: i32, sprite: SpriteSheetBundle) -> AntBundle {
        AntBundle { ant: Ant, health: AntHealth::new(health), rizz: RizzPoints { current_rizz: rizz}, sprite: sprite }
    }

    pub fn spawn_ant(self, mut commands: Commands) {
        let ant = self;
        commands.spawn(ant);
    }
}


#[derive(Component)]
pub enum AntType {
    queen,
    worker,
    warrior,
}

pub struct AntDefinition {
  pub texture_atlas: Handle<TextureAtlas>,
}

#[derive(Component)]
pub struct AntCount {
    count: i32
}


pub fn ant_wander_system(mut query: Query<(&mut Transform, &AntHealth, With<Ant>)>, time : Res<Time>) {

    for (mut transform, health, ant) in query.iter_mut(){
        let mut rand_thread = rand::thread_rng();
        let rand_int: i32 = rand_thread.gen_range(1..4);

        match rand_int {
            1 => transform.translation.x += time.delta_seconds() * 200.,
            2 =>transform.translation.y += time.delta_seconds() * 200.,
            3 => transform.translation.x -= time.delta_seconds() * 200.,
            4 => transform.translation.y -= time.delta_seconds() * 200.,

            _ => println!("Nothing happens")
        }
        Timer::from_seconds(0.3, TimerMode::Repeating);
    }
}
