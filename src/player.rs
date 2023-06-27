use bevy::prelude::*;
use rand::Rng;
use std::{thread, time};

#[derive(Component)]
pub struct PlayerMovement
{
    pub speed : f32,
}

pub fn move_player(
    mut query: Query<(&PlayerMovement,&mut Transform)>
)
{
    for(player_movement, mut transform) in query.iter_mut()
    {
        let mut rng = rand::thread_rng();
        let direction = rng.gen_range(1..5);
        let time = rng.gen_range(0.5..2.0);
        let dtime = 0.01;
        let mut index = 0.0;
        
        //up
        if direction == 1
        {
            while index <= time
            {
                transform.translation.y+=player_movement.speed*dtime;
                thread::sleep(time::Duration::from_millis(10));
                index += dtime;
            }
        }
        //left
        if direction == 2
        {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            while index <= time
            {
                transform.translation.x-=player_movement.speed*dtime;
                thread::sleep(time::Duration::from_millis(10));
                index += dtime;
            }
        }
        //down
        if direction == 3
        {
            while index <= time
            {
                transform.translation.y-=player_movement.speed*dtime;
                thread::sleep(time::Duration::from_millis(10));
                index += dtime;
            }
        }
        //right
        if direction == 4
        {
            transform.rotation = Quat::default();
            while index <= time
            {
                transform.translation.x+=player_movement.speed*dtime;
                thread::sleep(time::Duration::from_millis(10));
                index += dtime;
            }
        }
    }
}
