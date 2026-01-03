#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("src/icons/app.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {
    // Do nothing on non-Windows platforms
}
