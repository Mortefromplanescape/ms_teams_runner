mod browser_finder;
mod utils;

use crate::browser_finder::{find_browser, show_error};

fn main() {
    if let Err(e) = find_browser() {
        show_error(&e);
    }
}