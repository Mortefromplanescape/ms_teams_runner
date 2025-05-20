#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

use core::panic::PanicInfo;
use core::ptr;
use windows_sys::Win32::{
    Foundation::{CloseHandle, GetLastError, INVALID_HANDLE_VALUE},
    Security::SECURITY_ATTRIBUTES,
    System::{
        Diagnostics::Debug::FormatMessageW,
        Environment::GetEnvironmentVariableW,
        Threading::{
            CREATE_NO_WINDOW, CREATE_UNICODE_ENVIRONMENT, PROCESS_INFORMATION, STARTUPINFOW,
        },
    },
    Storage::FileSystem::CreateFileW,
    UI::WindowsAndMessaging::MessageBoxW,
};

// Явные реализации для линковки
#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let c = c as u8;
    for i in 0..n {
        *s.add(i) = c;
    }
    s
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        *dest.add(i) = *src.add(i);
    }
    dest
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut msg_buffer = [0u16; 512];
    let msg = info.message()
        .as_str()  // Прямое преобразование в Option<&str>
        .unwrap_or("Unknown panic");
    
    wide_string(msg, &mut msg_buffer);
    
    unsafe {
        MessageBoxW(
            0,
            msg_buffer.as_ptr(),
            wide_string_const("Critical Error"),
            0x00000010,
        );
    }
    loop {}
}

fn wide_string<const N: usize>(s: &str, buffer: &mut [u16; N]) {
    let mut i = 0;
    for c in s.encode_utf16() {
        if i >= N - 1 {
            break;
        }
        buffer[i] = c;
        i += 1;
    }
    buffer[i] = 0;
}

fn wide_string_const(s: &str) -> *const u16 {
    let mut buffer = [0u16; 256];
    let mut i = 0;
    for c in s.encode_utf16() {
        if i >= 255 {
            break;
        }
        buffer[i] = c;
        i += 1;
    }
    buffer[i] = 0;
    buffer.as_ptr()
}

fn find_chrome() -> Option<[u16; 520]> {
    let paths = [
        r"Google\Chrome\Application\chrome.exe",
        r"Chromium\Application\chrome.exe",
    ];

    let mut search_paths = [[0u16; 260]; 3];
    let mut path_count = 0;

    unsafe {
        if get_env_var("ProgramFiles", &mut search_paths[0]) > 0 {
            path_count += 1;
        }
        if get_env_var("ProgramFiles(x86)", &mut search_paths[1]) > 0 {
            path_count += 1;
        }
        if get_env_var("LocalAppData", &mut search_paths[2]) > 0 {
            path_count += 1;
        }
    }

    for path in &search_paths[..path_count] {
        for chrome_path in &paths {
            let mut full_path = [0u16; 520];
            concat_wide(path, chrome_path, &mut full_path);
            
            if unsafe { file_exists(&full_path) } {
                return Some(full_path);
            }
        }
    }
    
    None
}

fn concat_wide(part1: &[u16], part2: &str, output: &mut [u16]) {
    let mut i = 0;
    
    for &c in part1.iter().take_while(|&&c| c != 0) {
        if i >= output.len() - 1 { break; }
        output[i] = c;
        i += 1;
    }
    
    if i > 0 && output[i-1] != b'\\' as u16 {
        output[i] = b'\\' as u16;
        i += 1;
    }
    
    for c in part2.encode_utf16() {
        if i >= output.len() - 1 { break; }
        output[i] = c;
        i += 1;
    }
    output[i] = 0;
}

unsafe fn get_env_var(name: &str, buffer: &mut [u16]) -> u32 {
    let mut name_buf = [0u16; 256];
    wide_string(name, &mut name_buf);
    
    GetEnvironmentVariableW(
        name_buf.as_ptr(),
        buffer.as_mut_ptr(),
        buffer.len() as u32,
    )
}

unsafe fn file_exists(path: &[u16]) -> bool {
    let handle = CreateFileW(
        path.as_ptr(),
        0x80000000,
        1,
        ptr::null(),
        3,
        0,
        0,
    );
    
    if handle != INVALID_HANDLE_VALUE {
        CloseHandle(handle);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "system" fn mainCRTStartup() {
    let chrome_path = match find_chrome() {
        Some(path) => path,
        None => {
            unsafe {
                MessageBoxW(
                    0,
                    wide_string_const("       MS Edge, Chrome or Chromium should be installed to continue..."),
                    wide_string_const("Error!\n"),
                    0x00000010,
                );
            }
            return;
        }
    };

    let mut full_cmd = [0u16; 2048];
    concat_wide(&chrome_path, " --app=https://teams.live.com/v2", &mut full_cmd);

    unsafe {
        let mut si: STARTUPINFOW = core::mem::zeroed();
        si.cb = core::mem::size_of::<STARTUPINFOW>() as u32;
        
        let mut pi: PROCESS_INFORMATION = core::mem::zeroed();
        
        let success = windows_sys::Win32::System::Threading::CreateProcessW(
            ptr::null(),
            full_cmd.as_ptr() as *mut _,
            ptr::null::<SECURITY_ATTRIBUTES>(),
            ptr::null::<SECURITY_ATTRIBUTES>(),
            0,
            CREATE_NO_WINDOW | CREATE_UNICODE_ENVIRONMENT,
            ptr::null(),
            ptr::null(),
            &si,
            &mut pi,
        );

        if success == 0 {
            let mut buffer = [0u16; 512];
            let len = FormatMessageW(
                0x00000200 | 0x00001000,
                ptr::null(),
                GetLastError(),
                0x0409,
                buffer.as_mut_ptr(),
                buffer.len() as u32,
                ptr::null_mut(),
            );
            
            if len > 0 {
                MessageBoxW(
                    0,
                    buffer.as_ptr(),
                    wide_string_const("Launch Error"),
                    0x00000010,
                );
            }
        }
    }
}