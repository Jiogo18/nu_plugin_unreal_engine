use nu_plugin::{MsgPackSerializer, Plugin, PluginCommand, serve_plugin};

pub struct UnrealEnginePlugin;

impl Plugin for UnrealEnginePlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
        ]
    }
}

fn main() {
    serve_plugin(&UnrealEnginePlugin, MsgPackSerializer);
}
