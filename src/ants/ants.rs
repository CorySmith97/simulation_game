use bevy::prelude::*;
use rand::Rng;
use crate::TIME_STEP;
use bevy::time::Stopwatch;
use super::{AntHealth, RizzPoints};
use bevy::sprite::MaterialMesh2dBundle;
use std::time::Duration;
use bevy::math::Vec3;


pub struct AntPlugin;

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ant_wander_system);
    }
}

#[derive(Component)]
pub struct Ant;

#[derive(Component)]
pub struct MoveCooldown {
    pub direction: Vec2,
    pub speed: f32,
    pub time: Timer,
    pub moving: bool,
}



#[derive(Bundle)]
pub struct AntBundle {
    pub ant: Ant,
    pub health : AntHealth,
    pub rizz: RizzPoints,
    pub movement: MoveCooldown,

    #[bundle]
    pub sprite : SpriteSheetBundle,
}

impl AntBundle {
    pub fn new( health: u32, rizz: i32, sprite: SpriteSheetBundle) -> AntBundle {
  
        AntBundle {
            ant: Ant,
            health: AntHealth::new(health),
            rizz: RizzPoints { current_rizz: rizz},
            movement: MoveCooldown{
                direction: Vec2 { x: 1., y: 1. },
                speed: 5.,
                time: Timer::from_seconds(0.5, TimerMode::Repeating),
                moving: false},
            sprite: sprite }
    }

    pub fn spawn_ant(self, mut commands: Commands) {
        let ant = self;
        commands.spawn(ant);
    }
}

pub fn ant_wander_system(
    mut query: Query<(&mut Transform, &mut MoveCooldown, With<Ant>)>,
    time: Res<Time>,
) {
    for (mut transform, mut movement, _) in query.iter_mut() {
        if movement.moving {
            let movement_vector = movement.direction * movement.speed * time.delta_seconds();
            transform.translation += movement_vector.extend(0.);

            if movement.time.tick(time.delta()).just_finished() {
                movement.time.reset();
                movement.moving = false;
            }
        } else {
            movement.moving = true;
            movement.time.set_duration(Duration::from_secs(1));
            movement.time.reset();
            movement.direction = random_direction();
        }
    }
}

fn random_direction() -> Vec2 {
    let mut rng = rand::thread_rng();

    Vec2::new(
        rng.gen_range(-5.0..5.0),
        rng.gen_range(-5.0..5.0),
    )
    .normalize()
}


pub fn birth_ants(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    keyboard_input: Res<Input<KeyCode>>
)  {
    let texture_handle = asset_server.load("ant.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(64.0, 64.0),
        1,
        1,
        Some(Vec2::new(1., 1.)),
        None,
    );
    let mut rand_thread = rand::thread_rng();
    let x = rand_thread.gen_range(-200.0..200.00);
    let y = rand_thread.gen_range(-200.0..200.00);
        
    let ant_sprite_sheet = SpriteSheetBundle {
        texture_atlas: texture_atlases.add(texture_atlas.clone()),
        transform: Transform::from_xyz(x -1500., y -1500., 1.),
        ..Default::default()
    };
    let ant = AntBundle::new(100, 10, ant_sprite_sheet.clone());
    commands.spawn(ant);
}

