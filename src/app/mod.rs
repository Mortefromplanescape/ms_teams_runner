use crate::constants::{BROWSER_PATHS, URL};
use crate::utils::{files, strings};
use crate::process;

// pub fn run() -> Result<(), String> {
//     for path in &BROWSER_PATHS {
//         let expanded_path = shellexpand::env(path)
//             .map_err(|e| e.to_string())?
//             .to_string();

//         let wide_path = strings::to_wide(&expanded_path);
        
//         if files::file_exists(&wide_path) {
//             return process::execute_browser(&expanded_path, URL);
//         }
//     }
    
//     Err("В системе не найден браузер на движке Blink (семейство Chromium)!".into())
// }

pub fn run() -> Result<(), String> {
    for path in &BROWSER_PATHS {
        // Используем full_env_with_context для корректного раскрытия переменных
        let expanded_path = shellexpand::full(path)
            .map_err(|e| format!("Ошибка раскрытия переменных: {}", e))?
            .into_owned();
        
        println!("[DEBUG] Раскрытый путь: {}", expanded_path);
        
        let wide_path = strings::to_wide(&expanded_path);
        
        if files::file_exists(&wide_path) {
            println!("[SUCCESS] Найден браузер: {}", expanded_path);
            return process::execute_browser(&expanded_path, URL);
        }
    }
    
    Err("Браузер не найден".into())
}