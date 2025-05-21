use std::ptr;
use winapi::um::winuser::MessageBoxW;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
use winapi::um::winbase::{CREATE_NO_WINDOW, CREATE_UNICODE_ENVIRONMENT};
use winapi::um::fileapi::GetFileAttributesW;

fn main() {
    if let Err(e) = run() {
        show_error(&e);
    }
}

fn run() -> Result<(), String> {
    let browsers = [
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Users\%USERNAME%\AppData\Local\Google\Chrome\Application\chrome.exe",

        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
        r"C:\Users\%USERNAME%\AppData\Local\Microsoft\Edge\Application\msedge.exe",

        r"C:\Program Files (x86)\Chromium\Application\chrome.exe",
        r"C:\Program Files\Chromium\Application\chrome.exe",
        r"C:\Users\%USERNAME%\AppData\Local\Chromium\Application\chrome.exe",

        r"C:\Program Files (x86)\Vivaldi\Application\vivaldi.exe",
        r"C:\Program Files\Vivaldi\Application\vivaldi.exe",
        r"C:\Users\%USERNAME%\AppData\Local\Vivaldi\Application\vivaldi.exe",

        r"C:\Program Files (x86)\BraveSoftware\Brave-Browser\Application\brave.exe",
        r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe",
        r"C:\Users\%USERNAME%\AppData\Local\BraveSoftware\Brave-Browser\Application\brave.exe",

        r"C:\Program Files (x86)\Yandex\YandexBrowser\Application\browser.exe",
        r"C:\Program Files\Yandex\YandexBrowser\Application\browser.exe",
        r"C:\Users\%USERNAME%\AppData\Local\Yandex\YandexBrowser\Application\browser.exe",

        r"C:\Program Files (x86)\Maxthon\Maxthon.exe",
        r"C:\Program Files\Maxthon\Maxthon.exe",
        r"C:\Users\%USERNAME%\AppData\Local\Maxthon\Maxthon.exe",

        r"C:\Program Files (x86)\Supermium\chrome.exe",
        r"C:\Program Files\Supermium\chrome.exe",
        r"C:\Users\%USERNAME%\AppData\Local\Supermium\chrome.exe",

        r"C:\Program Files (x86)\Microsoft\Edge Beta\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge Beta\Application\msedge.exe",
        r"C:\Users\%USERNAME%\AppData\Local\Microsoft\Edge Beta\Application\msedge.exe",

        r"C:\Program Files (x86)\Microsoft\Edge Dev\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge Dev\Application\msedge.exe",
        r"C:\Users\%USERNAME%\AppData\Local\Microsoft\Edge Dev\Application\msedge.exe",
    ];

    let url = "https://teams.live.com/v2";

    // Поиск существующего браузера
    for path in &browsers {
        let expanded_path = shellexpand::env(path).map_err(|e| e.to_string())?;
        let wide_path = to_wide(&expanded_path);
        
        if file_exists(&wide_path) {
            return execute_browser(&wide_path, url);
        }
    }

    Err("Не найден ни один из поддерживаемых браузеров!".into())
}

fn execute_browser(path: &[u16], url: &str) -> Result<(), String> {
    let mut command_line = to_wide(&format!("\"{}\" --app={}", from_wide(path), url));
    
    let mut si: STARTUPINFOW = unsafe { std::mem::zeroed() };
    si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
    
    let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };
    
    let success = unsafe {
        CreateProcessW(
            ptr::null(),
            command_line.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            CREATE_NO_WINDOW | CREATE_UNICODE_ENVIRONMENT,
            ptr::null_mut(),
            ptr::null(),
            &mut si,
            &mut pi,
        )
    };
    
    if success != 0 {
        Ok(())
    } else {
        Err(format!("Ошибка запуска: {}", last_error()))
    }
}

fn file_exists(path: &[u16]) -> bool {
    unsafe { GetFileAttributesW(path.as_ptr()) != 0xFFFFFFFF } // Исправлено условие
}

fn show_error(msg: &str) {
    let wide_msg = to_wide(msg);
    let wide_title = to_wide("Ошибка");
    
    unsafe {
        MessageBoxW(
            ptr::null_mut(),
            wide_msg.as_ptr(),
            wide_title.as_ptr(),
            0x00000010, // MB_ICONERROR
        );
    }
}

fn last_error() -> String {
    let error_code = unsafe { GetLastError() };
    format!("Код ошибки: {}", error_code)
}

// Вспомогательные функции для работы с Wide-строками
fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

fn from_wide(wide: &[u16]) -> String {
    let len = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16_lossy(&wide[..len])
}