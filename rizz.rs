use rand::{Rng, thread_rng};
pub struct RizzPoints(pub i32);

impl RizzPoints {
    fn starting_rizz() -> i32 {
        let current_rizz = 10;
        return current_rizz;
    } 

    fn lose_rizz(&mut self, mut _neg_prompts: [&str; 5], current_rizz: i32) -> i32 {
        let mut _neg_prompts = ["Is Rey's last name Skywalker?",
                                           "Is your favorite MCU character Captain Marvel?",
                                           "Is mayonnaise an instrument?",
                                           "Is a hot dog a sandwich?",
                                           "Would you sacrifice a loved one to save 7 strangers?"];
        let rizz_loss: i32 = current_rizz - thread_rng().gen_range(10..30);
        return rizz_loss;
    } 

    fn gain_rizz(&mut self, mut _pos_prompts: [&str; 5], current_rizz: i32) -> i32 {
        let mut _pos_prompts = ["Do you dislike mumble rap?",
                                           "Do you prefer Root Beer over Diet Coke?",
                                           "Is Infinity War your favorite MCU movie?",
                                           "Are you going to see Barbie on opening night instead of Oppenheimer?",
                                           "Is your favorite season Fall?"];
        let rizz_gain = current_rizz + thread_rng().gen_range(10..20);
        return rizz_gain;
    }
}