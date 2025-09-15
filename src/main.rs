mod setup;
mod input;
mod playground;
mod tile;

use setup::setup;
use playground::spawn_playground;
use tile::spawn_tiles;


use bevy::prelude::*;


fn main() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window:Some(Window{
            title:"2048 Game".to_string(),
            ..default()
        }), ..default()
    }))
        .add_systems(Startup,(setup,spawn_playground,spawn_tiles).chain())
        .run();
}
