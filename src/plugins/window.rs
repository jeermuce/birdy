use bevy::prelude::*;

use crate::PIXEL_RATIO;

pub fn create_window() -> WindowPlugin {
    let res_sides = 128.0 * PIXEL_RATIO;

    WindowPlugin {
        primary_window: Some(Window {
            title: "Birdy".to_string(),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            resolution: Vec2::splat(res_sides).into(),
            resizable: true,
            ..Default::default()
        }),
        ..Default::default()
    }
}
