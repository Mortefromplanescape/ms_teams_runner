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

static mut INITIALIZED: bool = false;

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
    
    let location_info = info.location().map(|loc| {
        let mut buf = [0u16; 150];
        let mut i = 0;
        
        for c in "File: ".encode_utf16() {
            if i >= 149 { break; }
            buf[i] = c;
            i += 1;
        }
        
        for c in loc.file().encode_utf16() {
            if i >= 149 { break; }
            buf[i] = c;
            i += 1;
        }
        
        for c in ", Line: ".encode_utf16() {
            if i >= 149 { break; }
            buf[i] = c;
            i += 1;
        }
        
        let mut line_buf = [0u8; 12];
        let line_str = u32_to_str(loc.line(), &mut line_buf);
        for c in line_str.encode_utf16() {
            if i >= 149 { break; }
            buf[i] = c;
            i += 1;
        }
        
        buf[i] = 0;
        buf
    }).unwrap_or([0; 150]);

    let payload = info.message()
        .as_str()
        .unwrap_or("Unspecified panic");

    let mut i = 0;
    for c in payload.encode_utf16() {
        if i >= 510 { break; }
        msg_buffer[i] = c;
        i += 1;
    }
    
    for c in "\n".encode_utf16() {
        if i >= 510 { break; }
        msg_buffer[i] = c;
        i += 1;
    }
    
    for &c in &location_info {
        if c == 0 || i >= 510 { break; }
        msg_buffer[i] = c;
        i += 1;
    }
    msg_buffer[i] = 0;

    unsafe {
        MessageBoxW(
            0,
            msg_buffer.as_ptr(),
            wide_string_const("CRITICAL ERROR"),
            0x00000010 | 0x00010000,
        );
        
        if !INITIALIZED {
            windows_sys::Win32::System::Threading::ExitProcess(1);
        }
    }
    
    loop {}
}

fn wide_string<const N: usize>(s: &str, buffer: &mut [u16; N]) {
    let mut i = 0;
    for c in s.encode_utf16() {
        if i >= N - 1 { break; }
        buffer[i] = c;
        i += 1;
    }
    buffer[i] = 0;
}

fn wide_string_const(s: &str) -> *const u16 {
    let mut buffer = [0u16; 256];
    let mut i = 0;
    for c in s.encode_utf16() {
        if i >= 255 { break; }
        buffer[i] = c;
        i += 1;
    }
    buffer[i] = 0;
    buffer.as_ptr()
}

fn find_browser() -> Option<[u16; 520]> {
    let browsers = [
        (r"Google\Chrome\Application\chrome.exe", &["ProgramFiles", "ProgramFiles(x86)", "LocalAppData"] as &[&str]),
        (r"Microsoft\Edge\Application\msedge.exe", &["ProgramFiles(x86)", "ProgramFiles"]),
        (r"Chromium\Application\chrome.exe", &["LocalAppData"]),
        (r"Microsoft\Edge Beta\Application\msedge.exe", &["ProgramFiles(x86)"]),
        (r"Microsoft\Edge Dev\Application\msedge.exe", &["ProgramFiles(x86)"]),
        (r"Mozilla Firefox\firefox.exe", &["ProgramFiles", "ProgramFiles(x86)"])
    ];

    let mut search_paths = [[0u16; 260]; 8];
    let mut path_count = 0;

    unsafe {
        for (_, locations) in &browsers {
            for &var in *locations {
                if path_count >= search_paths.len() { break; }
                
                let mut exists = false;
                for existing in &search_paths[..path_count] {
                    if wide_str_eq(existing, var) {
                        exists = true;
                        break;
                    }
                }
                
                if !exists {
                    let success = get_env_var(var, &mut search_paths[path_count]);
                    if success > 0 && search_paths[path_count][0] != 0 {
                        path_count += 1;
                    }
                }
            }
        }

        let fallback_paths = [
            r"C:\Program Files\",
            r"C:\Program Files (x86)\",
            r"C:\Users\%USERNAME%\AppData\Local\",
        ];

        for path in &fallback_paths {
            if path_count >= search_paths.len() { break; }
            let mut buf = [0u16; 260];
            wide_string(path, &mut buf);
            search_paths[path_count] = buf;
            path_count += 1;
        }
    }

    for (exe_path, _) in &browsers {
        for path in &search_paths[..path_count] {
            let mut full_path = [0u16; 520];
            concat_wide(path, exe_path, &mut full_path);
            
            if unsafe { file_exists(&full_path) } {
                return Some(full_path);
            }
        }
    }
    
    None
}

