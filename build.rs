#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon.ico")
        .set("InternalName", "MS_TEAMS_RUNNER")
        .set("ProductName", "Microsoft Teams Runner");
    res.compile().unwrap();
}		

#[cfg(unix)]
fn main() {
}