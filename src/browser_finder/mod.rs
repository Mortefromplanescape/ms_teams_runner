use std::path::PathBuf;
use walkdir::WalkDir;
use crate::utils::process;

const BROWSER_EXES: &[&str] = &[
    "chrome.exe",
    "msedge.exe",
    "vivaldi.exe", 
    "brave.exe",
    "browser.exe",
    "Maxthon.exe"
];

const SEARCH_DIRS: &[&str] = &[
    r"${PROGRAMFILES(x86)}",
    r"${PROGRAMFILES}",
    r"${LOCALAPPDATA}",
    r"${APPDATA}"
];

pub fn find_browser() -> Result<(), String> {
    let search_paths = SEARCH_DIRS
        .iter()
        .filter_map(|dir| {
            let expanded = shellexpand::full(dir)
                .map_err(|e| eprintln!("Ошибка раскрытия {}: {}", dir, e))
                .ok()?;
                
            let path = PathBuf::from(expanded.as_ref());
            // println!("Раскрытый путь: {}", path.display());
            path.canonicalize().ok()
        })
        .collect::<Vec<_>>();

    for exe in BROWSER_EXES {
        // println!("Поиск: {}", exe);
        if let Some(path) = search_in_dirs(&search_paths, exe) {
            // println!("Найден браузер: {}", path.display());
            return process::launch_browser(&path);
        }
    }
    Err("Браузеры не найдены!".into())
}

fn search_in_dirs(dirs: &[PathBuf], target: &str) -> Option<PathBuf> {
    for dir in dirs {
        // println!("Сканируем: {}", dir.display());
        
        if !dir.exists() {
            // println!("Директория не существует: {}", dir.display());
            continue;
        }

        for entry in WalkDir::new(dir)
            .max_depth(5)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) 
        {
            if entry.file_name().to_string_lossy().eq_ignore_ascii_case(target) {
                return Some(entry.into_path().canonicalize().ok()?);
            }
        }
    }
    None
}