use bevy::prelude::*;
use rand::{rng, rngs::ThreadRng, Rng};

use crate::{
    GameManager, Obstacle, OBSTACLE_AMOUNT, OBSTACLE_GAP_SIZE, OBSTACLE_HEIGHT, OBSTACLE_SPACING,
    OBSTACLE_VERTICAL_OFFSET, OBSTACLE_WIDTH, PIXEL_RATIO, SCROLL_SPEED,
};

pub fn spawn_obstacles(
    commands: &mut Commands,
    rand: &mut ThreadRng,
    window_width: f32,
    pipe_image: &Handle<Image>,
) {
    for i in 0..=OBSTACLE_AMOUNT {
        let y_offset = generate_offset(rand);

        let x_pos = window_width / 2.0 + (OBSTACLE_SPACING * PIXEL_RATIO * i as f32);

        let translation = Vec3::X * x_pos + Vec3::Y * (calc_centered_pipe_position() + y_offset);

        let translation_rev =
            Vec3::X * x_pos + Vec3::Y * (-calc_centered_pipe_position() + y_offset);

        let pipe_direction: f32 = 1.0;

        spawn_obstacle(translation, pipe_direction, commands, pipe_image);

        spawn_obstacle(translation_rev, -pipe_direction, commands, pipe_image)
    }
}

pub fn calc_centered_pipe_position() -> f32 {
    (OBSTACLE_HEIGHT / 2.0 + OBSTACLE_GAP_SIZE) * PIXEL_RATIO
}

pub fn spawn_obstacle(
    translation: Vec3,
    pipe_direction: f32,
    commands: &mut Commands,
    pipe_image: &Handle<Image>,
) {
    commands.spawn((
        Sprite {
            image: pipe_image.clone(),
            ..Default::default()
        },
        Transform::from_translation(translation).with_scale(Vec3::new(
            PIXEL_RATIO,
            PIXEL_RATIO * -pipe_direction,
            PIXEL_RATIO,
        )),
        Obstacle { pipe_direction },
    ));
}

pub fn generate_offset(rand: &mut ThreadRng) -> f32 {
    rand.random_range(-OBSTACLE_VERTICAL_OFFSET..=OBSTACLE_VERTICAL_OFFSET) * PIXEL_RATIO
}

pub fn update_obstacles(
    time: Res<Time>,
    game_manager: Res<GameManager>,
    mut obstacle_query: Query<(&mut Obstacle, &mut Transform)>,
) {
    let mut rand = rng();

    let y_offset = generate_offset(&mut rand);

    for (obstacle, mut transform) in obstacle_query.iter_mut() {
        transform.translation.x -= time.delta_secs() * SCROLL_SPEED;

        if transform.translation.x + OBSTACLE_WIDTH * PIXEL_RATIO / 2.0
            < -game_manager.window_dimensions.x / 2.0
        {
            transform.translation.x += OBSTACLE_AMOUNT as f32 * OBSTACLE_SPACING * PIXEL_RATIO;

            transform.translation.y =
                calc_centered_pipe_position() * obstacle.pipe_direction + y_offset;
        }
    }
}
