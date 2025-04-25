use std::process::Command;

use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, Signature, Spanned, SyntaxShape, Value};

use crate::{UnrealEnginePlugin, utils::uproject};

pub struct UEStart;

// https://dev.epicgames.com/documentation/en-us/unreal-engine/unreal-engine-command-line-arguments-reference
// https://unrealcommunity.wiki/command-line-interface-cli-3mcqmc4z

impl SimplePluginCommand for UEStart {
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
                "uproject_path",
                SyntaxShape::Filepath,
                "Path to a uproject, defaults is the .uproject file of the current directory",
                Some('u'),
            )
            .switch("log", "Start with -log", Some('l'))
            .named(
                "level",
                SyntaxShape::String,
                "Override default level name to start from",
                None,
            )
            .named(
                "args",
                SyntaxShape::String,
                "Additional arguments to pass to UnrealEditor-Cmd",
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
            .category(Category::Plugin)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: "ue start",
                description: "Start the editor for the given project",
                result: None,
            },
            Example {
                example: "ue start --game --windowed",
                description: "Start a client game for the given project",
                result: None,
            },
            Example {
                example: "ue start --server --log",
                description: "Start a server for the given project",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        _plugin: &UnrealEnginePlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let uproject_path: Option<Spanned<String>> = call.get_flag("uproject_path")?;
        let level: Option<Spanned<String>> = call.get_flag("level")?;
        let log: bool = call.has_flag("log")?;
        let args: Option<Spanned<String>> = call.get_flag("args")?;
        let editor: bool = call.has_flag("editor")?;
        let game: bool = call.has_flag("game")?;
        let windowed: bool = call.has_flag("windowed")?;
        let port: Option<Spanned<i32>> = call.get_flag("port")?;
        let server: bool = call.has_flag("server")?;
        let nosteam: bool = call.has_flag("nosteam")?;

        let mut command = Command::new("UnrealEditor-Cmd");

        if let Some(uproject_path) = uproject_path {
            command.arg(uproject_path.item);
        } else {
            command.arg(uproject::find_uproject()?);
        }

        if editor {
            command.arg("-editor");
        } else if game {
            command.arg("-game");
            if windowed {
                command.arg("-windowed");
            }
            if let Some(port) = port {
                command.arg("-port");
                command.arg(port.item.to_string());
            }
        } else if server {
            command.arg("-server");
            if nosteam {
                command.arg("-nosteam");
            }
        }

        if log {
            command.arg("-log");
        }

        if let Some(level) = level {
            command.arg("-Level");
            command.arg(level.item);
        }

        if let Some(args) = args {
            command.arg(args.item);
        }

        // Execute the command and return the output
        match command.spawn() {
            Ok(child) => {
                let output = child.wait_with_output().map_err(|e| {
                    LabeledError::new(format!("Failed to wait for command: {}", e.to_string()))
                })?;
                Ok(Value::string(
                    String::from_utf8_lossy(&output.stdout).to_string(),
                    call.head,
                ))
            }
            Err(e) => Err(LabeledError::new(format!(
                "Failed to spawn command: {}",
                e.to_string()
            ))),
        }
    }
}
