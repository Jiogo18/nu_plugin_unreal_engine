use std::path::PathBuf;

use nu_protocol::LabeledError;

/**
 * Returns the path to the .uproject file in the current directory
 */
pub fn find_uproject() -> Result<PathBuf, LabeledError> {
    let current_dir = std::env::current_dir().map_err(|e| {
        LabeledError::new(format!(
            "Failed to get current directory: {}",
            e.to_string()
        ))
    })?;
    
    let uproject_path = current_dir.join(current_dir.file_name().unwrap()).join(".uproject");
    
    if uproject_path.exists() {
        Ok(uproject_path)
    } else {
        // Find with any name
        let uproject_path = current_dir.join(".uproject");

        Err(LabeledError::new(format!(
            "Failed to find .uproject file in current directory: {}",
            uproject_path.to_str().unwrap()
        )))
    }
}
