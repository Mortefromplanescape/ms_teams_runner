pub const URL: &str = "https://teams.live.com/v2";

// Генерация путей через макросы
macro_rules! generate_paths {
    ($( ($folder:expr, $exe:expr) ),* ) => {
        [
            $(
                concat!(r"%PROGRAMFILES(x86)%\", $folder, r"\", $exe),
                concat!(r"%PROGRAMFILES%\", $folder, r"\", $exe),
                concat!(r"%LOCALAPPDATA%\", $folder, r"\", $exe),
            )*
        ]
    };
}

pub const BROWSER_PATHS: [&'static str; 30] = generate_paths!(
    ("Google\\Chrome\\Application", "chrome.exe"),
    ("Microsoft\\Edge\\Application", "msedge.exe"),
    ("Chromium\\Application", "chrome.exe"),
    ("Vivaldi\\Application", "vivaldi.exe"),
    ("BraveSoftware\\Brave-Browser\\Application", "brave.exe"),
    ("Yandex\\YandexBrowser\\Application", "browser.exe"),
    ("Maxthon", "Maxthon.exe"),
    ("Supermium", "chrome.exe"),
    ("Microsoft\\Edge Beta\\Application", "msedge.exe"),
    ("Microsoft\\Edge Dev\\Application", "msedge.exe")
);