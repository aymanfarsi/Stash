#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{cell::RefCell, rc::Rc};

use eframe::{icon_data::from_png_bytes, Theme};
use egui::ViewportBuilder;
use stash::{app::StashApp, enums::TrayMessage, utils::load_icon};
use tokio::runtime::Runtime;
use tray_icon::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    TrayIconBuilder,
};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let rt = Runtime::new().expect("Unable to create Runtime");
    let _enter = rt.enter();

    let menu = Menu::new();
    menu.append_items(&[
        &MenuItem::with_id(
            TrayMessage::ShowHide.to_menu_id(),
            TrayMessage::ShowHide.to_str(),
            true,
            None,
        ),
        &PredefinedMenuItem::separator(),
        // &MenuItem::with_id(
        //     TrayMessage::About.to_menu_id(),
        //     TrayMessage::About.to_str(),
        //     true,
        //     None,
        // ),
        &MenuItem::with_id(
            TrayMessage::Quit.to_menu_id(),
            TrayMessage::Quit.to_str(),
            true,
            None,
        ),
    ])
    .expect("Failed to append items");

    let icon = load_icon("assets/app-icon.png");

    #[cfg(target_os = "linux")]
    std::thread::spawn(|| {
        use tray_icon::menu::Menu;

        gtk::init().unwrap();
        let _tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_icon(icon.clone())
            .build()
            .unwrap();

        gtk::main();
    });

    #[cfg(not(target_os = "linux"))]
    let mut _tray_icon = Rc::new(RefCell::new(None));
    #[cfg(not(target_os = "linux"))]
    let tray_c = _tray_icon.clone();

    let min_size = [320.0, 240.0];
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(min_size)
            .with_min_inner_size(min_size)
            .with_decorations(true)
            .with_transparent(false)
            .with_close_button(true)
            .with_maximize_button(false)
            .with_minimize_button(true)
            .with_titlebar_buttons_shown(true)
            .with_drag_and_drop(true)
            .with_active(true)
            .with_resizable(true)
            .with_taskbar(true)
            .with_visible(true)
            .with_icon(
                from_png_bytes(include_bytes!("../assets/app-icon.png"))
                    .expect("Failed to load icon"),
            )
            .with_app_id("io.github.aymanfarsi.stash"),
        default_theme: Theme::Dark,
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Stash",
        options,
        Box::new(move |_cc| {
            #[cfg(not(target_os = "linux"))]
            {
                tray_c.borrow_mut().replace(
                    TrayIconBuilder::new()
                        .with_icon(icon)
                        .with_menu(Box::new(menu))
                        .build()
                        .unwrap(),
                );
            }
            Box::<StashApp>::default()
        }),
    )
}
