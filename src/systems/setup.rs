use crate::{systems::obstacles::spawn_obstacles, BIRD_IMAGE_BYTES};
use crate::{Birdy, GameManager, PIPE_IMAGE_BYTES, PIXEL_RATIO};
use anyhow::{Error, Result};
use bevy::asset::RenderAssetUsages;
use bevy::image::{CompressedImageFormats, ImageType};
use bevy::{prelude::*, window::PrimaryWindow};
use rand::rng;

pub fn setup_level(
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
