use std::fs;
use std::path::Path;
use std::ffi::OsStr;

// main function
pub fn main(show_path: &str) {
    let show_dir = Path::new(show_path);
    if !show_dir.is_dir() {
        eprintln!("Error: Provided path is not a directory");
        return;
    }

    rename_seasons(show_dir);
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
