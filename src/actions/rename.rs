use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

// main function
pub fn main(show_path: &str, new_name: Option<&str>) {
    let show_dir = Path::new(show_path);
    if !show_dir.is_dir() {
        eprintln!("Error: Provided path is not a directory");
        return;
    }

    let show_dir = if let Some(new_name) = new_name {
        match rename_show_directory(show_dir, new_name) {
            Ok(new_path) => new_path,
            Err(err) => {
                eprintln!("Error: {}", err);
                return;
            }
        }
    } else {
        show_dir.to_path_buf()
    };

    rename_seasons(&show_dir);
}

// rename entire show
fn rename_show_directory(show_dir: &Path, new_name: &str) -> Result<PathBuf, String> {
    let parent_dir = show_dir
        .parent()
        .ok_or("Failed to get parent directory".to_string())?;
    let new_show_path = parent_dir.join(new_name);

    fs::rename(show_dir, &new_show_path)
        .map_err(|err| format!("Failed to rename show directory: {}", err))?;

    Ok(new_show_path)
}

// rename seasons
fn rename_seasons(show_dir: &Path) {
    let mut season_dirs: Vec<_> = show_dir
        .read_dir()
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .collect();

    season_dirs.sort_by_key(|entry| entry.path());

    for (season_idx, season_entry) in season_dirs.iter().enumerate() {
        let season_num = season_idx + 1;
        let new_season_name = format!("Season {:02}", season_num);
        let new_season_path = show_dir.join(&new_season_name);

        fs::rename(season_entry.path(), &new_season_path)
            .expect("Failed to rename season directory");

        rename_episodes(&new_season_path, season_num);
    }
}

// rename episodes
fn rename_episodes(season_path: &Path, season_num: usize) {
    let mut episodes: Vec<_> = season_path
        .read_dir()
        .expect("Failed to read season directory")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .collect();

    episodes.sort_by_key(|entry| entry.path());

    for (episode_idx, episode_entry) in episodes.iter().enumerate() {
        let episode_num = episode_idx + 1;
        let path = episode_entry.path();
        let ext = path.extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        let new_episode_name = format!("S{:02}E{:02}.{}", season_num, episode_num, ext);
        let new_episode_path = season_path.join(&new_episode_name);

        fs::rename(&path, &new_episode_path)
            .expect("Failed to rename episode file");
    }
}
