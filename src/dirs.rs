use std::{fs, path::PathBuf};

use directories::ProjectDirs;

pub fn local_data_dir() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Oxide", "Melody") {
        let local_data_dir = proj_dirs.data_local_dir();
        if !local_data_dir.exists() && fs::create_dir_all(local_data_dir).is_err() {
            return None;
        }
        return Some(local_data_dir.into());
    }
    None
}

pub fn config_dir() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Oxide", "Melody") {
        let local_data_dir = proj_dirs.config_dir();
        if !local_data_dir.exists() && fs::create_dir_all(local_data_dir).is_err() {
            return None;
        }
        return Some(local_data_dir.into());
    }
    None
}
