mod setup;
mod input;
mod playground;
mod tile;
mod events;
mod score;

use setup::setup;
use playground::spawn_playground;
use tile::{spawn_tiles, move_tiles, render_tiles, render_tile_points, new_tile_handler};
use events::NewTileEvent;
use crate::input::MoveTiles;
use score::{setup_score, update_score};


use bevy::prelude::*;


fn main() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window:Some(Window{
            title:"2048 Game".to_string(),
            ..default()
        }), ..default()
    }))
        .add_event::<NewTileEvent>()
        .add_systems(Startup,(setup,spawn_playground,spawn_tiles,setup_score).chain())
        .add_systems(Update,(
            render_tile_points,
            move_tiles,
            render_tiles,
            new_tile_handler,
            update_score,
        ))
        .run();
}
