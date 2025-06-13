use std::{path::PathBuf, process::Command};

use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, Example, LabeledError, PipelineData, Signature, SyntaxShape};

use crate::{
    UnrealEnginePlugin,
    utils::{ue_paths, ue_tools, uproject},
};

// https://github.com/EpicGames/UnrealEngine/blob/4.18/Engine/Source/Programs/UnrealVersionSelector/Private/UnrealVersionSelector.cpp

pub struct UESwitchVersion;
pub struct UEGenerateProjectFiles;
pub struct UERegisterEngine;
pub struct UEUpdateFileAssociations;

fn get_unreal_version_selector_command() -> Result<Command, LabeledError> {
    let unreal_version_selector_path = ue_paths::get_unreal_version_selector_path()?;
    Ok(Command::new(&unreal_version_selector_path))
}

impl PluginCommand for UESwitchVersion {
    type Plugin = UnrealEnginePlugin;

    fn name(&self) -> &str {
        "ue switch-version"
    }

    fn description(&self) -> &str {
        "Switch the Unreal Engine version of the project and generate project files"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .named(
                "uproject",
                SyntaxShape::Filepath,
                "Path to a uproject, default is the .uproject file of the current directory",
                Some('u'),
            )
            .optional(
                "version",
                SyntaxShape::String,
                "The Unreal Engine version to switch to. It can be an identifier like '5.5' or a path. If not provided or not recognized, it will open the 'Select Unreal Engine Version' dialog.",
            )
            .category(Category::Plugin)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            example: "ue switch-version 5.5",
            description: "Switch the Unreal Engine version of the project",
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
        let version: Option<String> = call.opt(0)?;

        let mut command = get_unreal_version_selector_command()?;

        command.current_dir(&engine.get_current_dir()?);

        if version.is_none() {
            command.arg("/switchversion").arg(&uproject_path);
        } else {
            let version = version.unwrap();
            println!("Switching to version: {:?}", &version);
            command
                .arg("/switchversionsilent")
                .arg(&uproject_path)
                .arg(&version);
        }

        ue_tools::run(&mut command, call.head)
    }
}

impl PluginCommand for UEGenerateProjectFiles {
    type Plugin = UnrealEnginePlugin;

    fn name(&self) -> &str {
        "ue generate-project"
    }

    fn description(&self) -> &str {
        "Generate Visual Studio project files for the project"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .named(
                "uproject",
                SyntaxShape::Filepath,
                "Path to a uproject, default is the .uproject file of the current directory",
                Some('u'),
            )
            .category(Category::Plugin)
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

        let mut command = get_unreal_version_selector_command()?;
        command
            .current_dir(&engine.get_current_dir()?)
            .arg("/projectfiles")
            .arg(&uproject_path);

        ue_tools::run(&mut command, call.head)
    }
}

impl PluginCommand for UERegisterEngine {
    type Plugin = UnrealEnginePlugin;

    fn name(&self) -> &str {
        "ue register-engine"
    }

    fn description(&self) -> &str {
        "Register the engine in the current folder to Epic Games Launcher"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).category(Category::Plugin)
    }

    fn run(
        &self,
        _plugin: &UnrealEnginePlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let mut command = get_unreal_version_selector_command()?;

        command.current_dir(&engine.get_current_dir()?);
        // Register is the default action of UnrealVersionSelector

        ue_tools::run(&mut command, call.head)
    }
}

impl PluginCommand for UEUpdateFileAssociations {
    type Plugin = UnrealEnginePlugin;

    fn name(&self) -> &str {
        "ue update-file-associations"
    }

    fn description(&self) -> &str {
        "Update file associations of Epic Games Launcher. Requires admin privileges."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).category(Category::Plugin)
    }

    fn run(
        &self,
        _plugin: &UnrealEnginePlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let mut command = get_unreal_version_selector_command()?;
        command
            .current_dir(&engine.get_current_dir()?)
            .arg("/fileassociations");
        ue_tools::run(&mut command, call.head)
    }
}
