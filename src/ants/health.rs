use bevy::prelude::*;

use super::{Ant};

#[derive(Component)]
pub struct AntHealth {
    current_health: u32,
    max_health: u32,
}

impl AntHealth {
    pub fn new(max_health: u32) -> Self {
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
    pub fn to_string(&self) -> String {
        format!("{} / {}", self.current_health, self.max_health)
    }
}

pub fn print_health_system(query: Query<(Entity, &AntHealth)>) {
    for (entity,health) in query.iter() {
        println!("Health: {}", health.to_string());
    }
}

pub fn damage_ant(mut commands: Commands, mut query: Query<(Entity, &mut AntHealth)>) {
    for (entity, mut health) in query.iter_mut() {
        health.take_damage(10);
        if health.is_dead() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn press_a_to_damage_ant(mut commands: Commands, mut health: Query<&mut AntHealth>, mut query: Query<Entity, With<Ant>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::A) {
        for mut health in health.iter_mut() {
            health.take_damage(10);
            if health.is_dead() {
                for entity in query.iter() {
                    commands.entity(entity).remove::<Ant>();
                }
            }
        }
    }
}   
