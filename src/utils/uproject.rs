use std::path::PathBuf;

use nu_plugin::EngineInterface;
use nu_protocol::LabeledError;
use regex::Regex;

/**
 * Returns the path to the .uproject file in the current directory
 */
pub fn find_uproject(engine: &EngineInterface) -> Result<PathBuf, LabeledError> {
    let current_dir = PathBuf::from(engine.get_current_dir()?);
    current_dir
        .read_dir()
        .map_err(|e| {
            LabeledError::new(format!(
                "Failed to read current directory: {}",
                e.to_string()
            ))
        })?
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.file_name().to_str().unwrap().ends_with(".uproject"))
        .map(|entry| entry.path())
        .ok_or(LabeledError::new(format!(
            "Failed to find .uproject file in current directory: {}",
            current_dir.to_str().unwrap()
        )))
}

pub fn uproject_from_arg_or_current_dir(
    engine: &EngineInterface,
    uproject_path: Option<nu_protocol::Spanned<String>>,
) -> Result<PathBuf, LabeledError> {
    if let Some(uproject_path) = uproject_path {
        return Ok(PathBuf::from(uproject_path.item));
    } else {
        return find_uproject(&engine);
    }
}

pub enum IDE {
    Unknown,
    VisualStudio,
    VisualStudioCode,
    Rider,
}

impl std::fmt::Display for IDE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IDE::Unknown => write!(f, "Unknown"),
            IDE::VisualStudio => write!(f, "VisualStudio"),
            IDE::VisualStudioCode => write!(f, "VisualStudioCode"),
            IDE::Rider => write!(f, "Rider"),
        }
    }
}

pub struct UProject {
    pub uproject_path: PathBuf,
    pub name: String,
    pub ide: IDE,
    pub unreal_engine_path: Option<PathBuf>,
}

// Assume uproject_path is valid
// Get the Engine path of a project
fn get_unreal_engine_path(uproject: &UProject) -> Result<Option<PathBuf>, LabeledError> {
    // Saved/Config/WindowsEditor/EditorPerProjectUserSettings.ini contains Directories2.Project, but not updated when moving UE
    // Intermediate/PipInstall/Lib/site-packages/plugin_site_package.pth is updated with any IDE
    const PIP_INSTALL_SITE_PACKAGE: &str =
        "Intermediate/PipInstall/Lib/site-packages/plugin_site_package.pth";
    let pip_install_site_package = uproject
        .uproject_path
        .parent()
        .unwrap()
        .join(PIP_INSTALL_SITE_PACKAGE);
    let sln_path = uproject
        .uproject_path
        .parent()
        .unwrap()
        .join(uproject.name.to_owned() + ".sln");

    let engine_path = if sln_path.exists() {
        // open Vestige.sln | parse --regex "Project.+\"UnrealBuildTool\", *(.+), .+" | get capture0.0
        let content = std::fs::read_to_string(&sln_path).map_err(|e| {
            LabeledError::new(format!(
                "Failed to read {}: {}",
                sln_path.display(),
                e.to_string()
            ))
        })?;
        let re = Regex::new(r#"Project.+"UnrealBuildTool", *\"(.+)Engine[\\/]Source[\\/]Programs[\\/]UnrealBuildTool[\\/]UnrealBuildTool.csproj\", .+"#).unwrap();

        let first_match: Option<String> = re
            .captures_iter(&content)
            .map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .flatten()
            .next();

        if first_match.is_none() {
            return Err(LabeledError::new(format!(
                "Failed to find Engine path in {}",
                sln_path.display()
            )));
        }

        let engine_relative_path = PathBuf::from(&first_match.unwrap());

        uproject
            .uproject_path
            .parent()
            .unwrap()
            .join(engine_relative_path)
    } else if pip_install_site_package.exists() {
        let content = std::fs::read_to_string(pip_install_site_package).map_err(|e| {
            LabeledError::new(format!(
                "Failed to read {}: {}",
                PIP_INSTALL_SITE_PACKAGE,
                e.to_string()
            ))
        })?;

        const CONTENT_END_PATH: &str =
            "Engine/Plugins/Runtime/USDCore/Content/Python/Lib/Win64/site-packages";
        let content_split = content.split(CONTENT_END_PATH).collect::<Vec<&str>>();
        assert!(content_split.len() == 2);
        PathBuf::from(content_split[0])
    } else {
        println!(
            "Warning: Failed to find {} or {} to get Engine path",
            sln_path.display(),
            PIP_INSTALL_SITE_PACKAGE
        );
        return Ok(None);
    };

    if !engine_path.exists() {
        println!("Warning: Engine {} does not exist", engine_path.display());
    }
    return Ok(Some(engine_path));
}

impl UProject {
    pub fn from_path(uproject_path: &PathBuf) -> Result<UProject, LabeledError> {
        if !uproject_path.exists() {
            return Err(LabeledError::new(format!(
                "Failed to find .uproject file in current directory: {}",
                uproject_path.to_str().unwrap()
            )));
        }
        let uproject_name = uproject_path.file_stem().unwrap().to_str().unwrap();

        // Check for uproject_path replace .uproject by .sln
        let sln_path = uproject_path
            .parent()
            .unwrap()
            .join(uproject_name.to_owned() + ".sln");
        let code_workspace_path = uproject_path
            .parent()
            .unwrap()
            .join(uproject_name.to_owned() + ".code-workspace");

        let ide = if sln_path.exists() {
            if uproject_path.parent().unwrap().join(".idea").exists() {
                IDE::Rider
            } else {
                IDE::VisualStudio
            }
        } else if code_workspace_path.exists() {
            IDE::VisualStudioCode
        } else {
            IDE::Unknown
        };

        let mut uproject = UProject {
            uproject_path: uproject_path.clone(),
            name: uproject_name.to_string(),
            ide,
            unreal_engine_path: None,
        };

        uproject.unreal_engine_path = get_unreal_engine_path(&uproject)?;

        return Ok(uproject);
    }

    pub fn get_unreal_engine_path_str(&self) -> Result<&PathBuf, LabeledError> {
        match &self.unreal_engine_path {
            Some(path) => Ok(&path),
            None => Err(LabeledError::new(
                "Unreal Engine path not found".to_string(),
            )),
        }
    }
}
