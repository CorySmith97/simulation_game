use bevy::prelude::*;

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

pub fn health_query(mut commands: Commands, mut query: Query<&mut AntHealth> )-> String {
    let mut health_string = String::new();    
    for mut health in query.iter_mut() {
        health_string = health.to_string();
    }
    return health_string;
}

pub fn damage_ant(mut commands: Commands, mut query: Query<(Entity, &mut AntHealth)>) {
    for (entity, mut health) in query.iter_mut() {
        health.take_damage(10);
        if health.is_dead() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn press_a_to_damage_ant(mut commands: Commands, mut query: Query<(Entity, &mut AntHealth)>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::A) {
        for (entity, mut health) in query.iter_mut() {
            health.take_damage(10);
            if health.is_dead() {
                commands.entity(entity).despawn();
            }
        }
    }
}
