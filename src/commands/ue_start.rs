use std::{path::PathBuf, process::Command};

use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, Example, LabeledError, PipelineData, Signature, Spanned, SyntaxShape};

use crate::{
    UnrealEnginePlugin,
    utils::{ue_paths, ue_tools, uproject},
};

pub struct UEStart;

// https://dev.epicgames.com/documentation/en-us/unreal-engine/unreal-engine-command-line-arguments-reference
// https://unrealcommunity.wiki/command-line-interface-cli-3mcqmc4z

impl PluginCommand for UEStart {
    type Plugin = UnrealEnginePlugin;

    fn name(&self) -> &str {
        "ue start"
    }

    fn description(&self) -> &str {
        "Start an Unreal Engine project in the editor, game or server"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            // General options
            .named(
                "uproject",
                SyntaxShape::Filepath,
                "Path to a uproject, default is the .uproject file of the current directory",
                Some('u'),
            )
            .switch("log", "Start with -log", Some('l'))
            .switch("stdout", "Start with -stdout", None)
            .named(
                "level",
                SyntaxShape::String,
                "Override default level name to start from",
                None,
            )
            // Editor options
            .switch("editor", "Start as an editor (default)", Some('e'))
            // Game options
            .switch("game", "Start as a game", Some('g'))
            .switch("windowed", "Start in windowed mode (Game only)", Some('w'))
            .named(
                "port",
                SyntaxShape::Int,
                "Port to connect to the server (Game only)",
                Some('p'),
            )
            // Server options
            .switch("server", "Start as a server", Some('s'))
            .switch(
                "nosteam",
                "Start a server with -nosteam flag (Server only)",
                None,
            )
            .allows_unknown_args()
            .category(Category::Plugin)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: "ue start",
                description: "Start the editor for the project",
                result: None,
            },
            Example {
                example: "ue start --game --windowed",
                description: "Start a client game for the project",
                result: None,
            },
            Example {
                example: "ue start --server --log",
                description: "Start a server for the project",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        _plugin: &UnrealEnginePlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let uproject_path: PathBuf =
            uproject::uproject_from_arg_or_current_dir(&engine, call.get_flag("uproject")?)?;
        let level: Option<Spanned<String>> = call.get_flag("level")?;
        let log: bool = call.has_flag("log")?;
        let stdout: bool = call.has_flag("stdout")?;
        let editor: bool = call.has_flag("editor")?;
        let game: bool = call.has_flag("game")?;
        let windowed: bool = call.has_flag("windowed")?;
        let port: Option<Spanned<i32>> = call.get_flag("port")?;
        let server: bool = call.has_flag("server")?;
        let nosteam: bool = call.has_flag("nosteam")?;
        let args: Vec<String> = call.rest(0).map_err(|e| LabeledError::new(e.to_string()))?;

        let uproject = uproject::UProject::from_path(&uproject_path)?;
        let unreal_editor_path =
            ue_paths::get_unreal_editor_path(uproject.get_unreal_engine_path_str()?);
        let mut command = Command::new(&unreal_editor_path);

        command
            .current_dir(&engine.get_current_dir()?)
            .arg(uproject_path);

        if let Some(level) = level {
            command.arg(level.item);
        }

        if editor {
            command.arg("-editor");
        }

        if game {
            command.arg("-game");
        }

        if server {
            command.arg("-server");
        }

        if windowed {
            command.arg("-windowed");
        }

        if let Some(port) = port {
            command.arg("-port");
            command.arg(port.item.to_string());
        }

        if nosteam {
            command.arg("-nosteam");
        }

        if log {
            command.arg("-log");
        }

        if stdout {
            command.arg("-stdout");
        }

        command.args(&args);

        ue_tools::run(&mut command, call.head)
    }
}
