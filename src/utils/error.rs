use winapi::um::winuser::MessageBoxW;
use std::ptr;
use crate::utils::process;

pub fn show_error(msg: &str) {
    let wide_msg = process::to_wide(msg);
    let wide_title = process::to_wide("Error");
    
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
    format!("Error code: {}", error_code)
}