use nu_plugin::{MsgPackSerializer, Plugin, PluginCommand, serve_plugin};

mod commands;
pub use commands::*;

mod utils;

pub struct UnrealEnginePlugin;

impl Plugin for UnrealEnginePlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(FromUAsset),
            Box::new(UE),
            Box::new(UEBuild),
            Box::new(UERunUAT),
            Box::new(UEStart),
            Box::new(UEGenerateProjectFiles),
            Box::new(UERegisterEngine),
            Box::new(UESwitchVersion),
            Box::new(UEUpdateFileAssociations),
        ]
    }
}

fn main() {
    serve_plugin(&UnrealEnginePlugin, MsgPackSerializer);
}
