use bevy::prelude::*;

struct Stockpile {
    food_count: u32,
}

struct MousePosition {
    position: Vec2,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(move_ants.system())
        .add_system(stockpile_inquiry.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn().insert(Ant {
        position: Vec2::new(0.0, 0.0),
        carrying_food: false,
    });
    commands.spawn().insert(Stockpile { food_count: 0 });
}

fn move_ants(
    mut query: Query<(&mut Ant, &mut Transform)>,
    stockpile_query: Query<&Transform, With<Stockpile>>,
) {
    for (mut ant, mut transform) in query.iter_mut() {
        if ant.carrying_food {
            let stockpile_transform = stockpile_query.single().unwrap();
            let target_position = Vec3::new(stockpile_transform.translation.x, stockpile_transform.translation.y, 0.0);
            let direction = (target_position.xy() - transform.translation.xy()).normalize();
            transform.translation += direction.extend(0.0) * 2.0;
            if (transform.translation.xy() - target_position.xy()).length() < 1.0 {
                ant.carrying_food = false;
            }
        } else {
            // Move the ant randomly
            let random_translation = Vec2::new(rand::random::<f32>() * 2.0 - 1.0, rand::random::<f32>() * 2.0 - 1.0);
            ant.position += random_translation * 0.1;
            transform.translation = ant.position.extend(0.0);
        }
    }
}

fn stockpile_inquiry(
    mouse_position: Res<MousePosition>,
    stockpile_query: Query<&Transform, With<Stockpile>>,
) {
    let stockpile_transform = stockpile_query.single().unwrap();
    let stockpile_position = Vec2::new(stockpile_transform.translation.x, stockpile_transform.translation.y);
    if (mouse_position.position - stockpile_position).length() < 1.0 {
        let stockpile = stockpile_query.single().unwrap();
        println!("Stockpile food count: {}", stockpile.food_count);
    }
}