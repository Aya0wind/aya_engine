pub mod prelude {
    pub use bevy_utils::tracing::{
        debug, debug_span, error, error_span, info, info_span, trace, trace_span, warn, warn_span,
    };
}
use std::io::{Write};

use bevy::prelude::{World, Res, IntoSystem};
use bevy_spicy_networking::NetworkServer;
pub use bevy_utils::tracing::{
    debug, debug_span, error, error_span, info, info_span, trace, trace_span, warn, warn_span,
    Level,
};

use bevy_app::{AppBuilder, Plugin, EventWriter};

use tracing_subscriber::{prelude::*, registry::Registry,EnvFilter};
//use super::message_types::SendToUserMessage;

/// Adds logging to Apps.
#[derive(Default)]
pub struct LogPlugin;

/// LogPlugin settings
pub struct LogSettings {
    /// Filters logs using the [EnvFilter] format
    pub filter: String,

    /// Filters out logs that are "less than" the given level.
    /// This can be further filtered using the `filter` setting.
    pub level: Level,

    pub listen_endpoint:String,
}

impl Default for LogSettings {
    fn default() -> Self {
        Self {
            filter: "wgpu=error".to_string(),
            level: Level::INFO,
            listen_endpoint:"127.0.0.1:9999".into()
        }
    }
}


// pub struct ServerOut<'writer>{
//     pub server:Option<&'writer NetworkServer>
// }
//
// impl<'writer> std::io::Write for ServerOut<'writer>{
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         if let Some(server) = self.server{
//             server.broadcast(SendToUserMessage{ message: String::from_utf8(buf.to_vec()).unwrap() });
//         }
//         Ok(buf.len())
//     }
//
//     fn flush(&mut self) -> std::io::Result<()> {
//         Ok(())
//     }
// }


// impl Plugin for LogPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         init_system(app);
//     }
// }

// fn init_system(app:&mut AppBuilder){
//     let default_filter = {
//         let settings =
//         app.world_mut()
//         .get_resource_or_insert_with(LogSettings::default);
//         format!("{},{}", settings.level, settings.filter)
//     };
//         let filter_layer = EnvFilter::try_from_default_env()
//             .or_else(|_| EnvFilter::try_new(&default_filter))
//             .unwrap();
//         let subscriber = Registry::default().with(filter_layer);
//         let out = app.world().get_resource::<&NetworkServer>().unwrap();
//         #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
//         {
//             let fmt_layer = tracing_subscriber::fmt::Layer::default().with_writer(||ServerOut{server:Some(out)});
//             let subscriber = subscriber.with(fmt_layer);
//
//             #[cfg(not(feature = "tracing-chrome"))]
//             {
//                 let guard = bevy_utils::tracing::subscriber::set_default(subscriber);
//                 app.insert_resource(guard);
//             }
//         }
// }

