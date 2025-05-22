mod browser_finder;
mod utils;

use crate::browser_finder::find_browser;

fn main() {
    if let Err(e) = find_browser() {
        utils::error::show_error(&e);
    }
}