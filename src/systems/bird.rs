use bevy::prelude::*;
use rand::rng;

use crate::{
    Birdy, GameManager, Obstacle, FLAP_FORCE, GRAVITY, OBSTACLE_HEIGHT, OBSTACLE_WIDTH,
    PIXEL_RATIO, VELOCITY_TO_ROTATION_RATIO,
};

use super::obstacles::spawn_obstacles;

pub fn update_bird(
    mut commands: Commands,
    obstacle_query: Query<(&Transform, Entity), With<Obstacle>>,
    mut query: Query<(&mut Birdy, &mut Transform), Without<Obstacle>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    game_manager: Res<GameManager>,
) {
    if let Ok((mut birdy, mut transform)) = query.get_single_mut() {
        if keys.just_pressed(KeyCode::Space) {
            birdy.velocity = FLAP_FORCE;
        }
        birdy.velocity -= time.delta_secs() * GRAVITY;

        transform.translation.y += birdy.velocity * time.delta_secs();

        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(birdy.velocity / VELOCITY_TO_ROTATION_RATIO, -90.0, 90.0).to_radians(),
        );

        let mut dead = false;

        let left_window_area = transform.translation.y <= -game_manager.window_dimensions.y / 2.0;

        if left_window_area {
            dead = true;
        } else {
            obstacle_query.iter().for_each(|(pipe_transform, _entity)| {
                if (pipe_transform.translation.y - transform.translation.y).abs()
                    < OBSTACLE_HEIGHT * PIXEL_RATIO * 0.5
                    && (pipe_transform.translation.x - transform.translation.x).abs()
                        < OBSTACLE_WIDTH * PIXEL_RATIO * 0.5
                {
                    dead = true;
                }
            });
        }
        if dead {
            transform.translation = Vec3::ZERO;

            birdy.velocity = 0.0;

            obstacle_query.iter().for_each(|(_pipe_transform, entity)| {
                commands.entity(entity).despawn();
            });

            let mut rand = rng();

            spawn_obstacles(
                &mut commands,
                &mut rand,
                game_manager.window_dimensions.x,
                &game_manager.pipe_image,
            );
        }
    }
}
