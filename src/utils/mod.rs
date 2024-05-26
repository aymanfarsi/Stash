use std::{fs, process::Command};

use directories::UserDirs;

#[cfg(target_os = "windows")]
use {std::os::windows::process::CommandExt, winapi::um::winbase};

use self::enums::OpenLocationType;

pub mod enums;
pub mod run_first_error_app;
pub mod run_main_app;

pub fn calc_btn_size_from_text(text: &str) -> f32 {
    text.len() as f32 * 10.0
}

pub fn open_urls(urls: &[String]) {
    for url in urls {
        if let Err(e) = webbrowser::open(url) {
            eprintln!("Failed to open URL: {} with error: {:?}", url, e);
        }
    }
}

pub fn backup_bookmarks() {
    let dirs = UserDirs::new().expect("Failed to get user directories");
    let document_dir = dirs
        .document_dir()
        .expect("Failed to get documents directory")
        .join("stash");

    let bookmarks_file = document_dir.join("bookmarks.json");

    if !bookmarks_file.exists() {
        println!("Bookmarks file does not exist, skipping backup");
        return;
    }

    let backup_dir = document_dir.join("backups");
    fs::create_dir_all(&backup_dir).expect("Failed to create backup directory");

    let backup_file = backup_dir.join(format!(
        "bookmarks_{}.json",
        chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
    ));

    fs::copy(&bookmarks_file, backup_file).expect("Failed to copy bookmarks file");

    open_file_location(OpenLocationType::Custom(
        backup_dir
            .to_str()
            .expect("Failed to convert path to string")
            .to_string(),
    ));
}

pub fn open_file_location(location: OpenLocationType) {
    let dirs = UserDirs::new().expect("Failed to get user directories");
    let document_dir = dirs
        .document_dir()
        .expect("Failed to get documents directory")
        .join("stash");

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(match location {
                OpenLocationType::Documents => document_dir
                    .to_str()
                    .expect("Failed to convert path to string")
                    .to_string(),
                OpenLocationType::Custom(file_path) => file_path,
            })
            .creation_flags(winbase::CREATE_NO_WINDOW)
            .spawn()
            .expect("Failed to open file location");
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(match location {
                OpenLocationType::Documents => document_dir
                    .to_str()
                    .expect("Failed to convert path to string")
                    .to_string(),
                OpenLocationType::Custom(file_path) => file_path,
            })
            .spawn()
            .expect("Failed to open file location");
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(match location {
                OpenLocationType::Documents => document_dir
                    .to_str()
                    .expect("Failed to convert path to string")
                    .to_string(),
                OpenLocationType::Custom(file_path) => file_path,
            })
            .spawn()
            .expect("Failed to open file location");
    }
}
