use std::{path::PathBuf, process::Command};

use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, Signature, SyntaxShape, Value};

use crate::{
    UnrealEnginePlugin,
    utils::{ue_tools, uproject},
};

pub struct UEBuild;

// https://dev.epicgames.com/documentation/en-us/unreal-engine/unreal-build-tool-in-unreal-engine

impl SimplePluginCommand for UEBuild {
    type Plugin = UnrealEnginePlugin;

    fn name(&self) -> &str {
        "ue build"
    }

    fn description(&self) -> &str {
        "Compile an Unreal Engine project with UnrealBuildTool"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
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
            example: "ue build VestigeServer Win64 Development",
            description: "Build the current project",
            result: None,
        }]
    }

    fn run(
        &self,
        _plugin: &UnrealEnginePlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let uproject_path: PathBuf =
            uproject::uproject_from_arg_or_current_dir(&engine, call.get_flag("uproject")?)?;

        let args: Vec<String> = call.rest(0).map_err(|e| LabeledError::new(e.to_string()))?;

        let uproject = uproject::UProject::from_path(&uproject_path)?;
        let unreal_build_path = ue_tools::get_ubt_path(&uproject.unreal_engine_path);
        let mut command = Command::new(&unreal_build_path);

        command
            .current_dir(&engine.get_current_dir()?)
            .args(["-uproject=", uproject_path.to_str().unwrap()])
            .args(&args);

        // Execute the command and return the output
        let output = ue_tools::run(&mut command)?;
        Ok(Value::string(
            String::from_utf8_lossy(&output.stdout).to_string(),
            call.head,
        ))
    }
}
