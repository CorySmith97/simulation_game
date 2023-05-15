use bevy::prelude::*;

#[derive(Component)]
pub struct AntHealth {
    current_health: u32,
    max_health: u32,
}

impl AntHealth {
    fn new(max_health: u32) -> Self {
        AntHealth {
            current_health: max_health,
            max_health,
        }
    }
    
    fn take_damage(&mut self, damage: u32) {
        if damage >= self.current_health {
            self.current_health = 0;
        } else {
            self.current_health -= damage;
        }
    }
    
    fn is_dead(&self) -> bool {
        self.current_health == 0
    }
    
    fn heal(&mut self, amount: u32) {
        self.current_health += amount;
        if self.current_health > self.max_health {
            self.current_health = self.max_health;
        }
    }
}