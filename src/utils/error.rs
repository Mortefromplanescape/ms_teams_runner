use winapi::um::winuser::MessageBoxW;
use std::ptr;
use crate::utils;

pub fn show_error(msg: &str) {
    let wide_msg = utils::to_wide(msg);
    let wide_title = utils::to_wide("Error");
    
    unsafe {
        MessageBoxW(
            ptr::null_mut(),
            wide_msg.as_ptr(),
            wide_title.as_ptr(),
            0x00000010,
        );
    }
}