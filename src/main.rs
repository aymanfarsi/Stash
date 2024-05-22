#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::runtime::Runtime;

use stash::utils::{run_first_error_app::run_first_error_app, run_main_app::run_main_app};

fn check_env() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("Stash is only supported on Windows for now!".to_string())
    }
}

fn main() -> Result<(), eframe::Error> {
    let rt = Runtime::new().expect("Unable to create Runtime");
    let _enter = rt.enter();

    match check_env() {
        Ok(_) => run_main_app(),
        Err(e) => run_first_error_app(e),
    }
}
