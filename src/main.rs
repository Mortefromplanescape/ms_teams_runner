#![windows_subsystem = "windows"]

use std::process::Command;

fn main() {
    Command::new(r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe")
        .arg("--app=https://teams.live.com/v2")
        .spawn()
        .unwrap();
}