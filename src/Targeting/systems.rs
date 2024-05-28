use bevy::prelude::*;

use crate::{Enemies::components::*, Turret, TURRET_REACH};

pub fn tracking_target(
    enemies_query: Query<&Transform, With<Enemy>>,
    mut turrets_query: Query<(&Transform, &mut Turret), With<Turret>>
) {
    for (turret_transform , mut turret) in turrets_query.iter_mut() {
        let mut direction: Vec3 = Vec3::ZERO;
        let mut closer: f32 = TURRET_REACH;

        for enemy_transform in enemies_query.iter() {
            let distance = enemy_transform.translation.distance(turret_transform.translation);

            if closer > distance {
                closer = distance;
                direction = enemy_transform.translation;
            }

        }

        turret.dir_look = direction;
    }
}

pub fn mov_turret(
    mut turrets_query: Query<(&mut Transform, &Turret), With<Turret>>
) {
    for (mut turret_transform , turret) in turrets_query.iter_mut() {
        let direction = turret.dir_look - turret_transform.translation;
        let correction = std::f32::consts::FRAC_PI_2; // Correction : rotation de 90° sinon elle montre le coté
        let angle = direction.y.atan2(direction.x) - correction;

        turret_transform.rotation = Quat::from_rotation_z(angle);
    }
}
