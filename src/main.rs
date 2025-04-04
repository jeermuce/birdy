use bevy::prelude::*;
use systems::setup::setup_level;
mod plugins;
use crate::plugins::window::create_window;

mod systems;
use crate::systems::{bird::update_bird, error::handle_level_error, obstacles::update_obstacles};
pub const BIRD_IMAGE_BYTES: &[u8] = include_bytes!("../assets/razor.png");
pub const PIPE_IMAGE_BYTES: &[u8] = include_bytes!("../assets/pipe.png");
pub const PIXEL_RATIO: f32 = 4.0;
pub const FLAP_FORCE: f32 = 500.0;
pub const GRAVITY: f32 = 2000.0;
pub const VELOCITY_TO_ROTATION_RATIO: f32 = 10.0;
pub const OBSTACLE_AMOUNT: u8 = 40;
pub const OBSTACLE_WIDTH: f32 = 32.0;
pub const OBSTACLE_HEIGHT: f32 = 144.0;
pub const OBSTACLE_VERTICAL_OFFSET: f32 = 30.0;
pub const OBSTACLE_GAP_SIZE: f32 = 15.0;
pub const OBSTACLE_SPACING: f32 = 60.0;
pub const SCROLL_SPEED: f32 = 150.0;

#[derive(Component, Debug)]
struct Birdy {
    velocity: f32,
}
#[derive(Component)]
struct Obstacle {
    pipe_direction: f32,
}
#[derive(Resource)]
struct GameManager {
    pub pipe_image: Handle<Image>,
    pub window_dimensions: Vec2,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(create_window())
            .set(ImagePlugin::default_nearest()),
    );

    app.add_systems(Startup, setup_level.pipe(handle_level_error));

    app.add_systems(Update, (update_bird, update_obstacles));

    app.run();
}
