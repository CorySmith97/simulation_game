use rand::{Rng, thread_rng};
pub struct RizzPoints(pub i32);

impl RizzPoints {
    pub fn starting_rizz() -> i32 {
        return 10
    } 
    pub fn lose_rizz(current_rizz: i32) -> i32 {
        current_rizz - thread_rng().gen_range(5..40)
    } 
    pub fn gain_rizz(current_rizz: i32) -> i32 {
        current_rizz + thread_rng().gen_range(10..40)
    }
}