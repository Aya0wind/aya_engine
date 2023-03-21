use std::net::SocketAddr;
use std::sync::mpsc::{channel, Receiver, Sender};
// use log::ServerOut;
use std::time::Duration;

use engine::{App, ConnectionId, CrosstermPlugin, CrosstermWindow, CrosstermWindowSettings, EventReader, input, MouseEventKind};
use engine::{MouseButton, MouseEvent, Position, Sprite, SpriteBundle, StyleMap};
use engine::app::ScheduleRunnerSettings;
use engine::asset::{Assets, Handle};
use engine::core::{Time, Timer};
use engine::ecs::prelude::{Commands, IntoSystem, Query, QuerySet, Res, ResMut, With};
use engine::plugins::DefaultPlugins;

use crate::map::init_system;

//mod debug_server;
//use debug_server::DebugServer;
mod map;

fn main() {
    let settings = CrosstermWindowSettings::default();
    let schedule_settings = ScheduleRunnerSettings::run_loop(Duration::from_millis(1000));
    let mut builder = App::build();
    // Add our window settings
    builder.insert_resource(settings)
        .insert_resource(schedule_settings)
        .add_plugins(DefaultPlugins)
        .add_plugin(CrosstermPlugin)
        //.add_plugin(log::DebugServer)
        //.add_system(print_mouse_pos.system())
        .add_system(input::system::exit_on_esc_system.system());
    init_system(&mut builder);
    builder.run();
}


fn print_mouse_pos(mut sprites: ResMut<Assets<Sprite>>,
                   mut events: EventReader<MouseEvent>,
                   mut box_sprite: Query<(&mut Position, &Handle<Sprite>)>,
) {
    for event in events.iter() {
        let event = event as &MouseEvent;
        for mut x in box_sprite.iter_mut() {
            x.0.x = event.column as i32;
            x.0.y = event.row as i32;
            let event_text = if event.kind == MouseEventKind::Moved {
                "mouse move".to_string()
            } else if event.kind == MouseEventKind::Down(MouseButton::Left) {
                "mouse left button pressed".to_string()
            } else if event.kind == MouseEventKind::Drag(MouseButton::Left) {
                "mouse drag".to_string()
            } else {
                "no action".to_string()
            };
            let handle = x.1;
            let coord_text = format!("坐标:({},{})", event.column as i32, event.row as i32);
            *sprites.get_mut(handle).unwrap() = Sprite::new(
                format!("{}{}", coord_text, event_text)
            );
        }
    }
}


// fn cross(time:Res<Time>,mut timer:ResMut<UpdateTimer>,mut query:QuerySet<(
//     Query<&mut Position, With<Left>>,
//     Query<&mut Position, With<Right>>)
//     >
// ){
//     if timer.0.tick(time.delta()).just_finished(){
//         let left = query.q0_mut().single_mut().unwrap().clone();
//         let right = query.q1_mut().single_mut().unwrap().clone();
//         if left<=right{
//             let mut left_pos = query.q0_mut().single_mut().unwrap();
//             left_pos.x+=1;
//             left_pos.y+=1;
//             let mut right_pos = query.q1_mut().single_mut().unwrap();
//             right_pos.x-=1;
//             right_pos.y-=1;
//         }
//     }
// }
//
