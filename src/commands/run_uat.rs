use std::{path::PathBuf, process::Command};

use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, Example, LabeledError, PipelineData, Signature, Spanned, SyntaxShape};

use crate::{
    UnrealEnginePlugin,
    utils::{ue_paths, ue_tools, uproject},
};

pub struct UERunUAT;

// https://dev.epicgames.com/documentation/en-us/unreal-engine/build-operations-cooking-packaging-deploying-and-running-projects-in-unreal-engine

impl PluginCommand for UERunUAT {
    type Plugin = UnrealEnginePlugin;

    fn name(&self) -> &str {
        "ue RunUAT"
    }

    fn description(&self) -> &str {
        "Use RunUAT on an Unreal Engine project"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("command", SyntaxShape::String, "RunUAT command to run")
            .named(
                "uproject",
                SyntaxShape::Filepath,
                "Path to a uproject, defaults is the .uproject file of the current directory",
                Some('u'),
            )
            .allows_unknown_args()
            .category(Category::Plugin)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            example: "ue RunUAT BuildCookRun -clientconfig=Development",
            description: "Execute RunUAT for the current project",
            result: None,
        }]
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
        let uproject = uproject::UProject::from_path(&uproject_path)?;

        let command_name: Spanned<String> = call.req(0)?;
        let args: Vec<String> = call.rest(1)?;

        let unreal_uat_path = ue_paths::get_uat_path(&uproject.unreal_engine_path);
        let mut command = Command::new(&unreal_uat_path);

        command
            .current_dir(&engine.get_current_dir()?)
            .arg(&command_name.item);
        if command_name.item != "-List" {
            command.args(["-uproject=", uproject_path.to_str().unwrap()]);
        }
        command.args(&args);

        // Execute the command and return the output
        ue_tools::run(&mut command, call.head)
    }
}
