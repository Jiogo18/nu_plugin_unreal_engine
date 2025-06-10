use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use nu_protocol::{LabeledError, PipelineData, Span, Value};

// Get UnrealEditor-Cmd.exe path
pub fn get_unreal_editor_path(unreal_engine_path: &PathBuf) -> PathBuf {
    unreal_engine_path.join("Engine/Binaries/Win64/UnrealEditor-Cmd.exe")
}

pub fn get_unreal_scripts_path(unreal_engine_path: &PathBuf) -> PathBuf {
    unreal_engine_path.join("Engine/Build/BatchFiles")
}

// Get Build.bat path
pub fn get_ubt_path(unreal_engine_path: &PathBuf) -> PathBuf {
    get_unreal_scripts_path(unreal_engine_path).join("Build.bat")
}

// Get RunUAT.bat path
pub fn get_uat_path(unreal_engine_path: &PathBuf) -> PathBuf {
    get_unreal_scripts_path(unreal_engine_path).join("RunUAT.bat")
}

pub fn run(command: &mut Command, span: Span) -> Result<PipelineData, LabeledError> {
    match command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => match child.wait_with_output() {
            Ok(output) => Ok(PipelineData::Value(
                Value::String {
                    val: String::from_utf8_lossy(&output.stdout).to_string(),
                    internal_span: span,
                },
                None,
            )),
            Err(e) => Err(LabeledError::new(format!(
                "Failed to wait for command: {}",
                e.to_string()
            ))),
        },
        Err(e) => Err(LabeledError::new(format!(
            "Failed to spawn command: {}",
            e.to_string()
        ))),
    }
}
