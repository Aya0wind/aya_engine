use std::time::Duration;

use bevy::app::ScheduleRunnerSettings;
use bevy::prelude::*;

use term_backend::{CrosstermPlugin, CrosstermWindowSettings};
use term_backend::prelude::{MouseEvent, MouseEventKind, Position, Sprite, SpriteBundle, StyleMap};

//mod debug_server;
//use debug_server::DebugServer;
pub fn main() {
    let settings = CrosstermWindowSettings::default();
    let schedule_settings = ScheduleRunnerSettings::run_loop(Duration::from_millis(50));
    App::build()
        // Add our window settings
        .insert_resource(settings)
        .insert_resource(schedule_settings)
        .insert_resource(UpdateTimer(Timer::from_seconds(2.0, true)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CrosstermPlugin)
        //.add_system(update_system.system())
        .add_system(print_mouse_pos.system())
        .add_startup_system(startup_system.system())
        .run();
}


fn print_mouse_pos(mut events: EventReader<MouseEvent>, mut box_sprite: Query<(&mut Position, &Handle<Sprite>)>) {
    for event in events.iter() {
        let event = event as &MouseEvent;
        if event.kind == MouseEventKind::Moved {
            for mut x in box_sprite.iter_mut() {
                x.0.x = event.column as i32;
                x.0.y = event.row as i32;
            }
        }
    }
}

fn update_system(time: Res<Time>, mut timer: ResMut<UpdateTimer>, mut box_sprite: Query<(&mut Position, &Handle<Sprite>)>) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut x in box_sprite.iter_mut() {
            x.0.y += 1;
        }
    }
}


fn startup_system(
    mut commands: Commands,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
) {
    commands.spawn_bundle(SpriteBundle {
        sprite: sprites.add(Sprite::new("Hello, world1111111!")),
        position: Position { x: 1, y: 1, z: 1 },
        stylemap: stylemaps.add(StyleMap::default()),
        ..Default::default()
    });
}

struct HelloEntity(u32);

struct UpdateTimer(Timer);

