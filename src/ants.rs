use bevy::prelude::*;
use super::AntHealth;
use rand::Rng;


#[derive(Bundle)]
pub struct AntBundle {
    pub health : AntHealth,

    #[bundle]
    pub sprite : SpriteSheetBundle,
}

#[derive(Component)]
pub struct Coordinate {
    x : f32,
    y : f32,
}
impl Coordinate {
    fn new() -> Self {
        Self {
            x : f32,
            y : f32,
        }
    }

    //Generates random coordinates by the ant for the wandering system
    fn wander_coor(&mut self, mut query: Query<&mut Transform>) -> Coordinate {
        let mut rng = rand::thread_rng();
        for mut transform in query.iter_mut() {
            let mut x_lower_bound:f32 = transform.translation.x - 100.;
            let mut x_upper_bound: f32 = transform.translation.x + 100.;
            let mut y_lower_bound: f32 = transform.translation.y - 100.;
            let mut y_upper_bound: f32 = transform.translation.y + 100.;

            let rand_x: f32 = rng.gen_range(x_lower_bound..x_upper_bound);
            let rand_y: f32 = rng.gen_range(y_lower_bound..y_upper_bound);

            let coor = Coordinate{x : rand_x, y : rand_y};
            return coor;
        }
        Coordinate{x : 0., y : 0.}
    }

    //Checks to see if the ant's location is the coordinates given
    fn is_coor_reached(mut query: Query<(&mut Transform, &mut Coordinate)>) -> bool {
        for (mut transform, mut coor) in query.iter_mut() {
            if coor.x == transform.translation.x && coor.y == transform.translation.y{
                return true;
            }
            else {
                return false;
            }
        }
        return false;
    }
}



//Checks to see if the coordinate has been reached,
//it will translate horisonally until aligned with
//the x coordinate and then vertically for y
pub fn pathfinding_system(mut query: Query<(&mut Transform, &mut Coordinate)>, time: Res<Time>) {
    for (mut transform, mut coor) in query.iter_mut() {
        if is_coor_reached(query) == false {
            if transform.translation.x != coor.x {
                if transform.translation.x < coor.x {
                    //Right
                    transform.translation.x += time.delta_seconds() * 100.;
                }
                else {
                    //Left
                    transform.translation.x -= time.delta_seconds() * 100.;
                }
            }
            else {
                if transform.translation.y < coor.y {
                    //Up
                    transform.translation.y += time.delta_seconds() * 100.;
                }
                else {
                    //Down
                    transform.translation.y -= time.delta_seconds() * 100.;
                }
            }
        }
    }
}

pub fn ant_wander_system(mut query: Query<(&mut Transform, &mut Coordinate)>, time : Res<Time>) {
    for (mut transform, mut coor) in query.iter_mut() {
        if is_coor_reached(query) {
            let mut new_coor = gen_coor(query);
        }
        else {
            pathfinding_system(query, time);
        }
    }
}
