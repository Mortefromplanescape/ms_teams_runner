use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
use winapi::um::winbase::{CREATE_NO_WINDOW, CREATE_UNICODE_ENVIRONMENT};
use std::ptr;
use crate::utils::strings;

pub fn execute_browser(path: &str, url: &str) -> Result<(), String> {
    let command = format!("\"{}\" --app={}", path, url);
    let mut command_line = strings::to_wide(&command);
    
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
        Err(format!("Ошибка запуска: {}", crate::utils::error::last_error()))
    }
}