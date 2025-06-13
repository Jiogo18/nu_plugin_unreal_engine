use nu_protocol::LabeledError;
use std::path::PathBuf;

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

pub fn get_existing_path(
    known_paths: &[&str],
    error_message: &str,
) -> Result<PathBuf, LabeledError> {
    for path in known_paths {
        let path_buf = PathBuf::from(path);
        if path_buf.exists() {
            return Ok(path_buf);
        }
    }
    Err(LabeledError::new(error_message.to_string()))
}

#[cfg(target_os = "windows")]
pub fn get_unreal_version_selector_path() -> Result<PathBuf, LabeledError> {
    get_existing_path(
        &vec![
            "UnrealVersionSelector.exe",
            "C:/Program Files (x86)/Epic Games/Launcher/Engine/Binaries/Win64/UnrealVersionSelector.exe",
            "C:/Program Files/Epic Games/Launcher/Engine/Binaries/Win64/UnrealVersionSelector.exe",
        ],
        "UnrealVersionSelector.exe not found",
    )
}

#[cfg(not(target_os = "windows"))]
pub fn get_unreal_version_selector_path() -> Result<PathBuf, LabeledError> {
    get_existing_path(
        &vec!["UnrealVersionSelector"],
        "UnrealVersionSelector not found",
    )
}