fn wide_str_eq(a: &[u16], b: &str) -> bool {
    let mut i = 0;
    for (wc, bc) in a.iter().map(|&c| c as u8).zip(b.bytes()) {
        if wc != bc || wc == 0 {
            return i == b.len();
        }
        i += 1;
    }
    false
}

fn concat_wide(part1: &[u16], part2: &str, output: &mut [u16]) {
    if output.is_empty() { return; }
    
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
    if path.is_empty() || path[0] == 0 { return false; }
    
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

fn u32_to_str(n: u32, buffer: &mut [u8]) -> &str {
    let mut i = 0;
    let mut num = n;
    
    if num == 0 {
        buffer[0] = b'0';
        i = 1;
    } else {
        while num > 0 && i < buffer.len() {
            buffer[i] = (num % 10) as u8 + b'0';
            num /= 10;
            i += 1;
        }
        buffer[..i].reverse();
    }
    
    core::str::from_utf8(&buffer[..i]).unwrap_or("")
}

unsafe fn debug_message(msg: &str) {
    let mut msg_buf = [0u16; 512];
    wide_string(msg, &mut msg_buf);
    MessageBoxW(0, msg_buf.as_ptr(), wide_string_const("DEBUG"), 0x40);
}

#[no_mangle]
pub extern "system" fn mainCRTStartup() {
    unsafe { INITIALIZED = true; }
    
    let mut si: STARTUPINFOW = unsafe { core::mem::zeroed() };
    si.cb = core::mem::size_of::<STARTUPINFOW>() as u32;
    
    let browser_path = match find_browser() {
        Some(path) => {
            unsafe { 
                debug_message("Found browser path:");
                let mut path_buf = [0u16; 520];
                ptr::copy_nonoverlapping(path.as_ptr(), path_buf.as_mut_ptr(), path.len());
                MessageBoxW(0, path_buf.as_ptr(), wide_string_const("PATH"), 0x40);
            }
            path
        },
        None => {
            unsafe {
                MessageBoxW(
                    0,
                    wide_string_const("Browsers not found!\nInstall Chrome/Edge/Chromium/Firefox"),
                    wide_string_const("Error"),
                    0x00000010 | 0x00010000,
                );
                windows_sys::Win32::System::Threading::ExitProcess(1);
            }
        }
    };

    let mut full_cmd = [0u16; 2048];
    concat_wide(&browser_path, " --app=https://teams.live.com/v2", &mut full_cmd);

    unsafe {
        debug_message("Launching command:");
        MessageBoxW(0, full_cmd.as_ptr(), wide_string_const("CMD"), 0x40);
        
        let mut pi: PROCESS_INFORMATION = core::mem::zeroed();
        let success = windows_sys::Win32::System::Threading::CreateProcessW(
            browser_path.as_ptr(),
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
            let error = GetLastError();
            let mut buffer = [0u16; 512];
            let len = FormatMessageW(
                0x00000200 | 0x00001000,
                ptr::null(),
                error,
                0x0409,
                buffer.as_mut_ptr(),
                buffer.len() as u32,
                ptr::null_mut(),
            );
            
            if len > 0 {
                MessageBoxW(
                    0,
                    buffer.as_ptr(),
                    wide_string_const("Error"),
                    0x00000010,
                );
            }
        }
        windows_sys::Win32::System::Threading::ExitProcess(0);
    }
}