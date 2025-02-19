use anyhow::{Error, Result};

use bevy::asset::RenderAssetUsages;
use bevy::image::{CompressedImageFormats, ImageType};
use bevy::prelude::In;

use bevy::{prelude::*, window::PrimaryWindow};

use rand::{rng, rngs::ThreadRng, Rng};

const BIRD_IMAGE_BYTES: &[u8] = include_bytes!("../assets/razor.png");
const PIPE_IMAGE_BYTES: &[u8] = include_bytes!("../assets/pipe.png");
const PIXEL_RATIO: f32 = 4.0;
const FLAP_FORCE: f32 = 500.0;
const GRAVITY: f32 = 2000.0;
const VELOCITY_TO_ROTATION_RATIO: f32 = 10.0;
const OBSTACLE_AMOUNT: u8 = 40;
const OBSTACLE_WIDTH: f32 = 32.0;
const OBSTACLE_HEIGHT: f32 = 144.0;
const OBSTACLE_VERTICA_OFFSET: f32 = 30.0;
const OBSTACLE_GAP_SIZE: f32 = 15.0;
const OBSTACLE_SPACING: f32 = 60.0;
const SCROLL_SPEED: f32 = 150.0;

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

fn create_window() -> WindowPlugin {
    let res_sides = 128.0 * PIXEL_RATIO;

    WindowPlugin {
        primary_window: Some(Window {
            title: "Birdy".to_string(),
            position: WindowPosition::Centered(MonitorSelection::Primary),

            resolution: Vec2::splat(res_sides).into(),
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn handle_level_error(In(result): In<Result<(), Error>>) {
    if let Err(e) = result {
        eprintln!("Error setting up the level: {e}")
    }
}

fn setup_level(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) -> Result<(), Error> {
    let window = window_query.get_single()?;

    let bird_image = load_image_from_bytes(BIRD_IMAGE_BYTES)?;
    let pipe_image = load_image_from_bytes(PIPE_IMAGE_BYTES)?;

    let bird_handle: Handle<Image> = images.add(bird_image);
    let pipe_handle: Handle<Image> = images.add(pipe_image);
    commands.insert_resource(GameManager {
        pipe_image: pipe_handle.clone(),
        window_dimensions: Vec2::new(window.width(), window.height()),
    });

    commands.insert_resource(ClearColor(Color::srgb_u8(128, 179, 204)));

    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: bird_handle,
            ..Default::default()
        },
        Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO / 90.0)),
        Birdy { velocity: 0.0 },
    ));

    let mut rand = rng();

    spawn_obstacles(&mut commands, &mut rand, window.width(), &pipe_handle);

    Ok(())
}
fn load_image_from_bytes(bytes: &[u8]) -> Result<Image, Error> {
    Ok(Image::from_buffer(
        bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        false,
        bevy::image::ImageSampler::Default,
        RenderAssetUsages::all(),
    )?)
}
fn update_bird(
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
fn spawn_obstacles(
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

fn calc_centered_pipe_position() -> f32 {
    (OBSTACLE_HEIGHT / 2.0 + OBSTACLE_GAP_SIZE) * PIXEL_RATIO
}

fn spawn_obstacle(
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

fn generate_offset(rand: &mut ThreadRng) -> f32 {
    rand.random_range(-OBSTACLE_VERTICA_OFFSET..=OBSTACLE_VERTICA_OFFSET) * PIXEL_RATIO
}

fn update_obstacles(
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
