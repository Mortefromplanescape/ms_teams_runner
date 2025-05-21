use winapi::um::fileapi::GetFileAttributesW;

pub fn file_exists(path: &[u16]) -> bool {
    !path.is_empty() && path.last() == Some(&0) && 
    unsafe { GetFileAttributesW(path.as_ptr()) != 0xFFFFFFFF }
}