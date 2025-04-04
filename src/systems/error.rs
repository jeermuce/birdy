use anyhow::{Error, Result};
use bevy::prelude::In;

pub fn handle_level_error(In(result): In<Result<(), Error>>) {
    if let Err(e) = result {
        eprintln!("Error setting up the level: {e}")
    }
}
