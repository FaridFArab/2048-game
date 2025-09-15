use create::playground::Playground;
use beavy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;

#[derive(Component)]
pub struct TileText;

#[derive(Component)]
pub struct Points{
    pub value:u32
}

#[derive(Component, PartialEq, Clone, Copy)]
pub struct Position{
    pub x:u8,
    pub y:u8
}

pub fn spawn_tile(commands: &mut Commands, playground: &Playground, pos: &Position) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            transform: Transform::from_xyz(
                playground.tile_pos(pos.x),
                playground.tile_pos(pos.y),
                2.0,
            ),
            ..default()
        }
            .with_children(|child_builder| {
                child_builder.spawn(Text2Bundle {
                    text: Text::from_section(
                        "2",
                        TextStyle{font_size:50.0, color:Color::rgb(0.2, 0.2, 0.2)},
                        ..default()
                    )
                }, transform: Transform::from_xyz(0.0, 0.0, 1.0)
            
            }) .insert(TileText)
        )
        .insert(Points{value:2})
        .insert(pos);
}

pub fn spawn_tiles(mut commands: &mut Commands, query_playground: Query<&Playground>) {
    let playground = query_playground.single();
    let mut rng = rand::rng();
    let starting_tiles: Vec<u8,u8> = (0..playground.grid).cartesian_product(0..playground.grid).
        choose_multiple(&mut rng, 2);

    for (x,y) in starting_tiles.iter {
        let pos = Position{x:*x, y:*y};
        spawn_tile(&mut commands, playground, pos)
    }
}