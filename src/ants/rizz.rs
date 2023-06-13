use rand::{Rng, thread_rng};
use bevy::prelude::*;

#[derive(Component)]
pub struct RizzPoints{
    pub current_rizz: i32,

}


impl RizzPoints {
    pub fn new(starting_rizz: i32) -> Self {
        RizzPoints {
            current_rizz: starting_rizz,
        }
    } 
    pub fn lose_rizz(current_rizz: i32) -> i32 {
        current_rizz - thread_rng().gen_range(5..40)
    } 
    pub fn gain_rizz(current_rizz: i32) -> i32 {
        current_rizz + thread_rng().gen_range(10..40)
    }
}

pub fn query_rizz(rizz_points: Query<&mut RizzPoints>) {
    for health in rizz_points.iter() {
        println!("Rizz points: {}", health.current_rizz);
    }
    //{
      // let rizz_loss = RizzPoints::lose_rizz(current_rizz);
       //println!("Thanks to your horrible rizz, your rizz points are now {}. Great job.", current_rizz - rizz_loss);
    //} else
    //{
      // let gain_rizz = RizzPoints::gain_rizz(current_rizz);
      // println!("Thanks to your amazing rizz, your rizz points are now {}. Fantastic job, Rizzler!", current_rizz + gain_rizz);
    //}
 }
