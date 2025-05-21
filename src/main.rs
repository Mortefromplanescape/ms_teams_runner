mod app;
mod constants;
mod process;
mod utils;

use crate::app::run;
use crate::utils::error::show_error;

fn main() {
    if let Err(e) = run() {
        show_error(&e);
    }
}