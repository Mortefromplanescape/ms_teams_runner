use std::path::Path;
use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
use winapi::um::winbase::{CREATE_NO_WINDOW, CREATE_UNICODE_ENVIRONMENT};
use std::ptr;

pub fn launch_browser(path: &Path) -> Result<(), String> {
    let url = "https://web.telegram.org/a/";
    let command = format!("\"{}\" --app=\"{}\"", path.display(), url);
    let mut command_line = to_wide(&command);
    
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
        Err(format!("Launch error: {}", super::error::last_error()))
    }
}

pub fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}