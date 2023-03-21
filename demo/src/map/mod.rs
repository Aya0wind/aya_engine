use engine::AppBuilder;
use crate::{App, Assets, Commands, CrosstermWindow, Handle, IntoSystem, Position, Query, Res, ResMut, Sprite, SpriteBundle, StyleMap, With};

struct GameMapBorder;


fn create_game_map_sprite(width:u16,height:u16)->Sprite{
    let mut buffer = String::from('+');
    for _ in 0..width-5 {
        buffer.push('-');
    }
    buffer.push('+');
    buffer.push('\n');
    for _ in 0..height-1 {
        buffer.push('+');
        for _ in 0..width-5 {
            buffer.push(' ');
        }
        buffer.push('+');
        buffer.push('\n');
    }
    buffer.push('+');
    for _ in 0..width-5 {
        buffer.push('-');
    }
    buffer.push('+');
    Sprite::new(buffer)
}

fn create_game_map_initializer(
    mut commands: Commands,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
    window: Res<CrosstermWindow>, ) {
    commands.
        spawn()
        .insert_bundle(SpriteBundle {
            sprite: sprites.add(create_game_map_sprite(window.width(),window.height())),
            position: Position { x: 0, y: 0, z: 1 },
            stylemap: stylemaps.add(StyleMap::default()),
            ..Default::default()
        })
        .insert(GameMapBorder);
}

fn update_map(
    mut sprites: ResMut<Assets<Sprite>>,
    window: Res<CrosstermWindow>,
    mut map:Query<&Handle<Sprite>,With<GameMapBorder>>
){
    let map_mut = map.single_mut().unwrap();
    *sprites.get_mut(map_mut).unwrap() = create_game_map_sprite(window.width(),window.height())
}


pub fn init_system(app:&mut AppBuilder){
    app.add_startup_system(create_game_map_initializer.system());
    app.add_system(update_map.system());
}