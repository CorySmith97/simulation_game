mod rizz;
use rand::{Rng, thread_rng};
fn main() {
   let starting_rizz = rizz::RizzPoints::starting_rizz();
   let current_rizz: i32 = starting_rizz;
   let good_or_bad_interaction = thread_rng().gen_range(0..3);
   if good_or_bad_interaction == 0 || good_or_bad_interaction == 1
   {
      let rizz_loss = rizz::RizzPoints::lose_rizz(current_rizz);
      println!("Thanks to your horrible rizz, your rizz points are now {}. Great job.", current_rizz - rizz_loss);
   } else
   {
      let gain_rizz = rizz::RizzPoints::gain_rizz(current_rizz);
      println!("Thanks to your amazing rizz, your rizz points are now {}. Fantastic job, Rizzler!", current_rizz + gain_rizz);
   }
}