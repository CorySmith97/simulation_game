use bevy::prelude::*;
use super::AntHealth;

#[derive(Bundle)]
pub struct AntBundle {
    pub health : AntHealth,

    #[bundle]
    pub sprite : SpriteSheetBundle,
}

pub fn ant_wander_system(mut query: Query<(&mut Transform, &AntHealth)>, time : Res<Time>) {
    for (mut transform, health) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * 10.;
        transform.translation.y += time.delta_seconds() * 10.;
        transform.translation.x += time.delta_seconds() * 10.;
        transform.translation.y += time.delta_seconds() * 10.;
        transform.translation.x += time.delta_seconds() * 10.;
        transform.translation.y += time.delta_seconds() * 10.;
        transform.translation.x += time.delta_seconds() * 10.;
        transform.translation.y += time.delta_seconds() * 10.;
        transform.translation.x += time.delta_seconds() * 10.;
        transform.translation.y += time.delta_seconds() * 10.;
        transform.translation.x += time.delta_seconds() * 10.;
        transform.translation.y += time.delta_seconds() * 10.;
        transform.translation.x += time.delta_seconds() * 10.;
        transform.translation.y += time.delta_seconds() * 10.;
    }
}
