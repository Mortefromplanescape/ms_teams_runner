use winapi::um::winuser::MessageBoxW;
use std::ptr;

pub fn show_error(msg: &str) {
    let wide_msg = crate::utils::strings::to_wide(msg);
    let wide_title = crate::utils::strings::to_wide("Ошибка");
    
    unsafe {
        MessageBoxW(
            ptr::null_mut(),
            wide_msg.as_ptr(),
            wide_title.as_ptr(),
            0x00000010,
        );
    }
}

pub fn last_error() -> String {
    let error_code = unsafe { winapi::um::errhandlingapi::GetLastError() };
    format!("Код ошибки: {}", error_code)
}