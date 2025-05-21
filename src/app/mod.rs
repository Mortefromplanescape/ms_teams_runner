use crate::constants::{BROWSER_PATHS, URL};
use crate::utils::{files, strings};
use crate::process;

pub fn run() -> Result<(), String> {
    for path in &BROWSER_PATHS {
        let expanded_path = shellexpand::env(path)
            .map_err(|e| e.to_string())?
            .to_string();

        let wide_path = strings::to_wide(&expanded_path);
        
        if files::file_exists(&wide_path) {
            return process::execute_browser(&expanded_path, URL);
        }
    }
    
    Err("В системе не найден браузер на движке Blink (семейство Chromium)!".into())
}