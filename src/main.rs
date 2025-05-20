#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

use core::panic::PanicInfo;
use core::ptr;
use windows_sys::Win32::{
    Foundation::CloseHandle,
    Security::SECURITY_ATTRIBUTES,
    System::Threading::{
        CREATE_NO_WINDOW, CREATE_UNICODE_ENVIRONMENT, PROCESS_INFORMATION, STARTUPINFOW,
    },
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn wide_string(s: &str) -> [u16; 260] {
    let mut buffer = [0u16; 260];
    let mut i = 0;
    for c in s.encode_utf16() {
        if i >= 259 {
            break;
        }
        buffer[i] = c;
        i += 1;
    }
    buffer[i] = 0;
    buffer
}

#[no_mangle]
pub extern "system" fn mainCRTStartup() {
    let command_line = wide_string(
        r#""C:\Program Files (x86)\Google\Chrome\Application\chrome.exe" --app=https://teams.live.com/v2"#
    );

    unsafe {
        let mut si: STARTUPINFOW = core::mem::zeroed();
        si.cb = core::mem::size_of::<STARTUPINFOW>() as u32;
        
        let mut pi: PROCESS_INFORMATION = core::mem::zeroed();
        
        let success = windows_sys::Win32::System::Threading::CreateProcessW(
            ptr::null(),
            command_line.as_ptr() as *mut _,
            ptr::null::<SECURITY_ATTRIBUTES>(), // lpProcessAttributes
            ptr::null::<SECURITY_ATTRIBUTES>(),  // lpThreadAttributes
            0,                                   // bInheritHandles
            CREATE_NO_WINDOW | CREATE_UNICODE_ENVIRONMENT,
            ptr::null(),                        // lpEnvironment
            ptr::null(),                        // lpCurrentDirectory
            &si,
            &mut pi,
        );

        if success != 0 {
            CloseHandle(pi.hProcess);
            CloseHandle(pi.hThread);
        }
    }
}