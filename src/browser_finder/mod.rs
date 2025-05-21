use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::utils::process;

const BROWSER_EXES: &[&str] = &[
    "chrome.exe",
    "msedge.exe",
    "vivaldi.exe",
    "brave.exe",
    "browser.exe",
    "Maxthon.exe",
];

const SEARCH_DIRS: &[&str] = &[
    r"%PROGRAMFILES(x86)%",
    r"%PROGRAMFILES%",
    r"%LOCALAPPDATA%",
];

pub fn find_browser() -> Result<(), String> {
    let search_paths = SEARCH_DIRS
        .iter()
        .filter_map(|dir| expand_env_var(dir))
        .collect::<Vec<_>>();

    for exe in BROWSER_EXES {
        if let Some(path) = search_in_dirs(&search_paths, exe) {
            return process::launch_browser(&path);
        }
    }

    Err("No supported browsers found!".into())
}

fn expand_env_var(dir: &str) -> Option<PathBuf> {
    shellexpand::full(dir)
        .ok()
        .map(|expanded| PathBuf::from(expanded.into_owned()))
}

fn search_in_dirs(dirs: &[PathBuf], target: &str) -> Option<PathBuf> {
    for dir in dirs {
        if !dir.exists() { continue; }
        
        for entry in WalkDir::new(dir)
            .min_depth(1)
            .max_depth(5)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_name() == OsString::from(target) {
                return Some(entry.into_path());
            }
        }
    }
    None
}

pub fn show_error(msg: &str) {
    utils::error::show_error(msg);
}