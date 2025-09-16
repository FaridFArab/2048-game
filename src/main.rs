mod setup;
mod input;
mod playground;
mod tile;
mod events;

use setup::setup;
use playground::spawn_playground;
use tile::{spawn_tiles, move_tiles};
use events::NewTileEvent;
use crate::input::MoveTiles;


use bevy::prelude::*;


fn main() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window:Some(Window{
            title:"2048 Game".to_string(),
            ..default()
        }), ..default()
    }))
        .add_event::<NewTileEvent>()
        .add_systems(Startup,(setup,spawn_playground,spawn_tiles).chain())
        .add_systems(Update,{move_tiles})
        .run();
}
