use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy::{core::CorePlugin,transform::TransformPlugin,diagnostic::DiagnosticsPlugin,asset::AssetPlugin,scene::ScenePlugin};
use bevy::input::InputPlugin;
use bevy::window::WindowPlugin;

pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {

        group.add(CorePlugin::default());
        group.add(TransformPlugin::default());
        group.add(DiagnosticsPlugin::default());
        group.add(AssetPlugin::default());
        group.add(ScenePlugin::default());
        group.add(InputPlugin::default());
        group.add(WindowPlugin::default());
      
        #[cfg(feature = "bevy_audio")]
        group.add(bevy_audio::AudioPlugin::default());

        #[cfg(feature = "bevy_gilrs")]
        group.add(bevy_gilrs::GilrsPlugin::default());
    }
}
