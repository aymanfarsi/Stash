#[cfg(target_os = "windows")]
fn main() {
    extern crate winres;

    let mut res = winres::WindowsResource::new();

    res.set_icon_with_id("assets/stash.ico", "1");

    res.compile().unwrap();
}

#[cfg(target_os = "linux")]
fn main() {
    println!("cargo:rerun-if-changed=assets/stash.ico");
}

#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rerun-if-changed=assets/stash.ico");
}
