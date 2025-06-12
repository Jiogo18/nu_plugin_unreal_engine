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
