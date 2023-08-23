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

pub fn playlists_path() -> Option<PathBuf> {
    if let Some(mut local_data_path) = local_data_dir() {
        local_data_path.push("data.toml");
        if !local_data_path.exists() && fs::File::create(&local_data_path).is_err() {
            return None;
        }
        Some(local_data_path)
    } else {
        None
    }
}
