use bevy::{math::vec2, prelude::*};

const PIXEL_RATIO: f32 = 4.0;
const FLAP_FORCE: f32 = 500.0;
const GRAVITY: f32 = 2000.0;
const VELOCITY_TO_ROTATION_RATIO: f32 = 7.5;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(create_window())
            .set(ImagePlugin::default_nearest()),
    );
    app.add_systems(Startup, setup_level);
    app.add_systems(Update, update_bird);
    app.run();
}

fn create_window() -> WindowPlugin {
    let res_sides = 128.0 * PIXEL_RATIO;
    WindowPlugin {
        primary_window: Some(Window {
            title: "Birdy".to_string(),
            position: WindowPosition::Centered(MonitorSelection::Current),
            resolution: Vec2::splat(res_sides).into(),
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[derive(Component, Debug)]
struct Birdy {
    velocity: f32,
}

fn setup_level(mut commands: Commands, assetserver: Res<AssetServer>) {
    commands.insert_resource(ClearColor(Color::srgb_u8(128, 179, 204)));
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            image: assetserver.load("razor.png"),
            ..Default::default()
        },
        Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO)),
        Birdy { velocity: 0.0 },
    ));

    // commands.spawn((
    //     Sprite {
    //         image: assetserver.load("bird.png"),
    //         ..Default::default()
    //     },
    //     Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO)),
    //     Birdy { velocity: 0.0 },
    // ));
}

fn update_bird(
    mut query: Query<(&mut Birdy, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
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
        )
    }
}
