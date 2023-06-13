use rand::{Rng, thread_rng};
use bevy::prelude::*;
use std::env;


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

pub fn query_rizz(mut commands: Commands,rizz_points: Query<&mut RizzPoints>) {

    // TODO query is a pub func, query anything that has ants 
    //     randomly select two ants to have an interaction
    for health in rizz_points.iter() {

        let random_interaction = thread_rng().gen_range(3..4);
        let random_ants = thread_rng().gen_range(1..2);
        if random_ants != 1 {
            if random_interaction == 3 {
                
                let convert_paremeter_to_i32: i32 = health.current_rizz;

                RizzPoints::gain_rizz(convert_paremeter_to_i32);
                
            } 
            else if random_interaction == 4 {
                
                let convert_paremeter_to_i32: i32 = health.current_rizz;

                RizzPoints::lose_rizz(convert_paremeter_to_i32);
            }
            println!("Rizz points: {}", health.current_rizz);

        }
    }   
 }
