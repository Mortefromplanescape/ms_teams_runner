pub const URL: &str = "https://teams.live.com/v2";

// Генерация путей через макросы
macro_rules! generate_paths {
    ($( ($folder:expr, $exe:expr) ),* ) => {
        [
            $(
                concat!(r#"${PROGRAMFILES(x86)}\"#, $folder, r"\", $exe),
                concat!(r#"${PROGRAMFILES}\"#, $folder, r"\", $exe),
                concat!(r#"${LOCALAPPDATA}\"#, $folder, r"\", $exe),
            )*
        ]
    };
}

pub const BROWSER_PATHS: [&'static str; 30] = generate_paths!(
    (r#"Google\Chrome\Application"#, "chrome.exe"),
    (r#"Microsoft\Edge\Application"#, "msedge.exe"),
    (r#"Chromium\Application"#, "chrome.exe"),
    (r#"Vivaldi\Application"#, "vivaldi.exe"),
    (r#"BraveSoftware\Brave-Browser\Application"#, "brave.exe"),
    (r#"Yandex\YandexBrowser\Application"#, "browser.exe"),
    (r#"Maxthon"#, "Maxthon.exe"),
    (r#"Supermium"#, "chrome.exe"),
    (r#"Microsoft\Edge Beta\Application"#, "msedge.exe"),
    (r#"Microsoft\Edge Dev\Application"#, "msedge.exe")
);